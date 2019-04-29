extern crate chrono;

use std::path::PathBuf;

use chrono::Local;

use crate::app::framework::Framework;

pub fn with_timestamp(target_file: &str, framework_type: &Framework) -> String {
    let ext = get_extension(framework_type);
    return format!("{}{}.{}", generate_timestamp(framework_type), target_file, ext);
}

fn generate_timestamp(framework_type: &Framework) -> String {
    let fmt = match framework_type {
        Framework::Laravel => "%Y_%m_%d_%H%M%S_",
    };
    return Local::now().format(fmt).to_string();
}

fn get_extension(framework_type: &Framework) -> String {
    let ext = match framework_type {
        Framework::Laravel => "php",
    };
    return ext.to_string();
}