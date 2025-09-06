use std::collections::HashSet;

use decent::{PrimitiveRepr, Version};
use proc_macro::TokenStream as RustTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Ident, LitInt,
    parse::ParseStream, parse_macro_input, spanned::Spanned, token::Comma,
};
// TODO: deduplicate a whole bunch of things
// TODO: x.x.x parsing (x.x is considered a float which adds complications)
fn parse_version(stream: ParseStream) -> syn::Result<Version> {
    let major = stream.parse::<LitInt>()?.base10_parse::<u64>()?;
    stream.parse::<Comma>()?;
    let minor = stream.parse::<LitInt>()?.base10_parse::<u64>()?;
    stream.parse::<Comma>()?;
    let patch = stream.parse::<LitInt>()?.base10_parse::<u64>()?;
    Ok(Version(major, minor, patch))
}

fn parse_primitive_repr(stream: ParseStream) -> syn::Result<PrimitiveRepr> {
    match &stream.parse::<Ident>()?.to_string().to_lowercase()[..] {
        "be" | "big" | "big_endian" | "bigendian" => Ok(PrimitiveRepr::BigEndian),
        "le" | "little" | "little_endian" | "littleendian" => Ok(PrimitiveRepr::LittleEndian),
        "ne" | "native" | "native_endian" | "nativeendian" => Ok(PrimitiveRepr::Native),
        "var" | "varint" => Ok(PrimitiveRepr::Varint),
        other => Err(stream.error(format!("Unknown repr `{other}`"))),
    }
}

fn parse_integer_type(stream: ParseStream) -> syn::Result<Option<String>> {
    let ty = stream.parse::<Ident>()?.to_string();
    if [
        "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128",
    ]
    .contains(&&ty[..])
    {
        return Ok(Some(ty));
    } else {
        return Ok(None);
    }
}

struct FieldAttributes {
    modifications: TokenStream,
    usage_is_conditional: bool,
    version_overridden: bool,
    bind_to_name: bool,
    encode_with: Option<Expr>,
    decode_with: Option<Expr>,
}

