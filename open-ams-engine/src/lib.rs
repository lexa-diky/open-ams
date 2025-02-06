mod assets;
pub mod entity;
pub mod resolver;
mod util;

pub(crate) use assets::Assets;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{entity::Environment, resolver::Resolver};

    #[test]
    fn test_example() {
        let mut environment: Environment = Environment::default().unwrap();
        environment.load_local("./../example").unwrap();
        environment.set_target_project(environment.projects()[1].identifier());

        let resolver = Resolver::of(&environment).form_target_project();

        println!("{:#?}", resolver);
    }
}
