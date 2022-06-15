use std::cmp::Ordering;

fn clamp(value: usize, min: usize, max: usize) -> usize {
    let val2 = match min.cmp(&value) {
        Ordering::Less => value,
        Ordering::Greater => min,
        Ordering::Equal => min,
    };

    match max.cmp(&val2) {
        Ordering::Less => val2,
        Ordering::Greater => max,
        Ordering::Equal => max,
    }
}
