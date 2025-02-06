mod assets;
pub mod resolver;
pub mod source;
mod util;

pub(crate) use assets::Assets;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{resolver::Resolver, source::entity::Environment};

    #[test]
    fn test_example() {
        let mut environment: Environment = Environment::default().unwrap();
        environment.load_local("./../example").unwrap();
        environment.set_target_project(environment.projects()[1].identifier());

        let resolver = Resolver::of(&environment);

        println!("{:#?}", resolver);
    }
}
