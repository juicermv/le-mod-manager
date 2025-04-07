

pub fn to_ascii_array<const N: usize>(s: &str) -> [ascii::AsciiChar; N] {
    let mut arr = [ascii::AsciiChar::default(); N];
    for (i, c) in s.chars().take(N).enumerate() {
        arr[i] = ascii::AsciiChar::from_ascii(c as u8).unwrap();
    }
    arr
}

pub fn from_ascii_array<const N: usize>(arr: &[ascii::AsciiChar; N]) -> String {
    arr.iter()
        .map(|&c| c.as_char())
        .collect::<String>()
        .trim_end_matches('\0')
        .to_string()
}