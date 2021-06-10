use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_compressing_and_decompressing() -> Result<(), Box<dyn std::error::Error>> {
    let content = b"Sasha Grey";

    let compress_result = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .write_stdin(content.to_owned())
        .assert()
        .success()
        .stdout(predicate::ne(content as &[u8]));

    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("--decompress")
        .write_stdin(compress_result.get_output().stdout.to_owned())
        .assert()
        .success()
        .stdout(predicate::eq(content as &[u8]));

    Ok(())
}

#[test]
fn test_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));

    Ok(())
}

#[test]
fn test_invalid_level_range() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("--level")
        .arg("10")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Level must be in the 0-9 range"));

    Ok(())
}

#[test]
fn test_invalid_level() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))?
        .arg("--level")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid number"));

    Ok(())
}
