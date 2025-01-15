//  POINTERS.rs
//    by Lut99
//
//  Created:
//    13 Dec 2024, 14:22:51
//  Last edited:
//    15 Jan 2025, 10:41:07
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines an attribute macro for automatically implementing
//!   pointer-like types for a trait.
//

use std::collections::{HashMap, HashSet};

use bitvec::prelude::BitVec;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::token::Brace;
use syn::visit_mut::{self, VisitMut};
use syn::{
    AngleBracketedGenericArguments, Attribute, Error, Expr, ExprPath, FnArg, GenericArgument, GenericParam, Generics, Ident, ItemTrait, Lifetime,
    LifetimeParam, Meta, MetaList, Pat, Path, PathArguments, PathSegment, Token, TraitBound, TraitBoundModifier, TraitItem, TraitItemConst,
    TraitItemFn, TraitItemType, Type, TypeInfer, TypeParam, TypeParamBound, TypePath, TypeReference,
};


/***** HELPER FUNCTIONS *****/
/// Defines the default set of types, represented by the wildcard (*).
///
/// # Returns
/// A set of [`TypeToImpl`]s that describe the implementations to generate for the default types.
fn default_types() -> HashSet<TypeToImpl> {
    HashSet::from([
        // &'a _
        TypeToImpl {
            ty: Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: Some(Lifetime { apostrophe: Span::call_site(), ident: Ident::new("a", Span::call_site()) }),
                mutability: None,
                elem: Box::new(Type::Infer(TypeInfer { underscore_token: Default::default() })),
            }),
            mutable: false,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // &'a mut _
        TypeToImpl {
            ty: Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: Some(Lifetime { apostrophe: Span::call_site(), ident: Ident::new("a", Span::call_site()) }),
                mutability: Some(Default::default()),
                elem: Box::new(Type::Infer(TypeInfer { underscore_token: Default::default() })),
            }),
            mutable: true,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::std::boxed::Box<_>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("boxed", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("Box", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: true,
            generics: None,
            closure: None,
        },
        // ::std::rc::Rc<_>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("rc", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("Rc", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: false,
            generics: None,
            closure: None,
        },
        // ::std::sync::Arc<_>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("sync", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("Arc", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: false,
            generics: None,
            closure: None,
        },
        // ::std::cell::Ref<'a, _>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("cell", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("Ref", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: false,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::std::cell::RefMut<'a, _>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("cell", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("RefMut", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: true,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::std::sync::MutexGuard<'a, _>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("sync", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("MutexGuard", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: true,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::std::sync::RwLockReadGuard<'a, _>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("sync", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("RwLockReadGuard", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: false,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::std::sync::RwLockWriteGuard<'a, _>
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("sync", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("RwLockWriteGuard", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: true,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::parking_lot::MutexGuard<'a, _>
        #[cfg(feature = "parking_lot")]
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("parking_lot", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("MutexGuard", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: true,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::parking_lot::RwLockReadGuard<'a, _>
        #[cfg(feature = "parking_lot")]
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("parking_lot", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("RwLockReadGuard", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: false,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
        // ::parking_lot::RwLockWriteGuard<'a, _>
        #[cfg(feature = "parking_lot")]
        TypeToImpl {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("parking_lot", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment {
                            ident:     Ident::new("RwLockWriteGuard", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: Default::default(),
                                args: {
                                    let mut punct = Punctuated::new();
                                    punct.push(GenericArgument::Lifetime(Lifetime {
                                        apostrophe: Span::call_site(),
                                        ident:      Ident::new("a", Span::call_site()),
                                    }));
                                    punct.push(GenericArgument::Type(Type::Infer(TypeInfer { underscore_token: Default::default() })));
                                    punct
                                },
                                gt_token: Default::default(),
                            }),
                        });
                        punct
                    },
                },
            }),
            mutable: true,
            generics: Some(Generics {
                lt_token: Some(Default::default()),
                params: {
                    let mut params = Punctuated::new();
                    params.push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime::new("'a", Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                    params
                },
                gt_token: Some(Default::default()),
                where_clause: None,
            }),
            closure: None,
        },
    ])
}

