use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token::Const, Item, ItemMod};

#[proc_macro_attribute]
pub fn const_fn(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let muts: Vec<(usize, (usize, proc_macro::TokenTree))> = input
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, tree)| match tree {
            proc_macro::TokenTree::Ident(ident) => ident.to_string() == "mut",
            _ => false,
        })
        .enumerate()
        .collect();
    let mut_fns: Vec<(usize, usize)> = muts
        .iter()
        .filter(|(i, _)| {
            if let Some((_, (_, proc_macro::TokenTree::Ident(ident)))) = muts.get(i + 1) {
                ident.to_string() == "fn"
            } else {
                false
            }
        })
        .map(|(i, (j, _))| (i.to_owned(), j.to_owned()))
        .collect();
    let input = input
        .into_iter()
        .enumerate()
        .filter_map(|(i, tree)| {
            let (_, ignores): (Vec<usize>, Vec<usize>) = mut_fns.iter().cloned().unzip();

            if !ignores.contains(&i) {
                dbg!("fine at ", i);
                Some(tree)
            } else {
                dbg!("Deleted tree ", tree, " at ", i);
                None
            }
        })
        .collect();
    let mut curr_mod = parse_macro_input!(input as ItemMod);
    let items: Vec<&mut Item> = curr_mod
        .content
        .iter_mut()
        .flat_map(|(_, items)| items)
        .collect();
    //dbg! {&items};
    items
        .into_iter()
        .filter_map(|item| match item {
            Item::Fn(x) => Some(x),
            _ => None,
        })
        .zip(mut_fns)
        .enumerate()
        .for_each(|(j, (fun, (_, i)))| match fun.sig.constness {
            Some(_) => (),
            None => {
                if i == j {
                } else {
                    fun.sig.constness = Some(Const::default());
                }
            }
        });
    let res = quote! {
        #curr_mod
    };
    res.into()
}

#[proc_macro_attribute]
pub fn no_op(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

#[proc_macro]
pub fn set_lints(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lint = lints();
    let input: TokenStream = input.into();
    quote::quote! {
        #lint

        #input
    }
    .into()
}

fn lints() -> TokenStream {
    quote::quote! {
        #![warn(
        clippy::enum_glob_use,
        clippy::pedantic,
        clippy::nursery,
        clippy::unwrap_used,
    )]}
}
