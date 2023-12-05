// Takes a string of numbers, and returns them as a vector of u32s
// e.g. " 42 17 24 13" -> [42, 17, 24, 13]
pub fn get_numbers_from_str(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .filter_map(|word| word.parse::<u64>().ok())
        .collect()
}
