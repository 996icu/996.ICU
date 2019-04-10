use assert_cmd::prelude::*;
use dir_diff;
use std::process::Command;
use tempdir;
use tempdir::TempDir;

#[test]
fn test_go_help() {
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
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

#[test]
fn test_go_create_license_996icu() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen").arg("--996icu").arg("--996icu").args("en-us");
    assert!(!dir_diff::is_different(
        &tmp_dir.path(),
        "../../gen-license-go/licenses/996.icu.template.en-us.txt"
    )
    .unwrap());
}
#[test]
fn test_go_create_license_agpl() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen")
        .arg("--agpl-3.0")
        .arg("--996icu")
        .args("en-us");
    assert!(!dir_diff::is_different(
        &tmp_dir.path(),
        "../../gen-license-go/licenses/agpl-3.0.txt"
    )
    .unwrap());
}

#[test]
fn test_go_create_license_apache() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen")
        .arg("--apache-2.0")
        .arg("--996icu")
        .args("en-us");
    assert!(!dir_diff::is_different(
        &tmp_dir.path(),
        "../../gen-license-go/licenses/apache-2.0.txt"
    )
    .unwrap());
}

#[test]
fn test_go_create_license_bsd_2() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen")
        .arg("--bsd-2-clause")
        .arg("--996icu")
        .args("en-us");
    assert!(!dir_diff::is_different(
        &tmp_dir.path(),
        "../../gen-license-go/licenses/bsd-2-clause.txt"
    )
    .unwrap());
}

#[test]
fn test_go_create_license_bsd_3() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen")
        .arg("bsd-3-clause")
        .arg("--996icu")
        .args("en-us");
    assert!(!dir_diff::is_different(
        &tmp_dir.path(),
        "../../gen-license-go/licenses/bsd-3-clause.txt"
    )
    .unwrap());
}

#[test]
fn test_go_create_license_epl() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen").arg("epl-2.0").arg("--996icu").args("en-us");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/996.txt").unwrap()
    );
}

#[test]
fn test_go_create_license_996() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen").arg("--996");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/996.txt").unwrap()
    );
}

#[test]
fn test_go_create_license_996() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen").arg("--996");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/996.txt").unwrap()
    );
}

#[test]
fn test_go_create_license_996() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen").arg("--996");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/996.txt").unwrap()
    );
}

#[test]
fn test_go_create_license_996() {
    let tmp_dir = TempDir::new("foo").expect("create temp dir failed");
    let path = if cfg!(targe_os = "windows") {
        "../gen-license-go/bin/windows/gen-license-go.exe"
    } else if cfg!(target_os = "linux") {
        "../gen-license-go/bin/linux/gen-license-go"
    } else {
        "../gen-license-go/bin/osx/gen-license-go"
    };
    let mut cmd = Command::new(&path);
    cmd.arg("gen").arg("--996");
    assert!(
        !dir_diff::is_different(&tmp_dir.path(), "../../gen-license-go/licenses/996.txt").unwrap()
    );
}
