/// A list of provided hashes, implementing `Hash`.
#[derive(Debug, Clone, Copy)]
pub enum Hash {
    MD5,
    SHA1,
    SHA2_224,
    SHA2_256,
    SHA2_384,
    SHA2_512,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    MD5SHA1,
    RIPEMD160,
}

impl Hash {
    /// Returns the length in bytes of a digest.
    pub fn size(&self) -> usize {
        match *self {
            Hash::MD5 => 16,
            Hash::SHA1 => 20,
            Hash::SHA2_224 => 28,
            Hash::SHA2_256 => 32,
            Hash::SHA2_384 => 48,
            Hash::SHA2_512 => 64,
            Hash::SHA3_256 => 32,
            Hash::SHA3_384 => 48,
            Hash::SHA3_512 => 64,
            Hash::MD5SHA1 => 36,
            Hash::RIPEMD160 => 20,
        }
    }

    /// Returns the ASN1 DER prefix for the the hash function.
    pub fn asn1_prefix(&self) -> &'static [u8] {
        match *self {
            Hash::MD5 => &[
                0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x05,
                0x05, 0x00, 0x04, 0x10,
            ],
            Hash::SHA1 => &[
                0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04,
                0x14,
            ],
            Hash::SHA2_224 => &[
                0x30, 0x2d, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x04, 0x05, 0x00, 0x04, 0x1c,
            ],
            Hash::SHA2_256 => &[
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x01, 0x05, 0x00, 0x04, 0x20,
            ],
            Hash::SHA2_384 => &[
                0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x02, 0x05, 0x00, 0x04, 0x30,
            ],

            Hash::SHA2_512 => &[
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x03, 0x05, 0x00, 0x04, 0x40,
            ],

            // A special TLS case which doesn't use an ASN1 prefix
            Hash::MD5SHA1 => &[],
            Hash::RIPEMD160 => &[
                0x30, 0x20, 0x30, 0x08, 0x06, 0x06, 0x28, 0xcf, 0x06, 0x03, 0x00, 0x31, 0x04, 0x14,
            ],

            Hash::SHA3_256 => &[
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x08, 0x05, 0x00, 0x04, 0x20,
            ],
            Hash::SHA3_384 => &[
                30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x08, 0x05, 0x00, 0x04, 0x20,
            ],

            Hash::SHA3_512 => &[
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x0a, 0x05, 0x00, 0x04, 0x40,
            ],
        }
    }
}

/* FIXME: This trait should be refactored into per-digest implementations returning OID */
pub trait AssociatedHash {
    const HASH: Hash;
}

#[cfg(feature = "sha1")]
impl AssociatedHash for sha1::Sha1 {
    const HASH: Hash = Hash::SHA1;
}

#[cfg(feature = "sha2")]
impl AssociatedHash for sha2::Sha224 {
    const HASH: Hash = Hash::SHA2_224;
}

#[cfg(feature = "sha2")]
impl AssociatedHash for sha2::Sha256 {
    const HASH: Hash = Hash::SHA2_256;
}

#[cfg(feature = "sha2")]
impl AssociatedHash for sha2::Sha384 {
    const HASH: Hash = Hash::SHA2_384;
}

#[cfg(feature = "sha2")]
impl AssociatedHash for sha2::Sha512 {
    const HASH: Hash = Hash::SHA2_512;
}

/*
#[cfg(feature = "sha3")]
impl AssociatedHash for sha3::Sha3_224 {
    const HASH: Hash = Hash::SHA3_224;
}
*/

#[cfg(feature = "sha3")]
impl AssociatedHash for sha3::Sha3_256 {
    const HASH: Hash = Hash::SHA3_256;
}

#[cfg(feature = "sha3")]
impl AssociatedHash for sha3::Sha3_384 {
    const HASH: Hash = Hash::SHA3_384;
}

#[cfg(feature = "sha3")]
impl AssociatedHash for sha3::Sha3_512 {
    const HASH: Hash = Hash::SHA3_512;
}
