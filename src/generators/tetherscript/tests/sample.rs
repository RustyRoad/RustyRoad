//! Writes a sample generated model to /tmp for inspection. Run with:
//!   cargo test --lib generators::tetherscript::tests::sample -- --ignored --nocapture

use super::support::schema;
use crate::generators::tetherscript::write;

#[test]
#[ignore = "writes a durable sample for manual inspection"]
fn write_sample_to_tmp() {
    let out = std::path::PathBuf::from("/tmp/rustyroad-tether-sample");
    let _ = std::fs::remove_dir_all(&out);

    write(&out, &schema(), false).expect("write should succeed");

    println!("wrote sample to {}", out.display());
}
