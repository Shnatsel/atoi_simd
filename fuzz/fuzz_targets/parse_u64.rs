#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let result = atoi_simd::parse::<u64>(data);
    if let Ok(number) = result {
        if let Ok(string) = std::str::from_utf8(data) {
            let std_number = string.parse::<u64>().expect("atoi_simd parsing succeeded when std failed");
            assert_eq!(std_number, number);
        }
    }
});