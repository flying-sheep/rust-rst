
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

	let input = Path::new(&manifest_dir).join("build_tests.py");
	let output = Path::new(&env::var("OUT_DIR").unwrap()).join("html_tests.rs");
    println!("cargo:rerun-if-changed={}", input.display());
	
    let output = Command::new("python3")
		.arg("build_tests.py")
		.arg(format!("{}", output.display()))
		.output()
		.expect("failed to execute process");
	io::stdout().write_all(&output.stdout).unwrap();
	io::stderr().write_all(&output.stderr).unwrap();
	assert!(output.status.success());
}
