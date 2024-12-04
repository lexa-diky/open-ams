mod iter_utils;
mod json;
mod layout;

#[cfg(test)]
mod tests {
    use layout::project_layout::ProjectLayout;

    use super::*;

    #[test]
    fn it_works() {
        let layout = ProjectLayout::auto_detect("../example").unwrap();
        println!("{:#?}", layout.module_by_path("myapp/types/documents/"));
    }
}
