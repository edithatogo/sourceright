use sourceright::{normalize_doi, normalize_identifier, normalize_item_type, normalize_title};

fn xorshift64(state: &mut u64) -> u64 {
    let mut value = *state;
    value ^= value << 13;
    value ^= value >> 7;
    value ^= value << 17;
    *state = value;
    value
}

fn sample_string(seed: u64, max_len: usize) -> String {
    let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \t\n-_./:()";
    let mut state = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let len = (xorshift64(&mut state) as usize % max_len) + 1;
    let mut output = String::with_capacity(len);

    for _ in 0..len {
        let idx = (xorshift64(&mut state) as usize) % alphabet.len();
        output.push(alphabet[idx] as char);
    }

    output
}

fn sample_doi(seed: u64) -> String {
    match seed % 5 {
        0 => format!("doi:{}", sample_string(seed, 48)),
        1 => format!("https://doi.org/{}", sample_string(seed, 48)),
        2 => format!("http://doi.org/{}", sample_string(seed, 48)),
        3 => format!("/{}", sample_string(seed, 48)),
        _ => sample_string(seed, 48),
    }
}

#[test]
fn identifier_normalization_is_idempotent_over_generated_inputs() {
    for seed in 0..256 {
        let input = sample_string(seed, 96);
        let once = normalize_identifier(&input);
        let twice = normalize_identifier(&once);
        assert_eq!(once, twice);
        assert_eq!(once, once.trim());
    }
}

#[test]
fn title_normalization_is_idempotent_over_generated_inputs() {
    for seed in 256..512 {
        let input = sample_string(seed, 96);
        let once = normalize_title(&input);
        let twice = normalize_title(&once);
        assert_eq!(once, twice);
        assert_eq!(once, once.trim());
    }
}

#[test]
fn item_type_normalization_is_lowercase_and_idempotent() {
    for seed in 512..768 {
        let input = sample_string(seed, 64);
        let once = normalize_item_type(&input);
        let twice = normalize_item_type(&once);
        assert_eq!(once, twice);
        assert_eq!(once, once.to_ascii_lowercase());
    }
}

#[test]
fn doi_normalization_is_idempotent_over_generated_inputs() {
    for seed in 768..1024 {
        let input = sample_doi(seed);
        let once = normalize_doi(&input);
        let twice = normalize_doi(&once);
        assert_eq!(once, twice);
        assert_eq!(once, once.to_ascii_lowercase());
    }
}
