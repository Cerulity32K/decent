//! This module implements [`Encodable`] and [`Decodable`] for common types.
//!
//! Signed and unsigned primitive integer types (bar `isize` and `usize`) are implemented, as well as [`BigInt`] and [`BigUint`].
//!
//! Arrays are both encodable and decodable. Slices are only encodable due to their non-static size.
//!
//! Collections like [`Vec`] and [`HashMap`] are implemented.

#[cfg(feature = "platform_dependent")]
use std::num::{NonZeroIsize, NonZeroUsize};
use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    ffi::{CStr, CString},
    hash::Hash,
    io::{self, ErrorKind, Read, Write},
    mem::MaybeUninit,
    num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroU8, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU128,
    },
    ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
    rc::Rc,
    sync::{
        Arc,
        atomic::{
            self, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
            AtomicU32, AtomicU64, AtomicUsize,
        },
    },
    time::Duration,
};

use num::{BigInt, BigUint};

use crate::{
    Decodable, PrimitiveRepr, Version,
    iter::Decoder,
    read,
    write::{self, Encodable},
    zigzag::{read_varint, read_varuint, write_varint, write_varuint},
};

macro_rules! impl_uint_binary {
    (for $($type:ty),*) => {
        $(
            impl Encodable for $type {
                fn encode(&self, to: &mut dyn Write, _: Version, int_repr: PrimitiveRepr) -> io::Result<()> {
                    match int_repr {
                        PrimitiveRepr::BigEndian => to.write_all(&self.to_be_bytes()),
                        PrimitiveRepr::LittleEndian => to.write_all(&self.to_le_bytes()),
                        PrimitiveRepr::Native => to.write_all(&self.to_ne_bytes()),
                        PrimitiveRepr::Varint => write_varuint(*self, to),
                    }
                }
            }
            impl Decodable for $type {
                fn decode(from: &mut dyn Read, _: Version, int_repr: PrimitiveRepr) -> io::Result<Self> {
                    if let PrimitiveRepr::Varint = int_repr {
                        return read_varuint(from).and_then(|int| {
                            Self::try_from(int).map_err(|err| io::Error::new(ErrorKind::InvalidData, err))
                        });
                    }
                    let mut buf = [0u8; size_of::<Self>()];
                    from.read_exact(&mut buf)?;
                    match int_repr {
                        PrimitiveRepr::BigEndian => Ok(Self::from_be_bytes(buf)),
                        PrimitiveRepr::LittleEndian => Ok(Self::from_le_bytes(buf)),
                        PrimitiveRepr::Native => Ok(Self::from_ne_bytes(buf)),
                        PrimitiveRepr::Varint => unreachable!(),
                    }
                }
            }
        )*
    };
}
macro_rules! impl_int_binary {
    (for $($type:ty),*) => {
        $(
            impl Encodable for $type {
                fn encode(&self, to: &mut dyn Write, _: Version, int_repr: PrimitiveRepr) -> io::Result<()> {
                    match int_repr {
                        PrimitiveRepr::BigEndian => to.write_all(&self.to_be_bytes()),
                        PrimitiveRepr::LittleEndian => to.write_all(&self.to_le_bytes()),
                        PrimitiveRepr::Native => to.write_all(&self.to_ne_bytes()),
                        PrimitiveRepr::Varint => write_varint(*self, to),
                    }
                }
            }
            impl Decodable for $type {
                fn decode(from: &mut dyn Read, _: Version, int_repr: PrimitiveRepr) -> io::Result<Self> {
                    if let PrimitiveRepr::Varint = int_repr {
                        return read_varint(from).and_then(|int| {
                            Self::try_from(int).map_err(|err| io::Error::new(ErrorKind::InvalidData, err))
                        });
                    }
                    let mut buf = [0u8; size_of::<Self>()];
                    from.read_exact(&mut buf)?;
                    match int_repr {
                        PrimitiveRepr::BigEndian => Ok(Self::from_be_bytes(buf)),
                        PrimitiveRepr::LittleEndian => Ok(Self::from_le_bytes(buf)),
                        PrimitiveRepr::Native => Ok(Self::from_ne_bytes(buf)),
                        PrimitiveRepr::Varint => unreachable!(),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_tuple_binary {
    ($($type:ident $index:tt),+) => {
        impl<$($type: Decodable),+> Decodable for ($($type),+,) {
            fn decode(
                from: &mut dyn Read,
                version: Version,
                primitive_repr: PrimitiveRepr,
            ) -> io::Result<Self> {
                Ok(($($type::decode(from, version, primitive_repr)?),+,))
            }
        }
        impl<$($type: Encodable),+> Encodable for ($($type),+,) {
            fn encode(
                &self,
                to: &mut dyn Write,
                version: Version,
                primitive_repr: PrimitiveRepr,
            ) -> io::Result<()> {
                $(self.$index.encode(to, version, primitive_repr)?;)+
                Ok(())
            }
        }
    };
}

macro_rules! impl_new_deref_binary {
    (for $($type:ident),+) => {
        $(impl<T: Encodable> Encodable for $type<T> {
            fn encode(
                &self,
                to: &mut dyn Write,
                version: Version,
                primitive_repr: PrimitiveRepr,
            ) -> io::Result<()> {
                (**self).encode(to, version, primitive_repr)
            }
        }
        impl<T: Decodable> Decodable for $type<T> {
            fn decode(
                from: &mut dyn Read,
                version: Version,
                primitive_repr: PrimitiveRepr,
            ) -> io::Result<Self> {
                T::decode(from, version, primitive_repr).map($type::new)
            }
        })+
    };
}

macro_rules! impl_collection_binary {
    ($(for<$($generics:ident $(as decode $($decode_bounds:ident)|+)? $(as encode $($encode_bounds:ident)|+)?),*> $type:ty: $iter_type:ty),+ $(,)?) => {
        $(
            impl<$($generics: Encodable $($(+ $encode_bounds)+)?),+> Encodable for $type {
                fn encode(
                    &self,
                    to: &mut dyn Write,
                    version: Version,
                    primitive_repr: PrimitiveRepr,
                ) -> io::Result<()> {
                    write::write_length(self.into_iter().len(), to, version, primitive_repr)?;
                    self.iter().try_for_each(|item| item.encode(to, version, primitive_repr))
                }
            }
            impl<$($generics: Decodable $($(+ $decode_bounds)+)?),+> Decodable for $type {
                fn decode(
                    from: &mut dyn Read,
                    version: Version,
                    primitive_repr: PrimitiveRepr,
                ) -> io::Result<Self> {
                    let length = read::read_length(from, version, primitive_repr)?;
                    io::Result::<$type>::from_iter(Decoder::<&mut dyn Read, $iter_type>::new(from, version, primitive_repr).take(length))
                }
            }
        )+
    };
}

pub const ATOMIC_ENCODE_LOAD_ORDERING: atomic::Ordering = atomic::Ordering::Relaxed;
macro_rules! impl_atomic_binary {
    (for $($atomic_type:ty: $inner_type:ty),+) => {
        $(
            impl Encodable for $atomic_type {
                fn encode(
                    &self,
                    to: &mut dyn Write,
                    version: Version,
                    primitive_repr: PrimitiveRepr,
                ) -> io::Result<()> {
                    self.load(ATOMIC_ENCODE_LOAD_ORDERING).encode(to, version, primitive_repr)
                }
            }
            impl Decodable for $atomic_type {
                fn decode(
                    from: &mut dyn Read,
                    version: Version,
                    primitive_repr: PrimitiveRepr,
                ) -> io::Result<Self> {
                    <$inner_type>::decode(from, version, primitive_repr).map(<$atomic_type>::new)
                }
            }
        )+
    };
}
macro_rules! impl_nonzero_binary {
    (for $($nonzero_type:ty: $inner_type:ty),+) => {
        $(
            impl Encodable for $nonzero_type {
                fn encode(
                    &self,
                    to: &mut dyn Write,
                    version: Version,
                    primitive_repr: PrimitiveRepr,
                ) -> io::Result<()> {
                    self.get().encode(to, version, primitive_repr)
                }
            }
            impl Decodable for $nonzero_type {
                fn decode(
                    from: &mut dyn Read,
                    version: Version,
                    primitive_repr: PrimitiveRepr,
                ) -> io::Result<Self> {
                    <$inner_type>::decode(from, version, primitive_repr)
                        .and_then(|value| {
                            <$nonzero_type>::new(value)
                                .ok_or_else(|| io::Error::new(ErrorKind::InvalidData, concat!("`", stringify!($nonzero_type), "` received zero")))
                        })
                }
            }
        )+
    };
}

impl_uint_binary!(for u8, u16, u32, u64, u128);
impl_int_binary!(for i8, i16, i32, i64, i128);

impl_atomic_binary!(
    for AtomicU8: u8, AtomicU16: u16, AtomicU32: u32, AtomicU64: u64,
    AtomicI8: i8, AtomicI16: i16, AtomicI32: i32, AtomicI64: i64
);
impl_nonzero_binary!(
    for NonZeroU8: u8, NonZeroU16: u16, NonZeroU32: u32, NonZeroU64: u64, NonZeroU128: u128,
    NonZeroI8: i8, NonZeroI16: i16, NonZeroI32: i32, NonZeroI64: i64, NonZeroI128: i128
);

#[cfg(feature = "platform_dependent")]
impl_uint_binary!(for usize);
#[cfg(feature = "platform_dependent")]
impl_int_binary!(for isize);
#[cfg(feature = "platform_dependent")]
impl_atomic_binary!(for AtomicUsize: usize, AtomicIsize: isize);
#[cfg(feature = "platform_dependent")]
impl_nonzero_binary!(for NonZeroUsize: usize, NonZeroIsize: isize);

impl_new_deref_binary!(for Box, Arc, Rc);
impl_collection_binary!(
    for<T> Vec<T>: T,
    for<T> VecDeque<T>: T,
    for<T> LinkedList<T>: T,

    for<K as decode Hash | Eq, V> HashMap<K, V>: (K, V),
    for<K as decode Ord, V> BTreeMap<K, V>: (K, V),

    for<T as decode Hash | Eq> HashSet<T>: T,
    for<T as decode Ord> BTreeSet<T>: T,

    for<T as decode Ord> BinaryHeap<T>: T,
);

impl Decodable for () {
    fn decode(_: &mut dyn Read, _: Version, _: PrimitiveRepr) -> io::Result<Self> {
        Ok(())
    }
}
impl Encodable for () {
    fn encode(&self, _: &mut dyn Write, _: Version, _: PrimitiveRepr) -> io::Result<()> {
        Ok(())
    }
}

impl_tuple_binary!(T0 0);
impl_tuple_binary!(T0 0, T1 1);
impl_tuple_binary!(T0 0, T1 1, T2 2);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14);
impl_tuple_binary!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15);

impl<T: Encodable + Copy> Encodable for Cell<T> {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.get().encode(to, version, primitive_repr)
    }
}
impl<T: Decodable> Decodable for Cell<T> {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        T::decode(from, version, primitive_repr).map(Cell::new)
    }
}

impl<T: Encodable> Encodable for &T {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        (*self).encode(to, version, primitive_repr)
    }
}

impl Decodable for bool {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        _primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(u8::decode(from, version, PrimitiveRepr::Native)? != 0)
    }
}
impl Encodable for bool {
    fn encode(
        &self,
        to: &mut dyn Write,
        _version: Version,
        _primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        to.write_all(&[*self as u8])
    }
}

