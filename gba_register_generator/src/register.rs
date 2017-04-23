use syn::Ident;
use quote::Tokens;
use std::rc::Rc;

#[derive(Builder, Debug, Clone)]
#[builder(setter(into))]
pub struct Variant {
    pub title: Ident,
    pub doc: String,

    #[builder(default="None")]
    pub value: Option<usize>,

    #[builder(default="None")]
    pub quick_get: Option<Ident>,

    #[builder(default="None")]
    pub quick_set: Option<Ident>,
}


#[derive(Clone)]
pub enum Format {
    Type {
        type_ident: Ident,
        as_bits: Option<Rc<Fn(Tokens) -> Tokens>>,
        from_bits: Option<Rc<Fn(Tokens) -> Tokens>>
    },
    Enum {
        title: Ident,
        variants: Vec<Variant>,
    },
    Bool {
        quick_set_true: Option<Ident>,
        quick_set_false: Option<Ident>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Clone, Builder)]
#[builder(setter(into))]
pub struct Field {
    pub name: Ident,
    pub doc: String,
    pub access: Access,

    pub start: usize,
    pub size: usize,

    pub format: Format,
}

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct Register {
    pub title: Ident,
    pub doc: String,
    pub repr: Ident,

    #[builder(default="0")]
    pub reset_value: usize,

    pub fields: Vec<Field>,
}