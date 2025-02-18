use std::collections::HashMap;

use syn::{spanned::Spanned, Field, Ident};

use crate::{AttrMetas, MaybeOwned};

use super::{
    field_arg::{ConvertFieldArg, ConvertFieldMap},
    ConvertTarget, GenType,
};

pub(crate) struct ConvertField<'a> {
    pub ident: &'a Ident,
    pub all: ConvertFieldArg,
    pub target: HashMap<ConvertTarget, ConvertFieldArg>,
}

impl<'a> ConvertField<'a> {
    pub(crate) fn get_arg_with_merge(
        &'a self,
        gen_type: &GenType<'a>,
    ) -> MaybeOwned<'a, ConvertFieldArg> {
        macro_rules! get_merge {
            ($target:expr,$merge:expr) => {
                match self.target.get($target) {
                    Some(target) => $merge.map(|v| MaybeOwned::Owned(v.merge(target))),
                    None => $merge,
                }
            };
        }

        macro_rules! get {
            ($target:expr) => {
                match self.target.get($target) {
                    Some(target) => MaybeOwned::Owned(target.merge(&self.all)),
                    None => MaybeOwned::Borrowed(&self.all),
                }
            };
        }

        match gen_type {
            GenType::From(ident) => {
                let from = get!(&ConvertTarget::From((*ident).clone()));
                get_merge!(&ConvertTarget::FromInto((*ident).clone()), from)
            }
            GenType::Into(ident) => {
                let into = get!(&ConvertTarget::Into((*ident).clone()));
                get_merge!(&ConvertTarget::FromInto((*ident).clone()), into)
            }
        }
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

        Ok(ConvertField {
            ident,
            all,
            target: target_arg,
        })
    }
}
