fn main() {
    let arr = vec![10, 5, 2, 3, 1];
    let result = quick_sort(&arr);
    println!("quick sort: {:?}", result);
}

fn quick_sort(arr: &[usize]) -> Vec<usize>{
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let pivot = arr[0];
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for v in arr.iter().skip(1) {
        if *v > pivot {
            v2.push(*v);
        }else {
            v1.push(*v)
        }
    }
    return [quick_sort(&v1), quick_sort(&v2)].join(&pivot).into()
}