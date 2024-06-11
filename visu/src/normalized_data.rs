use std::{array::from_fn, ops::Range};

use count_digits::CountDigits;

#[derive(Debug)]
struct NormalizedData<const SIZE: usize> {
    x_range: Range<u128>,
    y_range: Range<u128>,
    data:    [Vec<u128>; SIZE],
}

fn normalize_data<const SIZE: usize>(data: [&[u128]; SIZE]) -> NormalizedData<SIZE> {
    let Some(size) = data.first().map(|a| a.len()) else {
        panic!("Trying to visualise empty data set");
    };

    assert!(
        data.iter().all(|a| a.len() == size),
        "All data samples must be of the same size"
    );

    let all_max: Vec<u128> = data.iter().map(|a| *a.iter().max().unwrap()).collect();

    let max_digits = all_max.iter().map(|a| a.count_digits()).max().unwrap();

    let data: [Vec<u128>; SIZE] = from_fn(|i| {
        let set_max_digits = all_max[i].count_digits();

        let shift = max_digits - set_max_digits;

        data[i].iter().map(|val| val * 10_u128.pow(shift.try_into().unwrap())).collect()
    });

    let all_max: Vec<u128> = data.iter().map(|a| *a.iter().max().unwrap()).collect();

    let total_max = *all_max.iter().max().unwrap();

    NormalizedData {
        x_range: 0..size as u128,
        y_range: 0..total_max,
        data,
    }
}

#[cfg(test)]
mod test {
    use crate::normalized_data::normalize_data;

    #[test]
    fn test_normalize() {
        let norm = normalize_data([&[3, 8], &[10, 30]]);
        assert_eq!(norm.x_range, 0..2);
        assert_eq!(norm.y_range, 0..80);
        assert_eq!(norm.data, [[30, 80], [10, 30]]);

        let norm = normalize_data([&[9_000], &[15_000_000_000_000_000]]);
        assert_eq!(norm.x_range, 0..1);
        assert_eq!(norm.y_range, 0..90_000_000_000_000_000);
    }
}
