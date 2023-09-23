#![warn(missing_docs, non_ascii_idents, trivial_numeric_casts,
    unused_crate_dependencies, noop_method_call, single_use_lifetimes, trivial_casts,
    unused_lifetimes, nonstandard_style, variant_size_differences)]
#![deny(keyword_idents)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(clippy::needless_return, clippy::while_let_on_iterator)]

pub use struct_metadata_derive::Described;

use std::collections::HashMap;



#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    Struct { name: String, children: Vec<Entry>, },
    Aliased { name: String, kind: Box<Descriptor> },
    Bool,
    U64,
    String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub label: String,
    pub docs: Option<Vec<&'static str>>,
    pub metadata: HashMap<&'static str, &'static str>,
    pub type_info: Descriptor,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Descriptor<Metadata=HashMap<&'static str, &'static str>> {
    pub docs: Option<Vec<&'static str>>,
    pub metadata: Metadata,
    pub kind: Kind,
}

pub trait Described<M: Default=HashMap<&'static str, &'static str>> {
    fn metadata() -> Descriptor<M>;
}

impl<M: Default> Described<M> for bool {
    fn metadata() -> Descriptor<M> {
        Descriptor { docs: None, metadata: Default::default(), kind: Kind::Bool }
    }
}

impl<M: Default> Described<M> for u64 {
    fn metadata() -> Descriptor<M> {
        Descriptor { docs: None, metadata: Default::default(), kind: Kind::U64 }
    }
}

impl<M: Default> Described<M> for String {
    fn metadata() -> Descriptor<M> {
        Descriptor { docs: None, metadata: Default::default(), kind: Kind::String }
    }
}
