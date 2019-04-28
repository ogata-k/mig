use std::path::PathBuf;

use crate::app::framework::Framework;

pub fn get_output_file_absolute_path(from: &str, framework_type: &Framework) -> PathBuf {
    // TODO get output file name by using framework's type
    return PathBuf::from(from);
}
