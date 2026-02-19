use std::path::Path;

#[test]
fn test_wallpaper_generation() {
    let output = "test_output.png";

    let _ = std::fs::remove_file(output);

    let status = std::process::Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--bin",
            "ascii-cli",
            "--",
            "Test",
            "-o",
            output,
        ])
        .status()
        .expect("Failed to execute command");

    assert!(status.success(), "Command failed");
    assert!(Path::new(output).exists(), "Output file not created");

    let _ = std::fs::remove_file(output);
}

#[test]
fn test_list_fonts() {
    let output = std::process::Command::new("cargo")
        .args(&["run", "--release", "--bin", "ascii-cli", "--", "list-fonts"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("standard"));
    assert!(stdout.contains("banner"));
}

#[test]
fn test_list_colors() {
    let output = std::process::Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--bin",
            "ascii-cli",
            "--",
            "list-colors",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nord"));
    assert!(stdout.contains("dracula"));
}
