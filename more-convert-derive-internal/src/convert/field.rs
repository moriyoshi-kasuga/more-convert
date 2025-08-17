use std::collections::HashMap;

use syn::{spanned::Spanned, Field, Ident};

use crate::{AttrMetas, MaybeOwned};

use super::{
    field_arg::{ConvertFieldArg, ConvertFieldMap},
    target::{parse_field_conversion_meta, Conversion},
};

pub(crate) struct ConvertField<'a> {
    pub ident: &'a Ident,
    pub all: ConvertFieldArg,
    pub target: HashMap<Conversion, ConvertFieldArg>,
}

impl<'a> ConvertField<'a> {
    pub(crate) fn get_arg_for_conversion(
        &'a self,
        conversion: &Conversion,
    ) -> MaybeOwned<'a, ConvertFieldArg> {
        match self.target.get(conversion) {
            Some(target_arg) => MaybeOwned::Owned(self.all.merge(target_arg)),
            None => MaybeOwned::Borrowed(&self.all),
        }
    }

    pub(crate) fn from_field(field: &'a Field, self_ident: &Ident) -> syn::Result<Self> {
        let Some(ref ident) = field.ident else {
            return Err(syn::Error::new(field.span(), "expected named field"));
        };

        let mut all: Option<ConvertFieldArg> = None;
        let mut target_arg = HashMap::new();

        for attr in &field.attrs {
            if !attr.path().is_ident("convert") {
                continue;
            }

            let nested = attr.parse_args_with(AttrMetas::parse_terminated)?;
            let mut iter = nested.into_iter().peekable();

            let Some(meta) = iter.peek() else {
                return Err(syn::Error::new(
                    attr.span(),
                    "expected at least one argument",
                ));
            };

            match parse_field_conversion_meta(meta, self_ident)? {
                Some(conversions) => {
                    iter.next();
                    let arg = ConvertFieldArg::from_meta_iter(&field.ty, iter)?;
                    for conversion in conversions {
                        target_arg.insert(conversion, arg.clone());
                    }
                }
                None => {
                    let arg = ConvertFieldArg::from_meta_iter(&field.ty, iter)?;
                    all = Some(arg);
                }
            }
        }

        let all = all.unwrap_or_else(|| ConvertFieldArg {
            ignore: false,
            map: ConvertFieldMap::gen_suffix(&field.ty),
            rename: None,
        });

        Ok(ConvertField {
            ident,
            all,
            target: target_arg,
        })
    }
}
