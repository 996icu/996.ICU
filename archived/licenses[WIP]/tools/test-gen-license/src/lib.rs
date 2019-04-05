pub mod test_go;
pub mod test_py;
pub mod test_rust;

use assert_cmd::prelude::*;
use dir_diff;
use std::process::Command;
use tempdir;
use tempdir::TempDir;

#[test]
fn test_rust_help() {
    /// todo: compile rust executive file
    let path = "cargo";
    let mut cmd = Command::new("run");
    /// todo: add directory
    cmd.dir("");
    // assert_eq!(output, help);
    let assert = cmd.assert();
    assert.success().stdout(
        r#"gen-license-go is a 996.icu license generator implemented in Go,
this generator is developed to generate various open-source licenses including MIT, Apache, etc.
More importantly, the main purpose of this tool is to incorporate those aforesaid licenses into
a brand new license: 996.icu, defined by this repository.

Usage:
  gen-license-go [flags]
  gen-license-go [command]

Available Commands:
  gen         gen is a 996.icu license generator-command.
  help        Help about any command

Flags:
  -h, --help   help for gen-license-go
  -l, --list   list all licenses (default true)

Use "gen-license-go [command] --help" for more information about a command."#,
    );
}
