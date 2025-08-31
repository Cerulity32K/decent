use std::io::{self, ErrorKind, Read};

use crate::{PrimitiveRepr, Version};

/// An object that can be created from its binary representation.
pub trait Decodable: Sized {
    fn decode(
        from: &mut dyn Read,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<Self>;
}

pub fn read_length(
    from: &mut dyn Read,
    version: Version,
    primitive_repr: PrimitiveRepr,
) -> io::Result<usize> {
    u64::decode(from, version, primitive_repr)?
        .try_into()
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))
}
