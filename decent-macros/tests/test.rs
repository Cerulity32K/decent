use std::{fmt::Debug, num::NonZeroU32};

use decent::{encoders::*, decoders::*, Decodable, Encodable, PrimitiveRepr, Version};
use decent_macros::Binary;

#[derive(Binary, PartialEq, Debug)]
enum TestEnum {
    Testa(
        #[override_repr(BigEndian)]
        #[since(1, 0, 0)]
        u32,
    ),
    Testa2(
        #[version]
        #[override_repr(BigEndian)]
        Version,
    ),
    Testa3(),
    Testb {
        #[override_repr(BigEndian)]
        #[since(1, 0, 0)]
        asdf: u32,
    },
    Testb2 {
        #[version]
        #[override_repr(BigEndian)]
        test: Version,
    },
    Testb3 {},
    Testc,
}

#[derive(Binary, PartialEq, Debug)]
struct Asdf1 {
    #[version]
    version: Version,
    #[since(1, 0, 0)]
    asdf: u32,
}

#[derive(Binary, PartialEq, Debug)]
struct Asdf2(#[since(1, 0, 0)] u32, #[version] Version);

#[derive(Binary, PartialEq, Debug)]
struct Asdf3;

#[derive(Binary, PartialEq, Debug)]
struct Asdf4 {
    #[encode_with(npo_encode::<u32>)]
    #[decode_with(npo_decode::<u32>)]
    x: Option<NonZeroU32>,
    y: Vec<i32>,
}

fn round_trip<T: Encodable + Decodable + PartialEq + Debug>(
    source: T,
    version: Version,
    repr: PrimitiveRepr,
) {
    let mut destination = vec![];
    source
        .encode(&mut destination, version, repr)
        .expect("encode failed");
    let decoded = T::decode(&mut &destination[..], version, repr)
        .expect(&format!("decode failed from {destination:?}"));
    assert_eq!(
        source, decoded,
        "round trip encode/decode failed, bytes encoded were {destination:?}"
    )
}
fn round_trip_with_size<T: Encodable + Decodable + PartialEq + Debug>(
    source: T,
    version: Version,
    repr: PrimitiveRepr,
    byte_count: usize,
) {
    let mut destination = vec![];
    source.encode(&mut destination, version, repr).unwrap();
    assert_eq!(destination.len(), byte_count);
    let decoded = T::decode(&mut &destination[..], version, repr)
        .expect(&format!("decode failed from {destination:?}"));
    assert_eq!(
        source, decoded,
        "round trip encode/decode failed, bytes encoded were {destination:?}"
    )
}
#[test]
fn unit() {
    round_trip(Asdf3, Version(0, 0, 0), PrimitiveRepr::Varint);
}

#[test]
fn struct_with_vec() {
    round_trip(
        Asdf4 {
            x: NonZeroU32::new(5),
            y: vec![
                2, 6, 435897357, 4, 3, 3, 54, 5, -3, -6, -2, -982342256, 53, 563, 345, 54, 22,
            ],
        },
        Version(0, 0, 0),
        PrimitiveRepr::BigEndian,
    );
}

#[test]
fn versions() {
    round_trip_with_size(
        Asdf2(0, Version::ZERO),
        Version::ZERO,
        PrimitiveRepr::Native,
        24,
    );
    round_trip_with_size(
        Asdf2(0, Version(1, 0, 0)),
        Version::ZERO,
        PrimitiveRepr::Native,
        24,
    );
    round_trip_with_size(
        Asdf2(0, Version::ZERO),
        Version(1, 0, 0),
        PrimitiveRepr::Native,
        28,
    );
    round_trip_with_size(
        Asdf2(0, Version(1, 0, 0)),
        Version(1, 0, 0),
        PrimitiveRepr::Native,
        28,
    );

    round_trip_with_size(
        Asdf1 {
            version: Version::ZERO,
            asdf: 0,
        },
        Version::ZERO,
        PrimitiveRepr::Native,
        24,
    );
    round_trip_with_size(
        Asdf1 {
            version: Version(1, 0, 0),
            asdf: 0,
        },
        Version::ZERO,
        PrimitiveRepr::Native,
        28,
    );
    round_trip_with_size(
        Asdf1 {
            version: Version::ZERO,
            asdf: 0,
        },
        Version(1, 0, 0),
        PrimitiveRepr::Native,
        24,
    );
    round_trip_with_size(
        Asdf1 {
            version: Version(1, 0, 0),
            asdf: 0,
        },
        Version(1, 0, 0),
        PrimitiveRepr::Native,
        28,
    );
}

#[test]
fn enums() {
    for variant in [
        TestEnum::Testa(0),
        TestEnum::Testa2(Version(1, 0, 0)),
        TestEnum::Testa3(),
        TestEnum::Testb { asdf: 0 },
        TestEnum::Testb2 {
            test: Version(2, 3, 4),
        },
        TestEnum::Testb3 {},
    ] {
        println!("{variant:?}");
        round_trip(variant, Version::ZERO, PrimitiveRepr::Varint);
    }
}
