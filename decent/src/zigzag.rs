use std::{
    fmt::Display,
    io::{self, Read, Write},
    ops::{AddAssign, BitAnd, ShrAssign, SubAssign},
};

use num::{BigInt, BigUint, Signed, Zero, bigint::Sign};

pub mod unsigned {
    pub const CONTINUATION_BIT: u8 = 0b1000_0000;
}

/// Writes a single unsigned variable integer into a stream.
/// Used for the [Varint][`PrimitiveRepr::Varint`] repr on unsigned integer types, as well as for every [BigUint][`num::BigUint`].
pub fn write_varuint<
    I: Zero + PartialOrd + Clone + TryFrom<u8> + TryInto<u8> + ShrAssign<u32> + BitAnd<I, Output = I>,
>(
    mut uint: I,
    to: &mut dyn Write,
) -> io::Result<()>
where
    <I as TryFrom<u8>>::Error: Display,
    <I as TryInto<u8>>::Error: Display,
{
    let zero = I::zero();

    loop {
        let mask = I::try_from(0b0111_1111u8)
            .map_err(|err| io::Error::other(format!("failed to encode byte ({err})")))?;

        let mut byte = (uint.clone() & mask)
            .try_into()
            .map_err(|err| io::Error::other(format!("failed to encode byte ({err})")))?;

        uint >>= 7;
        if uint > zero {
            byte |= unsigned::CONTINUATION_BIT;
        }

        to.write_all(&[byte])?;

        if uint <= zero {
            break;
        }
    }

    Ok(())
}

/// Writes a single signed variable integer into a stream.
/// Used for the [Varint][`PrimitiveRepr::Varint`] repr on signed integer types, as well as for every [BigInt][`num::BigInt`].
pub fn write_varint<
    I: Signed
        + SubAssign
        + TryFrom<u8>
        + TryInto<u8>
        + PartialOrd
        + Clone
        + ShrAssign<u32>
        + BitAnd<Output = I>,
>(
    int: I,
    to: &mut dyn Write,
) -> io::Result<()>
where
    <I as TryFrom<u8>>::Error: Display,
    <I as TryInto<u8>>::Error: Display,
{
    let abs = int.abs();
    let mut number = abs.clone() + abs;
    if int.is_negative() {
        number -= I::one();
    }
    write_varuint(number, to)?;
    Ok(())
}

// TODO: do some num trickery to not use bigint?
pub fn read_varuint(from: &mut dyn Read) -> io::Result<BigUint> {
    let mut out = BigUint::ZERO;
    let mut bit_index = 0u32;
    loop {
        let mut buf = [0u8];
        from.read_exact(&mut buf)?;
        let byte = buf[0];
        out |= BigUint::from(byte & !unsigned::CONTINUATION_BIT) << bit_index;
        if byte & unsigned::CONTINUATION_BIT == 0 {
            break;
        }
        bit_index += 7;
    }
    Ok(out)
}

pub fn read_varint(from: &mut dyn Read) -> io::Result<BigInt> {
    let int = read_varuint(from)?;
    let sign_bit_set = int.trailing_ones() > 0;
    let integer = BigInt::from_biguint(
        if sign_bit_set {
            Sign::Minus
        } else {
            Sign::Plus
        },
        (int >> 1) + if sign_bit_set { 1u32 } else { 0 },
    );
    Ok(integer)
}
