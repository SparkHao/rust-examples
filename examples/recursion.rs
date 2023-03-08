fn main() {
    let arr = vec![2, 4, 6, 8];
    let result = recursion(0, &arr);
    println!("sum: {}", result);
    let result = recursion_max_num(0, &arr);
    println!("max num: {}", result);
    let result = recursion_dic(8, &arr);
    println!("find num: {}", result)
}

fn recursion(sum: usize, arr: &[usize]) -> usize {
    if arr.len() == 0 {
        return sum;
    }else {
        let sum = sum + arr[0];
        let new_arr = &arr[1..];
        return recursion(sum, new_arr);
    }
}

fn recursion_max_num(max: usize, arr: &[usize]) -> usize {
    if arr.len() == 0 {
        max
    }else {
        let max = if max < arr[0] {
            arr[0]
        }else {
            max
        };
        let new_arr = &arr[1..];
        recursion_max_num(max, new_arr)
    }
}

fn recursion_dic(target: usize, arr: &[usize]) -> bool {
    let left = 0;
    let right = arr.len() - 1;
    let mid = (left + right + 1) / 2;
    let value = arr[mid];
    println!("arr: {:?}, target: {}, mid: {}, value: {}", &arr, target, mid, value);
    if arr.len() == 1 {
        if target == value {
            true
        }else {
            false
        }
    }else {
        if target == value {
            true
        }else if target > value {
            recursion_dic(target, &arr[mid..])
        }else {
            recursion_dic(target, &arr[0..mid])
        }
    }
}