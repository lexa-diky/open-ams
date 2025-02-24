use crate::entity::epath::EPath;
use crate::entity::event::Event;

pub enum ComponentType {
    Module, Page, Element
}

pub struct Component {
    pub epath: EPath,
    pub component_type: ComponentType,
    pub description: String,
    pub events: Vec<Event>,
}