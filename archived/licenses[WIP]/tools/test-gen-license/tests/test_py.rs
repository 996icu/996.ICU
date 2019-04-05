use assert_cmd::prelude::*;
use dir_diff;
use std::process::Command;
use tempdir;
use tempdir::TempDir;

/// todo: The python test is to be finished.. I'm having lunch now!
#[test]
fn test_go_help() {
    let path = "../gen_license/genlicense/__init_.py";
    let mut cmd = Command::new("python");
    cmd.arg("-h");
    // assert_eq!(output, help);
    let assert = cmd.assert();
    assert.success().stdout(
        /// todo: replace this
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

#[test]
fn test_go_create_license_mit() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen").arg("mit").arg("--996icu").args("en-us");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/mit.txt").unwrap()
    );
}
