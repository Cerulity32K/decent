# Decent
A crate to easily make data serialisable.

The cores of this crate are the `Decodable` and `Encodable` traits. These signify that an object is serialisable to a binary stream. These are opinionated interfaces, accepting a version and a primitive representation for integers and floating-point numbers (whose bits are used to encode). The four primitive representations are as follows:
- `BigEndian` - Primitives are stored in big-endian byte order.
- `LittleEndian` - Primitives are stored in little-endian byte order.
- `Native` - Primitives are stored using the byte order of the host architecture.
- `Varint` - Primitives are encoded with variable-length integer encoding.

Do note that ***stability/functionality is not guaranteed by this crate***. There are many more considerations and caveats to be managed before this can be a reliable crate. This was really meant for me and my own projects, and issues are fixed case-by-case.

## Variable-length Integer Encoding
There is one primitive representation called `Varint`. This is a little-endian format, where every byte has seven magnitude bits and one continuation bit. A variable-length integer consists of a string of bytes with set continuation bits terminated by one byte with a cleared continuation bit. The magnitude bits are concatenated to form the final integer. For larger integer types that usually do not actually hold many large integers, this encoding can save space.

Signed integers use zigzag encoding, where the number is multiplied by two, with one added if the original number was negative.

`u8` and `i8` do not use `Varint` when told to, as they have no reason to other than consistency. This may be feature-gated in the future.

Floating-point numbers will defer to `BigEndian` encoding, as they don't commonly save space, often losing space instead.

## Versioning
The `Version` type is similar to semantic versioning, representing a major.minor.patch version format. When encoding/decoding, one of these is passed in to notify the type of the targetted version.

# Macros
The crux of Decent is derivation. The traits are admittedly simple, but the `decent-macros` child crate is what pulls it together. There exists a derive macro that can be used by placing `#[derive(Binary)]` atop an enum or structure. This will implement both `Encodable` and `Decodable` onto the structure in the following ways:
- Tuple and dictionary structures will serialise their fields in the order they were defined in. (TODO: lexicographical sorting for dictionary structures).
- Unit structures will not encode any data; they are zero-sized.
- Enums will first encode the index they are defined in before encoding their data as if they were structures. The index type is `u32` by default, but it will follow the integer type given by `#[repr(T)]` if present.

## Attributes
### Version Control
The `#[version]` attribute will override the passed-in `version` parameter for subsequent fields. The type must be `Version`. Having a `Version` as the first field and using the `#[version]` attribute will make the structure depend on the encoded data instead of external context; the `version` parameter will not matter, and `Version::ZERO` can be safely passed in.

### Conditional Encoding
Fields of structures can be conditionally encoded with the `#[since]` attribute. This takes in three parameters, being the major, minor, and patch version numbers. If the version passed by the `version` parameter (or overriden by the `#[version]` attribute) is equal to or greater than `#[since]`'s attribute, the field will be encoded. Otherwise, it will be skipped, and a `Default` value will take its place.

### Fixed Representation
You can set the primitive representation for a field with the `#[override_repr]` attribute, which sets a fixed representation for the attributed field given an identifier argument. The argument is case-insensitively matched against the following strings to set the representation (casing for clarity):

    be | big | big_endian | BigEndian => BigEndian
    le | little | little_endian | LittleEndian => LittleEndian
    ne | native | native_endian | NativeEndian => Native
    var | Varint => Varint

# Default Implementations
Many types have implementations for `Encodable` and `Decodable`. The implementations can be found in the `impls` module. Some notes:
- Standard collections are encoded with their length (given by their `ExactSizeIterator` implementations), followed by their items.
- Smart pointers will dereference and wrap their targets.
- Atomics are acquired using `Relaxed` ordering.
- Tuples up to 16 items are implemented.
- `bool` will encode `1u8` or `0u8`, and will decode `false` for `0u8` and `true` otherwise.
- Strings (`String`, `str`, `CString`, and `CStr`) will not use variable-length integer encoding, and will instead use regular UTF-8.
- Implementations for pointer-sized types (`usize`, `isize`, and their atomic/nonzero variants) are feature-gated behind `platform_dependent`.
- `Option<NonZeroX>` will not have the same encoding as `X`. This pattern may be checked in the derive macro, but I cannot figure out how to specially implement it. Ideas are appreciated.

# Checklist
- [ ] Lexicographical sorting options for dictionary structures.
- [ ] Actual fetching of enum discriminants instead of encoding by index.
- [ ] Proper gating of `Native` encoding behind `platform_dependent`.
- [ ] Feature-gate opt-outs of `Varint` encoding for byte and floating-point types.
- [ ] Options for providing defaults for skipped fields.
- [ ] Flexible default implementations for `Pin`.
- [ ] Expose the macros originally used for default implementations for use in user code.
- [ ] Configurable atomic loading/storing ordering methods.
- [ ] Null-pointer optimisation considerations for `Option<NonZeroX>`.
- [ ] A more exhaustive test suite.
