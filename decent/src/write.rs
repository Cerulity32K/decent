use std::io::{self, ErrorKind, Write};

use crate::{PrimitiveRepr, Version};

/// An object that can be converted to a binary representation.
pub trait Encodable {
    fn encode(
        &self,
        to: &mut dyn Write,
        version: Version,
        primitive_repr: PrimitiveRepr,
    ) -> io::Result<()>;
}

pub fn write_length(
    length: usize,
    to: &mut dyn Write,
    version: Version,
    primitive_repr: PrimitiveRepr,
) -> io::Result<()> {
    match u64::try_from(length) {
        Ok(value) => value.encode(to, version, primitive_repr),
        Err(err) => Err(io::Error::new(ErrorKind::InvalidData, err)),
    }
}
