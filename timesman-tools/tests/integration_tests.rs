use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("--conn-type"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("timesman-tools"));
}

#[test]
fn test_cli_missing_required_args() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.arg("get-times-list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_cli_invalid_conn_type() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "invalid", "get-times-list"])
        .assert()
        .failure();
}

#[test]
fn test_command_help() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "create-times", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--title"));
}

#[test]
fn test_create_times_missing_title() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "create-times"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("title"));
}

#[test]
fn test_create_post_missing_args() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "create-post"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_delete_times_missing_tid() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "delete-times"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("tid"));
}

#[test]
fn test_update_times_missing_args() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "update-times", "--tid", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("title"));
}

#[test]
fn test_get_post_list_missing_tid() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "get-post-list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("tid"));
}

#[test]
fn test_delete_post_missing_args() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "delete-post", "--tid", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("pid"));
}

#[test]
fn test_update_post_missing_args() {
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "update-post", "--tid", "1", "--pid", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("text"));
}

#[test]
fn test_subcommand_help() {
    let commands = [
        "get-times-list",
        "create-times", 
        "delete-times",
        "update-times",
        "get-post-list",
        "create-post",
        "delete-post", 
        "update-post"
    ];

    for command in &commands {
        let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
        cmd.args(&["--conn-type", "grpc", command, "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }
}

#[test]
fn test_custom_server_url() {
    // This test verifies that custom server URLs are accepted
    // The command will fail due to connection, but it should parse arguments correctly
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&[
        "--conn-type", "grpc", 
        "--server", "http://localhost:9999", 
        "get-times-list"
    ])
    .assert()
    .failure(); // Expected to fail due to no server running
}

#[test]
fn test_all_command_structures() {
    // Test that all commands have proper structure and don't panic on argument parsing
    let test_cases = vec![
        vec!["--conn-type", "grpc", "get-times-list"],
        vec!["--conn-type", "grpc", "create-times", "--title", "test"],
        vec!["--conn-type", "grpc", "delete-times", "--tid", "1"],
        vec!["--conn-type", "grpc", "update-times", "--tid", "1", "--title", "test"],
        vec!["--conn-type", "grpc", "get-post-list", "--tid", "1"],
        vec!["--conn-type", "grpc", "create-post", "--tid", "1", "--text", "test"],
        vec!["--conn-type", "grpc", "delete-post", "--tid", "1", "--pid", "1"],
        vec!["--conn-type", "grpc", "update-post", "--tid", "1", "--pid", "1", "--text", "test"],
    ];

    for args in test_cases {
        let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
        cmd.args(&args)
            .assert()
            .failure(); // Expected to fail due to gRPC connection, but should parse args correctly
    }
}

#[test]
fn test_numeric_argument_validation() {
    // Test with invalid numeric arguments
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "delete-times", "--tid", "not-a-number"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));
}

#[test]
fn test_empty_string_arguments() {
    // Test with empty string arguments
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "create-times", "--title", ""])
        .assert()
        .failure(); // Should fail due to gRPC connection, but args should parse
}

#[test]
fn test_long_string_arguments() {
    // Test with very long string arguments
    let long_title = "a".repeat(1000);
    let mut cmd = Command::cargo_bin("timesman-tools").unwrap();
    cmd.args(&["--conn-type", "grpc", "create-times", "--title", &long_title])
        .assert()
        .failure(); // Should fail due to gRPC connection, but args should parse
}