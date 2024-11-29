use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ams-json.pest"]
pub(crate) struct AmsJsonParser;
