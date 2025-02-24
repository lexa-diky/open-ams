use std::fs::OpenOptions;
use crate::entity::TypeDefinitionIdentifier;

#[derive(Debug)]
pub struct EventArgument {
    name: String,
    description: OpenOptions,
    type_ref: TypeDefinitionIdentifier
}

#[derive(Debug)]
pub struct Event {
    name: String,
    description: OpenOptions,
    arguments: Vec<EventArgument>,
}