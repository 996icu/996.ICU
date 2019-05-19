use assert_cmd::prelude::*;
use dir_diff;
use std::process::Command;
use tempdir;
use tempdir::TempDir;

#[test]
fn test_go_help() {
    let path = "../gen_license/genlicense/__init_.py";
    let mut cmd = Command::new("python");
    cmd.arg("-h");
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

#[test]
fn test_py_create_license_mit() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = "../gen_license/genlicense/__init_.py";
    let mut cmd = Command::new("python");
    cmd.arg("gen").arg("mit").arg("--996icu").args("en-us");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/mit.txt").unwrap()
    );
}

#[test]
fn test_py_create_license_mit() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = "../gen_license/genlicense/__init_.py";
    let mut cmd = Command::new("python");
    cmd.arg("gen").arg("mit").arg("--996icu").args("en-us");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/mit.txt").unwrap()
    );
}
#[test]
fn test_py_create_license_epl() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = "../gen_license/genlicense/__init_.py";
    let mut cmd = Command::new("python");
    cmd.arg("gen").arg("epl").arg("--996icu").args("en-us");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license/licenses/epl-2.txt").unwrap()
    );
}
#[test]
fn test_py_create_license_gpl() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = "../gen_license/genlicense/__init_.py";
    let mut cmd = Command::new("python");
    cmd.arg("gen").arg("gpl").arg("--996icu").args("en-us");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license/licenses/gpl.txt").unwrap()
    );
}
