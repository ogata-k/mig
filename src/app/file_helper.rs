extern crate chrono;

use std::path::PathBuf;

use chrono::Local;

use crate::app::framework::Framework;

pub fn with_timestamp(target_file: &str, framework_type: &Framework) -> String {
    let ext = get_extension_for_framework(framework_type);
    return format!("{}{}.{}", generate_timestamp(framework_type), target_file, ext);
}

fn generate_timestamp(framework_type: &Framework) -> String {
    let fmt = match framework_type {
        Framework::Laravel => "%Y_%m_%d_%H%M%S_",
    };
    return Local::now().format(fmt).to_string();
}

pub fn get_extension_for_framework(framework_type: &Framework) -> String {
    let ext = match framework_type {
        Framework::Laravel => "php",
    };
    return ext.to_string();
}

pub fn is_extension(file_path: &PathBuf, extension: &str) -> bool {
    let file_extension = file_path.extension();
    if file_extension.is_none() {
        return false;
    }
    if file_extension.unwrap() == extension {
        return true;
    }
    return false;
}