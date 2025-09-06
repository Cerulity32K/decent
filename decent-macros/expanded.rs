#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::{
    fmt::Debug, io::{self, Read, Write},
    num::NonZeroU32,
};
use decent::{Decodable, Encodable, PrimitiveRepr, Version, decoders::*, encoders::*};
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
                ((0 + 0) as u32).encode(to, version, primitive_repr)?;
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
                ((0 + 1) as u32).encode(to, version, primitive_repr)?;
                {
                    let mut use_field = true;
                    let primitive_repr = decent::PrimitiveRepr::BigEndian;
                    if use_field {
                        __self_0.encode(to, version, primitive_repr)?;
                    }
                }
            }
            Self::Testa3() => {
                ((0 + 2) as u32).encode(to, version, primitive_repr)?;
            }
            Self::Testb { asdf: __self_0 } => {
                ((0 + 3) as u32).encode(to, version, primitive_repr)?;
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
                ((0 + 4) as u32).encode(to, version, primitive_repr)?;
                {
                    let mut use_field = true;
                    let primitive_repr = decent::PrimitiveRepr::BigEndian;
                    if use_field {
                        __self_0.encode(to, version, primitive_repr)?;
                    }
                }
            }
            Self::Testb3 {} => {
                ((0 + 5) as u32).encode(to, version, primitive_repr)?;
            }
            Self::Testc => {
                ((0 + 6) as u32).encode(to, version, primitive_repr)?;
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
            __discriminant if __discriminant == (0) + 0 => {
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
            __discriminant if __discriminant == (0) + 1 => {
                Ok(
                    Self::Testa2({
                        let mut use_field = true;
                        let primitive_repr = decent::PrimitiveRepr::BigEndian;
                        if use_field {
                            {
                                version = <Version as Decodable>::decode(
                                    from,
                                    version,
                                    primitive_repr,
                                )?;
                                version
                            }
                        } else {
                            Default::default()
                        }
                    }),
                )
            }
            __discriminant if __discriminant == (0) + 2 => Ok(Self::Testa3()),
            __discriminant if __discriminant == (0) + 3 => {
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
            __discriminant if __discriminant == (0) + 4 => {
                Ok(Self::Testb2 {
                    test: {
                        let mut use_field = true;
                        let primitive_repr = decent::PrimitiveRepr::BigEndian;
                        if use_field {
                            {
                                version = <Version as Decodable>::decode(
                                    from,
                                    version,
                                    primitive_repr,
                                )?;
                                version
                            }
                        } else {
                            Default::default()
                        }
                    },
                })
            }
            __discriminant if __discriminant == (0) + 5 => Ok(Self::Testb3 {}),
            __discriminant if __discriminant == (0) + 6 => Ok(Self::Testc),
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
            if use_field {
                {
                    self.version.encode(to, version, primitive_repr)?;
                    version = self.version;
                }
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
        Ok({
            {
                let mut use_field = true;
                version = <Version as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_1;
            {
                let mut use_field = true;
                use_field &= version >= Version(1u64, 0u64, 0u64);
                __self_1 = if use_field {
                    <u32 as Decodable>::decode(from, version, primitive_repr)?
                } else {
                    Default::default()
                };
            }
            Self {
                version: version,
                asdf: __self_1,
            }
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
            if use_field {
                {
                    self.1.encode(to, version, primitive_repr)?;
                    version = self.1;
                }
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
        Ok({
            let __self_0;
            {
                let mut use_field = true;
                use_field &= version >= Version(1u64, 0u64, 0u64);
                __self_0 = if use_field {
                    <u32 as Decodable>::decode(from, version, primitive_repr)?
                } else {
                    Default::default()
                };
            };
            {
                let mut use_field = true;
                version = <Version as Decodable>::decode(from, version, primitive_repr)?;
            }
            Self(__self_0, version)
        })
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
        Ok({
            let __self_0;
            {
                let mut use_field = true;
                __self_0 = (npo_decode::<u32>)(&mut *from, version, primitive_repr)?;
            }
            let __self_1;
            {
                let mut use_field = true;
                __self_1 = <Vec<
                    i32,
                > as Decodable>::decode(from, version, primitive_repr)?;
            }
            Self { x: __self_0, y: __self_1 }
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
fn decode_major_version_u16(
    from: &mut dyn Read,
    current_version: Version,
    primitive_repr: PrimitiveRepr,
) -> io::Result<Version> {
    Ok(Version(u16::decode(from, current_version, primitive_repr)? as u64, 0, 0))
}
fn encode_major_version_u16(
    value: &Version,
    to: &mut dyn Write,
    current_version: Version,
    primitive_repr: PrimitiveRepr,
) -> io::Result<()> {
    u16::try_from(value.0)
        .map_err(|error| io::Error::other(error))?
        .encode(to, current_version, primitive_repr)
}
struct EncodedVersionTuple(
    #[version]
    #[encode_with(encode_major_version_u16)]
    #[decode_with(decode_major_version_u16)]
    Version,
);
impl Encodable for EncodedVersionTuple {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        {
            let mut use_field = true;
            if use_field {
                {
                    (encode_major_version_u16)(&self.0, to, version, primitive_repr)?;
                    version = self.0;
                }
            }
        }
        Ok(())
    }
}
impl Decodable for EncodedVersionTuple {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok({
            {
                let mut use_field = true;
                version = (decode_major_version_u16)(
                    &mut *from,
                    version,
                    primitive_repr,
                )?;
            }
            Self(version)
        })
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EncodedVersionTuple {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EncodedVersionTuple {
    #[inline]
    fn eq(&self, other: &EncodedVersionTuple) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EncodedVersionTuple {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(
            f,
            "EncodedVersionTuple",
            &&self.0,
        )
    }
}
struct EncodedVersionNamed {
    #[version]
    #[encode_with(encode_major_version_u16)]
    #[decode_with(decode_major_version_u16)]
    version: Version,
}
impl Encodable for EncodedVersionNamed {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        {
            let mut use_field = true;
            if use_field {
                {
                    (encode_major_version_u16)(
                        &self.version,
                        to,
                        version,
                        primitive_repr,
                    )?;
                    version = self.version;
                }
            }
        }
        Ok(())
    }
}
impl Decodable for EncodedVersionNamed {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok({
            {
                let mut use_field = true;
                version = (decode_major_version_u16)(
                    &mut *from,
                    version,
                    primitive_repr,
                )?;
            }
            Self { version: version }
        })
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EncodedVersionNamed {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EncodedVersionNamed {
    #[inline]
    fn eq(&self, other: &EncodedVersionNamed) -> bool {
        self.version == other.version
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EncodedVersionNamed {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "EncodedVersionNamed",
            "version",
            &&self.version,
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
                    ::core::option::Option::Some(
                        format_args!("encoded the wrong number of bytes"),
                    ),
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
        start_line: 130usize,
        start_col: 4usize,
        end_line: 130usize,
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
        start_line: 135usize,
        start_col: 4usize,
        end_line: 135usize,
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
        start_line: 149usize,
        start_col: 4usize,
        end_line: 149usize,
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
        start_line: 214usize,
        start_col: 4usize,
        end_line: 214usize,
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
fn asdf<T>(thing: T) -> io::Result<()> {
    Ok(())
}
struct Uiop {
    #[encode_with(|_, _, _, _|asdf(self))]
    nothing: (),
}
impl Encodable for Uiop {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        {
            let mut use_field = true;
            if use_field {
                (|_, _, _, _| asdf(self))(&self.nothing, to, version, primitive_repr)?;
            }
        }
        Ok(())
    }
}
impl Decodable for Uiop {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok({
            let __self_0;
            {
                let mut use_field = true;
                __self_0 = <() as Decodable>::decode(from, version, primitive_repr)?;
            }
            Self { nothing: __self_0 }
        })
    }
}
struct BindTest {
    a: u32,
    b: u32,
    c: u32,
    #[bind]
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    #[decode_with(|_, _, _|->io::Result<u32>{Ok(d)})]
    h: u32,
    i: u32,
    j: u32,
}
impl Encodable for BindTest {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        {
            let mut use_field = true;
            if use_field {
                self.a.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.b.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.c.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.d.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.e.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.f.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.g.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.h.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.i.encode(to, version, primitive_repr)?;
            }
        }
        {
            let mut use_field = true;
            if use_field {
                self.j.encode(to, version, primitive_repr)?;
            }
        }
        Ok(())
    }
}
impl Decodable for BindTest {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        Ok({
            let __self_0;
            {
                let mut use_field = true;
                __self_0 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_1;
            {
                let mut use_field = true;
                __self_1 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_2;
            {
                let mut use_field = true;
                __self_2 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let d;
            {
                let mut use_field = true;
                d = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_4;
            {
                let mut use_field = true;
                __self_4 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_5;
            {
                let mut use_field = true;
                __self_5 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_6;
            {
                let mut use_field = true;
                __self_6 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_7;
            {
                let mut use_field = true;
                __self_7 = (|_, _, _| -> io::Result<u32> {
                    Ok(d)
                })(&mut *from, version, primitive_repr)?;
            }
            let __self_8;
            {
                let mut use_field = true;
                __self_8 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            let __self_9;
            {
                let mut use_field = true;
                __self_9 = <u32 as Decodable>::decode(from, version, primitive_repr)?;
            }
            Self {
                a: __self_0,
                b: __self_1,
                c: __self_2,
                d: d,
                e: __self_4,
                f: __self_5,
                g: __self_6,
                h: __self_7,
                i: __self_8,
                j: __self_9,
            }
        })
    }
}
enum Asdf {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I = 20,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}
impl Encodable for Asdf {
    fn encode(
        &self,
        to: &mut dyn std::io::Write,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<()> {
        match self {
            Self::A => {
                ((0 + 0) as u32).encode(to, version, primitive_repr)?;
            }
            Self::B => {
                ((0 + 1) as u32).encode(to, version, primitive_repr)?;
            }
            Self::C => {
                ((0 + 2) as u32).encode(to, version, primitive_repr)?;
            }
            Self::D => {
                ((0 + 3) as u32).encode(to, version, primitive_repr)?;
            }
            Self::E => {
                ((0 + 4) as u32).encode(to, version, primitive_repr)?;
            }
            Self::F => {
                ((0 + 5) as u32).encode(to, version, primitive_repr)?;
            }
            Self::G => {
                ((0 + 6) as u32).encode(to, version, primitive_repr)?;
            }
            Self::H => {
                ((0 + 7) as u32).encode(to, version, primitive_repr)?;
            }
            Self::I => {
                (((20) + 0) as u32).encode(to, version, primitive_repr)?;
            }
            Self::J => {
                (((20) + 1) as u32).encode(to, version, primitive_repr)?;
            }
            Self::K => {
                (((20) + 2) as u32).encode(to, version, primitive_repr)?;
            }
            Self::L => {
                (((20) + 3) as u32).encode(to, version, primitive_repr)?;
            }
            Self::M => {
                (((20) + 4) as u32).encode(to, version, primitive_repr)?;
            }
            Self::N => {
                (((20) + 5) as u32).encode(to, version, primitive_repr)?;
            }
            Self::O => {
                (((20) + 6) as u32).encode(to, version, primitive_repr)?;
            }
            Self::P => {
                (((20) + 7) as u32).encode(to, version, primitive_repr)?;
            }
            Self::Q => {
                (((20) + 8) as u32).encode(to, version, primitive_repr)?;
            }
            Self::R => {
                (((20) + 9) as u32).encode(to, version, primitive_repr)?;
            }
            Self::S => {
                (((20) + 10) as u32).encode(to, version, primitive_repr)?;
            }
            Self::T => {
                (((20) + 11) as u32).encode(to, version, primitive_repr)?;
            }
            Self::U => {
                (((20) + 12) as u32).encode(to, version, primitive_repr)?;
            }
            Self::V => {
                (((20) + 13) as u32).encode(to, version, primitive_repr)?;
            }
            Self::W => {
                (((20) + 14) as u32).encode(to, version, primitive_repr)?;
            }
            Self::X => {
                (((20) + 15) as u32).encode(to, version, primitive_repr)?;
            }
            Self::Y => {
                (((20) + 16) as u32).encode(to, version, primitive_repr)?;
            }
            Self::Z => {
                (((20) + 17) as u32).encode(to, version, primitive_repr)?;
            }
        }
        Ok(())
    }
}
impl Decodable for Asdf {
    fn decode(
        from: &mut dyn std::io::Read,
        mut version: decent::Version,
        primitive_repr: decent::PrimitiveRepr,
    ) -> std::io::Result<Self> {
        match u32::decode(from, version, primitive_repr)? {
            __discriminant if __discriminant == (0) + 0 => Ok(Self::A),
            __discriminant if __discriminant == (0) + 1 => Ok(Self::B),
            __discriminant if __discriminant == (0) + 2 => Ok(Self::C),
            __discriminant if __discriminant == (0) + 3 => Ok(Self::D),
            __discriminant if __discriminant == (0) + 4 => Ok(Self::E),
            __discriminant if __discriminant == (0) + 5 => Ok(Self::F),
            __discriminant if __discriminant == (0) + 6 => Ok(Self::G),
            __discriminant if __discriminant == (0) + 7 => Ok(Self::H),
            __discriminant if __discriminant == (20) + 0 => Ok(Self::I),
            __discriminant if __discriminant == (20) + 1 => Ok(Self::J),
            __discriminant if __discriminant == (20) + 2 => Ok(Self::K),
            __discriminant if __discriminant == (20) + 3 => Ok(Self::L),
            __discriminant if __discriminant == (20) + 4 => Ok(Self::M),
            __discriminant if __discriminant == (20) + 5 => Ok(Self::N),
            __discriminant if __discriminant == (20) + 6 => Ok(Self::O),
            __discriminant if __discriminant == (20) + 7 => Ok(Self::P),
            __discriminant if __discriminant == (20) + 8 => Ok(Self::Q),
            __discriminant if __discriminant == (20) + 9 => Ok(Self::R),
            __discriminant if __discriminant == (20) + 10 => Ok(Self::S),
            __discriminant if __discriminant == (20) + 11 => Ok(Self::T),
            __discriminant if __discriminant == (20) + 12 => Ok(Self::U),
            __discriminant if __discriminant == (20) + 13 => Ok(Self::V),
            __discriminant if __discriminant == (20) + 14 => Ok(Self::W),
            __discriminant if __discriminant == (20) + 15 => Ok(Self::X),
            __discriminant if __discriminant == (20) + 16 => Ok(Self::Y),
            __discriminant if __discriminant == (20) + 17 => Ok(Self::Z),
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
struct Vec2 {
    x: f32,
    y: f32,
}
extern crate test;
#[rustc_test_marker = "custom_encoded_version"]
#[doc(hidden)]
pub const custom_encoded_version: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("custom_encoded_version"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "decent-macros/tests/test.rs",
        start_line: 300usize,
        start_col: 4usize,
        end_line: 300usize,
        end_col: 26usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(custom_encoded_version()),
    ),
};
fn custom_encoded_version() {
    round_trip_with_size(
        EncodedVersionTuple(Version(105, 0, 0)),
        Version::ZERO,
        PrimitiveRepr::Native,
        2,
    );
    round_trip_with_size(
        EncodedVersionNamed {
            version: Version(105, 0, 0),
        },
        Version::ZERO,
        PrimitiveRepr::Native,
        2,
    );
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[&custom_encoded_version, &enums, &struct_with_vec, &unit, &versions],
    )
}
