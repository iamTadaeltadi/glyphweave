use std::{io::ErrorKind, path::PathBuf, process::Command};

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

fn capture(tool: &str, command: &mut Command) -> Option<String> {
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
    Some(String::from_utf8(output.stdout).expect("reference output must be UTF-8"))
}
