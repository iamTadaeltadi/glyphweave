use std::{
    fs,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Output},
};

pub(crate) fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/reference.ttf")
}

pub(crate) fn ttx_xml(tables: &[&str]) -> Option<String> {
    let mut command = Command::new("ttx");
    command.args(["-q", "-o", "-"]);
    for table in tables {
        command.args(["-t", table]);
    }
    command.arg(fixture_path());
    capture("ttx", &mut command)
}

pub(crate) fn subset(tool: &str, glyph: u16) -> Option<Vec<u8>> {
    subset_with(tool, format!("--gids={glyph}"))
}

pub(crate) fn subset_unicode(tool: &str, codepoint: u32) -> Option<Vec<u8>> {
    subset_with(tool, format!("--unicodes=U+{codepoint:X}"))
}

fn subset_with(tool: &str, selection: String) -> Option<Vec<u8>> {
    let output_path = std::env::temp_dir().join(format!(
        "glyphweave-{tool}-{}-subset.ttf",
        std::process::id()
    ));
    let mut command = Command::new(tool);
    command.arg(fixture_path()).args([
        selection,
        "--retain-gids".to_owned(),
        format!("--output-file={}", output_path.display()),
    ]);
    execute(tool, &mut command)?;
    let bytes = fs::read(&output_path).expect("reference subset must be readable");
    fs::remove_file(&output_path).expect("reference subset must be removable");
    Some(bytes)
}

fn capture(tool: &str, command: &mut Command) -> Option<String> {
    let output = execute(tool, command)?;
    Some(String::from_utf8(output.stdout).expect("reference output must be UTF-8"))
}

fn execute(tool: &str, command: &mut Command) -> Option<Output> {
    let output = match command.output() {
        Ok(output) => output,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            eprintln!("skipping differential reference because {tool} is unavailable");
            return None;
        }
        Err(error) => panic!("failed to invoke {tool}: {error}"),
    };
    assert!(
        output.status.success(),
        "{tool} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Some(output)
}
