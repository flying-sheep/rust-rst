#![warn(clippy::pedantic)]

use clap::Parser;

use rst_parser::parse;
use rst_renderer::{
    SchemaSettings, render_html, render_json, render_json_schema_document, render_xml,
};

use std::io::{self, Read};

#[derive(Debug, Clone, clap::ValueEnum)]
enum Format {
    Json,
    Xml,
    Html,
}

#[derive(Debug, Default, Clone, clap::ValueEnum)]
enum SchemaVersion {
    // tooling is hopelessly outdated, draft 7 is kind of the best bet
    #[default]
    #[value(name = "draft7")]
    Draft07,
    #[value(name = "2019-09")]
    Draft2019_09,
    #[value(name = "2020-12")]
    Draft2020_12,
    #[value(name = "openapi3")]
    OpenApi3,
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
    /// Print schema
    #[arg(long, num_args = ..=1, require_equals = true, default_missing_value = "draft7")]
    schema: Option<SchemaVersion>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();

    let level_filter = args
        .verbosity
        .log_level()
        .map_or(log::LevelFilter::Off, |l| l.to_level_filter());
    env_logger::Builder::new()
        .filter(Some("rst"), level_filter)
        .filter(None, log::Level::Warn.to_level_filter())
        .try_init()?;

    let stdout = std::io::stdout();

    if let Some(schema) = &args.schema {
        let settings = match schema {
            SchemaVersion::Draft07 => SchemaSettings::draft07(),
            SchemaVersion::Draft2019_09 => SchemaSettings::draft2019_09(),
            SchemaVersion::Draft2020_12 => SchemaSettings::draft2020_12(),
            SchemaVersion::OpenApi3 => SchemaSettings::openapi3(),
        };
        render_json_schema_document(stdout, settings, level_filter.to_level().is_some());
        return Ok(());
    }

    let content = preprocess_content(args.file.as_deref())?;
    let document = parse(&content)?;
    match args.format {
        Format::Json => render_json(&document, stdout)?,
        Format::Xml => render_xml(&document, stdout)?,
        Format::Html => render_html(&document, stdout, true)?,
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
