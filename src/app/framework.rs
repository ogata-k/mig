#[derive(Clone, Copy, Debug)]
pub enum Framework {
    Laravel,
}

pub fn to_framework_type(from: &str) -> Option<Framework> {
    if from.is_empty() {
        return None;
    }

    let fw_opt = match from.to_lowercase().as_str() {
        "laravel" => Some(Framework::Laravel),
        _ => None,
    };
    return fw_opt;
}
