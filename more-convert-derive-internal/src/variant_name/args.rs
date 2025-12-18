use convert_case::Case;
use syn::{
    parse::{Parse, ParseStream},
    LitStr, Token,
};

use crate::from_str_to_case;

mod kw {
    syn::custom_keyword!(without_trait);
    syn::custom_keyword!(rename_all);
    syn::custom_keyword!(prefix);
    syn::custom_keyword!(suffix);
    syn::custom_keyword!(rename);
    syn::custom_keyword!(nest);
}

#[derive(Default)]
pub(crate) struct VariantNameArgs {
    pub without_trait: Option<kw::without_trait>,
    pub rename_all: Option<Case<'static>>,
    pub prefix: Option<LitStr>,
    pub suffix: Option<LitStr>,
    pub rename: Option<LitStr>,
    pub nest: Option<kw::nest>,
}

impl Parse for VariantNameArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::without_trait) {
                args.without_trait = Some(input.parse()?);
            } else if lookahead.peek(kw::rename_all) {
                let _: kw::rename_all = input.parse()?;
                let _: Token![=] = input.parse()?;
                let lit: LitStr = input.parse()?;
                args.rename_all = Some(
                    from_str_to_case(&lit.value())
                        .ok_or_else(|| syn::Error::new(lit.span(), "invalid rename_all"))?,
                );
            } else if lookahead.peek(kw::prefix) {
                let _: kw::prefix = input.parse()?;
                let _: Token![=] = input.parse()?;
                args.prefix = Some(input.parse()?);
            } else if lookahead.peek(kw::suffix) {
                let _: kw::suffix = input.parse()?;
                let _: Token![=] = input.parse()?;
                args.suffix = Some(input.parse()?);
            } else if lookahead.peek(kw::rename) {
                let _: kw::rename = input.parse()?;
                let _: Token![=] = input.parse()?;
                args.rename = Some(input.parse()?);
            } else if lookahead.peek(kw::nest) {
                args.nest = Some(input.parse()?);
            } else {
                return Err(lookahead.error());
            }

            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
            }
        }
        Ok(args)
    }
}
