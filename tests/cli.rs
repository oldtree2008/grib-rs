use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;

mod utils;

const CMD_NAME: &'static str = "gribber";

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(
            predicate::str::contains("USAGE:")
                .and(predicate::str::contains("FLAGS:"))
                .and(predicate::str::contains("SUBCOMMANDS:")),
        )
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn no_subcommand_specified() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("--help");
    let help_msg = cmd.output()?.stdout;
    let help_msg = format!("{}", String::from_utf8(help_msg)?);

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::similar(help_msg));

    Ok(())
}

#[test]
fn no_such_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("foo");
    cmd.assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(
            predicate::str::starts_with(
                "error: Found argument 'foo' which wasn't expected, or isn't valid in this context",
            )
            .and(predicate::str::contains("USAGE:"))
            .and(predicate::str::contains("SUBCOMMANDS:").not()),
        );

    Ok(())
}

#[test]
fn info() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let out_str = "\
Originating/generating centre:          34
Originating/generating sub-centre:      0
GRIB Master Tables Version Number:      code '5' is not implemented
GRIB Local Tables Version Number:       code '1' is not implemented
Significance of Reference Time:         Analysis
Reference time of data:                 2016-08-22 02:00:00 UTC
Production status of processed data:    Operational products
Type of processed data:                 Analysis and forecast products
";

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("info").arg(arg_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(out_str))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn list() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let out_str = "\
[
    SubMessage {
        section2: None,
        section3: Some(
            2,
        ),
        section4: Some(
            3,
        ),
        section5: Some(
            4,
        ),
        section6: Some(
            5,
        ),
        section7: Some(
            6,
        ),
    },
    SubMessage {
        section2: None,
        section3: Some(
            2,
        ),
        section4: Some(
            7,
        ),
        section5: Some(
            8,
        ),
        section6: Some(
            9,
        ),
        section7: Some(
            10,
        ),
    },
    SubMessage {
        section2: None,
        section3: Some(
            2,
        ),
        section4: Some(
            11,
        ),
        section5: Some(
            12,
        ),
        section6: Some(
            13,
        ),
        section7: Some(
            14,
        ),
    },
    SubMessage {
        section2: None,
        section3: Some(
            2,
        ),
        section4: Some(
            15,
        ),
        section5: Some(
            16,
        ),
        section6: Some(
            17,
        ),
        section7: Some(
            18,
        ),
    },
    SubMessage {
        section2: None,
        section3: Some(
            2,
        ),
        section4: Some(
            19,
        ),
        section5: Some(
            20,
        ),
        section6: Some(
            21,
        ),
        section7: Some(
            22,
        ),
    },
    SubMessage {
        section2: None,
        section3: Some(
            2,
        ),
        section4: Some(
            23,
        ),
        section5: Some(
            24,
        ),
        section6: Some(
            25,
        ),
        section7: Some(
            26,
        ),
    },
    SubMessage {
        section2: None,
        section3: Some(
            2,
        ),
        section4: Some(
            27,
        ),
        section5: Some(
            28,
        ),
        section6: Some(
            29,
        ),
        section7: Some(
            30,
        ),
    },
]
";

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("list").arg(arg_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(out_str))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn inspect() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let out_str = "\
Sections:
0000000000000000 - 0000000000000010 : Section 0
0000000000000010 - 0000000000000025 : Section 1
0000000000000025 - 000000000000006d : Section 3
000000000000006d - 000000000000008f : Section 4
000000000000008f - 00000000000000a6 : Section 5
00000000000000a6 - 00000000000000ac : Section 6
00000000000000ac - 000000000000061b : Section 7
000000000000061b - 000000000000063d : Section 4
000000000000063d - 0000000000000654 : Section 5
0000000000000654 - 000000000000065a : Section 6
000000000000065a - 0000000000000bd1 : Section 7
0000000000000bd1 - 0000000000000bf3 : Section 4
0000000000000bf3 - 0000000000000c0a : Section 5
0000000000000c0a - 0000000000000c10 : Section 6
0000000000000c10 - 000000000000118c : Section 7
000000000000118c - 00000000000011ae : Section 4
00000000000011ae - 00000000000011c5 : Section 5
00000000000011c5 - 00000000000011cb : Section 6
00000000000011cb - 000000000000173e : Section 7
000000000000173e - 0000000000001760 : Section 4
0000000000001760 - 0000000000001777 : Section 5
0000000000001777 - 000000000000177d : Section 6
000000000000177d - 0000000000001cf0 : Section 7
0000000000001cf0 - 0000000000001d12 : Section 4
0000000000001d12 - 0000000000001d29 : Section 5
0000000000001d29 - 0000000000001d2f : Section 6
0000000000001d2f - 00000000000022a4 : Section 7
00000000000022a4 - 00000000000022c6 : Section 4
00000000000022c6 - 00000000000022dd : Section 5
00000000000022dd - 00000000000022e3 : Section 6
00000000000022e3 - 000000000000284d : Section 7
000000000000284d - 0000000000002851 : Section 8

