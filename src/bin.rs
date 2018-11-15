pub mod parser;


use pest::Parser;
use structopt::StructOpt;
use clap::{_clap_count_exprs, arg_enum};
use quicli::{main, fs::read_file, prelude::Verbosity};

use self::parser::{RstParser, Rule, serialize::PairsWrap};


arg_enum! {
    #[derive(Debug)]
    enum Format { json }
}

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(
        long = "format", short = "f", default_value = "json",
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
    )]
    format: Format,
    file: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    let content = read_file(args.file)?;
    let parsed = RstParser::parse(Rule::document, &content)?;
    let stdout = std::io::stdout();
    match args.format {
        Format::json => serde_json::to_writer(stdout, &PairsWrap(parsed))?,
    }
});
