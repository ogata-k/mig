extern crate chrono;

use std::path::PathBuf;

use chrono::Local;

use crate::app::framework::Framework;

pub fn with_timestamp<'a>(target_file: &'a str, framework_type: &Framework) -> &'a str {
    // TODO set timestamp header to target file
    return target_file;
}

fn generate_timestamp(framework_type: &Framework) -> String {
    let fmt = match framework_type {
        Framework::Laravel => "%Y_%m_%d %H%M%S",
    };
    return Local::now().format(fmt).to_string();
}
