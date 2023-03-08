fn main() {
    let mut arr = vec![5, 3, 6, 2, 10];
    let resp = selection_sort(&mut arr.to_vec());
    println!("sort arr: {:?}", resp);
}  

fn find_smallest(arr: &[i32]) -> i32 {
    let mut smallest = arr[0];
    let mut small_index = 0i32;
    for (i, v) in arr.iter().enumerate() {
        if *v < smallest {
            smallest = *v;
            small_index = i as i32;
        }
    } 
    println!("arr: {:?}, small & index: {},{}", arr, smallest, small_index);
    small_index
}

fn selection_sort(arr: &mut Vec<i32>) -> Vec<i32>{
    let mut new_arr = Vec::new();
    for v in 0..arr.len() {
        let smallest = find_smallest(arr);
        let vv = arr.remove(smallest as usize);
        new_arr.push(vv);
    }
    
    new_arr
}