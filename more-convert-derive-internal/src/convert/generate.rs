use syn::{parenthesized, parse::Parse};

pub(crate) struct GenerateArg {
    pub into_ident: syn::Ident,
    pub field_ident: syn::Ident,
    pub expr: syn::Expr,
}

impl Parse for GenerateArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let into_ident = input.parse()?;
        let content;
        let _ = parenthesized!(content in input);
        let field_ident = content.parse()?;
        content.parse::<syn::Token![=]>()?;
        let expr = content.parse()?;
        Ok(Self {
            into_ident,
            field_ident,
            expr,
        })
    }
}
