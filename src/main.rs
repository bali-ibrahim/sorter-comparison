mod sample;
mod sort;

fn main() {
    match sample::write() {
        Ok(_) => {}
        Err(_) => {}
    }
    match sample::read() {
        Ok(mut array) => {
            // let mut array = [9, 4, 8, 3, -5, 2, 1, 6];
            println!("The initial array is {:?}", array);

            sort::with_selection(&mut array);
            sort::with_merge(&mut array);
            sort::with_quicksort(&mut array);
            println!("The sorted array is {:?}", array);
        }
        Err(_) => {}
    }
}