use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::protocol;
use crate::protocol::PacketBuffer;
use crate::vl1::{Address, ApplicationLayer, Identity, Node, PacketHandlerResult, Peer, MAC};
use crate::vl2::{MulticastGroup, NetworkId};

use zerotier_utils::buffer::OutOfBoundsError;
use zerotier_utils::sync::RMaybeWLockGuard;

/// Handler implementations for VL2_MULTICAST_LIKE and VL2_MULTICAST_GATHER.
///
/// Both controllers and roots will want to handle these, with the latter supporting them for legacy
/// reasons only. Regular nodes may also want to handle them in the future. So, break this out to allow
/// easy code reuse. To integrate call the appropriate method when the appropriate message type is
/// received and pass in a function to check whether specific network/identity combinations should be
/// processed. The GATHER implementation will send reply packets to the source peer.
pub struct MulticastAuthority {
    subscriptions: RwLock<HashMap<(NetworkId, MulticastGroup), Mutex<HashMap<Address, i64>>>>,
}

impl MulticastAuthority {
    pub fn new() -> Self {
        Self { subscriptions: RwLock::new(HashMap::new()) }
    }

    /// Call this every VL2_DEFAULT_MULTICAST_LIKE_EXPIRE (or more frequently) to clean expired multicast subscriptions.
    pub fn clean(&self, time_ticks: i64) {
        let exp_before = time_ticks - protocol::VL2_DEFAULT_MULTICAST_LIKE_EXPIRE;
        let mut empty_subscription_entries = Vec::new();

        for (network_group, subs) in self.subscriptions.read().unwrap().iter() {
            let mut subs = subs.lock().unwrap();
            subs.retain(|_, t| *t > exp_before);
            if subs.is_empty() {
                empty_subscription_entries.push(network_group.clone());
            }
        }

        if !empty_subscription_entries.is_empty() {
            let mut ms = self.subscriptions.write().unwrap();
            for e in empty_subscription_entries.iter() {
                ms.remove(e);
            }
        }
    }

    /// Call for VL2_MULTICAST_LIKE packets.
    pub fn handle_vl2_multicast_like<Authenticator: Fn(NetworkId, &Identity) -> bool>(
        &self,
        auth: Authenticator,
        time_ticks: i64,
        source: &Arc<Peer>,
        payload: &PacketBuffer,
        mut cursor: usize,
    ) -> PacketHandlerResult {
        let mut subscriptions = RMaybeWLockGuard::new_read(&self.subscriptions);

        while (cursor + 8 + 6 + 4) <= payload.len() {
            let network_id = NetworkId::from_bytes_fixed(payload.read_bytes_fixed(&mut cursor).unwrap());
            if let Some(network_id) = network_id {
                let mac = MAC::from_bytes_fixed(payload.read_bytes_fixed(&mut cursor).unwrap());
                if let Some(mac) = mac {
                    if auth(network_id, &source.identity) {
                        let sub_key = (network_id, MulticastGroup { mac, adi: payload.read_u32(&mut cursor).unwrap() });
                        if let Some(sub) = subscriptions.read().get(&sub_key) {
                            let _ = sub.lock().unwrap().insert(source.identity.address, time_ticks);
                        } else {
                            let _ = subscriptions
                                .write(&self.subscriptions)
                                .entry(sub_key)
                                .or_insert_with(|| Mutex::new(HashMap::new()))
                                .lock()
                                .unwrap()
                                .insert(source.identity.address, time_ticks);
                        }
                    }
                }
            }
        }

        PacketHandlerResult::Ok
    }

    /// Call for VL2_MULTICAST_GATHER packets.
    pub fn handle_vl2_multicast_gather<HostSystemImpl: ApplicationLayer + ?Sized, Authenticator: Fn(NetworkId, &Identity) -> bool>(
        &self,
        auth: Authenticator,
        time_ticks: i64,
        host_system: &HostSystemImpl,
        node: &Node,
        source: &Arc<Peer>,
        message_id: u64,
        payload: &PacketBuffer,
        mut cursor: usize,
    ) -> PacketHandlerResult {
        if let Some(network_id) = payload
            .read_bytes_fixed(&mut cursor)
            .map_or(None, |network_id| NetworkId::from_bytes_fixed(network_id))
        {
            if auth(network_id, &source.identity) {
                cursor += 1; // skip flags, currently unused
                if let Some(mac) = payload.read_bytes_fixed(&mut cursor).map_or(None, |mac| MAC::from_bytes_fixed(mac)) {
                    let mut gathered = Vec::new();

                    let adi = payload.read_u32(&mut cursor).unwrap_or(0);
                    let subscriptions = self.subscriptions.read().unwrap();
                    if let Some(sub) = subscriptions.get(&(network_id, MulticastGroup { mac, adi })) {
                        let sub = sub.lock().unwrap();
                        for a in sub.keys() {
                            gathered.push(*a);
                        }
                    }

                    while !gathered.is_empty() {
                        source.send(host_system, node, None, time_ticks, |packet| -> Result<(), OutOfBoundsError> {
                            let ok_header = packet.append_struct_get_mut::<protocol::OkHeader>()?;
                            ok_header.verb = protocol::message_type::VL1_OK;
                            ok_header.in_re_verb = protocol::message_type::VL2_MULTICAST_GATHER;
                            ok_header.in_re_message_id = message_id.to_be_bytes();

                            packet.append_bytes_fixed(&network_id.to_bytes())?;
                            packet.append_bytes_fixed(&mac.to_bytes())?;
                            packet.append_u32(adi)?;
                            packet.append_u32(gathered.len() as u32)?;

                            let in_this_packet = gathered
                                .len()
                                .clamp(1, (packet.capacity() - packet.len()) / protocol::ADDRESS_SIZE)
                                .min(u16::MAX as usize);

                            packet.append_u16(in_this_packet as u16)?;
                            for _ in 0..in_this_packet {
                                packet.append_bytes_fixed(&gathered.pop().unwrap().to_bytes())?;
                            }

                            Ok(())
                        });
                    }
                }
            }
        }

        PacketHandlerResult::Ok
    }
}