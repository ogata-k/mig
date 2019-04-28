#[derive(Clone, Copy, Debug)]
pub enum Framework {
    Laravel,
    Rails,
}

pub fn to_framework_type(from: &str) -> Option<Framework> {
    // TODO convert from string to Framework type is the type existing
    return Some(Framework::Laravel);
}
