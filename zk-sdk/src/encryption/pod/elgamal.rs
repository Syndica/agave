//! Plain Old Data types for the ElGamal encryption scheme.

#[cfg(not(target_os = "solana"))]
use {
    crate::{
        encryption::elgamal::{DecryptHandle, ElGamalCiphertext, ElGamalPubkey},
        errors::ElGamalError,
    },
    curve25519_dalek::ristretto::CompressedRistretto,
};
use {
    crate::{
        encryption::{DECRYPT_HANDLE_LEN, ELGAMAL_CIPHERTEXT_LEN, ELGAMAL_PUBKEY_LEN},
        pod::{impl_from_bytes, impl_from_str},
    },
    base64::{prelude::BASE64_STANDARD, Engine},
    bytemuck::Zeroable,
    std::fmt,
};
#[cfg(target_arch = "wasm32")]
use {
    js_sys::{Array, Uint8Array},
    wasm_bindgen::prelude::*,
};

/// Maximum length of a base64 encoded ElGamal public key
const ELGAMAL_PUBKEY_MAX_BASE64_LEN: usize = 44;

/// Maximum length of a base64 encoded ElGamal ciphertext
const ELGAMAL_CIPHERTEXT_MAX_BASE64_LEN: usize = 88;

/// Maximum length of a base64 encoded ElGamal decrypt handle
const DECRYPT_HANDLE_MAX_BASE64_LEN: usize = 44;

/// The `ElGamalCiphertext` type as a `Pod`.
#[derive(Clone, Copy, bytemuck_derive::Pod, bytemuck_derive::Zeroable, PartialEq, Eq)]
#[repr(transparent)]
pub struct PodElGamalCiphertext(pub(crate) [u8; ELGAMAL_CIPHERTEXT_LEN]);

impl fmt::Debug for PodElGamalCiphertext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for PodElGamalCiphertext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.0))
    }
}

impl Default for PodElGamalCiphertext {
    fn default() -> Self {
        Self::zeroed()
    }
}

impl_from_str!(
    TYPE = PodElGamalCiphertext,
    BYTES_LEN = ELGAMAL_CIPHERTEXT_LEN,
    BASE64_LEN = ELGAMAL_CIPHERTEXT_MAX_BASE64_LEN
);

impl_from_bytes!(
    TYPE = PodElGamalCiphertext,
    BYTES_LEN = ELGAMAL_CIPHERTEXT_LEN
);

#[cfg(not(target_os = "solana"))]
impl From<ElGamalCiphertext> for PodElGamalCiphertext {
    fn from(decoded_ciphertext: ElGamalCiphertext) -> Self {
        Self(decoded_ciphertext.to_bytes())
    }
}

#[cfg(not(target_os = "solana"))]
impl TryFrom<PodElGamalCiphertext> for ElGamalCiphertext {
    type Error = ElGamalError;

    fn try_from(pod_ciphertext: PodElGamalCiphertext) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_ciphertext.0).ok_or(ElGamalError::CiphertextDeserialization)
    }
}

/// The `ElGamalPubkey` type as a `Pod`.
#[derive(Clone, Copy, Default, bytemuck_derive::Pod, bytemuck_derive::Zeroable, PartialEq, Eq)]
#[repr(transparent)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct PodElGamalPubkey(pub(crate) [u8; ELGAMAL_PUBKEY_LEN]);

#[cfg(target_arch = "wasm32")]
#[allow(non_snake_case)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PodElGamalPubkey {
    /// Create a new `PodElGamalPubkey` object
    ///
    /// * `value` - optional public key as a base64 encoded string, `Uint8Array`, `[number]`
    #[wasm_bindgen(constructor)]
    pub fn constructor(value: JsValue) -> Result<PodElGamalPubkey, JsValue> {
        if let Some(base64_str) = value.as_string() {
            base64_str
                .parse::<PodElGamalPubkey>()
                .map_err(|e| e.to_string().into())
        } else if let Some(uint8_array) = value.dyn_ref::<Uint8Array>() {
            bytemuck::try_from_bytes(&uint8_array.to_vec())
                .map_err(|err| JsValue::from(format!("Invalid Uint8Array ElGamalPubkey: {err:?}")))
                .map(|pubkey| *pubkey)
        } else if let Some(array) = value.dyn_ref::<Array>() {
            let mut bytes = vec![];
            let iterator = js_sys::try_iter(&array.values())?.expect("array to be iterable");
            for x in iterator {
                let x = x?;

                if let Some(n) = x.as_f64() {
                    if (0. ..=255.).contains(&n) {
                        bytes.push(n as u8);
                        continue;
                    }
                }
                return Err(format!("Invalid array argument: {:?}", x).into());
            }

            bytemuck::try_from_bytes(&bytes)
                .map_err(|err| JsValue::from(format!("Invalid Array pubkey: {err:?}")))
                .map(|pubkey| *pubkey)
        } else if value.is_undefined() {
            Ok(PodElGamalPubkey::default())
        } else {
            Err("Unsupported argument".into())
        }
    }

    /// Return the base64 string representation of the public key
    pub fn toString(&self) -> String {
        self.to_string()
    }

    /// Checks if two `ElGamalPubkey`s are equal
    pub fn equals(&self, other: &PodElGamalPubkey) -> bool {
        self == other
    }

    /// Return the `Uint8Array` representation of the public key
    pub fn toBytes(&self) -> Box<[u8]> {
        self.0.into()
    }

    pub fn compressed(decoded: &ElGamalPubkey) -> PodElGamalPubkey {
        (*decoded).into()
    }

    pub fn decompressed(&self) -> Result<ElGamalPubkey, JsValue> {
        (*self)
            .try_into()
            .map_err(|err| JsValue::from(format!("Invalid ElGamalPubkey: {err:?}")))
    }
}

