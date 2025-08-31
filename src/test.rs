#[cfg(test)]
mod tests {
    use crate::{Decodable, Encodable, PrimitiveRepr, Version};
    use num::{BigInt, BigUint, bigint::Sign};
    use std::fmt::Debug;

    fn round_trip<T: Encodable + Decodable + PartialEq + Debug>(source: T, repr: PrimitiveRepr) {
        let mut destination = vec![];
        source
            .encode(&mut destination, Version::ZERO, repr)
            .unwrap();
        println!("{source:?}: {}", destination.len());
        let decoded = T::decode(&mut &destination[..], Version::ZERO, repr).unwrap();
        assert_eq!(source, decoded)
    }

    #[test]
    fn big_varuint_round_trip() {
        round_trip(BigUint::new(vec![1, 2, 3, 4, 5]), PrimitiveRepr::Varint);
    }

    #[test]
    fn positive_big_varint_round_trip() {
        round_trip(
            BigInt::new(Sign::Plus, vec![1, 2, 3, 4, 5]),
            PrimitiveRepr::Varint,
        );
    }

    #[test]
    fn negative_big_varint_round_trip() {
        round_trip(
            BigInt::new(Sign::Minus, vec![1, 2, 3, 4, 5]),
            PrimitiveRepr::Varint,
        );
    }

    #[test]
    fn repr_tests() {
        round_trip(1000u32, PrimitiveRepr::BigEndian);
        round_trip(1000u32, PrimitiveRepr::LittleEndian);
        round_trip(1000u32, PrimitiveRepr::Native);
        round_trip(1000u32, PrimitiveRepr::Varint);
    }
}
