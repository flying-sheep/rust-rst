#![warn(clippy::pedantic)]

use anyhow::Result;
use clap::Parser;

use rst_parser::parse;
use rst_renderer::{render_html, render_json, render_xml};

use std::io::{self, Read};

#[derive(Debug, Clone, clap::ValueEnum)]
#[allow(non_camel_case_types)]
enum Format {
    json,
    xml,
    html,
}

#[derive(Debug, Parser)]
struct Cli {
    /// Output format
    #[arg(short = 'f', long, default_value = "html")]
    format: Format,
    /// Input file
    file: Option<String>,
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let level_filter = args.verbosity.log_level().unwrap().to_level_filter();
    env_logger::Builder::new()
        .filter(Some("rst"), level_filter)
        .filter(None, log::Level::Warn.to_level_filter())
        .try_init()?;

    let content = preprocess_content(args.file.as_deref())?;
    let document = parse(&content)?;
    let stdout = std::io::stdout();
    match args.format {
        Format::json => render_json(&document, stdout)?,
        Format::xml => render_xml(&document, stdout)?,
        Format::html => render_html(&document, stdout, true)?,
    }
    Ok(())
}

fn preprocess_content(file: Option<&str>) -> Result<String, clap::Error> {
    let mut content = if let Some(file) = file {
        std::fs::read_to_string(file)?
    } else {
        let mut stdin = String::new();
        io::stdin().read_to_string(&mut stdin)?;
        stdin
    };
    content = content.replace('\t', " ".repeat(8).as_ref());
    if !content.ends_with('\n') {
        content.push('\n');
    }
    Ok(content)
}
