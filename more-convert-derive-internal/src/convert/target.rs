use syn::{parenthesized, parse::Parse, punctuated::Punctuated, Ident, Meta, Token};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Conversion {
    pub from: Ident,
    pub to: Ident,
}

const EXPECT_TARGET: &str = "expected `from`, `into` or `from_into`";

// A single keyword argument, e.g., `from(A, B)`
struct ConvertArg {
    keyword: Ident,
    types: Punctuated<Ident, Token![,]>,
}

impl Parse for ConvertArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let keyword: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        let types = content.parse_terminated(Ident::parse, Token![,])?;

        match keyword.to_string().as_str() {
            "from" | "into" | "from_into" => Ok(Self { keyword, types }),
            _ => Err(syn::Error::new(keyword.span(), EXPECT_TARGET)),
        }
    }
}

// The full list of arguments in `#[convert(...)]`
pub(crate) struct ConvertArgs(Punctuated<ConvertArg, Token![,]>);

impl Parse for ConvertArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse_terminated(ConvertArg::parse, Token![,])?))
    }
}

impl ConvertArgs {
    pub(crate) fn into_conversions(self, self_ident: &Ident) -> syn::Result<Vec<Conversion>> {
        let mut conversions = Vec::new();
        for arg in self.0 {
            let keyword = arg.keyword.to_string();
            for ty in arg.types {
                match keyword.as_str() {
                    "from" => conversions.push(Conversion {
                        from: ty,
                        to: self_ident.clone(),
                    }),
                    "into" => conversions.push(Conversion {
                        from: self_ident.clone(),
                        to: ty,
                    }),
                    "from_into" => {
                        conversions.push(Conversion {
                            from: ty.clone(),
                            to: self_ident.clone(),
                        });
                        conversions.push(Conversion {
                            from: self_ident.clone(),
                            to: ty,
                        });
                    }
                    _ => {
                        return Err(syn::Error::new(
                            arg.keyword.span(),
                            format!("unexpected keyword: {}", keyword)
                        ));
                    }
                }
            }
        }
        conversions.sort();
        conversions.dedup();
        Ok(conversions)
    }
}

// For parsing field-level attributes like `#[convert(from(A), ...)]`
pub(crate) fn parse_field_conversion_meta(
    meta: &Meta,
    self_ident: &Ident,
) -> syn::Result<Option<Vec<Conversion>>> {
    let list = match meta {
        Meta::List(list) => list,
        _ => return Ok(None),
    };

    let Some(keyword) = list.path.get_ident() else {
        return Ok(None);
    };

    let idents = list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)?;

    let conversions = idents
        .into_iter()
        .flat_map(|ty| {
            let mut convs = Vec::new();
            match keyword.to_string().as_str() {
                "from" => convs.push(Conversion {
                    from: ty,
                    to: self_ident.clone(),
                }),
                "into" => convs.push(Conversion {
                    from: self_ident.clone(),
                    to: ty,
                }),
                "from_into" => {
                    convs.push(Conversion {
                        from: ty.clone(),
                        to: self_ident.clone(),
                    });
                    convs.push(Conversion {
                        from: self_ident.clone(),
                        to: ty,
                    });
                }
                _ => {}
            }
            convs
        })
        .collect::<Vec<_>>();

    if conversions.is_empty() {
        Ok(None)
    } else {
        Ok(Some(conversions))
    }
}
