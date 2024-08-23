/// Create a new `Vec<String>` that has been middle-padded.
pub fn midpad(input: Vec<String>) -> Vec<String> {
    let maxlen = input.iter().map(|s| s.len()).max().unwrap_or(0);

    input
        .into_iter()
        .map(|s| _midpad(s.trim(), maxlen))
        .collect()
}

/// Modify a mutable slice of `Strings` in-place to make it middle-padded.
pub fn midpad_in_place(input: &mut [String]) {
    let maxlen = input.iter().map(|s| s.len()).max().unwrap_or(0);

    input
        .iter_mut()
        .for_each(|s| *s = _midpad(s.trim(), maxlen));
}

/// Create a new `Vec<String>` that has been left-padded.
pub fn leftpad(input: Vec<String>) -> Vec<String> {
    input.into_iter().map(|s| s.trim().to_string()).collect()
}

/// Modify a mutable slice of `Strings` in-place to make it left-padded.
pub fn leftpad_in_place(input: &mut [String]) {
    input.iter_mut().for_each(|s| *s = s.trim().to_string());
}

/// Create a new `Vec<String>` that has been right-padded.
pub fn rightpad(input: Vec<String>) -> Vec<String> {
    let maxlen = input.iter().map(|s| s.len()).max().unwrap_or(0);

    input
        .into_iter()
        .map(|s| _rightpad(s.trim(), maxlen))
        .collect()
}

/// Modify a mutable slice of `Strings` in-place to make it right-padded.
pub fn rightpad_in_place(input: &mut [String]) {
    let maxlen = input.iter().map(|s| s.len()).max().unwrap_or(0);

    input
        .iter_mut()
        .for_each(|s| *s = _rightpad(s.trim(), maxlen));
}

/// Generate a string with only spaces.
fn space(n: usize) -> String {
    " ".repeat(n)
}

#[doc(hidden)]
fn _midpad(src: &str, len: usize) -> String {
    let srclen = src.len();

    let half = (len - srclen) / 2;

    space(half) + src
}

#[doc(hidden)]
fn _rightpad(src: &str, len: usize) -> String {
    let srclen = src.len();

    space(len - srclen) + src
}
