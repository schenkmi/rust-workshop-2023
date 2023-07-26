#[no_mangle]
pub unsafe extern "C" fn get_prime_numbers(
    mut buffer: *mut u32,
    start: u32,
    count: usize,
) -> usize {
    if buffer.is_null() || start < 2 {
        return 0;
    }

    let prime_number_iterator =
        (start..).filter(|number| (2..*number).all(|divisor| (number % divisor) != 0));

    let mut actual_count = 0;
    for number in prime_number_iterator.take(count) {
        *buffer = number;
        buffer = buffer.add(1);
        actual_count += 1;
    }

    return actual_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_prime_numbers_safe(start: u32, count: usize) -> Vec<u32> {
        let mut buffer = vec![0u32; count];
        unsafe {
            let actual_count = get_prime_numbers(buffer.as_mut_ptr(), start, count);
            buffer.resize(actual_count, 0);
        }
        buffer
    }

    #[test]
    fn test_get_prime_numbers() {
        assert_eq!(get_prime_numbers_safe(1, 5), vec![]);
        assert_eq!(get_prime_numbers_safe(2, 5), vec![2, 3, 5, 7, 11]);
        assert_eq!(get_prime_numbers_safe(2, 1), vec![2]);
        assert_eq!(get_prime_numbers_safe(2, 0), vec![]);
        assert_eq!(
            get_prime_numbers_safe(10, 8),
            vec![11, 13, 17, 19, 23, 29, 31, 37]
        );

        unsafe {
            assert_eq!(get_prime_numbers(std::ptr::null_mut(), 2, 5), 0);
        }
    }
}
