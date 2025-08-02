//! Run with
//!
//! ```bash
//! cargo run -p rusmppyc --bin stub-gen
//! ```

use std::{env::current_dir, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stub = rusmppyc::stub_info()?;

    // Because we have a python directory, pyo3_stub_gen will generate the stub file in it so we move it to the correct place
    stub.python_root = current_dir()?.join("python/rusmppyc");

    stub.generate()?;

    Ok(())
}