/// Injects additional types into the given generics.
///
/// # Arguments
/// - `t`: The name of the special `T` to inject as the type bearing the target trait.
/// - `todo`: The [`ItemTrait`] encoding the trait to implement.
/// - `to_impl`: The type wrapping `T` for which we actually implement. Any of its generics are inject, EXCEPT if they ALREADY OCCUR (including `T`).
/// - `generics`: The [`Generics`] to inject in.
fn inject_additional_types(t: &Ident, todo: &ItemTrait, type_to_impl_gen: &Option<Generics>, generics: &mut Generics) {
    if let Some(type_to_impl_gen) = type_to_impl_gen {
        // Inject lifetimes first
        generics.params = type_to_impl_gen
            .lifetimes()
            .map(|l| GenericParam::Lifetime(l.clone()))
            .chain({
                let mut gens = Punctuated::new();
                std::mem::swap(&mut gens, &mut generics.params);
                gens.into_iter()
            })
            .collect();
        // Then any const- and type params
        generics.params.extend(
            type_to_impl_gen
                .const_params()
                .map(|c| GenericParam::Const(c.clone()))
                .chain(type_to_impl_gen.type_params().map(|t| GenericParam::Type(t.clone()))),
        );
    }

    // Push `T`
    generics.params.push(GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident: t.clone(),
        colon_token: Some(Default::default()),
        bounds: {
            let mut bounds = Punctuated::new();
            bounds.push(TypeParamBound::Trait(TraitBound {
                paren_token: Some(Default::default()),
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: Path {
                    leading_colon: None,
                    segments:      {
                        let mut segments = Punctuated::new();
                        segments.push(PathSegment {
                            ident:     todo.ident.clone(),
                            arguments: if !todo.generics.params.is_empty() {
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Default::default(),
                                    args: todo
                                        .generics
                                        .params
                                        .iter()
                                        .map(|a| match a {
                                            GenericParam::Const(c) => GenericArgument::Const(Expr::Path(ExprPath {
                                                attrs: c.attrs.clone(),
                                                qself: None,
                                                path:  Path {
                                                    leading_colon: None,
                                                    segments:      {
                                                        let mut segments = Punctuated::new();
                                                        segments.push(PathSegment { ident: c.ident.clone(), arguments: PathArguments::None });
                                                        segments
                                                    },
                                                },
                                            })),
                                            GenericParam::Lifetime(l) => GenericArgument::Lifetime(l.lifetime.clone()),
                                            GenericParam::Type(t) => GenericArgument::Type(Type::Path(TypePath {
                                                qself: None,
                                                path:  Path {
                                                    leading_colon: None,
                                                    segments:      {
                                                        let mut segments = Punctuated::new();
                                                        segments.push(PathSegment { ident: t.ident.clone(), arguments: PathArguments::None });
                                                        segments
                                                    },
                                                },
                                            })),
                                        })
                                        .collect(),
                                    gt_token: Default::default(),
                                })
                            } else {
                                PathArguments::None
                            },
                        });
                        segments
                    },
                },
            }));
            bounds
        },
        eq_token: None,
        default: None,
    }));
}





/***** VISITORS *****/
/// Visitor that resolves all self types with the given one.
struct SelfResolver {
    ident: Ident,
}
impl VisitMut for SelfResolver {
    fn visit_type_mut(&mut self, node: &mut Type) {
        // If the type is the self one, then replace it and done
        if let Type::Path(ty) = node {
            // Check if the first segment is Self
            if let Some(PathSegment { ident, arguments: _ }) = ty.path.segments.iter_mut().next() {
                if ident == "Self" {
                    // It is; now replace it
                    *ident = self.ident.clone();
                }
            }

            // Any other type is handled with the default impl!
            visit_mut::visit_type_mut(self, node);
        } else {
            // Any other type is handled with the default impl!
            visit_mut::visit_type_mut(self, node)
        }
    }
}

/// Visitor that resolves all inferred types with the given one.
struct InferResolver {
    ty: TypePath,
}
impl VisitMut for InferResolver {
    fn visit_type_mut(&mut self, node: &mut Type) {
        // If the type is the inferred one, then replace it and done
        if matches!(node, Type::Infer(_)) {
            *node = Type::Path(self.ty.clone());
        } else {
            // Any other type is handled with the default impl!
            visit_mut::visit_type_mut(self, node)
        }
    }
}





/***** GENERATOR *****/
/// Specifies that which we need to know about every to-be-generated type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TypeToImpl {
    /// The type to implement for.
    ty: Type,
    /// Whether this type is interior mutable or not.
    mutable: bool,
    /// The generics to add for this type.
    generics: Option<Generics>,
    /// The optional closure that maps `self` to whatever.
    closure: Option<Expr>,
}

