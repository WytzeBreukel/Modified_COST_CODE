pub fn median(numbers: &mut [std::time::Duration]) -> std::time::Duration {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}