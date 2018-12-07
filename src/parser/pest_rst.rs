#![allow(clippy::redundant_closure)]

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;
