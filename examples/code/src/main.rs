fn main() {
    println!("Hello, world!");
    let arr = vec![1, 3, 5, 7, 9, 11];
    let mut left = 0;
    let mut right = arr.len() - 1;
    let target = 10;
    let mut index = arr.len();
    while left <= right {
        let arr = arr.clone();
        println!("left: {}, right: {}, idnex: {}", left, right, index);
        let mid = (left + right) / 2;
        let guess = arr[mid];
        if guess == target {
            index = mid;
            break;
        }else if guess < target {
            left = mid + 1;
        }else if guess > target {
            right = mid - 1;
        }
    }
    if index == arr.len() {
        println!("not guess right value, {}", target)
    }
    println!("index: {}", index);
}