/// Specifies the attributes we're parsing from the attribute.
struct Attributes {
    /// The generic type to use in the impls
    generic: Ident,
    /// The list of types for which to generate the impls
    types:   HashSet<TypeToImpl>,
}
impl Default for Attributes {
    #[inline]
    fn default() -> Self { Self { generic: Ident::new("T", Span::call_site()), types: default_types() } }
}
impl Parse for Attributes {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let default_types: HashSet<TypeToImpl> = default_types();
        let mut first: bool = true;
        let mut attr = Self::default();
        while !input.is_empty() {
            // Parse any punctuation
            if !first {
                input.parse::<Token![,]>()?;
                // This could've been the trailing
                if input.is_empty() {
                    break;
                }
            }

            // Parse either 'impl' or 'unimpl'
            let add: bool = input.parse::<Token![impl]>().is_ok();
            if !add {
                // Attempt to parse an identifier instead
                match input.parse::<Ident>() {
                    Ok(ident) => {
                        let sident: String = ident.to_string();
                        if sident == "T" {
                            // Change tacks; we now enter generics mode!
                            input.parse::<Token![=]>()?;
                            attr.generic = input.parse::<Ident>()?;
                            first = false;
                            continue;
                        } else if sident != "unimpl" {
                            return Err(Error::new(ident.span(), "Expected either 'impl' or 'unimpl'"));
                        }
                    },
                    Err(_) => return Err(input.error("Expected either 'impl' or 'unimpl'")),
                }
            }

            // Parse the optional `mut`
            let mutable: bool = if add { input.parse::<Token![mut]>().is_ok() } else { false };

            // Then parse any generics
            let generics: Option<Generics> = if add { input.parse().ok() } else { None };

            // Then parse either a wildcard OR a type
            if !add && input.parse::<Token![*]>().is_ok() {
                if add {
                    attr.types.extend(default_types.iter().cloned());
                } else {
                    attr.types.retain(|t| !default_types.contains(t));
                }
            } else {
                // Parse the type first
                let ty: Type = input.parse()?;

                // Optionally parse the closure
                let closure: Option<Expr> = if input.parse::<Token![=]>().is_ok() { Some(input.parse()?) } else { None };

                // Process the changes
                if add {
                    attr.types.insert(TypeToImpl { ty, mutable, generics, closure });
                } else {
                    attr.types.retain(|todo| todo.ty != ty);
                }
            }

            // Note we've done one impl
            first = false;
        }
        Ok(attr)
    }
}



/// Specifies that which we need to know about every item in the input trait.
struct ImplsToDo {
    /// The original definition.
    def: ItemTrait,
    /// A mask of items in `def` to actually do.
    item_mask: BitVec,
    /// A list of attributes for items that we generate (i.e., items with the mask on 1)
    item_attrs: HashMap<usize, ItemAttributes>,
    /// Whether anything in this trait requires interior mutability of the pointer.
    requires_mutable: bool,
}
impl Parse for ImplsToDo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Ensure we're parsing a trait implementation
        let mut def: ItemTrait = input.parse()?;

        // Go through its items to find the interior mutability status
        let mut item_attrs: HashMap<usize, ItemAttributes> = HashMap::with_capacity(def.items.len());
        let mut item_mask: BitVec = BitVec::with_capacity(def.items.len());
        let mut requires_mutable: bool = false;
        for item in &mut def.items {
            // Get the attributes of this item and whether it would be included based on e.g. not
            // having a default implementation
            let attrs: &mut Vec<Attribute> = match item {
                TraitItem::Const(c) => {
                    // Assert first that there are no generics (wtf does that even mean on
                    // associated constants)
                    if c.generics.params.is_empty() {
                        return Err(Error::new(c.generics.span(), "Associated constants with generics are not supported by `#[pointer_impls]`"));
                    }

                    // OK, now return the info we want
                    &mut c.attrs
                },
                TraitItem::Fn(f) => {
                    // First, we mark if this makes the trait require internal mutability
                    for arg in &f.sig.inputs {
                        match arg {
                            FnArg::Receiver(r) => {
                                requires_mutable |= r.mutability.is_some();
                                break;
                            },
                            _ => {},
                        }
                    }

                    // Then return the attributes
                    &mut f.attrs
                },
                TraitItem::Type(ty) => &mut ty.attrs,

                // Ignore macro invocations and vertabim
                TraitItem::Macro(m) => {
                    eprintln!("WARNING: Trait item '{m:?}' is ignored in impls provided by `#[pointer_impls]`");
                    item_mask.push(false);
                    continue;
                },
                TraitItem::Verbatim(v) => {
                    eprintln!("WARNING: Trait item '{v:?}' is ignored in impls provided by `#[pointer_impls]`");
                    item_mask.push(false);
                    continue;
                },
                _other => unimplemented!(),
            };

            // Decide whether to push based on the presence of the attribute & boolean
            item_attrs.insert(item_mask.len(), ItemAttributes::try_from(attrs)?);
            item_mask.push(true);
        }

        // OK, done
        Ok(Self { def, item_mask, item_attrs, requires_mutable })
    }
}

