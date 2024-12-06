mod compiler;
mod iter_utils;
mod json;
mod layout;

#[cfg(test)]
mod tests {
    use compiler::ir::IrParser;
    use layout::project_layout::ProjectLayout;

    use super::*;

    #[test]
    fn it_works() {
        let layout = ProjectLayout::auto_detect("../example").unwrap();
        let project = IrParser::new().parse(&layout);
        println!("{:#?}", project);
    }
}