fn get_field_attribute_modifications(
    field_accessor: Option<&TokenStream>,
    field_attributes: &[Attribute],
) -> syn::Result<FieldAttributes> {
    let mut modifications = quote! { let mut use_field = true; };
    let mut usage_is_conditional = false;
    let mut version_overridden = false;
    let mut bind_to_name = false;
    let mut encode_with = None;
    let mut decode_with = None;

    let mut seen_attributes = HashSet::new();

    let mut see_attr = |attr: &str| {
        if seen_attributes.contains(&attr[..]) {
            panic!("attribute `{attr}` doubly-specified");
        }
        seen_attributes.insert("since");
    };

    // let version_decoder = quote! { Version::decode(from, version, primitive_repr)? };
    for attribute in field_attributes {
        let attr = attribute.path().segments.last().unwrap().ident.to_string();
        match &attr[..] {
            "since" => {
                see_attr(&attr);
                let Version(major, minor, patch) = attribute.parse_args_with(parse_version)?;
                usage_is_conditional = true;
                modifications = quote! {
                    #modifications
                    use_field &= version >= Version(#major, #minor, #patch);
                };
            }
            "until" => {
                see_attr(&attr);
                let Version(major, minor, patch) = attribute.parse_args_with(parse_version)?;
                usage_is_conditional = true;
                modifications = quote! {
                    #modifications
                    use_field &= version < Version(#major, #minor, #patch);
                };
            }
            "bind" => {
                see_attr(&attr);
                bind_to_name = true;
            }
            _ => {}
        }
    }
    for attribute in field_attributes {
        let attr = attribute.path().segments.last().unwrap().ident.to_string();
        match &attr[..] {
            "decode_with" => {
                see_attr(&attr);
                decode_with = Some(attribute.parse_args::<Expr>()?);
            }
            "encode_with" => {
                see_attr(&attr);
                encode_with = Some(attribute.parse_args::<Expr>()?);
            }
            _ => {}
        }
    }
    for attribute in field_attributes {
        let attr = attribute.path().segments.last().unwrap().ident.to_string();
        match &attr[..] {
            "override_repr" => {
                see_attr(&attr);
                let new_repr = match attribute.parse_args_with(parse_primitive_repr).unwrap() {
                    PrimitiveRepr::BigEndian => "BigEndian",
                    PrimitiveRepr::LittleEndian => "LittleEndian",
                    PrimitiveRepr::Native => "Native",
                    PrimitiveRepr::Varint => "Varint",
                }
                .parse::<TokenStream>()?;
                modifications = quote! {
                    #modifications
                    let primitive_repr = decent::PrimitiveRepr::#new_repr;
                };
            }
            _ => {}
        }
    }
    for attribute in field_attributes {
        let attr = attribute.path().segments.last().unwrap().ident.to_string();
        match &attr[..] {
            "version" => {
                see_attr(&attr);
                if usage_is_conditional {
                    return Err(syn::Error::new(
                        attribute.span(),
                        "conditionally coded and overriding versions are currently unsupported",
                    ));
                }
                version_overridden = true;
                // let field_accessor = match field_accessor {
                //     Some(accessor) => accessor.clone(),
                //     None => match &decode_with {
                //         Some(expr) => quote! { (#expr)(&mut *from, version, primitive_repr)? },
                //         None => quote! { Version::decode(from, version, primitive_repr)? },
                //     },
                // };
                modifications = quote! {
                    #modifications
                    // version = #field_accessor;
                }
            }
            _ => {}
        }
    }
    return Ok(FieldAttributes {
        modifications,
        usage_is_conditional,
        version_overridden,
        bind_to_name,
        encode_with,
        decode_with,
    });
}

fn create_struct_encode_body(data_struct: &DataStruct) -> syn::Result<TokenStream> {
    let mut encode_body = quote! {};
    for (field_index, field) in data_struct.fields.iter().enumerate() {
        let accessor = format!(
            "self.{}",
            match &field.ident {
                Some(ident) => ident.to_string(),
                None => field_index.to_string(),
            }
        )
        .parse::<TokenStream>()
        .unwrap();

        let FieldAttributes {
            modifications,
            encode_with,
            version_overridden,
            ..
        } = get_field_attribute_modifications(Some(&accessor), &field.attrs)?;
        let mut encode = match encode_with {
            Some(expr) => quote! { (#expr)(&#accessor, to, version, primitive_repr)?; },
            None => quote! { #accessor.encode(to, version, primitive_repr)?; },
        };
        if version_overridden {
            encode = quote! { { #encode version = #accessor } };
        }
        encode_body = quote! {
            #encode_body
            {
                #modifications
                if use_field {
                    #encode
                }
            }
        }
    }
    Ok(encode_body)
}
fn create_enum_encode_body(
    data_enum: &DataEnum,
    discriminant_type: &str,
) -> syn::Result<TokenStream> {
    let mut variant_arms = quote! {};
    let mut discriminant = quote! { 0 };
    let mut discriminant_offset = 0;
    for variant in data_enum.variants.iter() {
        let name = &variant.ident;
        if let Some((_, discriminant_expression)) = &variant.discriminant {
            discriminant = quote! { (#discriminant_expression) };
            discriminant_offset = 0;
        }

        let variant_binder = match &variant.fields {
            Fields::Named(fields) => {
                let mut binder_body = quote! {};
                for (field_index, field) in fields.named.iter().enumerate() {
                    let name = &field.ident;
                    let bound_to = format!("__self_{field_index}")
                        .parse::<TokenStream>()
                        .unwrap();
                    binder_body = quote! { #binder_body #name: #bound_to, };
                }
                quote! { { #binder_body } }
            }
            Fields::Unnamed(fields) => {
                let mut binder_body = quote! {};
                for field_index in 0..fields.unnamed.len() {
                    let field_binding = format!("__self_{field_index}")
                        .parse::<TokenStream>()
                        .unwrap();
                    binder_body = quote! { #binder_body #field_binding, };
                }
                quote! { (#binder_body) }
            }
            Fields::Unit => quote! {},
        };

        let encoded_discriminant =
            format!("(({discriminant} + {discriminant_offset}) as {discriminant_type})")
                .parse::<TokenStream>()?;
        let mut variant_encode_body = quote! {
            #encoded_discriminant.encode(to, version, primitive_repr)?;
        };
        for (field_index, field) in variant.fields.iter().enumerate() {
            let field_binding = format!("__self_{field_index}").parse::<TokenStream>()?;

            let FieldAttributes {
                modifications,
                encode_with,
                ..
            } = get_field_attribute_modifications(Some(&quote! { *#field_binding }), &field.attrs)?;
            let encode = match encode_with {
                Some(expr) => quote! {(#expr)(#field_binding, to, version, primitive_repr)?;},
                None => quote! { #field_binding.encode(to, version, primitive_repr)?; },
            };
            variant_encode_body = quote! {
                #variant_encode_body
                {
                    #modifications
                    if use_field {
                        #encode
                    }
                }
            }
        }

        variant_arms = quote! {
            #variant_arms
            Self::#name #variant_binder => { #variant_encode_body }
        };

        discriminant_offset += 1;
    }
    Ok(quote! {
        match self {
            #variant_arms
        }
    })
}

fn create_struct_decode_body(data_struct: &DataStruct) -> syn::Result<TokenStream> {
    let mut field_movers = quote! {};
    let mut binders = quote! {};
    for (field_index, field) in data_struct.fields.iter().enumerate() {
        let field_type = &field.ty;
        let field_name = &field.ident;

        let FieldAttributes {
            modifications,
            usage_is_conditional,
            version_overridden,
            bind_to_name,
            decode_with,
            ..
        } = get_field_attribute_modifications(None, &field.attrs)?;

        let (declaration, accessor) = if version_overridden {
            (quote! {}, quote! { version })
        } else if bind_to_name {
            let name = field_name
                .as_ref()
                .expect("`bind` must be used on named fields");
            (quote! { let #name }, quote! { #name })
        } else {
            (
                format!("let __self_{field_index}").parse::<TokenStream>()?,
                format!("__self_{field_index}").parse::<TokenStream>()?,
            )
        };

        let mut value = match decode_with {
            Some(expr) => quote! { (#expr)(&mut *from, version, primitive_repr)? },
            None => quote! { <#field_type as Decodable>::decode(from, version, primitive_repr)? },
        };

        // TODO: defaulting customisation
        if usage_is_conditional {
            value = quote! {
                if use_field {
                    #value
                } else {
                    Default::default()
                }
            }
        }

        let field_mover;
        if field_name.is_some() {
            field_mover = quote! {
                #field_name: #accessor,
            };
        } else {
            field_mover = quote! { #accessor, };
        }

        binders = quote! {
            #binders
            #declaration;
            {
                #modifications
                #accessor = #value;
            }
        };
        field_movers = quote! {
            #field_movers
            #field_mover
        }
    }
    Ok(match &data_struct.fields {
        Fields::Named(_) => quote! { Ok({#binders Self { #field_movers }}) },
        Fields::Unnamed(_) => quote! { Ok({#binders Self (#field_movers)}) },
        Fields::Unit => quote! { Ok(Self) },
    })
}
fn create_enum_decode_body(
    data_enum: &DataEnum,
    discriminant_type: &str,
) -> syn::Result<TokenStream> {
    let discriminant_type_as_identifier = discriminant_type.parse::<TokenStream>()?;
    let mut variant_arms = quote! {};
    let mut discriminant = quote! { 0 };
    let mut discriminant_offset = 0;
    for variant in data_enum.variants.iter() {
        if let Some((_, discriminant_expression)) = &variant.discriminant {
            discriminant = quote! { #discriminant_expression };
            discriminant_offset = 0;
        }
        let offset_tokens = discriminant_offset.to_string().parse::<TokenStream>()?;
        let pattern =
            quote! { __discriminant if __discriminant == (#discriminant) + #offset_tokens };
        let arm;
        let variant_name = &variant.ident;
        if let Fields::Unit = variant.fields {
            arm = quote! { #pattern => { Ok(Self::#variant_name) } };
        } else {
            match &variant.fields {
                Fields::Named(fields) => {
                    let mut field_decoders = quote! {};
                    for field in &fields.named {
                        let FieldAttributes {
                            modifications,
                            usage_is_conditional,
                            version_overridden,
                            decode_with,
                            ..
                        } = get_field_attribute_modifications(None, &field.attrs)?;

                        let field_type = &field.ty;
                        let field_name = &field.ident;
                        let decode = match decode_with {
                            Some(expr) => quote! { (#expr)(&mut *from, version, primitive_repr)? },
                            None => {
                                quote! { <#field_type as Decodable>::decode(from, version, primitive_repr)? }
                            }
                        };

                        let mut value = decode;
                        if version_overridden {
                            value = quote! { { version = #value; version } }
                        }
                        // let mut value = if version_overridden {
                        //     quote! { version }
                        // } else {
                        //     decode
                        // };

                        // TODO: defaulting customisation
                        if usage_is_conditional || true {
                            value = quote! {
                                if use_field {
                                    #value
                                } else {
                                    Default::default()
                                }
                            };
                        }

                        field_decoders = quote! {
                            #field_decoders
                            #field_name: { #modifications #value },
                        };
                    }
                    arm = quote! { #pattern => { Ok(Self::#variant_name { #field_decoders }) } };
                }
                Fields::Unnamed(fields) => {
                    let mut field_decoders = quote! {};
                    for field in &fields.unnamed {
                        let FieldAttributes {
                            modifications,
                            usage_is_conditional,
                            version_overridden,
                            decode_with,
                            ..
                        } = get_field_attribute_modifications(None, &field.attrs)?;

                        let field_type = &field.ty;
                        let decode = match decode_with {
                            Some(expr) => quote! { (#expr)(&mut *from, version, primitive_repr)? },
                            None => {
                                quote! { <#field_type as Decodable>::decode(from, version, primitive_repr)? }
                            }
                        };

                        let mut value = decode;
                        if version_overridden {
                            value = quote! { { version = #value; version } }
                        }
                        // let mut value = if version_overridden {
                        //     quote! { version }
                        // } else {
                        //     decode
                        // };

                        if usage_is_conditional || true {
                            value = quote! {
                                if use_field {
                                    #value
                                } else {
                                    Default::default()
                                }
                            };
                        }

                        field_decoders = quote! {
                            #field_decoders
                            { #modifications #value },
                        };
                    }
                    arm = quote! { #pattern => { Ok(Self::#variant_name(#field_decoders)) } };
                }
                Fields::Unit => unreachable!(),
            }
        }
        variant_arms = quote! {
            #variant_arms
            #arm
        };
        discriminant_offset += 1;
    }
    Ok(quote! {
        match #discriminant_type_as_identifier::decode(from, version, primitive_repr)? {
            #variant_arms
            other => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("unknown discriminant {other}"))),
        }
    })
}

// TODO: `until` and `fixed_repr` field attributes
#[proc_macro_derive(
    Binary,
    attributes(since, until, override_repr, version, encode_with, decode_with, bind)
)]
pub fn decent(input: RustTokenStream) -> RustTokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut discriminant_type = "u32".to_owned();
    for attr in input.attrs {
        if let Some(ident) = attr.path().get_ident()
            && ident.to_string() == "repr"
            && let Some(repr_type) = attr.parse_args_with(parse_integer_type).ok().flatten()
        {
            discriminant_type = repr_type;
        }
    }

    let encode_body = match &input.data {
        Data::Struct(data_struct) => create_struct_encode_body(data_struct).unwrap(),
        Data::Enum(data_enum) => create_enum_encode_body(data_enum, &discriminant_type).unwrap(),
        Data::Union(_) => panic!("raw unions are not supported for binary formats"),
    };

    let decode_body = match &input.data {
        Data::Struct(data_struct) => create_struct_decode_body(data_struct).unwrap(),
        Data::Enum(data_enum) => create_enum_decode_body(data_enum, &discriminant_type).unwrap(),
        Data::Union(_) => panic!("raw unions are not supported for binary formats"),
    };

    let name = input.ident;
    RustTokenStream::from(quote! {
        impl Encodable for #name {
            fn encode(
                &self,
                to: &mut dyn std::io::Write,
                mut version: decent::Version,
                primitive_repr: decent::PrimitiveRepr,
            ) -> std::io::Result<()> {
                #encode_body
                Ok(())
            }
        }
        impl Decodable for #name {
            fn decode(
                from: &mut dyn std::io::Read,
                mut version: decent::Version,
                primitive_repr: decent::PrimitiveRepr,
            ) -> std::io::Result<Self> {
                #decode_body
            }
        }
    })
}
