//! The generated code as the real runtime sees it.
//!
//! Substring assertions prove what was written, not that it is valid TetherScript: a
//! generated file with a syntax error or a wrong builtin passes every one of them and then
//! fails on the developer's first run. So this drives the real `tetherscript` binary over
//! the output.

use super::driver::{DRIVER, EXPECTED, FILES};
use super::runner::{binary, run};
use super::support::{schema, scratch};
use crate::generators::tetherscript::write;

#[test]
fn every_generated_file_passes_the_real_parser() {
    let Some(binary) = binary() else {
        eprintln!("skipping: tetherscript is not installed");
        return;
    };

    let out = scratch("check");
    write(&out, &schema(), false).expect("write should succeed");

    for name in FILES {
        let (passed, output) = run(&binary, "check", &out.join("product").join(name), &out);
        assert!(passed, "{name} failed to check:\n{output}");
    }

    let (passed, output) = run(&binary, "check", &out.join("mod.tether"), &out);
    assert!(passed, "parent mod.tether failed to check:\n{output}");

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn the_generated_validation_accepts_and_rejects_rows_at_runtime() {
    let Some(binary) = binary() else {
        eprintln!("skipping: tetherscript is not installed");
        return;
    };

    let out = scratch("runtime");
    write(&out, &schema(), false).expect("write should succeed");

    let driver = out.join("driver.tether");
    std::fs::write(&driver, DRIVER).unwrap();

    let (passed, output) = run(&binary, "run", &driver, &out);
    assert!(passed, "driver failed:\n{output}");

    for expected in EXPECTED {
        assert!(
            output.contains(expected),
            "missing {expected} in:\n{output}"
        );
    }

    let _ = std::fs::remove_dir_all(&out);
}
