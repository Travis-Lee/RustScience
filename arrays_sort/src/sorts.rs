// src/sorts.rs

pub fn sort_array<T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
    let mut result = arr.to_vec();
    let len = result.len();

    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if result[j] < result[min_index] {
                min_index = j;
            }
        }
        result.swap(i, min_index);
    }

    result
}

