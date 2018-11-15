pub mod token;
pub mod serialize;
#[cfg(test)]
pub mod tests;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;
