mod context;
mod iter_utils;
mod json;
mod layout;

#[cfg(test)]
mod tests {
    use std::io::Read;

    use context::ProjectContext;
    use layout::project_layout::ProjectLayout;

    use super::*;

    #[test]
    fn it_works() {
        let layout = ProjectLayout::scan("../example", "myapp").unwrap();
        let context = ProjectContext::new(&layout);
    }
}
