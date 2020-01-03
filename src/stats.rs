use std::collections::HashMap;

// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.
pub fn list_stats(list: &[i32]) {
    let mut counts = HashMap::new();
    let mut total = 0;
    for num in list.iter() {
        *counts.entry(num).or_insert(0) += 1;
        total += num;
    }

    let mean = total / (list.len() as i32);
    println!("Mean: {}", mean);

    let mut sorted = list.to_vec();
    sorted.sort_unstable();
    let median;
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        median = (sorted[middle] + sorted[middle + 1]) / 2;
    } else {
        median = sorted[middle];
    }
    println!("Median: {}", median);

    let mode = counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
    println!("Mode: {}", mode);
}
