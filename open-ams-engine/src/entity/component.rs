use crate::entity::epath::EPath;
use crate::entity::event::Event;

#[derive(Debug)]
pub enum ComponentType {
    Module,
    Page,
    Element,
    Widget,
}

type ComponentEPath = EPath;

#[derive(Debug)]
pub struct Component {
    epath: ComponentEPath,
    component_type: ComponentType,
    description: String,
    events: Vec<Event>,
    children: Vec<ComponentEPath>,
}

impl Component {
    pub fn module(epath: EPath) -> Self {
        Component {
            epath,
            component_type: ComponentType::Module,
            description: "".to_string(),
            events: vec![],
            children: vec![],
        }
    }

    pub fn epath(&self) -> &ComponentEPath {
        &self.epath
    }

    pub fn component_type(&self) -> &ComponentType {
        &self.component_type
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn children(&self) -> &Vec<ComponentEPath> {
        &self.children
    }
}