impl Encodable for BigUint {
    fn encode(&self, to: &mut dyn Write, _: Version, _: PrimitiveRepr) -> io::Result<()> {
        write_varuint(self.clone(), to)
    }
}
impl Decodable for BigUint {
    fn decode(from: &mut dyn Read, _: Version, _: PrimitiveRepr) -> io::Result<Self> {
        read_varuint(from)
    }
}

impl Encodable for BigInt {
    fn encode(&self, to: &mut dyn Write, _: Version, _: PrimitiveRepr) -> io::Result<()> {
        write_varint(self.clone(), to)
    }
}
impl Decodable for BigInt {
    fn decode(from: &mut dyn Read, _: Version, _: PrimitiveRepr) -> io::Result<Self> {
        read_varint(from)
    }
}

impl<T: Encodable> Encodable for [T] {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        write::write_length(self.len(), to, version, primitive_repr)?;
        self.iter()
            .try_for_each(|item| item.encode(to, version, primitive_repr))
    }
}
impl Encodable for str {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        write::write_length(self.len(), to, version, primitive_repr)?;
        to.write_all(self.as_bytes())
    }
}

impl Encodable for String {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        write::write_length(self.len(), to, version, primitive_repr)?;
        to.write_all(self.as_bytes())
    }
}
impl Decodable for String {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        let length = read::read_length(from, version, primitive_repr)?;
        let mut string_bytes = vec![0u8; length];
        from.read_exact(&mut string_bytes)?;
        String::from_utf8(string_bytes).map_err(|error| io::Error::other(error))
    }
}

