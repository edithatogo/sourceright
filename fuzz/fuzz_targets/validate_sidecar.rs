#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 64 * 1024 {
        return;
    }
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = sourceright::parse_verification_sidecar_json(input);
    }
});
