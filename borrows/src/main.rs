fn main() {

    let mut arr: Vec<u16> = Vec::new();
    
    fill_arr(&mut arr);
    let rrrr = add_one_to_array(&arr);

    println!("Hello, world!\n{:?}\n{:?}", arr, rrrr);
}
/*
 * A simple function that takes a mutable borrow and changes it :D
 */
fn fill_arr(array: &mut Vec<u16>) {
    for i in 0..9 {
        array.push(i);
    }
}

/*
 * Another simple function that creates a new array from a borrowed array.
 */
fn add_one_to_array(array: &Vec<u16>) -> Vec<u16> {
    let mut new_arr: Vec<u16> = Vec::new();
    for element in array {
        new_arr.push(element + 1);
    }
    new_arr
}