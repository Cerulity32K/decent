use std::{
    io::{self, Read},
    marker::PhantomData,
};

use crate::{Decodable, PrimitiveRepr, Version};

pub struct Decoder<R: Read, T: Decodable> {
    stream: R,
    version: Version,
    primitive_repr: PrimitiveRepr,
    pd: PhantomData<T>,
}
impl<R: Read, T: Decodable> Decoder<R, T> {
    pub fn new(stream: R, version: Version, primitive_repr: PrimitiveRepr) -> Self {
        Self {
            stream,
            version,
            primitive_repr,
            pd: PhantomData,
        }
    }
}
impl<R: Read, T: Decodable> Iterator for Decoder<R, T> {
    type Item = io::Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(T::decode(
            &mut self.stream,
            self.version,
            self.primitive_repr,
        ))
    }
}