impl Encodable for CString {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        write::write_length(self.count_bytes(), to, version, primitive_repr)?;
        to.write_all(self.as_bytes())
    }
}
impl Decodable for CString {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        let length = read::read_length(from, version, primitive_repr)?;
        let mut string_bytes = vec![0u8; length];
        from.read_exact(&mut string_bytes)?;
        CString::from_vec_with_nul(string_bytes).map_err(|error| io::Error::other(error))
    }
}

impl Encodable for CStr {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        write::write_length(self.count_bytes(), to, version, primitive_repr)?;
        to.write_all(self.to_bytes())
    }
}

impl Encodable for Version {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.0.encode(to, version, primitive_repr)?;
        self.1.encode(to, version, primitive_repr)?;
        self.2.encode(to, version, primitive_repr)?;
        Ok(())
    }
}
impl Decodable for Version {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self(
            Decodable::decode(from, version, primitive_repr)?,
            Decodable::decode(from, version, primitive_repr)?,
            Decodable::decode(from, version, primitive_repr)?,
        ))
    }
}

impl Encodable for Duration {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.as_secs().encode(to, version, primitive_repr)?;
        self.subsec_nanos().encode(to, version, primitive_repr)
    }
}
impl Decodable for Duration {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self::new(
            u64::decode(from, version, primitive_repr)?,
            u32::decode(from, version, primitive_repr)?,
        ))
    }
}

