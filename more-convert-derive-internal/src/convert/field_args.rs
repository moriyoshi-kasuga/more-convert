use std::collections::HashMap;

use syn::{spanned::Spanned, Field, Ident};

use crate::AttrMetas;

use super::{
    field_arg::{ConvertFieldArg, ConvertFieldMap},
    ConvertTarget,
};

pub(crate) struct ConvertFieldArgs<'a> {
    pub ident: &'a Ident,
    pub all: ConvertFieldArg,
    pub target: HashMap<ConvertTarget, ConvertFieldArg>,
}

impl<'a> ConvertFieldArgs<'a> {
    pub(crate) fn get_top_priority_arg(&self, target: &ConvertTarget) -> &ConvertFieldArg {
        self.target.get(target).unwrap_or(&self.all)
    }

    pub(crate) fn from_field(field: &'a Field) -> syn::Result<Self> {
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

            match ConvertTarget::option_from_meta(meta)? {
                Some(targets) => {
                    iter.next();
                    let arg = ConvertFieldArg::from_meta_iter(&field.ty, iter)?;
                    for target in targets {
                        target_arg.insert(target, arg.clone());
                    }
                }
                None => {
                    let arg = ConvertFieldArg::from_meta_iter(&field.ty, iter)?;
                    all = Some(arg);
                }
            }
        }

        let all = match all {
            Some(all) => all,
            None => ConvertFieldArg {
                ignore: false,
                map: ConvertFieldMap::gen_suffix(&field.ty),
                rename: None,
            },
        };

        Ok(ConvertFieldArgs {
            ident,
            all,
            target: target_arg,
        })
    }
}
