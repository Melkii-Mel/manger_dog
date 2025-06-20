use proc_macro2::Ident;
use syn::{LitStr, Type};

pub struct Entities {
    pub validator: Type,
    pub error: Type,
    pub entities: Vec<Entity>,
}

pub struct Entity {
    pub ident: Ident,
    pub dto_ident: Ident,
    pub error_ident: Ident,
    pub db_table_name: LitStr,
    pub paths_to_ownership: Vec<LitStr>,
    pub fields: Vec<Field>,
    pub dto_fields: Vec<DtoField>,
}

pub struct Field {
    pub ident: Ident,
    pub ty: Type,
    pub validators: Vec<Validator>,
}

pub struct DtoField {
    pub relationship: Relationship,
    pub foreign_ident: Ident,
    pub junction_ident: Ident,
}

pub struct Validator {
    pub ident: Ident,
    pub fields: Vec<Ident>,
}

pub enum Relationship {
    OTM,
    MTM,
}

impl Relationship {
    pub fn as_str(&self) -> &'static str {
        match self {
            Relationship::OTM => "otm",
            Relationship::MTM => "mtm",
        }
    }
}