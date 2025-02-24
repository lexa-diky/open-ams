use std::fs::OpenOptions;
use crate::entity::TypeDefinitionIdentifier;

pub struct EventArgument {
    name: String,
    description: OpenOptions,
    type_ref: TypeDefinitionIdentifier
}

pub struct Event {
    name: String,
    description: OpenOptions,
    arguments: Vec<EventArgument>,
}