/// Specifies the attributes users can give on trait items.
struct ItemAttributes {
    /// The list of generics to push for this item.
    generics: Option<AngleBracketedGenericArguments>,
}
impl TryFrom<&mut Vec<Attribute>> for ItemAttributes {
    type Error = syn::Error;

    fn try_from(value: &mut Vec<Attribute>) -> Result<Self, Self::Error> {
        // Collect the attributes of interest
        let mut attrs: Vec<TokenStream2> = Vec::with_capacity(value.len());
        value.retain_mut(|attr| match &mut attr.meta {
            // We're interested in `#[pointer_impl(...)]` only (for now)
            Meta::List(l) => {
                if l.path.is_ident("pointer_impl") {
                    // Match; extract the attribute from it (so it doesn't linger in the re-
                    // generated definition of the trait)
                    let mut tokens: TokenStream2 = TokenStream2::new();
                    std::mem::swap(&mut tokens, &mut l.tokens);
                    attrs.push(tokens);
                    false
                } else {
                    true
                }
            },

            Meta::Path(_) | Meta::NameValue(_) => true,
        });

        // Attempt to parse each of those
        let mut attr = Self { generics: None };
        for tokens in attrs {
            let metas: Punctuated<BetterMeta, Token![,]> = syn::parse2::<BetterMetas>(tokens)?.0;
            for meta in metas {
                match meta {
                    BetterMeta::NameValue(nv) => {
                        if nv.path.is_ident("generics") {
                            // Parse the value as the required list
                            attr.generics = Some(syn::parse2(nv.value)?);
                        } else {
                            return Err(Error::new(nv.path.span(), format!("Unknown pointer_impl attribute {}", nv.path.into_token_stream())));
                        }
                    },

                    BetterMeta::Path(p) => {
                        return Err(Error::new(p.span(), format!("Unknown pointer_impl attribute {}", p.into_token_stream())));
                    },
                    BetterMeta::List(l) => {
                        return Err(Error::new(l.path.span(), format!("Unknown pointer_impl attribute {}", l.path.into_token_stream())));
                    },
                }
            }
        }

        // OK!
        Ok(attr)
    }
}

/// Specifies a wrapper around [`Punctuated<BetterMeta, Token![,]>`](Punctuated) that parses it.
struct BetterMetas(Punctuated<BetterMeta, Token![,]>);
impl Parse for BetterMetas {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> { Ok(Self(Punctuated::parse_terminated(input)?)) }
}

/// Specifies a more lenient [`Meta`] based on a [`BetterMetaNameValue`].
enum BetterMeta {
    /// It's a path.
    Path(Path),
    /// It's a list of paths.
    List(MetaList),
    /// It's a name/value pair.
    NameValue(BetterMetaNameValue),
}
impl Parse for BetterMeta {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(nv) = input.parse::<BetterMetaNameValue>() {
            return Ok(Self::NameValue(nv));
        }
        if let Ok(l) = input.parse::<MetaList>() {
            return Ok(Self::List(l));
        }
        Ok(Self::Path(input.parse()?))
    }
}

/// Specifies a more lenient [`MetaNameValue`](syn::MetaNameValue).
struct BetterMetaNameValue {
    path:      Path,
    _eq_token: Token![=],
    value:     TokenStream2,
}
impl Parse for BetterMetaNameValue {
    #[inline]
    fn parse(input: ParseStream) -> syn::Result<Self> { Ok(Self { path: input.parse()?, _eq_token: input.parse()?, value: input.parse()? }) }
}



