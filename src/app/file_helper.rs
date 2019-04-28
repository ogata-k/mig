use std::path::PathBuf;

use crate::app::framework::Framework;

pub fn with_timestamp<'a>(target_file: &'a PathBuf, framework_type: &Framework) -> &'a PathBuf {
    // TODO set timestamp header to target file
    return target_file;
}
