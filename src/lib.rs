#![feature(iter_map_windows)]
use proc_macro2::{Group, TokenStream};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, DeriveInput, Generics, Ident, Token, Type, Visibility,
};

enum TopLevel {
    Function(Function),
    Module(Module),
    DeriveInput(DeriveInput),
    Value(Value),
    Else(TokenStream),
}

/**
Punct {
    ch: '#',
    spacing: Alone,
    span: #0 bytes(0..284),
},

Group {
    delimiter: Bracket,
    stream: TokenStream [
        Ident {
            ident: "no_op",
            span: #0 bytes(0..284),
        },
    ],
    span: #0 bytes(0..284),
},

Ident {
    ident: "pub",
    span: #0 bytes(0..284),
},

Ident {
    ident: "const",
    span: #0 bytes(0..234),
},

Ident {
    ident: "fn",
    span: #0 bytes(0..234),
},

Ident {
    ident: "test",
    span: #0 bytes(0..234),
},

Group {
    delimiter: Parenthesis,
    stream: TokenStream [
        Ident {
            ident: "x",
            span: #0 bytes(0..234),
        },
        Punct {
            ch: ':',
            spacing: Alone,
            span: #0 bytes(0..234),
        },
        Ident {
            ident: "i32",
            span: #0 bytes(0..234),
        },
    ],
    span: #0 bytes(0..234),
},

Punct {
    ch: '-',
    spacing: Joint,
    span: #0 bytes(0..234),
},

Punct {
    ch: '>',
    spacing: Alone,
    span: #0 bytes(0..234),
},

Ident {
    ident: "i32",
    span: #0 bytes(0..234),
},

Group {
    delimiter: Brace,
    stream: TokenStream [
        Ident {
            ident: "x",
            span: #0 bytes(0..234),
        },
    ],
    span: #0 bytes(0..234),
},

|vis|constness|'fn'|name|param|'-'|'>'|Type|Body|
|---|---|---|---|---|---|---|---|---|
|Option<Ident>| Option<Ident>| Ident| Ident| Group| Option<Punkt>| Option<Punkt>| Option<Ident>| Group|
*/
struct Function {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Constness,
    pub typ: Token![fn],
    pub ident: Ident,
    pub return_type: Option<(Token![-], Token![>], Type)>,
    pub body: Group,
}
impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            typ: input.parse()?,
            return_type: input.parse(),
            body: input.parse()?,
        })
    }
}
impl Function {}
enum Constness {
    Const(Token![const]),
    Mut(Token![mut]),
    None,
}
impl Constness {
    fn parse_mut(input: &syn::parse::ParseBuffer<'_>) -> Result<Self, syn::Error> {
        let mut_token = input.parse::<Token![mut]>()?;
        Ok(Self::Mut(mut_token))
    }

    fn parse_const(input: &syn::parse::ParseBuffer<'_>) -> Result<Self, syn::Error> {
        let const_token = input.parse::<Token![const]>()?;
        Ok(Self::Const(const_token))
    }
}
impl Parse for Constness {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![mut]) {
            Self::parse_mut(input)
        } else if input.peek(Token![const]) {
            Self::parse_const(input)
        } else {
            Ok(Self::None)
        }
    }
}

struct Module {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub key: Token![mod],
    pub ident: Ident,
    pub generics: Generics,
    pub body: TokenStream,
}
struct Value {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub body: TokenStream,
}
/// .
#[proc_macro_attribute]
pub fn const_fn(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    dbg!("{:?}", &input);
    input.into()
}

#[proc_macro_attribute]
pub fn no_op(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

#[proc_macro]
pub fn set_lints(input: proc_macro::TokenStream)
-> proc_macro::TokenStream{
    quote::quote!{
     #![warn(
     clippy::enum_glob_use,
     clippy::pedantic,
     clippy::nursery,
     clippy::unwrap_used,
 )]

    input}.into()
}