impl fmt::Debug for PodElGamalPubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for PodElGamalPubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.0))
    }
}

impl_from_str!(
    TYPE = PodElGamalPubkey,
    BYTES_LEN = ELGAMAL_PUBKEY_LEN,
    BASE64_LEN = ELGAMAL_PUBKEY_MAX_BASE64_LEN
);

impl_from_bytes!(TYPE = PodElGamalPubkey, BYTES_LEN = ELGAMAL_PUBKEY_LEN);

#[cfg(not(target_os = "solana"))]
impl From<ElGamalPubkey> for PodElGamalPubkey {
    fn from(decoded_pubkey: ElGamalPubkey) -> Self {
        Self(decoded_pubkey.into())
    }
}

#[cfg(not(target_os = "solana"))]
impl TryFrom<PodElGamalPubkey> for ElGamalPubkey {
    type Error = ElGamalError;

    fn try_from(pod_pubkey: PodElGamalPubkey) -> Result<Self, Self::Error> {
        Self::try_from(pod_pubkey.0.as_slice())
    }
}

/// The `DecryptHandle` type as a `Pod`.
#[derive(Clone, Copy, Default, bytemuck_derive::Pod, bytemuck_derive::Zeroable, PartialEq, Eq)]
#[repr(transparent)]
pub struct PodDecryptHandle(pub(crate) [u8; DECRYPT_HANDLE_LEN]);

impl fmt::Debug for PodDecryptHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(not(target_os = "solana"))]
impl From<DecryptHandle> for PodDecryptHandle {
    fn from(decoded_handle: DecryptHandle) -> Self {
        Self(decoded_handle.to_bytes())
    }
}

// For proof verification, interpret pod::DecryptHandle as CompressedRistretto
#[cfg(not(target_os = "solana"))]
impl From<PodDecryptHandle> for CompressedRistretto {
    fn from(pod_handle: PodDecryptHandle) -> Self {
        Self(pod_handle.0)
    }
}

#[cfg(not(target_os = "solana"))]
impl TryFrom<PodDecryptHandle> for DecryptHandle {
    type Error = ElGamalError;

    fn try_from(pod_handle: PodDecryptHandle) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_handle.0).ok_or(ElGamalError::CiphertextDeserialization)
    }
}

impl fmt::Display for PodDecryptHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.0))
    }
}

impl_from_str!(
    TYPE = PodDecryptHandle,
    BYTES_LEN = DECRYPT_HANDLE_LEN,
    BASE64_LEN = DECRYPT_HANDLE_MAX_BASE64_LEN
);

impl_from_bytes!(TYPE = PodDecryptHandle, BYTES_LEN = DECRYPT_HANDLE_LEN);

#[cfg(test)]
mod tests {
    use {super::*, crate::encryption::elgamal::ElGamalKeypair, std::str::FromStr};

    #[test]
    fn elgamal_pubkey_fromstr() {
        let elgamal_keypair = ElGamalKeypair::new_rand();
        let expected_elgamal_pubkey: PodElGamalPubkey = (*elgamal_keypair.pubkey()).into();

        let elgamal_pubkey_base64_str = format!("{}", expected_elgamal_pubkey);
        let computed_elgamal_pubkey =
            PodElGamalPubkey::from_str(&elgamal_pubkey_base64_str).unwrap();

        assert_eq!(expected_elgamal_pubkey, computed_elgamal_pubkey);
    }

    #[test]
    fn elgamal_ciphertext_fromstr() {
        let elgamal_keypair = ElGamalKeypair::new_rand();
        let expected_elgamal_ciphertext: PodElGamalCiphertext =
            elgamal_keypair.pubkey().encrypt(0_u64).into();

        let elgamal_ciphertext_base64_str = format!("{}", expected_elgamal_ciphertext);
        let computed_elgamal_ciphertext =
            PodElGamalCiphertext::from_str(&elgamal_ciphertext_base64_str).unwrap();

        assert_eq!(expected_elgamal_ciphertext, computed_elgamal_ciphertext);
    }
}
