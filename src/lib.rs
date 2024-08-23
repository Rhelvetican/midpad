pub fn midpad(input: Vec<String>) -> Vec<String> {
    let maxlen = input.iter().map(|s| s.len()).max().unwrap_or(0);

    input.into_iter().map(|s| pad(s.trim(), maxlen)).collect()
}

pub fn midpad_in_place(input: &mut [String]) {
    let maxlen = input.iter().map(|s| s.len()).max().unwrap_or(0);

    input.iter_mut().for_each(|s| *s = pad(s.trim(), maxlen));
}

fn space(n: usize) -> String {
    " ".repeat(n)
}

fn pad(src: &str, len: usize) -> String {
    let srclen = src.len();

    let half = (len - srclen) / 2;

    space(half) + src + &space(half)
}
