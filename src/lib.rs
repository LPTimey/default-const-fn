use proc_macro2::{TokenStream, TokenTree};
use syn::{Attribute, DeriveInput, Generics, Ident, Token, Type, Visibility};

enum TopLevel {
    Function(Function),
    Module(Module),
    DeriveInput(DeriveInput),
    Value(Value),
    Else(TokenStream),
}
struct Function {
    pub vis: Visibility,
    pub r#const: Constnes,
    pub ident: Ident,
    pub generics: Generics,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Option<Type>,
    pub body: TokenStream,
}
impl Function {
    /*
    // Punct {
        ch: '#',
        spacing: Alone,
        span: #0 bytes(0..284),
    },
    // Group {
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
    vis            constness      'fn'    name  param  '-'            '>'             Type      Body
    Option<Ident>, Option<Ident>, Ident, Ident, Group, Option<Punkt> Option<Punkt> Option<Ident>, Group
    */
    pub fn new(input: TokenTree) -> Option<Self> {
        todo!()
    }
}
enum Constnes {
    Const(Token![const]),
    Mut(Token![mut]),
    None,
}
struct Module {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
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
    input
}

#[proc_macro_attribute]
pub fn no_op(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}
