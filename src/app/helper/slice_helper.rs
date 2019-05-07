/// return is (head_part, (separator, sequence))
pub fn split_with_head_and_separator<T: Clone + PartialEq + Copy, F: Fn(&T) -> bool>(slice: &[T], p: F) -> (Vec<T>, Vec<(T, Vec<T>)>)
{
    let sl = slice.clone();
    if sl.is_empty() { return (vec!(), vec!()); }
    let header: Vec<T> = sl.iter()
        .take_while(|&t| !p(t))
        .map(|&t| t)
        .collect();
    let tail: Vec<T> = sl[header.len() + 1..].to_vec().clone();

    let mut res_vec: Vec<(T, Vec<T>)> = vec!();

    let mut t: T = sl[header.len()].clone();
    let mut seq: Vec<T> = vec!();
    for item in tail {
        if !p(&item) {
            seq.push(item);
        } else {
            res_vec.push((t.clone(), seq.clone()));
            seq.clear();
            t = item;
        }
    }
    res_vec.push((t.clone(), seq.clone()));
    return (header, res_vec);
}