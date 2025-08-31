#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(maybe_uninit_array_assume_init)]

//! This crate defines opinionated utilities for creating version-aware binary formats.
//!
//! Aims to be easy to use and future-proofable.

pub mod impls;
pub mod iter;
pub mod read;
pub mod zigzag;
pub mod write;
mod test;

pub use read::Decodable;
pub use write::Encodable;

pub trait BinarySerialisable: Encodable + Decodable {}
impl<T: Encodable + Decodable> BinarySerialisable for T {}

/// A representation a primitive value should take. Uses variable-length encoding by default.
///
/// ## Variable integers
/// The [Varint][`PrimitiveRepr::Varint`] encoding specifies that the encoding should be a variable-integer.
/// This is unlike the endianness representations, and is more Unicode-like.
///
/// Every byte in a variable integer consists of a continuation bit (as the MSB) and 7 data bits.
/// If the continuation bit is set, that means there is another byte after that byte.
/// Otherwise, if it is clear, that byte is the last one in the variable integer.
///
/// All the data bits are concatenated into one integer.
/// The ordering is little-endian (the first byte holds the 7 lowest bits).
///
/// As an example, the unsigned value `10` would be encoded as `00001010`. The unsigned value `127` would be encoded as `01111111`.
/// However, the value `128` would be encoded as `10000000 00000001`, as 7 bits is not enough to represent 128.
/// The eighth bit gets pushed into the second byte in a variable integer.
/// `0000001` and `0000000` are concatenated together to make `10000000`, which is `128` in binary.
///
/// ### Signed integers
/// The above explanation applies entirely for unsigned integers. Signed integers are handled ever-so-slightly differently:
/// - The most significant bit of the first byte is the sign bit.
/// - The continuation bit is moved to the second-most-significant bit.
/// - There are only 6 data bits in the first byte.
///
/// All other bytes are treated the same as with unsigned integers, and all bits are concatenated the same.
///
/// When the sign bit is set, the resulting unsigned result of concatenating all the bits is multiplied by `-1` to produce the signed result.
/// This makes signed variable integers ones-complement (`0` and `-0` are representable).
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrimitiveRepr {
    /// Big-endianness. Often called network endianness, and used most often for standards.
    BigEndian,
    /// Little-endianness.
    LittleEndian,
    // TODO: gate this behind the `platform_dependent` feature
    /// Native-endianness. Platform sensitive, so the most common application is for internal messaging.
    Native,
    /// Variable-length encoding with a continuation bit. Uses zig-zag encoding for signed integers.
    #[default]
    Varint,
}

/// A major.minor.patch version type. Used to conditionally encode/decode fields.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u64, pub u64, pub u64);
impl Version {
    pub const ZERO: Self = Version(0, 0, 0);
}
