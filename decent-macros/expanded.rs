#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::{fmt::Debug, num::NonZeroU32};
use decent::{encoders::*, decoders::*, Decodable, Encodable, PrimitiveRepr, Version};
use decent_macros::Binary;
enum TestEnum {
    Testa(#[override_repr(BigEndian)] #[since(1, 0, 0)] u32),
    Testa2(#[version] #[override_repr(BigEndian)] Version),
    Testa3(),
    Testb { #[override_repr(BigEndian)] #[since(1, 0, 0)] asdf: u32 },
    Testb2 { #[version] #[override_repr(BigEndian)] test: Version },
    Testb3 {},
    Testc,
}
impl Encodable for TestEnum {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        match self {
            Self::Testa(__self_0) => {
                0u32.encode(to, version, primitive_repr)?;
                {
                    let mut use_field = true;
                    use_field &= version >= Version(1u64, 0u64, 0u64);
                    let primitive_repr = decent::PrimitiveRepr::BigEndian;
                    if use_field {
                        __self_0.encode(to, version, primitive_repr)?;
                    }
                }
            }
            Self::Testa2(__self_0) => {
                1u32.encode(to, version, primitive_repr)?;
                {
                    let mut use_field = true;
                    let primitive_repr = decent::PrimitiveRepr::BigEndian;
                    version = *__self_0;
                    if use_field {
                        __self_0.encode(to, version, primitive_repr)?;
                    }
                }
            }
            Self::Testa3() => {
                2u32.encode(to, version, primitive_repr)?;
            }
            Self::Testb { asdf: __self_0 } => {
                3u32.encode(to, version, primitive_repr)?;
                {
                    let mut use_field = true;
                    use_field &= version >= Version(1u64, 0u64, 0u64);
                    let primitive_repr = decent::PrimitiveRepr::BigEndian;
                    if use_field {
                        __self_0.encode(to, version, primitive_repr)?;
                    }
                }
            }
            Self::Testb2 { test: __self_0 } => {
                4u32.encode(to, version, primitive_repr)?;
                {
                    let mut use_field = true;
                    let primitive_repr = decent::PrimitiveRepr::BigEndian;
                    version = *__self_0;
                    if use_field {
                        __self_0.encode(to, version, primitive_repr)?;
                    }
                }
            }
            Self::Testb3 {} => {
                5u32.encode(to, version, primitive_repr)?;
            }
            Self::Testc => {
                6u32.encode(to, version, primitive_repr)?;
            }
        }
        Ok(())
    }
}
impl Decodable for TestEnum {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        match u32::decode(from, version, primitive_repr)? {
            0u32 => {
                Ok(
                    Self::Testa({
                        let mut use_field = true;
                        use_field &= version >= Version(1u64, 0u64, 0u64);
                        let primitive_repr = decent::PrimitiveRepr::BigEndian;
                        if use_field {
                            <u32 as Decodable>::decode(from, version, primitive_repr)?
                        } else {
                            Default::default()
                        }
                    }),
                )
            }
            1u32 => {
                Ok(
                    Self::Testa2({
                        let mut use_field = true;
                        let primitive_repr = decent::PrimitiveRepr::BigEndian;
                        version = Version::decode(from, version, primitive_repr)?;
                        if use_field { version } else { Default::default() }
                    }),
                )
            }
            2u32 => Ok(Self::Testa3()),
            3u32 => {
                Ok(Self::Testb {
                    asdf: {
                        let mut use_field = true;
                        use_field &= version >= Version(1u64, 0u64, 0u64);
                        let primitive_repr = decent::PrimitiveRepr::BigEndian;
                        if use_field {
                            <u32 as Decodable>::decode(from, version, primitive_repr)?
                        } else {
                            Default::default()
                        }
                    },
                })
            }
            4u32 => {
                Ok(Self::Testb2 {
                    test: {
                        let mut use_field = true;
                        let primitive_repr = decent::PrimitiveRepr::BigEndian;
                        version = Version::decode(from, version, primitive_repr)?;
                        if use_field { version } else { Default::default() }
                    },
                })
            }
            5u32 => Ok(Self::Testb3 {}),
            6u32 => Ok(Self::Testc),
            other => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("unknown discriminant {0}", other),
                            )
                        }),
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TestEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TestEnum {
    #[inline]
    fn eq(&self, other: &TestEnum) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (TestEnum::Testa(__self_0), TestEnum::Testa(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (TestEnum::Testa2(__self_0), TestEnum::Testa2(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                (
                    TestEnum::Testb { asdf: __self_0 },
                    TestEnum::Testb { asdf: __arg1_0 },
                ) => __self_0 == __arg1_0,
                (
                    TestEnum::Testb2 { test: __self_0 },
                    TestEnum::Testb2 { test: __arg1_0 },
                ) => __self_0 == __arg1_0,
                _ => true,
            }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TestEnum {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TestEnum::Testa(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Testa", &__self_0)
            }
            TestEnum::Testa2(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Testa2", &__self_0)
            }
            TestEnum::Testa3() => ::core::fmt::Formatter::write_str(f, "Testa3"),
            TestEnum::Testb { asdf: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Testb",
                    "asdf",
                    &__self_0,
                )
            }
            TestEnum::Testb2 { test: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Testb2",
                    "test",
                    &__self_0,
                )
            }
            TestEnum::Testb3 {} => ::core::fmt::Formatter::write_str(f, "Testb3"),
            TestEnum::Testc => ::core::fmt::Formatter::write_str(f, "Testc"),
        }
    }
}
struct Asdf1 {
    #[version]
    version: Version,
    #[since(1, 0, 0)]
    asdf: u32,
}
impl Encodable for Asdf1 {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        {
            let mut use_field = true;
            version = self.version;
            if use_field {
                self.version.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            use_field &= version >= Version(1u64, 0u64, 0u64);
            if use_field {
                self.asdf.encode(to, version, primitive_repr)?;
            }
        }
        Ok(())
    }
}
impl Decodable for Asdf1 {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok(Self {
            version: {
                let mut use_field = true;
                version = Version::decode(from, version, primitive_repr)?;
                version
            },
            asdf: {
                let mut use_field = true;
                use_field &= version >= Version(1u64, 0u64, 0u64);
                if use_field {
                    <u32 as Decodable>::decode(from, version, primitive_repr)?
                } else {
                    Default::default()
                }
            },
        })
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Asdf1 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Asdf1 {
    #[inline]
    fn eq(&self, other: &Asdf1) -> bool {
        self.asdf == other.asdf && self.version == other.version
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Asdf1 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Asdf1",
            "version",
            &self.version,
            "asdf",
            &&self.asdf,
        )
    }
}
struct Asdf2(#[since(1, 0, 0)] u32, #[version] Version);
impl Encodable for Asdf2 {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        {
            let mut use_field = true;
            use_field &= version >= Version(1u64, 0u64, 0u64);
            if use_field {
                self.0.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            version = self.1;
            if use_field {
                self.1.encode(to, version, primitive_repr)?;
            }
        }
        Ok(())
    }
}
impl Decodable for Asdf2 {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok(
            Self(
                {
                    let mut use_field = true;
                    use_field &= version >= Version(1u64, 0u64, 0u64);
                    if use_field {
                        <u32 as Decodable>::decode(from, version, primitive_repr)?
                    } else {
                        Default::default()
                    }
                },
                {
                    let mut use_field = true;
                    version = Version::decode(from, version, primitive_repr)?;
                    version
                },
            ),
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Asdf2 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Asdf2 {
    #[inline]
    fn eq(&self, other: &Asdf2) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Asdf2 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field2_finish(f, "Asdf2", &self.0, &&self.1)
    }
}
struct Asdf3;
impl Encodable for Asdf3 {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        Ok(())
    }
}
impl Decodable for Asdf3 {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok(Self)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Asdf3 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Asdf3 {
    #[inline]
    fn eq(&self, other: &Asdf3) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Asdf3 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Asdf3")
    }
}
struct Asdf4 {
    #[encode_with(npo_encode::<u32>)]
    #[decode_with(npo_decode::<u32>)]
    x: Option<NonZeroU32>,
    y: Vec<i32>,
}
impl Encodable for Asdf4 {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        {
            let mut use_field = true;
            if use_field {
                (npo_encode::<u32>)(&self.x, to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.y.encode(to, version, primitive_repr)?;
            }
        }
        Ok(())
    }
}
impl Decodable for Asdf4 {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok(Self {
            x: {
                let mut use_field = true;
                (npo_decode::<u32>)(from, version, primitive_repr)?
            },
            y: {
                let mut use_field = true;
                <Vec<i32> as Decodable>::decode(from, version, primitive_repr)?
            },
        })
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Asdf4 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Asdf4 {
    #[inline]
    fn eq(&self, other: &Asdf4) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Asdf4 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Asdf4",
            "x",
            &self.x,
            "y",
            &&self.y,
        )
    }
}
fn round_trip<T: Encodable + Decodable + PartialEq + Debug>(
    source: T,
    version: Version,
    repr: PrimitiveRepr,
) {
    let mut destination = ::alloc::vec::Vec::new();
    source.encode(&mut destination, version, repr).expect("encode failed");
    let decoded = T::decode(&mut &destination[..], version, repr)
        .expect(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("decode failed from {0:?}", destination),
                )
            }),
        );
    match (&source, &decoded) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(
                        format_args!(
                            "round trip encode/decode failed, bytes encoded were {0:?}",
                            destination,
                        ),
                    ),
                );
            }
        }
    }
}
fn round_trip_with_size<T: Encodable + Decodable + PartialEq + Debug>(
    source: T,
    version: Version,
    repr: PrimitiveRepr,
    byte_count: usize,
) {
    let mut destination = ::alloc::vec::Vec::new();
    source.encode(&mut destination, version, repr).unwrap();
    match (&destination.len(), &byte_count) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let decoded = T::decode(&mut &destination[..], version, repr)
        .expect(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("decode failed from {0:?}", destination),
                )
            }),
        );
    match (&source, &decoded) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::Some(
                        format_args!(
                            "round trip encode/decode failed, bytes encoded were {0:?}",
                            destination,
                        ),
                    ),
                );
            }
        }
    }
}
extern crate test;
#[rustc_test_marker = "unit"]
#[doc(hidden)]
pub const unit: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("unit"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "decent-macros/tests/test.rs",
        start_line: 88usize,
        start_col: 4usize,
        end_line: 88usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(unit())),
};
fn unit() {
    round_trip(Asdf3, Version(0, 0, 0), PrimitiveRepr::Varint);
}
extern crate test;
#[rustc_test_marker = "struct_with_vec"]
#[doc(hidden)]
pub const struct_with_vec: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("struct_with_vec"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "decent-macros/tests/test.rs",
        start_line: 93usize,
        start_col: 4usize,
        end_line: 93usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(struct_with_vec()),
    ),
};
fn struct_with_vec() {
    round_trip(
        Asdf4 {
            x: NonZeroU32::new(5),
            y: <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    2,
                    6,
                    435897357,
                    4,
                    3,
                    3,
                    54,
                    5,
                    -3,
                    -6,
                    -2,
                    -982342256,
                    53,
                    563,
                    345,
                    54,
                    22,
                ]),
            ),
        },
        Version(0, 0, 0),
        PrimitiveRepr::BigEndian,
    );
}
extern crate test;
#[rustc_test_marker = "versions"]
#[doc(hidden)]
pub const versions: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("versions"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "decent-macros/tests/test.rs",
        start_line: 107usize,
        start_col: 4usize,
        end_line: 107usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(versions())),
};
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
extern crate test;
#[rustc_test_marker = "enums"]
#[doc(hidden)]
pub const enums: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("enums"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "decent-macros/tests/test.rs",
        start_line: 172usize,
        start_col: 4usize,
        end_line: 172usize,
        end_col: 9usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(enums())),
};
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
        {
            ::std::io::_print(format_args!("{0:?}\n", variant));
        };
        round_trip(variant, Version::ZERO, PrimitiveRepr::Varint);
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&enums, &struct_with_vec, &unit, &versions])
}
