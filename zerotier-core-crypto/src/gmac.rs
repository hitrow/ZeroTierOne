use std::io::Write;

/// GMAC portion of AES-GCM for use as a fast plain vanilla MAC.
pub struct GMAC(gcrypt::mac::Mac);

impl GMAC {
    /// Create a new keyed GMAC instance.
    /// The key may be 16, 24, or 32 bytes in length. This will panic otherwise.
    pub fn new(key: &[u8]) -> GMAC {
        if key.len() != 32 && key.len() != 24 && key.len() != 16 {
            panic!("AES supports 128, 192, or 256 bits keys");
        }
        let mut m = GMAC(gcrypt::mac::Mac::new(gcrypt::mac::Algorithm::GmacAes).unwrap());
        m.0.set_key(key).expect("GMAC set_key failed");
        m
    }

    /// Reset GMAC and set nonce.
    /// The nonce may be anywhere from 8 to 16 bytes in length but 12 bytes is strongly recommended.
    /// It may be sequential.
    #[inline(always)]
    pub fn init(&mut self, nonce: &[u8]) {
        let _ = self.0.reset();
        self.0.set_iv(nonce).expect("GMAC set_iv failed");
    }

    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) {
        let _ = self.0.update(data);
    }

    /// Flush GMAC and filll 'mac' with the final authentication code.
    #[inline(always)]
    pub fn finish(&mut self, mac: &mut [u8; 16]) {
        let _ = self.0.flush();
        let _ = self.0.get_mac(mac).expect("GMAC get_mac failed");
    }
}

/// A wrapper for GMAC with an incrementing 96-bit nonce.
/// The nonce here is incremented as a little-endian value.
/// This is for use with TCP streams. A maximum of 2^96 messages
/// should be sent or received with this, which is probably a large
/// enough limit to be safely ignored.
pub struct SequentialNonceGMAC(GMAC, u128);

impl SequentialNonceGMAC {
    #[inline(always)]
    pub fn new(key: &[u8], initial_nonce: &[u8]) -> Self {
        assert_eq!(initial_nonce.len(), 16);
        Self(GMAC::new(key), u128::from_ne_bytes(initial_nonce.try_into().unwrap()))
    }

    #[inline(always)]
    pub fn init_for_next_message(&mut self) {
        self.0.init(unsafe { &*(&self.1 as *const u128).cast::<[u8; 12]>() });
        self.1 = u128::from_le(self.1).wrapping_add(1).to_le();
    }

    #[inline(always)]
    pub fn update(&mut self, data: &[u8]) { self.0.update(data); }

    #[inline(always)]
    pub fn finish(&mut self, mac: &mut [u8; 16]) { self.0.finish(mac); }
}