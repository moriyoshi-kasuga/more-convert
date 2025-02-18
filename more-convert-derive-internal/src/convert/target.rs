use syn::{spanned::Spanned, Ident, Meta, MetaList};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ConvertTarget {
    From(Ident),
    Into(Ident),
    FromInto(Ident),
}

const EXPECT_TARGET: &str = "expected `from`, `into` or `from_into`";

impl ConvertTarget {
    pub(crate) fn from_attr(attr: &syn::Attribute) -> syn::Result<Vec<Self>> {
        type Targets = syn::punctuated::Punctuated<syn::MetaList, syn::Token![,]>;
        let targets = attr.parse_args_with(Targets::parse_terminated)?;

        Ok(targets
            .iter()
            .map(Self::from_meta_list)
            .collect::<syn::Result<Vec<Vec<Self>>>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    pub(crate) fn from_meta_list(list: &MetaList) -> syn::Result<Vec<Self>> {
        type Idents = syn::punctuated::Punctuated<syn::Ident, syn::Token![,]>;

        macro_rules! parse_target {
            ($target_literal:literal,$target:ident,$list:ident) => {
                if $list.path.is_ident($target_literal) {
                    let idents: Idents = $list.parse_args_with(Idents::parse_terminated)?;
                    return Ok(idents.into_iter().map(ConvertTarget::$target).collect());
                }
            };
        }

        parse_target!("from", From, list);
        parse_target!("into", Into, list);
        parse_target!("from_into", FromInto, list);

        Err(syn::Error::new(list.span(), EXPECT_TARGET))
    }

    pub(crate) fn option_from_meta(meta: &Meta) -> syn::Result<Option<Vec<Self>>> {
        macro_rules! parse_target {
            ($target:literal) => {
                if meta.path().is_ident($target) {
                    return ConvertTarget::from_meta_list(meta.require_list()?).map(Some);
                }
            };
        }

        parse_target!("from");
        parse_target!("into");
        parse_target!("from_into");

        Ok(None)
    }
}
