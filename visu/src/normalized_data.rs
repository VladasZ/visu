use std::{array::from_fn, ops::Range};

use count_digits::CountDigits;

#[derive(Debug)]
pub struct NormalizedData<const SIZE: usize> {
    pub x_range:   Range<u128>,
    pub y_range:   Range<u128>,
    pub data:      [Vec<u128>; SIZE],
    pub y_divider: u128,
}

pub fn normalize_data<const SIZE: usize>(data: [&[u128]; SIZE]) -> NormalizedData<SIZE> {
    let Some(size) = data.first().map(|a| a.len()) else {
        panic!("Trying to visualise empty data set");
    };

    assert!(
        data.iter().all(|a| a.len() == size),
        "All data samples must be of the same size"
    );

    let all_max: Vec<u128> = data.iter().map(|a| *a.iter().max().unwrap()).collect();

    let max_digits = all_max.iter().map(CountDigits::count_digits).max().unwrap();

    let data: [Vec<u128>; SIZE] = from_fn(|i| {
        let set_max_digits = all_max[i].count_digits();

        let shift = max_digits - set_max_digits;

        data[i].iter().map(|val| val * 10_u128.pow(shift.try_into().unwrap())).collect()
    });

    let all_max: Vec<u128> = data.iter().map(|a| *a.iter().max().unwrap()).collect();

    let total_max = *all_max.iter().max().unwrap();

    let max_digits = total_max.count_digits() - 1;

    let range_max_pow = 10_u128.pow(max_digits.try_into().unwrap());

    let first_digit = total_max / range_max_pow;

    let max_digit = first_digit + 1;

    let range_max = max_digit * range_max_pow;

    NormalizedData {
        x_range: 0..size as u128 - 1,
        y_range: 0..range_max,
        data,
        y_divider: range_max_pow / 10,
    }
}

#[cfg(test)]
mod test {
    use crate::normalized_data::normalize_data;

    #[test]
    fn test_normalize() {
        let norm = normalize_data([&[3, 8], &[10, 30]]);
        assert_eq!(norm.x_range, 0..1);
        assert_eq!(norm.y_range, 0..90);
        assert_eq!(norm.data, [[30, 80], [10, 30]]);

        let norm = normalize_data([&[9_000], &[15_000_000_000_000_000]]);
        assert_eq!(norm.x_range, 0..0);
        assert_eq!(norm.y_range, 0..100_000_000_000_000_000);
    }
}
