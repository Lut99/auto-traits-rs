//  POINTERS.rs
//    by Lut99
//
//  Created:
//    13 Dec 2024, 14:22:51
//  Last edited:
//    13 Dec 2024, 16:08:37
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines an attribute macro for automatically implementing
//!   pointer-like types for a trait.
//

use std::collections::HashSet;

use bitvec::prelude::BitVec;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit_mut::{self, VisitMut};
use syn::{
    AngleBracketedGenericArguments, Attribute, Error, ExprClosure, FnArg, GenericArgument, Ident, ItemTrait, Lifetime, Meta, Path, PathArguments,
    PathSegment, Token, TraitItem, Type, TypeInfer, TypePath, TypeReference,
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
            ty:      Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: Some(Lifetime { apostrophe: Span::call_site(), ident: Ident::new("a", Span::call_site()) }),
                mutability: None,
                elem: Box::new(Type::Infer(TypeInfer { underscore_token: Default::default() })),
            }),
            mutable: false,
            closure: None,
        },
        // &'a mut _
        TypeToImpl {
            ty:      Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: Some(Lifetime { apostrophe: Span::call_site(), ident: Ident::new("a", Span::call_site()) }),
                mutability: Some(Default::default()),
                elem: Box::new(Type::Infer(TypeInfer { underscore_token: Default::default() })),
            }),
            mutable: true,
            closure: None,
        },
        // ::std::box::Box<_>
        TypeToImpl {
            ty:      Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: Some(Default::default()),
                    segments:      {
                        let mut punct = Punctuated::new();
                        punct.push(PathSegment { ident: Ident::new("std", Span::call_site()), arguments: PathArguments::None });
                        punct.push(PathSegment { ident: Ident::new("box", Span::call_site()), arguments: PathArguments::None });
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
            closure: None,
        },
        // ::std::rc::Rc<_>
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::std::sync::Arc<_>
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::std::cell::Ref<'a, _>
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::std::cell::RefMut<'a, _>
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::std::sync::MutexGuard<'a, _>
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::std::sync::RwLockReadGuard<'a, _>
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::std::sync::RwLockWriteGuard<'a, _>
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::parking_lot::MutexGuard<'a, _>
        #[cfg(feature = "parking_lot")]
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::parking_lot::RwLockReadGuard<'a, _>
        #[cfg(feature = "parking_lot")]
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
        // ::parking_lot::RwLockWriteGuard<'a, _>
        #[cfg(feature = "parking_lot")]
        TypeToImpl {
            ty:      Type::Path(TypePath {
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
            closure: None,
        },
    ])
}





/***** VISITORS *****/
/// Visitor that resolve all inferred types with the given one.
struct TypeResolver {
    ty: Type,
}
impl VisitMut for TypeResolver {
    fn visit_type_mut(&mut self, node: &mut Type) {
        // If the type is the inferred one, then replace it and done
        if matches!(node, Type::Infer(_)) {
            *node = self.ty.clone();
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
    ty:      Type,
    /// Whether this type is interior mutable or not.
    mutable: bool,
    /// The optional closure that maps `self` to whatever.
    closure: Option<ExprClosure>,
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
        let mut attr = Self::default();
        while !input.is_empty() {
            // Parse an optional not
            let add: bool = input.parse::<Token![!]>().is_ok();

            // Parse the optional `mut`
            let mutable: bool = input.parse::<Token![mut]>().is_ok();

            // Then parse either a wildcard OR a type
            if input.parse::<Token![*]>().is_ok() {
                if add {
                    attr.types.extend(default_types.iter().cloned());
                } else {
                    attr.types.retain(|t| !default_types.contains(t));
                }
            } else {
                // Parse the type first
                let ty: Type = input.parse()?;

                // Optionally parse the closure
                let closure: Option<ExprClosure> = if input.parse::<Token![=]>().is_ok() { Some(input.parse()?) } else { None };

                // Process the changes
                if add {
                    attr.types.insert(TypeToImpl { ty, mutable, closure });
                } else {
                    attr.types.retain(|todo| todo.ty != ty);
                }
            }
        }
        Ok(attr)
    }
}



/// Specifies the attributes users can give on trait items.
struct ItemAttributes {
    /// Whether the use told us to include this regardless.
    include_impl: bool,
}
impl TryFrom<&mut Vec<Attribute>> for ItemAttributes {
    type Error = syn::Error;

    fn try_from(value: &mut Vec<Attribute>) -> Result<Self, Self::Error> {
        // Iterate over the values
        let mut include_impl: bool = false;
        value.retain(|attr| {
            // Parse the attribute's meta
            match &attr.meta {
                Meta::Path(p) => {
                    if p.is_ident("include_impl") {
                        include_impl = true;
                        false
                    } else {
                        true
                    }
                },
                Meta::List(_) | Meta::NameValue(_) => true,
            }
        });
        Ok(Self { include_impl })
    }
}

/// Specifies that which we need to know about every item in the input trait.
struct ImplsToDo {
    /// The original definition.
    def: ItemTrait,
    /// A mask of items in `def` to actually do.
    item_mask: BitVec,
    /// Whether anything in this trait requires interior mutability of the pointer.
    requires_mutable: bool,
}
impl Parse for ImplsToDo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Ensure we're parsing a trait implementation
        let mut def: ItemTrait = input.parse()?;

        // Go through its items to find the interior mutability status
        let mut item_mask: BitVec = BitVec::with_capacity(def.items.len());
        let mut requires_mutable: bool = false;
        for item in &mut def.items {
            match item {
                // Associate constants
                TraitItem::Fn(f) => {
                    // Parse the attributes
                    let item_attrs = ItemAttributes::try_from(&mut f.attrs)?;

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

                    // Then decide whether to push based on the presence of the attribute & boolean
                    item_mask.push(item_attrs.include_impl || f.default.is_none());
                },

                // The rest never requires mutability
                _ => {
                    item_mask.push(true);
                },
            }
        }

        // OK, done
        Ok(Self { def, item_mask, requires_mutable })
    }
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
        // First, write the original definition
        self.todo.def.to_tokens(tokens);

        // Then, write the types
        let mut resolver = TypeResolver {
            ty: Type::Path(TypePath {
                qself: None,
                path:  Path {
                    leading_colon: None,
                    segments:      {
                        let mut puncts = Punctuated::new();
                        puncts.push(PathSegment { ident: Ident::new("T", Span::call_site()), arguments: PathArguments::None });
                        puncts
                    },
                },
            }),
        };
        for to_impl in &self.attrs.types {
            // Resolve the type's inferred to concrete ones
            let mut ty: Type = to_impl.ty.clone();
            resolver.visit_type_mut(&mut ty);

            // Build the items of the impls
            let items: Vec<TokenStream2> = Vec::with_capacity(self.todo.item_mask.count_ones());
            for (i, item) in self.todo.def.items.iter().enumerate() {
                // Apply the mask
                if !self.todo.item_mask[i] {
                    continue;
                }

                // Generate the impl
                match item {
                    TraitItem::Const(c) => {
                        let name = c.items.push(quote! { const #name: #ty = #value; });
                    },
                }
            }
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
