#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let result = atoi_simd::parse::<i64>(data);
    if let Ok(string) = std::str::from_utf8(data) {
        let std_result = string.parse::<i64>();
        match (&result, &std_result) {
            (Ok(simd_num), Ok(std_num)) => assert_eq!(*simd_num, *std_num),
            (Err(_), Err(_)) => (), // both failed, nothing to do
            _ => panic!("Mismatch between parsing results! SIMD: {:?}, std: {:?}", &result, &std_result),
        }
    }
});