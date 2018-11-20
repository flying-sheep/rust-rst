pub mod document_tree;
pub mod parser;


use structopt::StructOpt;
use clap::{_clap_count_exprs, arg_enum};
use quicli::{main, fs::read_file, prelude::Verbosity};

use self::parser::{
    serialize_json,
    serialize_xml,
};

arg_enum! {
    #[derive(Debug)]
    enum Format { json, xml }
}

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(
        long = "format", short = "f", default_value = "json",  // xml is pretty defunct…
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
    )]
    format: Format,
    file: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    let content = read_file(args.file)?;
    let stdout = std::io::stdout();
    match args.format {
        Format::json => serialize_json(&content, stdout)?,
        Format::xml  => serialize_xml (&content, stdout)?,
    }
});
