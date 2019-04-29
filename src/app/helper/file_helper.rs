extern crate chrono;

use std::path::PathBuf;

use chrono::Local;

use crate::app::framework::Framework;

// TODO 出力先を相対パスなどで指定するようにするのならtimestampを最下ファイルの頭だけにつけるように変更
pub fn with_timestamp(target_file: &str, framework_type: &Framework) -> String {
    let target = get_file_name_for_framework(target_file, framework_type);
    return format!("{}{}", generate_timestamp(framework_type), target);
}

fn generate_timestamp(framework_type: &Framework) -> String {
    let fmt = match framework_type {
        Framework::Laravel => "%Y_%m_%d_%H%M%S_",
    };
    return Local::now().format(fmt).to_string();
}

pub fn get_file_name_for_framework(target_file: &str, framework_type: &Framework) -> String {
    let ext = get_extension_for_framework(framework_type);
    return format!("{}.{}", target_file, ext);
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
