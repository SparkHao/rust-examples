const THRESHOLD: usize = 2;
fn main() {
    let arr = vec![1, 25, -4, 10, 16, 11];
    let max = find_max_num(&arr);
    println!("max num: {:?}", max);
}

fn find_max_num(arr: &[i32]) -> Option<i32> {
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    crossbeam::scope(|s| {
        let left_handle = s.spawn(|_| {
            find_max_num(left)
        });
        let right_handle = s.spawn(|_| {
            find_max_num(right)
        });

        let left_max = left_handle.join().unwrap()?;
        let right_max = right_handle.join().unwrap()?;
        Some(left_max.max(right_max))
    }).unwrap()
}