use crate::source::entity::Environment;

#[derive(Debug)]
pub struct Resolver<'env> {
    pub(crate) env: &'env Environment,
}

impl<'env> Resolver<'env> {
    pub fn of(env: &'env Environment) -> Self {
        Resolver { env }
    }
}