impl Encodable for f32 {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.to_bits().encode(
            to,
            version,
            match primitive_repr {
                PrimitiveRepr::Varint => PrimitiveRepr::BigEndian,
                other => other,
            },
        )
    }
}
impl Decodable for f32 {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self::from_bits(u32::decode(
            from,
            version,
            match primitive_repr {
                PrimitiveRepr::Varint => PrimitiveRepr::BigEndian,
                other => other,
            },
        )?))
    }
}

impl Encodable for f64 {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.to_bits().encode(to, version, primitive_repr)
    }
}
impl Decodable for f64 {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self::from_bits(u64::decode(from, version, primitive_repr)?))
    }
}

impl<T: Encodable, const N: usize> Encodable for [T; N] {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.iter()
            .try_for_each(|item| item.encode(to, version, primitive_repr))
    }
}
impl<T: Decodable, const N: usize> Decodable for [T; N] {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        let mut array: [MaybeUninit<T>; N] = MaybeUninit::uninit().transpose();
        for i in 0..N {
            array[i].write(T::decode(from, version, primitive_repr)?);
        }
        Ok(unsafe { MaybeUninit::array_assume_init(array) })
    }
}

impl<T: Encodable> Encodable for Option<T> {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        match self {
            Some(value) => {
                1u8.encode(to, version, primitive_repr)?;
                value.encode(to, version, primitive_repr)
            }
            None => 0u8.encode(to, version, primitive_repr),
        }
    }
}
impl<T: Decodable> Decodable for Option<T> {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        let discriminant: u8 = u8::decode(from, version, primitive_repr)?;
        match discriminant {
            0 => Ok(None),
            1 => Ok(Some(T::decode(from, version, primitive_repr)?)),
            other => Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("unexpected Option discriminant {other}"),
            )),
        }
    }
}

impl<T: Encodable> Encodable for Range<T> {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.start.encode(to, version, primitive_repr)?;
        self.end.encode(to, version, primitive_repr)
    }
}
impl<T: Decodable> Decodable for Range<T> {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self {
            start: T::decode(from, version, primitive_repr)?,
            end: T::decode(from, version, primitive_repr)?,
        })
    }
}

impl<T: Encodable> Encodable for RangeInclusive<T> {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.start().encode(to, version, primitive_repr)?;
        self.end().encode(to, version, primitive_repr)
    }
}
impl<T: Decodable> Decodable for RangeInclusive<T> {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self::new(
            T::decode(from, version, primitive_repr)?,
            T::decode(from, version, primitive_repr)?,
        ))
    }
}

impl<T: Encodable> Encodable for RangeFrom<T> {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.start.encode(to, version, primitive_repr)
    }
}
impl<T: Decodable> Decodable for RangeFrom<T> {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self {
            start: T::decode(from, version, primitive_repr)?,
        })
    }
}

impl<T: Encodable> Encodable for RangeTo<T> {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.end.encode(to, version, primitive_repr)
    }
}
impl<T: Decodable> Decodable for RangeTo<T> {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self {
            end: T::decode(from, version, primitive_repr)?,
        })
    }
}

impl<T: Encodable> Encodable for RangeToInclusive<T> {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()> {
        self.end.encode(to, version, primitive_repr)
    }
}
impl<T: Decodable> Decodable for RangeToInclusive<T> {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self> {
        Ok(Self {
            end: T::decode(from, version, primitive_repr)?,
        })
    }
}