Templates:
3.0
4.0
5.200
";

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("inspect").arg(arg_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(out_str))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn inspect_with_opt_s() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let out_str = "\
0000000000000000 - 0000000000000010 : Section 0
0000000000000010 - 0000000000000025 : Section 1
0000000000000025 - 000000000000006d : Section 3
000000000000006d - 000000000000008f : Section 4
000000000000008f - 00000000000000a6 : Section 5
00000000000000a6 - 00000000000000ac : Section 6
00000000000000ac - 000000000000061b : Section 7
000000000000061b - 000000000000063d : Section 4
000000000000063d - 0000000000000654 : Section 5
0000000000000654 - 000000000000065a : Section 6
000000000000065a - 0000000000000bd1 : Section 7
0000000000000bd1 - 0000000000000bf3 : Section 4
0000000000000bf3 - 0000000000000c0a : Section 5
0000000000000c0a - 0000000000000c10 : Section 6
0000000000000c10 - 000000000000118c : Section 7
000000000000118c - 00000000000011ae : Section 4
00000000000011ae - 00000000000011c5 : Section 5
00000000000011c5 - 00000000000011cb : Section 6
00000000000011cb - 000000000000173e : Section 7
000000000000173e - 0000000000001760 : Section 4
0000000000001760 - 0000000000001777 : Section 5
0000000000001777 - 000000000000177d : Section 6
000000000000177d - 0000000000001cf0 : Section 7
0000000000001cf0 - 0000000000001d12 : Section 4
0000000000001d12 - 0000000000001d29 : Section 5
0000000000001d29 - 0000000000001d2f : Section 6
0000000000001d2f - 00000000000022a4 : Section 7
00000000000022a4 - 00000000000022c6 : Section 4
00000000000022c6 - 00000000000022dd : Section 5
00000000000022dd - 00000000000022e3 : Section 6
00000000000022e3 - 000000000000284d : Section 7
000000000000284d - 0000000000002851 : Section 8
";

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("inspect").arg("-s").arg(arg_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(out_str))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn inspect_with_opt_t() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let out_str = "\
3.0
4.0
5.200
";

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("inspect").arg("-t").arg(arg_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(out_str))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn inspect_with_all_opts() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("inspect").arg(arg_path);
    let msg_no_opt = cmd.output()?.stdout;
    let msg_no_opt = format!("{}", String::from_utf8(msg_no_opt)?);

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("inspect").arg("-s").arg("-t").arg(arg_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::similar(msg_no_opt))
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn decode_tornado() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("decode").arg(arg_path).arg("3");
    cmd.assert().success().stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn decode_kousa() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_kousa_file()?;
    let arg_path = tempfile.path();

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("decode").arg(arg_path).arg("3");
    cmd.assert().success().stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn decode_tornado_big_endian() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let dir = TempDir::new()?;
    let out_path = dir.path().join("out.bin");
    let out_path = format!("{}", out_path.display());

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("decode")
        .arg(arg_path)
        .arg("3")
        .arg("-b")
        .arg(&out_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let expected = utils::tornado_nowcast_be_bin_bytes()?;
    let expected: Vec<_> = expected
        .chunks(4)
        .into_iter()
        .map(|b| match b {
            [0x62, 0x58, 0xd1, 0x9a] => vec![0x7f, 0xc0, 0x00, 0x00],
            b => b.to_vec(),
        })
        .flatten()
        .collect();
    let actual = utils::cat_as_bytes(&out_path)?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn decode_tornado_little_endian() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_tornado_nowcast_file()?;
    let arg_path = tempfile.path();

    let dir = TempDir::new()?;
    let out_path = dir.path().join("out.bin");
    let out_path = format!("{}", out_path.display());

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("decode")
        .arg(arg_path)
        .arg("3")
        .arg("-l")
        .arg(&out_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let expected = utils::tornado_nowcast_le_bin_bytes()?;
    let expected: Vec<_> = expected
        .chunks(4)
        .into_iter()
        .map(|b| match b {
            [0x9a, 0xd1, 0x58, 0x62] => vec![0x00, 0x00, 0xc0, 0x7f],
            b => b.to_vec(),
        })
        .flatten()
        .collect();
    let actual = utils::cat_as_bytes(&out_path)?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn decode_kousa_big_endian() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_kousa_file()?;
    let arg_path = tempfile.path();

    let dir = TempDir::new()?;
    let out_path = dir.path().join("out.bin");
    let out_path = format!("{}", out_path.display());

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("decode")
        .arg(arg_path)
        .arg("3")
        .arg("-b")
        .arg(&out_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let expected = utils::kousa_be_bin_bytes()?;
    let actual = utils::cat_as_bytes(&out_path)?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn decode_kousa_little_endian() -> Result<(), Box<dyn std::error::Error>> {
    let tempfile = utils::jma_kousa_file()?;
    let arg_path = tempfile.path();

    let dir = TempDir::new()?;
    let out_path = dir.path().join("out.bin");
    let out_path = format!("{}", out_path.display());

    let mut cmd = Command::cargo_bin(CMD_NAME)?;
    cmd.arg("decode")
        .arg(arg_path)
        .arg("3")
        .arg("-l")
        .arg(&out_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    let expected = utils::kousa_le_bin_bytes()?;
    let actual = utils::cat_as_bytes(&out_path)?;
    assert_eq!(actual, expected);

    Ok(())
}

macro_rules! test_subcommands_without_args {
    ($(($name:ident, $str:expr),)*) => ($(
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = Command::cargo_bin(CMD_NAME)?;
            cmd.arg($str);
            cmd.assert()
                .failure()
                .stdout(predicate::str::is_empty())
                .stderr(
                    predicate::str::starts_with(
                        "error: The following required arguments were not provided:",
                    )
                        .and(predicate::str::contains("USAGE:"))
                        .and(predicate::str::contains("SUBCOMMANDS:").not()),
                );

            Ok(())
        }
    )*);
}

test_subcommands_without_args! {
    (info_without_args, "info"),
    (list_without_args, "list"),
    (inspect_without_args, "inspect"),
}

macro_rules! test_subcommands_with_nonexisting_file {
    ($(($name:ident, $str:expr),)*) => ($(
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let dir = TempDir::new()?;
            let file_path = dir.path().join("nosuchfile");
            let file_path = format!("{}", file_path.display());

            let mut cmd = Command::cargo_bin(CMD_NAME)?;
            cmd.arg($str).arg(file_path);
            cmd.assert()
                .failure()
                .stdout(predicate::str::is_empty());

            Ok(())
        }
    )*);
}

test_subcommands_with_nonexisting_file! {
    (info_with_nonexisting_file, "info"),
    (list_with_nonexisting_file, "list"),
    (inspect_with_nonexisting_file, "inspect"),
}

macro_rules! test_subcommands_with_non_grib {
    ($(($name:ident, $str:expr),)*) => ($(
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let tempfile = utils::non_grib_file()?;
            let arg_path = tempfile.path();

            let mut cmd = Command::cargo_bin(CMD_NAME)?;
            cmd.arg($str).arg(arg_path);
            cmd.assert()
                .failure()
                .stdout(predicate::str::is_empty())
                .stderr(predicate::str::similar("Not GRIB data\n"));

            Ok(())
        }
    )*);
}

test_subcommands_with_non_grib! {
    (info_with_non_grib, "info"),
    (list_with_non_grib, "list"),
    (inspect_with_non_grib, "inspect"),
}

macro_rules! test_subcommands_with_too_small_file {
    ($(($name:ident, $str:expr),)*) => ($(
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let tempfile = utils::too_small_file()?;
            let arg_path = tempfile.path();

            let mut cmd = Command::cargo_bin(CMD_NAME)?;
            cmd.arg($str).arg(arg_path);
            cmd.assert()
                .failure()
                .stdout(predicate::str::is_empty())
                .stderr(predicate::str::similar(
                    "Error in checking file type: failed to fill whole buffer\n",
                ));

            Ok(())
        }
    )*);
}

test_subcommands_with_too_small_file! {
    (info_with_too_small_file, "info"),
    (list_with_too_small_file, "list"),
    (inspect_with_too_small_file, "inspect"),
}
