mod sort;

fn main() {
    let mut array = [9, 4, 8, 3, -5, 2, 1, 6];
    println!("The initial array is {:?}", array);

    sort::with_selection(&mut array);
    sort::with_merge(&mut array);
    sort::with_quicksort(&mut array);
    println!(" The sorted array is {:?}", array);
}
