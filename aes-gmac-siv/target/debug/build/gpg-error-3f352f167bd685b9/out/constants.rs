impl Error {
pub const SOURCE_UNKNOWN: ErrorSource = ffi::GPG_ERR_SOURCE_UNKNOWN;
pub const SOURCE_GCRYPT: ErrorSource = ffi::GPG_ERR_SOURCE_GCRYPT;
pub const SOURCE_GPG: ErrorSource = ffi::GPG_ERR_SOURCE_GPG;
pub const SOURCE_GPGSM: ErrorSource = ffi::GPG_ERR_SOURCE_GPGSM;
pub const SOURCE_GPGAGENT: ErrorSource = ffi::GPG_ERR_SOURCE_GPGAGENT;
pub const SOURCE_PINENTRY: ErrorSource = ffi::GPG_ERR_SOURCE_PINENTRY;
pub const SOURCE_SCD: ErrorSource = ffi::GPG_ERR_SOURCE_SCD;
pub const SOURCE_GPGME: ErrorSource = ffi::GPG_ERR_SOURCE_GPGME;
pub const SOURCE_KEYBOX: ErrorSource = ffi::GPG_ERR_SOURCE_KEYBOX;
pub const SOURCE_KSBA: ErrorSource = ffi::GPG_ERR_SOURCE_KSBA;
pub const SOURCE_DIRMNGR: ErrorSource = ffi::GPG_ERR_SOURCE_DIRMNGR;
pub const SOURCE_GSTI: ErrorSource = ffi::GPG_ERR_SOURCE_GSTI;
pub const SOURCE_GPA: ErrorSource = ffi::GPG_ERR_SOURCE_GPA;
pub const SOURCE_KLEO: ErrorSource = ffi::GPG_ERR_SOURCE_KLEO;
pub const SOURCE_G13: ErrorSource = ffi::GPG_ERR_SOURCE_G13;
pub const SOURCE_ASSUAN: ErrorSource = ffi::GPG_ERR_SOURCE_ASSUAN;
pub const SOURCE_TLS: ErrorSource = ffi::GPG_ERR_SOURCE_TLS;
pub const SOURCE_ANY: ErrorSource = ffi::GPG_ERR_SOURCE_ANY;
pub const SOURCE_USER_1: ErrorSource = ffi::GPG_ERR_SOURCE_USER_1;
pub const SOURCE_USER_2: ErrorSource = ffi::GPG_ERR_SOURCE_USER_2;
pub const SOURCE_USER_3: ErrorSource = ffi::GPG_ERR_SOURCE_USER_3;
pub const SOURCE_USER_4: ErrorSource = ffi::GPG_ERR_SOURCE_USER_4;
pub const NO_ERROR: Error = Error(ffi::GPG_ERR_NO_ERROR);
pub const GENERAL: Error = Error(ffi::GPG_ERR_GENERAL);
pub const UNKNOWN_PACKET: Error = Error(ffi::GPG_ERR_UNKNOWN_PACKET);
pub const UNKNOWN_VERSION: Error = Error(ffi::GPG_ERR_UNKNOWN_VERSION);
pub const PUBKEY_ALGO: Error = Error(ffi::GPG_ERR_PUBKEY_ALGO);
pub const DIGEST_ALGO: Error = Error(ffi::GPG_ERR_DIGEST_ALGO);
pub const BAD_PUBKEY: Error = Error(ffi::GPG_ERR_BAD_PUBKEY);
pub const BAD_SECKEY: Error = Error(ffi::GPG_ERR_BAD_SECKEY);
pub const BAD_SIGNATURE: Error = Error(ffi::GPG_ERR_BAD_SIGNATURE);
pub const NO_PUBKEY: Error = Error(ffi::GPG_ERR_NO_PUBKEY);
pub const CHECKSUM: Error = Error(ffi::GPG_ERR_CHECKSUM);
pub const BAD_PASSPHRASE: Error = Error(ffi::GPG_ERR_BAD_PASSPHRASE);
pub const CIPHER_ALGO: Error = Error(ffi::GPG_ERR_CIPHER_ALGO);
pub const KEYRING_OPEN: Error = Error(ffi::GPG_ERR_KEYRING_OPEN);
pub const INV_PACKET: Error = Error(ffi::GPG_ERR_INV_PACKET);
pub const INV_ARMOR: Error = Error(ffi::GPG_ERR_INV_ARMOR);
pub const NO_USER_ID: Error = Error(ffi::GPG_ERR_NO_USER_ID);
pub const NO_SECKEY: Error = Error(ffi::GPG_ERR_NO_SECKEY);
pub const WRONG_SECKEY: Error = Error(ffi::GPG_ERR_WRONG_SECKEY);
pub const BAD_KEY: Error = Error(ffi::GPG_ERR_BAD_KEY);
pub const COMPR_ALGO: Error = Error(ffi::GPG_ERR_COMPR_ALGO);
pub const NO_PRIME: Error = Error(ffi::GPG_ERR_NO_PRIME);
pub const NO_ENCODING_METHOD: Error = Error(ffi::GPG_ERR_NO_ENCODING_METHOD);
pub const NO_ENCRYPTION_SCHEME: Error = Error(ffi::GPG_ERR_NO_ENCRYPTION_SCHEME);
pub const NO_SIGNATURE_SCHEME: Error = Error(ffi::GPG_ERR_NO_SIGNATURE_SCHEME);
pub const INV_ATTR: Error = Error(ffi::GPG_ERR_INV_ATTR);
pub const NO_VALUE: Error = Error(ffi::GPG_ERR_NO_VALUE);
pub const NOT_FOUND: Error = Error(ffi::GPG_ERR_NOT_FOUND);
pub const VALUE_NOT_FOUND: Error = Error(ffi::GPG_ERR_VALUE_NOT_FOUND);
pub const SYNTAX: Error = Error(ffi::GPG_ERR_SYNTAX);
pub const BAD_MPI: Error = Error(ffi::GPG_ERR_BAD_MPI);
pub const INV_PASSPHRASE: Error = Error(ffi::GPG_ERR_INV_PASSPHRASE);
pub const SIG_CLASS: Error = Error(ffi::GPG_ERR_SIG_CLASS);
pub const RESOURCE_LIMIT: Error = Error(ffi::GPG_ERR_RESOURCE_LIMIT);
pub const INV_KEYRING: Error = Error(ffi::GPG_ERR_INV_KEYRING);
pub const TRUSTDB: Error = Error(ffi::GPG_ERR_TRUSTDB);
pub const BAD_CERT: Error = Error(ffi::GPG_ERR_BAD_CERT);
pub const INV_USER_ID: Error = Error(ffi::GPG_ERR_INV_USER_ID);
pub const UNEXPECTED: Error = Error(ffi::GPG_ERR_UNEXPECTED);
pub const TIME_CONFLICT: Error = Error(ffi::GPG_ERR_TIME_CONFLICT);
pub const KEYSERVER: Error = Error(ffi::GPG_ERR_KEYSERVER);
pub const WRONG_PUBKEY_ALGO: Error = Error(ffi::GPG_ERR_WRONG_PUBKEY_ALGO);
pub const TRIBUTE_TO_D_A: Error = Error(ffi::GPG_ERR_TRIBUTE_TO_D_A);
pub const WEAK_KEY: Error = Error(ffi::GPG_ERR_WEAK_KEY);
pub const INV_KEYLEN: Error = Error(ffi::GPG_ERR_INV_KEYLEN);
pub const INV_ARG: Error = Error(ffi::GPG_ERR_INV_ARG);
pub const BAD_URI: Error = Error(ffi::GPG_ERR_BAD_URI);
pub const INV_URI: Error = Error(ffi::GPG_ERR_INV_URI);
pub const NETWORK: Error = Error(ffi::GPG_ERR_NETWORK);
pub const UNKNOWN_HOST: Error = Error(ffi::GPG_ERR_UNKNOWN_HOST);
pub const SELFTEST_FAILED: Error = Error(ffi::GPG_ERR_SELFTEST_FAILED);
pub const NOT_ENCRYPTED: Error = Error(ffi::GPG_ERR_NOT_ENCRYPTED);
pub const NOT_PROCESSED: Error = Error(ffi::GPG_ERR_NOT_PROCESSED);
pub const UNUSABLE_PUBKEY: Error = Error(ffi::GPG_ERR_UNUSABLE_PUBKEY);
pub const UNUSABLE_SECKEY: Error = Error(ffi::GPG_ERR_UNUSABLE_SECKEY);
pub const INV_VALUE: Error = Error(ffi::GPG_ERR_INV_VALUE);
pub const BAD_CERT_CHAIN: Error = Error(ffi::GPG_ERR_BAD_CERT_CHAIN);
pub const MISSING_CERT: Error = Error(ffi::GPG_ERR_MISSING_CERT);
pub const NO_DATA: Error = Error(ffi::GPG_ERR_NO_DATA);
pub const BUG: Error = Error(ffi::GPG_ERR_BUG);
pub const NOT_SUPPORTED: Error = Error(ffi::GPG_ERR_NOT_SUPPORTED);
pub const INV_OP: Error = Error(ffi::GPG_ERR_INV_OP);
pub const TIMEOUT: Error = Error(ffi::GPG_ERR_TIMEOUT);
pub const INTERNAL: Error = Error(ffi::GPG_ERR_INTERNAL);
pub const EOF_GCRYPT: Error = Error(ffi::GPG_ERR_EOF_GCRYPT);
pub const INV_OBJ: Error = Error(ffi::GPG_ERR_INV_OBJ);
pub const TOO_SHORT: Error = Error(ffi::GPG_ERR_TOO_SHORT);
pub const TOO_LARGE: Error = Error(ffi::GPG_ERR_TOO_LARGE);
pub const NO_OBJ: Error = Error(ffi::GPG_ERR_NO_OBJ);
pub const NOT_IMPLEMENTED: Error = Error(ffi::GPG_ERR_NOT_IMPLEMENTED);
pub const CONFLICT: Error = Error(ffi::GPG_ERR_CONFLICT);
pub const INV_CIPHER_MODE: Error = Error(ffi::GPG_ERR_INV_CIPHER_MODE);
pub const INV_FLAG: Error = Error(ffi::GPG_ERR_INV_FLAG);
pub const INV_HANDLE: Error = Error(ffi::GPG_ERR_INV_HANDLE);
pub const TRUNCATED: Error = Error(ffi::GPG_ERR_TRUNCATED);
pub const INCOMPLETE_LINE: Error = Error(ffi::GPG_ERR_INCOMPLETE_LINE);
pub const INV_RESPONSE: Error = Error(ffi::GPG_ERR_INV_RESPONSE);
pub const NO_AGENT: Error = Error(ffi::GPG_ERR_NO_AGENT);
pub const AGENT: Error = Error(ffi::GPG_ERR_AGENT);
pub const INV_DATA: Error = Error(ffi::GPG_ERR_INV_DATA);
pub const ASSUAN_SERVER_FAULT: Error = Error(ffi::GPG_ERR_ASSUAN_SERVER_FAULT);
pub const ASSUAN: Error = Error(ffi::GPG_ERR_ASSUAN);
pub const INV_SESSION_KEY: Error = Error(ffi::GPG_ERR_INV_SESSION_KEY);
pub const INV_SEXP: Error = Error(ffi::GPG_ERR_INV_SEXP);
pub const UNSUPPORTED_ALGORITHM: Error = Error(ffi::GPG_ERR_UNSUPPORTED_ALGORITHM);
pub const NO_PIN_ENTRY: Error = Error(ffi::GPG_ERR_NO_PIN_ENTRY);
pub const PIN_ENTRY: Error = Error(ffi::GPG_ERR_PIN_ENTRY);
pub const BAD_PIN: Error = Error(ffi::GPG_ERR_BAD_PIN);
pub const INV_NAME: Error = Error(ffi::GPG_ERR_INV_NAME);
pub const BAD_DATA: Error = Error(ffi::GPG_ERR_BAD_DATA);
pub const INV_PARAMETER: Error = Error(ffi::GPG_ERR_INV_PARAMETER);
pub const WRONG_CARD: Error = Error(ffi::GPG_ERR_WRONG_CARD);
pub const NO_DIRMNGR: Error = Error(ffi::GPG_ERR_NO_DIRMNGR);
pub const DIRMNGR: Error = Error(ffi::GPG_ERR_DIRMNGR);
pub const CERT_REVOKED: Error = Error(ffi::GPG_ERR_CERT_REVOKED);
pub const NO_CRL_KNOWN: Error = Error(ffi::GPG_ERR_NO_CRL_KNOWN);
pub const CRL_TOO_OLD: Error = Error(ffi::GPG_ERR_CRL_TOO_OLD);
pub const LINE_TOO_LONG: Error = Error(ffi::GPG_ERR_LINE_TOO_LONG);
pub const NOT_TRUSTED: Error = Error(ffi::GPG_ERR_NOT_TRUSTED);
pub const CANCELED: Error = Error(ffi::GPG_ERR_CANCELED);
pub const BAD_CA_CERT: Error = Error(ffi::GPG_ERR_BAD_CA_CERT);
pub const CERT_EXPIRED: Error = Error(ffi::GPG_ERR_CERT_EXPIRED);
pub const CERT_TOO_YOUNG: Error = Error(ffi::GPG_ERR_CERT_TOO_YOUNG);
pub const UNSUPPORTED_CERT: Error = Error(ffi::GPG_ERR_UNSUPPORTED_CERT);
pub const UNKNOWN_SEXP: Error = Error(ffi::GPG_ERR_UNKNOWN_SEXP);
pub const UNSUPPORTED_PROTECTION: Error = Error(ffi::GPG_ERR_UNSUPPORTED_PROTECTION);
pub const CORRUPTED_PROTECTION: Error = Error(ffi::GPG_ERR_CORRUPTED_PROTECTION);
pub const AMBIGUOUS_NAME: Error = Error(ffi::GPG_ERR_AMBIGUOUS_NAME);
pub const CARD: Error = Error(ffi::GPG_ERR_CARD);
pub const CARD_RESET: Error = Error(ffi::GPG_ERR_CARD_RESET);
pub const CARD_REMOVED: Error = Error(ffi::GPG_ERR_CARD_REMOVED);
pub const INV_CARD: Error = Error(ffi::GPG_ERR_INV_CARD);
pub const CARD_NOT_PRESENT: Error = Error(ffi::GPG_ERR_CARD_NOT_PRESENT);
pub const NO_PKCS15_APP: Error = Error(ffi::GPG_ERR_NO_PKCS15_APP);
pub const NOT_CONFIRMED: Error = Error(ffi::GPG_ERR_NOT_CONFIRMED);
pub const CONFIGURATION: Error = Error(ffi::GPG_ERR_CONFIGURATION);
pub const NO_POLICY_MATCH: Error = Error(ffi::GPG_ERR_NO_POLICY_MATCH);
pub const INV_INDEX: Error = Error(ffi::GPG_ERR_INV_INDEX);
pub const INV_ID: Error = Error(ffi::GPG_ERR_INV_ID);
pub const NO_SCDAEMON: Error = Error(ffi::GPG_ERR_NO_SCDAEMON);
pub const SCDAEMON: Error = Error(ffi::GPG_ERR_SCDAEMON);
pub const UNSUPPORTED_PROTOCOL: Error = Error(ffi::GPG_ERR_UNSUPPORTED_PROTOCOL);
pub const BAD_PIN_METHOD: Error = Error(ffi::GPG_ERR_BAD_PIN_METHOD);
pub const CARD_NOT_INITIALIZED: Error = Error(ffi::GPG_ERR_CARD_NOT_INITIALIZED);
pub const UNSUPPORTED_OPERATION: Error = Error(ffi::GPG_ERR_UNSUPPORTED_OPERATION);
pub const WRONG_KEY_USAGE: Error = Error(ffi::GPG_ERR_WRONG_KEY_USAGE);
pub const NOTHING_FOUND: Error = Error(ffi::GPG_ERR_NOTHING_FOUND);
pub const WRONG_BLOB_TYPE: Error = Error(ffi::GPG_ERR_WRONG_BLOB_TYPE);
pub const MISSING_VALUE: Error = Error(ffi::GPG_ERR_MISSING_VALUE);
pub const HARDWARE: Error = Error(ffi::GPG_ERR_HARDWARE);
pub const PIN_BLOCKED: Error = Error(ffi::GPG_ERR_PIN_BLOCKED);
pub const USE_CONDITIONS: Error = Error(ffi::GPG_ERR_USE_CONDITIONS);
pub const PIN_NOT_SYNCED: Error = Error(ffi::GPG_ERR_PIN_NOT_SYNCED);
pub const INV_CRL: Error = Error(ffi::GPG_ERR_INV_CRL);
pub const BAD_BER: Error = Error(ffi::GPG_ERR_BAD_BER);
pub const INV_BER: Error = Error(ffi::GPG_ERR_INV_BER);
pub const ELEMENT_NOT_FOUND: Error = Error(ffi::GPG_ERR_ELEMENT_NOT_FOUND);
pub const IDENTIFIER_NOT_FOUND: Error = Error(ffi::GPG_ERR_IDENTIFIER_NOT_FOUND);
pub const INV_TAG: Error = Error(ffi::GPG_ERR_INV_TAG);
pub const INV_LENGTH: Error = Error(ffi::GPG_ERR_INV_LENGTH);
pub const INV_KEYINFO: Error = Error(ffi::GPG_ERR_INV_KEYINFO);
pub const UNEXPECTED_TAG: Error = Error(ffi::GPG_ERR_UNEXPECTED_TAG);
pub const NOT_DER_ENCODED: Error = Error(ffi::GPG_ERR_NOT_DER_ENCODED);
pub const NO_CMS_OBJ: Error = Error(ffi::GPG_ERR_NO_CMS_OBJ);
pub const INV_CMS_OBJ: Error = Error(ffi::GPG_ERR_INV_CMS_OBJ);
pub const UNKNOWN_CMS_OBJ: Error = Error(ffi::GPG_ERR_UNKNOWN_CMS_OBJ);
pub const UNSUPPORTED_CMS_OBJ: Error = Error(ffi::GPG_ERR_UNSUPPORTED_CMS_OBJ);
pub const UNSUPPORTED_ENCODING: Error = Error(ffi::GPG_ERR_UNSUPPORTED_ENCODING);
pub const UNSUPPORTED_CMS_VERSION: Error = Error(ffi::GPG_ERR_UNSUPPORTED_CMS_VERSION);
pub const UNKNOWN_ALGORITHM: Error = Error(ffi::GPG_ERR_UNKNOWN_ALGORITHM);
pub const INV_ENGINE: Error = Error(ffi::GPG_ERR_INV_ENGINE);
pub const PUBKEY_NOT_TRUSTED: Error = Error(ffi::GPG_ERR_PUBKEY_NOT_TRUSTED);
pub const DECRYPT_FAILED: Error = Error(ffi::GPG_ERR_DECRYPT_FAILED);
pub const KEY_EXPIRED: Error = Error(ffi::GPG_ERR_KEY_EXPIRED);
pub const SIG_EXPIRED: Error = Error(ffi::GPG_ERR_SIG_EXPIRED);
pub const ENCODING_PROBLEM: Error = Error(ffi::GPG_ERR_ENCODING_PROBLEM);
pub const INV_STATE: Error = Error(ffi::GPG_ERR_INV_STATE);
pub const DUP_VALUE: Error = Error(ffi::GPG_ERR_DUP_VALUE);
pub const MISSING_ACTION: Error = Error(ffi::GPG_ERR_MISSING_ACTION);
pub const MODULE_NOT_FOUND: Error = Error(ffi::GPG_ERR_MODULE_NOT_FOUND);
pub const INV_OID_STRING: Error = Error(ffi::GPG_ERR_INV_OID_STRING);
pub const INV_TIME: Error = Error(ffi::GPG_ERR_INV_TIME);
pub const INV_CRL_OBJ: Error = Error(ffi::GPG_ERR_INV_CRL_OBJ);
pub const UNSUPPORTED_CRL_VERSION: Error = Error(ffi::GPG_ERR_UNSUPPORTED_CRL_VERSION);
pub const INV_CERT_OBJ: Error = Error(ffi::GPG_ERR_INV_CERT_OBJ);
pub const UNKNOWN_NAME: Error = Error(ffi::GPG_ERR_UNKNOWN_NAME);
pub const LOCALE_PROBLEM: Error = Error(ffi::GPG_ERR_LOCALE_PROBLEM);
pub const NOT_LOCKED: Error = Error(ffi::GPG_ERR_NOT_LOCKED);
pub const PROTOCOL_VIOLATION: Error = Error(ffi::GPG_ERR_PROTOCOL_VIOLATION);
pub const INV_MAC: Error = Error(ffi::GPG_ERR_INV_MAC);
pub const INV_REQUEST: Error = Error(ffi::GPG_ERR_INV_REQUEST);
pub const UNKNOWN_EXTN: Error = Error(ffi::GPG_ERR_UNKNOWN_EXTN);
pub const UNKNOWN_CRIT_EXTN: Error = Error(ffi::GPG_ERR_UNKNOWN_CRIT_EXTN);
pub const LOCKED: Error = Error(ffi::GPG_ERR_LOCKED);
pub const UNKNOWN_OPTION: Error = Error(ffi::GPG_ERR_UNKNOWN_OPTION);
pub const UNKNOWN_COMMAND: Error = Error(ffi::GPG_ERR_UNKNOWN_COMMAND);
pub const NOT_OPERATIONAL: Error = Error(ffi::GPG_ERR_NOT_OPERATIONAL);
pub const NO_PASSPHRASE: Error = Error(ffi::GPG_ERR_NO_PASSPHRASE);
pub const NO_PIN: Error = Error(ffi::GPG_ERR_NO_PIN);
pub const NOT_ENABLED: Error = Error(ffi::GPG_ERR_NOT_ENABLED);
pub const NO_ENGINE: Error = Error(ffi::GPG_ERR_NO_ENGINE);
pub const MISSING_KEY: Error = Error(ffi::GPG_ERR_MISSING_KEY);
pub const TOO_MANY: Error = Error(ffi::GPG_ERR_TOO_MANY);
pub const LIMIT_REACHED: Error = Error(ffi::GPG_ERR_LIMIT_REACHED);
pub const NOT_INITIALIZED: Error = Error(ffi::GPG_ERR_NOT_INITIALIZED);
pub const MISSING_ISSUER_CERT: Error = Error(ffi::GPG_ERR_MISSING_ISSUER_CERT);
pub const NO_KEYSERVER: Error = Error(ffi::GPG_ERR_NO_KEYSERVER);
pub const INV_CURVE: Error = Error(ffi::GPG_ERR_INV_CURVE);
pub const UNKNOWN_CURVE: Error = Error(ffi::GPG_ERR_UNKNOWN_CURVE);
pub const DUP_KEY: Error = Error(ffi::GPG_ERR_DUP_KEY);
pub const AMBIGUOUS: Error = Error(ffi::GPG_ERR_AMBIGUOUS);
pub const NO_CRYPT_CTX: Error = Error(ffi::GPG_ERR_NO_CRYPT_CTX);
pub const WRONG_CRYPT_CTX: Error = Error(ffi::GPG_ERR_WRONG_CRYPT_CTX);
pub const BAD_CRYPT_CTX: Error = Error(ffi::GPG_ERR_BAD_CRYPT_CTX);
pub const CRYPT_CTX_CONFLICT: Error = Error(ffi::GPG_ERR_CRYPT_CTX_CONFLICT);
pub const BROKEN_PUBKEY: Error = Error(ffi::GPG_ERR_BROKEN_PUBKEY);
pub const BROKEN_SECKEY: Error = Error(ffi::GPG_ERR_BROKEN_SECKEY);
pub const MAC_ALGO: Error = Error(ffi::GPG_ERR_MAC_ALGO);
pub const FULLY_CANCELED: Error = Error(ffi::GPG_ERR_FULLY_CANCELED);
pub const UNFINISHED: Error = Error(ffi::GPG_ERR_UNFINISHED);
pub const BUFFER_TOO_SHORT: Error = Error(ffi::GPG_ERR_BUFFER_TOO_SHORT);
pub const SEXP_INV_LEN_SPEC: Error = Error(ffi::GPG_ERR_SEXP_INV_LEN_SPEC);
pub const SEXP_STRING_TOO_LONG: Error = Error(ffi::GPG_ERR_SEXP_STRING_TOO_LONG);
pub const SEXP_UNMATCHED_PAREN: Error = Error(ffi::GPG_ERR_SEXP_UNMATCHED_PAREN);
pub const SEXP_NOT_CANONICAL: Error = Error(ffi::GPG_ERR_SEXP_NOT_CANONICAL);
pub const SEXP_BAD_CHARACTER: Error = Error(ffi::GPG_ERR_SEXP_BAD_CHARACTER);
pub const SEXP_BAD_QUOTATION: Error = Error(ffi::GPG_ERR_SEXP_BAD_QUOTATION);
pub const SEXP_ZERO_PREFIX: Error = Error(ffi::GPG_ERR_SEXP_ZERO_PREFIX);
pub const SEXP_NESTED_DH: Error = Error(ffi::GPG_ERR_SEXP_NESTED_DH);
pub const SEXP_UNMATCHED_DH: Error = Error(ffi::GPG_ERR_SEXP_UNMATCHED_DH);
pub const SEXP_UNEXPECTED_PUNC: Error = Error(ffi::GPG_ERR_SEXP_UNEXPECTED_PUNC);
pub const SEXP_BAD_HEX_CHAR: Error = Error(ffi::GPG_ERR_SEXP_BAD_HEX_CHAR);
pub const SEXP_ODD_HEX_NUMBERS: Error = Error(ffi::GPG_ERR_SEXP_ODD_HEX_NUMBERS);
pub const SEXP_BAD_OCT_CHAR: Error = Error(ffi::GPG_ERR_SEXP_BAD_OCT_CHAR);
pub const SUBKEYS_EXP_OR_REV: Error = Error(ffi::GPG_ERR_SUBKEYS_EXP_OR_REV);
pub const DB_CORRUPTED: Error = Error(ffi::GPG_ERR_DB_CORRUPTED);
pub const SERVER_FAILED: Error = Error(ffi::GPG_ERR_SERVER_FAILED);
pub const NO_NAME: Error = Error(ffi::GPG_ERR_NO_NAME);
pub const NO_KEY: Error = Error(ffi::GPG_ERR_NO_KEY);
pub const LEGACY_KEY: Error = Error(ffi::GPG_ERR_LEGACY_KEY);
pub const REQUEST_TOO_SHORT: Error = Error(ffi::GPG_ERR_REQUEST_TOO_SHORT);
pub const REQUEST_TOO_LONG: Error = Error(ffi::GPG_ERR_REQUEST_TOO_LONG);
pub const OBJ_TERM_STATE: Error = Error(ffi::GPG_ERR_OBJ_TERM_STATE);
pub const NO_CERT_CHAIN: Error = Error(ffi::GPG_ERR_NO_CERT_CHAIN);
pub const CERT_TOO_LARGE: Error = Error(ffi::GPG_ERR_CERT_TOO_LARGE);
pub const INV_RECORD: Error = Error(ffi::GPG_ERR_INV_RECORD);
pub const BAD_MAC: Error = Error(ffi::GPG_ERR_BAD_MAC);
pub const UNEXPECTED_MSG: Error = Error(ffi::GPG_ERR_UNEXPECTED_MSG);
pub const COMPR_FAILED: Error = Error(ffi::GPG_ERR_COMPR_FAILED);
pub const WOULD_WRAP: Error = Error(ffi::GPG_ERR_WOULD_WRAP);
pub const FATAL_ALERT: Error = Error(ffi::GPG_ERR_FATAL_ALERT);
pub const NO_CIPHER: Error = Error(ffi::GPG_ERR_NO_CIPHER);
pub const MISSING_CLIENT_CERT: Error = Error(ffi::GPG_ERR_MISSING_CLIENT_CERT);
pub const CLOSE_NOTIFY: Error = Error(ffi::GPG_ERR_CLOSE_NOTIFY);
pub const TICKET_EXPIRED: Error = Error(ffi::GPG_ERR_TICKET_EXPIRED);
pub const BAD_TICKET: Error = Error(ffi::GPG_ERR_BAD_TICKET);
pub const UNKNOWN_IDENTITY: Error = Error(ffi::GPG_ERR_UNKNOWN_IDENTITY);
pub const BAD_HS_CERT: Error = Error(ffi::GPG_ERR_BAD_HS_CERT);
pub const BAD_HS_CERT_REQ: Error = Error(ffi::GPG_ERR_BAD_HS_CERT_REQ);
pub const BAD_HS_CERT_VER: Error = Error(ffi::GPG_ERR_BAD_HS_CERT_VER);
pub const BAD_HS_CHANGE_CIPHER: Error = Error(ffi::GPG_ERR_BAD_HS_CHANGE_CIPHER);
pub const BAD_HS_CLIENT_HELLO: Error = Error(ffi::GPG_ERR_BAD_HS_CLIENT_HELLO);
pub const BAD_HS_SERVER_HELLO: Error = Error(ffi::GPG_ERR_BAD_HS_SERVER_HELLO);
pub const BAD_HS_SERVER_HELLO_DONE: Error = Error(ffi::GPG_ERR_BAD_HS_SERVER_HELLO_DONE);
pub const BAD_HS_FINISHED: Error = Error(ffi::GPG_ERR_BAD_HS_FINISHED);
pub const BAD_HS_SERVER_KEX: Error = Error(ffi::GPG_ERR_BAD_HS_SERVER_KEX);
pub const BAD_HS_CLIENT_KEX: Error = Error(ffi::GPG_ERR_BAD_HS_CLIENT_KEX);
pub const BOGUS_STRING: Error = Error(ffi::GPG_ERR_BOGUS_STRING);
pub const FORBIDDEN: Error = Error(ffi::GPG_ERR_FORBIDDEN);
pub const KEY_DISABLED: Error = Error(ffi::GPG_ERR_KEY_DISABLED);
pub const KEY_ON_CARD: Error = Error(ffi::GPG_ERR_KEY_ON_CARD);
pub const INV_LOCK_OBJ: Error = Error(ffi::GPG_ERR_INV_LOCK_OBJ);
pub const TRUE: Error = Error(ffi::GPG_ERR_TRUE);
pub const FALSE: Error = Error(ffi::GPG_ERR_FALSE);
pub const ASS_GENERAL: Error = Error(ffi::GPG_ERR_ASS_GENERAL);
pub const ASS_ACCEPT_FAILED: Error = Error(ffi::GPG_ERR_ASS_ACCEPT_FAILED);
pub const ASS_CONNECT_FAILED: Error = Error(ffi::GPG_ERR_ASS_CONNECT_FAILED);
pub const ASS_INV_RESPONSE: Error = Error(ffi::GPG_ERR_ASS_INV_RESPONSE);
pub const ASS_INV_VALUE: Error = Error(ffi::GPG_ERR_ASS_INV_VALUE);
pub const ASS_INCOMPLETE_LINE: Error = Error(ffi::GPG_ERR_ASS_INCOMPLETE_LINE);
pub const ASS_LINE_TOO_LONG: Error = Error(ffi::GPG_ERR_ASS_LINE_TOO_LONG);
pub const ASS_NESTED_COMMANDS: Error = Error(ffi::GPG_ERR_ASS_NESTED_COMMANDS);
pub const ASS_NO_DATA_CB: Error = Error(ffi::GPG_ERR_ASS_NO_DATA_CB);
pub const ASS_NO_INQUIRE_CB: Error = Error(ffi::GPG_ERR_ASS_NO_INQUIRE_CB);
pub const ASS_NOT_A_SERVER: Error = Error(ffi::GPG_ERR_ASS_NOT_A_SERVER);
pub const ASS_NOT_A_CLIENT: Error = Error(ffi::GPG_ERR_ASS_NOT_A_CLIENT);
pub const ASS_SERVER_START: Error = Error(ffi::GPG_ERR_ASS_SERVER_START);
pub const ASS_READ_ERROR: Error = Error(ffi::GPG_ERR_ASS_READ_ERROR);
pub const ASS_WRITE_ERROR: Error = Error(ffi::GPG_ERR_ASS_WRITE_ERROR);
pub const ASS_TOO_MUCH_DATA: Error = Error(ffi::GPG_ERR_ASS_TOO_MUCH_DATA);
pub const ASS_UNEXPECTED_CMD: Error = Error(ffi::GPG_ERR_ASS_UNEXPECTED_CMD);
pub const ASS_UNKNOWN_CMD: Error = Error(ffi::GPG_ERR_ASS_UNKNOWN_CMD);
pub const ASS_SYNTAX: Error = Error(ffi::GPG_ERR_ASS_SYNTAX);
pub const ASS_CANCELED: Error = Error(ffi::GPG_ERR_ASS_CANCELED);
pub const ASS_NO_INPUT: Error = Error(ffi::GPG_ERR_ASS_NO_INPUT);
pub const ASS_NO_OUTPUT: Error = Error(ffi::GPG_ERR_ASS_NO_OUTPUT);
pub const ASS_PARAMETER: Error = Error(ffi::GPG_ERR_ASS_PARAMETER);
pub const ASS_UNKNOWN_INQUIRE: Error = Error(ffi::GPG_ERR_ASS_UNKNOWN_INQUIRE);
pub const ENGINE_TOO_OLD: Error = Error(ffi::GPG_ERR_ENGINE_TOO_OLD);
pub const WINDOW_TOO_SMALL: Error = Error(ffi::GPG_ERR_WINDOW_TOO_SMALL);
pub const WINDOW_TOO_LARGE: Error = Error(ffi::GPG_ERR_WINDOW_TOO_LARGE);
pub const MISSING_ENVVAR: Error = Error(ffi::GPG_ERR_MISSING_ENVVAR);
pub const USER_ID_EXISTS: Error = Error(ffi::GPG_ERR_USER_ID_EXISTS);
pub const NAME_EXISTS: Error = Error(ffi::GPG_ERR_NAME_EXISTS);
pub const DUP_NAME: Error = Error(ffi::GPG_ERR_DUP_NAME);
pub const TOO_YOUNG: Error = Error(ffi::GPG_ERR_TOO_YOUNG);
pub const TOO_OLD: Error = Error(ffi::GPG_ERR_TOO_OLD);
pub const UNKNOWN_FLAG: Error = Error(ffi::GPG_ERR_UNKNOWN_FLAG);
pub const INV_ORDER: Error = Error(ffi::GPG_ERR_INV_ORDER);
pub const ALREADY_FETCHED: Error = Error(ffi::GPG_ERR_ALREADY_FETCHED);
pub const TRY_LATER: Error = Error(ffi::GPG_ERR_TRY_LATER);
pub const WRONG_NAME: Error = Error(ffi::GPG_ERR_WRONG_NAME);
pub const NO_AUTH: Error = Error(ffi::GPG_ERR_NO_AUTH);
pub const BAD_AUTH: Error = Error(ffi::GPG_ERR_BAD_AUTH);
pub const SYSTEM_BUG: Error = Error(ffi::GPG_ERR_SYSTEM_BUG);
pub const DNS_UNKNOWN: Error = Error(ffi::GPG_ERR_DNS_UNKNOWN);
pub const DNS_SECTION: Error = Error(ffi::GPG_ERR_DNS_SECTION);
pub const DNS_ADDRESS: Error = Error(ffi::GPG_ERR_DNS_ADDRESS);
pub const DNS_NO_QUERY: Error = Error(ffi::GPG_ERR_DNS_NO_QUERY);
pub const DNS_NO_ANSWER: Error = Error(ffi::GPG_ERR_DNS_NO_ANSWER);
pub const DNS_CLOSED: Error = Error(ffi::GPG_ERR_DNS_CLOSED);
pub const DNS_VERIFY: Error = Error(ffi::GPG_ERR_DNS_VERIFY);
pub const DNS_TIMEOUT: Error = Error(ffi::GPG_ERR_DNS_TIMEOUT);
pub const LDAP_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_GENERAL);
pub const LDAP_ATTR_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_ATTR_GENERAL);
pub const LDAP_NAME_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_NAME_GENERAL);
pub const LDAP_SECURITY_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_SECURITY_GENERAL);
pub const LDAP_SERVICE_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_SERVICE_GENERAL);
pub const LDAP_UPDATE_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_UPDATE_GENERAL);
pub const LDAP_E_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_E_GENERAL);
pub const LDAP_X_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_X_GENERAL);
pub const LDAP_OTHER_GENERAL: Error = Error(ffi::GPG_ERR_LDAP_OTHER_GENERAL);
pub const LDAP_X_CONNECTING: Error = Error(ffi::GPG_ERR_LDAP_X_CONNECTING);
pub const LDAP_REFERRAL_LIMIT: Error = Error(ffi::GPG_ERR_LDAP_REFERRAL_LIMIT);
pub const LDAP_CLIENT_LOOP: Error = Error(ffi::GPG_ERR_LDAP_CLIENT_LOOP);
pub const LDAP_NO_RESULTS: Error = Error(ffi::GPG_ERR_LDAP_NO_RESULTS);
pub const LDAP_CONTROL_NOT_FOUND: Error = Error(ffi::GPG_ERR_LDAP_CONTROL_NOT_FOUND);
pub const LDAP_NOT_SUPPORTED: Error = Error(ffi::GPG_ERR_LDAP_NOT_SUPPORTED);
pub const LDAP_CONNECT: Error = Error(ffi::GPG_ERR_LDAP_CONNECT);
pub const LDAP_NO_MEMORY: Error = Error(ffi::GPG_ERR_LDAP_NO_MEMORY);
pub const LDAP_PARAM: Error = Error(ffi::GPG_ERR_LDAP_PARAM);
pub const LDAP_USER_CANCELLED: Error = Error(ffi::GPG_ERR_LDAP_USER_CANCELLED);
pub const LDAP_FILTER: Error = Error(ffi::GPG_ERR_LDAP_FILTER);
pub const LDAP_AUTH_UNKNOWN: Error = Error(ffi::GPG_ERR_LDAP_AUTH_UNKNOWN);
pub const LDAP_TIMEOUT: Error = Error(ffi::GPG_ERR_LDAP_TIMEOUT);
pub const LDAP_DECODING: Error = Error(ffi::GPG_ERR_LDAP_DECODING);
pub const LDAP_ENCODING: Error = Error(ffi::GPG_ERR_LDAP_ENCODING);
pub const LDAP_LOCAL: Error = Error(ffi::GPG_ERR_LDAP_LOCAL);
pub const LDAP_SERVER_DOWN: Error = Error(ffi::GPG_ERR_LDAP_SERVER_DOWN);
pub const LDAP_SUCCESS: Error = Error(ffi::GPG_ERR_LDAP_SUCCESS);
pub const LDAP_OPERATIONS: Error = Error(ffi::GPG_ERR_LDAP_OPERATIONS);
pub const LDAP_PROTOCOL: Error = Error(ffi::GPG_ERR_LDAP_PROTOCOL);
pub const LDAP_TIMELIMIT: Error = Error(ffi::GPG_ERR_LDAP_TIMELIMIT);
pub const LDAP_SIZELIMIT: Error = Error(ffi::GPG_ERR_LDAP_SIZELIMIT);
pub const LDAP_COMPARE_FALSE: Error = Error(ffi::GPG_ERR_LDAP_COMPARE_FALSE);
pub const LDAP_COMPARE_TRUE: Error = Error(ffi::GPG_ERR_LDAP_COMPARE_TRUE);
pub const LDAP_UNSUPPORTED_AUTH: Error = Error(ffi::GPG_ERR_LDAP_UNSUPPORTED_AUTH);
pub const LDAP_STRONG_AUTH_RQRD: Error = Error(ffi::GPG_ERR_LDAP_STRONG_AUTH_RQRD);
pub const LDAP_PARTIAL_RESULTS: Error = Error(ffi::GPG_ERR_LDAP_PARTIAL_RESULTS);
pub const LDAP_REFERRAL: Error = Error(ffi::GPG_ERR_LDAP_REFERRAL);
pub const LDAP_ADMINLIMIT: Error = Error(ffi::GPG_ERR_LDAP_ADMINLIMIT);
pub const LDAP_UNAVAIL_CRIT_EXTN: Error = Error(ffi::GPG_ERR_LDAP_UNAVAIL_CRIT_EXTN);
pub const LDAP_CONFIDENT_RQRD: Error = Error(ffi::GPG_ERR_LDAP_CONFIDENT_RQRD);
pub const LDAP_SASL_BIND_INPROG: Error = Error(ffi::GPG_ERR_LDAP_SASL_BIND_INPROG);
pub const LDAP_NO_SUCH_ATTRIBUTE: Error = Error(ffi::GPG_ERR_LDAP_NO_SUCH_ATTRIBUTE);
pub const LDAP_UNDEFINED_TYPE: Error = Error(ffi::GPG_ERR_LDAP_UNDEFINED_TYPE);
pub const LDAP_BAD_MATCHING: Error = Error(ffi::GPG_ERR_LDAP_BAD_MATCHING);
pub const LDAP_CONST_VIOLATION: Error = Error(ffi::GPG_ERR_LDAP_CONST_VIOLATION);
pub const LDAP_TYPE_VALUE_EXISTS: Error = Error(ffi::GPG_ERR_LDAP_TYPE_VALUE_EXISTS);
pub const LDAP_INV_SYNTAX: Error = Error(ffi::GPG_ERR_LDAP_INV_SYNTAX);
pub const LDAP_NO_SUCH_OBJ: Error = Error(ffi::GPG_ERR_LDAP_NO_SUCH_OBJ);
pub const LDAP_ALIAS_PROBLEM: Error = Error(ffi::GPG_ERR_LDAP_ALIAS_PROBLEM);
pub const LDAP_INV_DN_SYNTAX: Error = Error(ffi::GPG_ERR_LDAP_INV_DN_SYNTAX);
pub const LDAP_IS_LEAF: Error = Error(ffi::GPG_ERR_LDAP_IS_LEAF);
pub const LDAP_ALIAS_DEREF: Error = Error(ffi::GPG_ERR_LDAP_ALIAS_DEREF);
pub const LDAP_X_PROXY_AUTH_FAIL: Error = Error(ffi::GPG_ERR_LDAP_X_PROXY_AUTH_FAIL);
pub const LDAP_BAD_AUTH: Error = Error(ffi::GPG_ERR_LDAP_BAD_AUTH);
pub const LDAP_INV_CREDENTIALS: Error = Error(ffi::GPG_ERR_LDAP_INV_CREDENTIALS);
pub const LDAP_INSUFFICIENT_ACC: Error = Error(ffi::GPG_ERR_LDAP_INSUFFICIENT_ACC);
pub const LDAP_BUSY: Error = Error(ffi::GPG_ERR_LDAP_BUSY);
pub const LDAP_UNAVAILABLE: Error = Error(ffi::GPG_ERR_LDAP_UNAVAILABLE);
pub const LDAP_UNWILL_TO_PERFORM: Error = Error(ffi::GPG_ERR_LDAP_UNWILL_TO_PERFORM);
pub const LDAP_LOOP_DETECT: Error = Error(ffi::GPG_ERR_LDAP_LOOP_DETECT);
pub const LDAP_NAMING_VIOLATION: Error = Error(ffi::GPG_ERR_LDAP_NAMING_VIOLATION);
pub const LDAP_OBJ_CLS_VIOLATION: Error = Error(ffi::GPG_ERR_LDAP_OBJ_CLS_VIOLATION);
pub const LDAP_NOT_ALLOW_NONLEAF: Error = Error(ffi::GPG_ERR_LDAP_NOT_ALLOW_NONLEAF);
pub const LDAP_NOT_ALLOW_ON_RDN: Error = Error(ffi::GPG_ERR_LDAP_NOT_ALLOW_ON_RDN);
pub const LDAP_ALREADY_EXISTS: Error = Error(ffi::GPG_ERR_LDAP_ALREADY_EXISTS);
pub const LDAP_NO_OBJ_CLASS_MODS: Error = Error(ffi::GPG_ERR_LDAP_NO_OBJ_CLASS_MODS);
pub const LDAP_RESULTS_TOO_LARGE: Error = Error(ffi::GPG_ERR_LDAP_RESULTS_TOO_LARGE);
pub const LDAP_AFFECTS_MULT_DSAS: Error = Error(ffi::GPG_ERR_LDAP_AFFECTS_MULT_DSAS);
pub const LDAP_VLV: Error = Error(ffi::GPG_ERR_LDAP_VLV);
pub const LDAP_OTHER: Error = Error(ffi::GPG_ERR_LDAP_OTHER);
pub const LDAP_CUP_RESOURCE_LIMIT: Error = Error(ffi::GPG_ERR_LDAP_CUP_RESOURCE_LIMIT);
pub const LDAP_CUP_SEC_VIOLATION: Error = Error(ffi::GPG_ERR_LDAP_CUP_SEC_VIOLATION);
pub const LDAP_CUP_INV_DATA: Error = Error(ffi::GPG_ERR_LDAP_CUP_INV_DATA);
pub const LDAP_CUP_UNSUP_SCHEME: Error = Error(ffi::GPG_ERR_LDAP_CUP_UNSUP_SCHEME);
pub const LDAP_CUP_RELOAD: Error = Error(ffi::GPG_ERR_LDAP_CUP_RELOAD);
pub const LDAP_CANCELLED: Error = Error(ffi::GPG_ERR_LDAP_CANCELLED);
pub const LDAP_NO_SUCH_OPERATION: Error = Error(ffi::GPG_ERR_LDAP_NO_SUCH_OPERATION);
pub const LDAP_TOO_LATE: Error = Error(ffi::GPG_ERR_LDAP_TOO_LATE);
pub const LDAP_CANNOT_CANCEL: Error = Error(ffi::GPG_ERR_LDAP_CANNOT_CANCEL);
pub const LDAP_ASSERTION_FAILED: Error = Error(ffi::GPG_ERR_LDAP_ASSERTION_FAILED);
pub const LDAP_PROX_AUTH_DENIED: Error = Error(ffi::GPG_ERR_LDAP_PROX_AUTH_DENIED);
pub const USER_1: Error = Error(ffi::GPG_ERR_USER_1);
pub const USER_2: Error = Error(ffi::GPG_ERR_USER_2);
pub const USER_3: Error = Error(ffi::GPG_ERR_USER_3);
pub const USER_4: Error = Error(ffi::GPG_ERR_USER_4);
pub const USER_5: Error = Error(ffi::GPG_ERR_USER_5);
pub const USER_6: Error = Error(ffi::GPG_ERR_USER_6);
pub const USER_7: Error = Error(ffi::GPG_ERR_USER_7);
pub const USER_8: Error = Error(ffi::GPG_ERR_USER_8);
pub const USER_9: Error = Error(ffi::GPG_ERR_USER_9);
pub const USER_10: Error = Error(ffi::GPG_ERR_USER_10);
pub const USER_11: Error = Error(ffi::GPG_ERR_USER_11);
pub const USER_12: Error = Error(ffi::GPG_ERR_USER_12);
pub const USER_13: Error = Error(ffi::GPG_ERR_USER_13);
pub const USER_14: Error = Error(ffi::GPG_ERR_USER_14);
pub const USER_15: Error = Error(ffi::GPG_ERR_USER_15);
pub const USER_16: Error = Error(ffi::GPG_ERR_USER_16);
pub const MISSING_ERRNO: Error = Error(ffi::GPG_ERR_MISSING_ERRNO);
pub const UNKNOWN_ERRNO: Error = Error(ffi::GPG_ERR_UNKNOWN_ERRNO);
pub const EOF: Error = Error(ffi::GPG_ERR_EOF);
pub const E2BIG: Error = Error(ffi::GPG_ERR_E2BIG);
pub const EACCES: Error = Error(ffi::GPG_ERR_EACCES);
pub const EADDRINUSE: Error = Error(ffi::GPG_ERR_EADDRINUSE);
pub const EADDRNOTAVAIL: Error = Error(ffi::GPG_ERR_EADDRNOTAVAIL);
pub const EADV: Error = Error(ffi::GPG_ERR_EADV);
pub const EAFNOSUPPORT: Error = Error(ffi::GPG_ERR_EAFNOSUPPORT);
pub const EAGAIN: Error = Error(ffi::GPG_ERR_EAGAIN);
pub const EALREADY: Error = Error(ffi::GPG_ERR_EALREADY);
pub const EAUTH: Error = Error(ffi::GPG_ERR_EAUTH);
pub const EBACKGROUND: Error = Error(ffi::GPG_ERR_EBACKGROUND);
pub const EBADE: Error = Error(ffi::GPG_ERR_EBADE);
pub const EBADF: Error = Error(ffi::GPG_ERR_EBADF);
pub const EBADFD: Error = Error(ffi::GPG_ERR_EBADFD);
pub const EBADMSG: Error = Error(ffi::GPG_ERR_EBADMSG);
pub const EBADR: Error = Error(ffi::GPG_ERR_EBADR);
pub const EBADRPC: Error = Error(ffi::GPG_ERR_EBADRPC);
pub const EBADRQC: Error = Error(ffi::GPG_ERR_EBADRQC);
pub const EBADSLT: Error = Error(ffi::GPG_ERR_EBADSLT);
pub const EBFONT: Error = Error(ffi::GPG_ERR_EBFONT);
pub const EBUSY: Error = Error(ffi::GPG_ERR_EBUSY);
pub const ECANCELED: Error = Error(ffi::GPG_ERR_ECANCELED);
pub const ECHILD: Error = Error(ffi::GPG_ERR_ECHILD);
pub const ECHRNG: Error = Error(ffi::GPG_ERR_ECHRNG);
pub const ECOMM: Error = Error(ffi::GPG_ERR_ECOMM);
pub const ECONNABORTED: Error = Error(ffi::GPG_ERR_ECONNABORTED);
pub const ECONNREFUSED: Error = Error(ffi::GPG_ERR_ECONNREFUSED);
pub const ECONNRESET: Error = Error(ffi::GPG_ERR_ECONNRESET);
pub const ED: Error = Error(ffi::GPG_ERR_ED);
pub const EDEADLK: Error = Error(ffi::GPG_ERR_EDEADLK);
pub const EDEADLOCK: Error = Error(ffi::GPG_ERR_EDEADLOCK);
pub const EDESTADDRREQ: Error = Error(ffi::GPG_ERR_EDESTADDRREQ);
pub const EDIED: Error = Error(ffi::GPG_ERR_EDIED);
pub const EDOM: Error = Error(ffi::GPG_ERR_EDOM);
pub const EDOTDOT: Error = Error(ffi::GPG_ERR_EDOTDOT);
pub const EDQUOT: Error = Error(ffi::GPG_ERR_EDQUOT);
pub const EEXIST: Error = Error(ffi::GPG_ERR_EEXIST);
pub const EFAULT: Error = Error(ffi::GPG_ERR_EFAULT);
pub const EFBIG: Error = Error(ffi::GPG_ERR_EFBIG);
pub const EFTYPE: Error = Error(ffi::GPG_ERR_EFTYPE);
pub const EGRATUITOUS: Error = Error(ffi::GPG_ERR_EGRATUITOUS);
pub const EGREGIOUS: Error = Error(ffi::GPG_ERR_EGREGIOUS);
pub const EHOSTDOWN: Error = Error(ffi::GPG_ERR_EHOSTDOWN);
pub const EHOSTUNREACH: Error = Error(ffi::GPG_ERR_EHOSTUNREACH);
pub const EIDRM: Error = Error(ffi::GPG_ERR_EIDRM);
pub const EIEIO: Error = Error(ffi::GPG_ERR_EIEIO);
pub const EILSEQ: Error = Error(ffi::GPG_ERR_EILSEQ);
pub const EINPROGRESS: Error = Error(ffi::GPG_ERR_EINPROGRESS);
pub const EINTR: Error = Error(ffi::GPG_ERR_EINTR);
pub const EINVAL: Error = Error(ffi::GPG_ERR_EINVAL);
pub const EIO: Error = Error(ffi::GPG_ERR_EIO);
pub const EISCONN: Error = Error(ffi::GPG_ERR_EISCONN);
pub const EISDIR: Error = Error(ffi::GPG_ERR_EISDIR);
pub const EISNAM: Error = Error(ffi::GPG_ERR_EISNAM);
pub const EL2HLT: Error = Error(ffi::GPG_ERR_EL2HLT);
pub const EL2NSYNC: Error = Error(ffi::GPG_ERR_EL2NSYNC);
pub const EL3HLT: Error = Error(ffi::GPG_ERR_EL3HLT);
pub const EL3RST: Error = Error(ffi::GPG_ERR_EL3RST);
pub const ELIBACC: Error = Error(ffi::GPG_ERR_ELIBACC);
pub const ELIBBAD: Error = Error(ffi::GPG_ERR_ELIBBAD);
pub const ELIBEXEC: Error = Error(ffi::GPG_ERR_ELIBEXEC);
pub const ELIBMAX: Error = Error(ffi::GPG_ERR_ELIBMAX);
pub const ELIBSCN: Error = Error(ffi::GPG_ERR_ELIBSCN);
pub const ELNRNG: Error = Error(ffi::GPG_ERR_ELNRNG);
pub const ELOOP: Error = Error(ffi::GPG_ERR_ELOOP);
pub const EMEDIUMTYPE: Error = Error(ffi::GPG_ERR_EMEDIUMTYPE);
pub const EMFILE: Error = Error(ffi::GPG_ERR_EMFILE);
pub const EMLINK: Error = Error(ffi::GPG_ERR_EMLINK);
pub const EMSGSIZE: Error = Error(ffi::GPG_ERR_EMSGSIZE);
pub const EMULTIHOP: Error = Error(ffi::GPG_ERR_EMULTIHOP);
pub const ENAMETOOLONG: Error = Error(ffi::GPG_ERR_ENAMETOOLONG);
pub const ENAVAIL: Error = Error(ffi::GPG_ERR_ENAVAIL);
pub const ENEEDAUTH: Error = Error(ffi::GPG_ERR_ENEEDAUTH);
pub const ENETDOWN: Error = Error(ffi::GPG_ERR_ENETDOWN);
pub const ENETRESET: Error = Error(ffi::GPG_ERR_ENETRESET);
pub const ENETUNREACH: Error = Error(ffi::GPG_ERR_ENETUNREACH);
pub const ENFILE: Error = Error(ffi::GPG_ERR_ENFILE);
pub const ENOANO: Error = Error(ffi::GPG_ERR_ENOANO);
pub const ENOBUFS: Error = Error(ffi::GPG_ERR_ENOBUFS);
pub const ENOCSI: Error = Error(ffi::GPG_ERR_ENOCSI);
pub const ENODATA: Error = Error(ffi::GPG_ERR_ENODATA);
pub const ENODEV: Error = Error(ffi::GPG_ERR_ENODEV);
pub const ENOENT: Error = Error(ffi::GPG_ERR_ENOENT);
pub const ENOEXEC: Error = Error(ffi::GPG_ERR_ENOEXEC);
pub const ENOLCK: Error = Error(ffi::GPG_ERR_ENOLCK);
pub const ENOLINK: Error = Error(ffi::GPG_ERR_ENOLINK);
pub const ENOMEDIUM: Error = Error(ffi::GPG_ERR_ENOMEDIUM);
pub const ENOMEM: Error = Error(ffi::GPG_ERR_ENOMEM);
pub const ENOMSG: Error = Error(ffi::GPG_ERR_ENOMSG);
pub const ENONET: Error = Error(ffi::GPG_ERR_ENONET);
pub const ENOPKG: Error = Error(ffi::GPG_ERR_ENOPKG);
pub const ENOPROTOOPT: Error = Error(ffi::GPG_ERR_ENOPROTOOPT);
pub const ENOSPC: Error = Error(ffi::GPG_ERR_ENOSPC);
pub const ENOSR: Error = Error(ffi::GPG_ERR_ENOSR);
pub const ENOSTR: Error = Error(ffi::GPG_ERR_ENOSTR);
pub const ENOSYS: Error = Error(ffi::GPG_ERR_ENOSYS);
pub const ENOTBLK: Error = Error(ffi::GPG_ERR_ENOTBLK);
pub const ENOTCONN: Error = Error(ffi::GPG_ERR_ENOTCONN);
pub const ENOTDIR: Error = Error(ffi::GPG_ERR_ENOTDIR);
pub const ENOTEMPTY: Error = Error(ffi::GPG_ERR_ENOTEMPTY);
pub const ENOTNAM: Error = Error(ffi::GPG_ERR_ENOTNAM);
pub const ENOTSOCK: Error = Error(ffi::GPG_ERR_ENOTSOCK);
pub const ENOTSUP: Error = Error(ffi::GPG_ERR_ENOTSUP);
pub const ENOTTY: Error = Error(ffi::GPG_ERR_ENOTTY);
pub const ENOTUNIQ: Error = Error(ffi::GPG_ERR_ENOTUNIQ);
pub const ENXIO: Error = Error(ffi::GPG_ERR_ENXIO);
pub const EOPNOTSUPP: Error = Error(ffi::GPG_ERR_EOPNOTSUPP);
pub const EOVERFLOW: Error = Error(ffi::GPG_ERR_EOVERFLOW);
pub const EPERM: Error = Error(ffi::GPG_ERR_EPERM);
pub const EPFNOSUPPORT: Error = Error(ffi::GPG_ERR_EPFNOSUPPORT);
pub const EPIPE: Error = Error(ffi::GPG_ERR_EPIPE);
pub const EPROCLIM: Error = Error(ffi::GPG_ERR_EPROCLIM);
pub const EPROCUNAVAIL: Error = Error(ffi::GPG_ERR_EPROCUNAVAIL);
pub const EPROGMISMATCH: Error = Error(ffi::GPG_ERR_EPROGMISMATCH);
pub const EPROGUNAVAIL: Error = Error(ffi::GPG_ERR_EPROGUNAVAIL);
pub const EPROTO: Error = Error(ffi::GPG_ERR_EPROTO);
pub const EPROTONOSUPPORT: Error = Error(ffi::GPG_ERR_EPROTONOSUPPORT);
pub const EPROTOTYPE: Error = Error(ffi::GPG_ERR_EPROTOTYPE);
pub const ERANGE: Error = Error(ffi::GPG_ERR_ERANGE);
pub const EREMCHG: Error = Error(ffi::GPG_ERR_EREMCHG);
pub const EREMOTE: Error = Error(ffi::GPG_ERR_EREMOTE);
pub const EREMOTEIO: Error = Error(ffi::GPG_ERR_EREMOTEIO);
pub const ERESTART: Error = Error(ffi::GPG_ERR_ERESTART);
pub const EROFS: Error = Error(ffi::GPG_ERR_EROFS);
pub const ERPCMISMATCH: Error = Error(ffi::GPG_ERR_ERPCMISMATCH);
pub const ESHUTDOWN: Error = Error(ffi::GPG_ERR_ESHUTDOWN);
pub const ESOCKTNOSUPPORT: Error = Error(ffi::GPG_ERR_ESOCKTNOSUPPORT);
pub const ESPIPE: Error = Error(ffi::GPG_ERR_ESPIPE);
pub const ESRCH: Error = Error(ffi::GPG_ERR_ESRCH);
pub const ESRMNT: Error = Error(ffi::GPG_ERR_ESRMNT);
pub const ESTALE: Error = Error(ffi::GPG_ERR_ESTALE);
pub const ESTRPIPE: Error = Error(ffi::GPG_ERR_ESTRPIPE);
pub const ETIME: Error = Error(ffi::GPG_ERR_ETIME);
pub const ETIMEDOUT: Error = Error(ffi::GPG_ERR_ETIMEDOUT);
pub const ETOOMANYREFS: Error = Error(ffi::GPG_ERR_ETOOMANYREFS);
pub const ETXTBSY: Error = Error(ffi::GPG_ERR_ETXTBSY);
pub const EUCLEAN: Error = Error(ffi::GPG_ERR_EUCLEAN);
pub const EUNATCH: Error = Error(ffi::GPG_ERR_EUNATCH);
pub const EUSERS: Error = Error(ffi::GPG_ERR_EUSERS);
pub const EWOULDBLOCK: Error = Error(ffi::GPG_ERR_EWOULDBLOCK);
pub const EXDEV: Error = Error(ffi::GPG_ERR_EXDEV);
pub const EXFULL: Error = Error(ffi::GPG_ERR_EXFULL);
}
