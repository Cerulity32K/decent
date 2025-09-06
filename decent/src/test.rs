#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::{Decodable, Encodable, PrimitiveRepr, Version};
    use num::{BigInt, BigUint, bigint::Sign};

    fn round_trip<T: Encodable + Decodable + PartialEq + Debug>(source: T, repr: PrimitiveRepr) {
        let mut destination = vec![];
        source
            .encode(&mut destination, Version::ZERO, repr)
            .unwrap();
        println!("{source:?}: {:?}", destination);
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
    fn integer_repr() {
        for uint in [1000u32] {
            round_trip(uint, PrimitiveRepr::BigEndian);
            round_trip(uint, PrimitiveRepr::LittleEndian);
            round_trip(uint, PrimitiveRepr::Native);
            round_trip(uint, PrimitiveRepr::Varint);
        }

        for sint in [12345i32, -12345i32, -1i32, 0i32, 1i32] {
            round_trip(sint, PrimitiveRepr::BigEndian);
            round_trip(sint, PrimitiveRepr::LittleEndian);
            round_trip(sint, PrimitiveRepr::Native);
            round_trip(sint, PrimitiveRepr::Varint);
        }
    }
}