/// Implements the main struct doing the heavy lifting.
struct Generator {
    /// What we parsed from the attribute stream.
    attrs: Attributes,
    /// What we parsed from the item stream.
    todo:  ImplsToDo,
}
impl ToTokens for Generator {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // Define the `T`-type
        let t = TypePath {
            qself: None,
            path:  Path {
                leading_colon: None,
                segments:      {
                    let mut puncts = Punctuated::new();
                    puncts.push(PathSegment { ident: self.attrs.generic.clone(), arguments: PathArguments::None });
                    puncts
                },
            },
        };
        let mut infer_resolver = InferResolver { ty: t.clone() };
        let mut self_resolver = SelfResolver { ident: self.attrs.generic.clone() };

        // Resolve the types in the original definition, once for `Self` and once for `T`
        let mut def = self.todo.def.clone();
        infer_resolver.visit_item_trait_mut(&mut def);

        // First, write the original definition
        def.to_tokens(tokens);

        // Extract some things from the def
        let name: &Ident = &def.ident;
        let mut generics: Generics = def.generics.clone();
        self_resolver.visit_generics_mut(&mut generics);
        let (_, trait_ty_gen, trait_where_clause) = generics.split_for_impl();

        // Generate an implementation for each of the given pointer types
        for to_impl in &self.attrs.types {
            // Skip this impl if it requires mutability
            if !to_impl.mutable && self.todo.requires_mutable {
                continue;
            }

            // Resolve the type's inferred to concrete ones
            let mut ty: Type = to_impl.ty.clone();
            infer_resolver.visit_type_mut(&mut ty);

            // Inject the necessary types
            let mut altered_generics = def.generics.clone();
            inject_additional_types(&self.attrs.generic, &def, &to_impl.generics, &mut altered_generics);
            let (trait_impl_gen, _, _) = altered_generics.split_for_impl();

            // Build the items of the impls
            let mut items: Vec<TokenStream2> = Vec::with_capacity(self.todo.item_mask.count_ones());
            for (i, item) in def.items.iter().enumerate() {
                // Apply the mask
                if !self.todo.item_mask[i] {
                    continue;
                }

                // Generate the impl
                match item {
                    // Associated constants
                    TraitItem::Const(c) => {
                        let TraitItemConst { attrs, const_token, ident, generics, colon_token, ty, default, semi_token } = c;
                        #[cfg(debug_assertions)]
                        if !generics.params.is_empty() {
                            panic!("Got non-empty associated constant generics after parsing");
                        }

                        // Generate the associated constant's impl as:
                        // ```
                        // #[foo]
                        // const BAR: Baz = <T as Quz>::BAR;
                        // ```
                        let mut tokens = quote! { #(#attrs)* #const_token #ident #colon_token #ty };
                        if let Some((eq, _)) = default {
                            eq.to_tokens(&mut tokens);
                        } else {
                            <Token![=]>::default().to_tokens(&mut tokens);
                        }
                        tokens.extend(quote! {<#t as #name #trait_ty_gen>::#ident #semi_token });

                        // Keep it!
                        items.push(tokens);
                    },

                    // Associated methods
                    TraitItem::Fn(f) => {
                        let TraitItemFn { attrs, sig, default, semi_token: _ } = f;
                        let ident: &Ident = &sig.ident;

                        // Collect the parameters (which are patterns, of course :#)
                        let mut this: Option<Ident> = None;
                        let passing_args: Punctuated<Pat, Token![,]> = sig
                            .inputs
                            .iter()
                            .filter_map(|a| match a {
                                FnArg::Receiver(r) => {
                                    this = Some(Ident::new("self", r.span()));
                                    None
                                },
                                FnArg::Typed(t) => Some((*t.pat).clone()),
                            })
                            .collect();

                        // Generate the associated method's impl as:
                        // ```
                        // #[foo]
                        // fn bar(&self, baz: Quz) -> Qux { <T as Cuz>::bar(self, baz) }
                        // ```
                        let mut tokens = quote! { #(#attrs)* #sig };
                        if let Some(block) = default {
                            block.brace_token.surround(&mut tokens, |tokens| {
                                // Either generate the default types, or the custom one
                                if let Some(generics) = &self.todo.item_attrs.get(&i).unwrap().generics {
                                    tokens.extend(quote! { <#t as #name #trait_ty_gen>::#ident :: #generics });
                                } else {
                                    let mut generics: Generics = sig.generics.clone();
                                    generics.params =
                                        generics.params.into_iter().filter(|param| !matches!(param, GenericParam::Lifetime(_))).collect();
                                    let (_, ty_gen, _) = generics.split_for_impl();
                                    let ty_gen = ty_gen.as_turbofish();
                                    tokens.extend(quote! { <#t as #name #trait_ty_gen>::#ident #ty_gen });
                                }

                                // Write the contents of the parenthesis
                                sig.paren_token.surround(tokens, |tokens| {
                                    if let Some(this) = &this {
                                        if let Some(closure) = &to_impl.closure {
                                            tokens.extend(quote! { #closure })
                                        } else {
                                            tokens.extend(quote! { #this })
                                        }
                                    }
                                    if !passing_args.is_empty() {
                                        if this.is_some() {
                                            tokens.extend(quote! {,});
                                        }
                                        passing_args.to_tokens(tokens);
                                    }
                                });
                            });
                        } else {
                            Brace::default().surround(&mut tokens, |tokens| {
                                // Either generate the default types, or the custom one
                                if let Some(generics) = &self.todo.item_attrs.get(&i).unwrap().generics {
                                    tokens.extend(quote! { <#t as #name #trait_ty_gen>::#ident :: #generics });
                                } else {
                                    let mut generics: Generics = sig.generics.clone();
                                    generics.params =
                                        generics.params.into_iter().filter(|param| !matches!(param, GenericParam::Lifetime(_))).collect();
                                    let (_, ty_gen, _) = generics.split_for_impl();
                                    let ty_gen = ty_gen.as_turbofish();
                                    tokens.extend(quote! { <#t as #name #trait_ty_gen>::#ident #ty_gen });
                                }

                                // Write the contents of the parenthesis
                                sig.paren_token.surround(tokens, |tokens| {
                                    if let Some(this) = &this {
                                        if let Some(closure) = &to_impl.closure {
                                            tokens.extend(quote! { #closure })
                                        } else {
                                            tokens.extend(quote! { #this })
                                        }
                                    }
                                    if !passing_args.is_empty() {
                                        if this.is_some() {
                                            tokens.extend(quote! {,});
                                        }
                                        passing_args.to_tokens(tokens);
                                    }
                                });
                            });
                        }

                        // Keep it!
                        items.push(tokens);
                    },

                    // Associated types
                    TraitItem::Type(ty) => {
                        let TraitItemType { attrs, type_token, ident, generics, colon_token: _, bounds: _, default, semi_token } = ty;
                        let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();

                        // Generate the associated type as:
                        // ```
                        // #[foo]
                        // type Bar<BAZ> = <T as Qux>::Bar<BAZ> where BAZ: 'static;
                        // ```
                        let mut tokens = quote! { #(#attrs)* #type_token #ident #impl_gen };
                        if let Some((eq, _)) = default {
                            eq.to_tokens(&mut tokens);
                        } else {
                            <Token![=]>::default().to_tokens(&mut tokens);
                        }
                        tokens.extend(quote! { <#t as #name #trait_ty_gen>::#ident #ty_gen #where_clause #semi_token });

                        // Keep it!
                        items.push(tokens);
                    },

                    // Things we don't care about
                    TraitItem::Macro(_) => panic!("Got macro item for trait even though the parsing should have filtered these out"),
                    TraitItem::Verbatim(_) => panic!("Got vertabim item for trait even though the parsing should have filtered these out"),
                    _other => panic!("Got other item for trait even though the parsing should have filtered these out"),
                }
            }

            // Now build the overall impl
            tokens.extend(quote! { impl #trait_impl_gen #name #trait_ty_gen for #ty #trait_where_clause { #(#items)* } })
        }
    }
}





/***** LIBRARY *****/
/// Actual implementation of the `pointer_impls`-macro.
///
/// # Arguments
/// - `attr`: The stream that is given with the attribute.
/// - `item`: The item that the attribute spans.
///
/// # Returns
/// A new [`TokenStream2`] that encodes the original item + additional implementations.
///
/// # Errors
/// This function may error if anything about the input was incompatible with this macro.
pub fn pointer_impls(attr: TokenStream2, item: TokenStream2) -> Result<TokenStream2, Error> {
    // Parse the two streams into the Generator, which will do the necessary generation
    let generator: Generator = Generator { attrs: syn::parse2(attr)?, todo: syn::parse2(item)? };

    // Aaaaaand generate it
    Ok(generator.to_token_stream())
}
