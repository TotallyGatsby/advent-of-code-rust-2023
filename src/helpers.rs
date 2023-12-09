// Takes a string of numbers, and returns them as a vector of u32s
// e.g. " 42 17 24 13" -> [42, 17, 24, 13]
pub fn get_numbers_from_str<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr + std::cmp::PartialEq,
{
    input
        .trim()
        .split(" ")
        .filter_map(|word| word.parse::<T>().ok())
        .collect()
}

/*
// Greatest Common Denominator
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Least Common Multiple
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// LCM of a list of numbers
fn lcm_of_list(numbers: Vec<u32>) -> u64 {
    numbers.iter().fold(1, |acc, n| lcm(acc, *n as u64))
}
*/
