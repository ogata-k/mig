use std::path::PathBuf;

use crate::app::framework::Framework;

mod token;

pub fn convert_to_migration_file(
    input: &PathBuf,
    output: &PathBuf,
    framework: Framework,
) -> Result<&'static str, &'static str> {
    // TODO convert from input file "input" to output file "output" with target framework "framework"
    // this function is controller for convert
    // return Ok("Success converted");
    return Err("failed convert");
}
