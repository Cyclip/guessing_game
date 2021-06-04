use std::io;

fn incrArr(arr: &mut [i8]) {
    for n in 0..arr.len() {
        arr[n] += 1;
    }
}

fn main() {
    let mut arr: [i8; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", &arr);
    incrArr(&mut arr);
    println!("Array: {:?}", &arr);
    incrArr(&mut arr);
    println!("Array: {:?}", &arr);
}
