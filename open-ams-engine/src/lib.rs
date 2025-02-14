mod assets;
pub mod resolver;
pub mod source;
mod util;
pub mod entity;

pub(crate) use assets::Assets;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{resolver::Resolver, source::entity::SourceEnvironment};

    #[test]
    fn test_example() {
        let mut environment: SourceEnvironment = SourceEnvironment::default().unwrap();
        environment.load_local("./../example").unwrap();
        environment.set_target_project(environment.projects()[1].identifier());

        let resolver = Resolver::of(&environment);

        println!("{:#?}", resolver.resolve());
    }
}
