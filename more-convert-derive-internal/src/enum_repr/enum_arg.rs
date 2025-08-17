use syn::{parse::Parse, punctuated::Punctuated, spanned::Spanned, Meta, Token};

#[derive(Default)]
pub(crate) struct EnumReprArg {
    pub serde: bool,
    pub implicit: bool,
}

impl Parse for EnumReprArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        let mut arg = Self::default();
        for meta in metas {
            match meta {
                Meta::Path(path) if path.is_ident("serde") => {
                    if arg.serde {
                        return Err(syn::Error::new(path.span(), "duplicate `serde` attribute"));
                    }
                    arg.serde = true;
                }
                Meta::Path(path) if path.is_ident("implicit") => {
                    if arg.implicit {
                        return Err(syn::Error::new(
                            path.span(),
                            "duplicate `implicit` attribute",
                        ));
                    }
                    arg.implicit = true;
                }
                _ => {
                    return Err(syn::Error::new(
                        meta.span(),
                        "unexpected attribute inside enum_repr, expected `serde` or `implicit`",
                    ));
                }
            };
        }
        Ok(arg)
    }
}
