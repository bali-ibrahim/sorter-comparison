mod sample;
mod sort;

fn main() -> Result<(), csv::Error> {
    sample::write()?;
    let mut array = sample::read()?;

    let now = std::time::Instant::now();
    // sort::with_selection(&mut array);
    // sort::with_merge(&mut array);
    sort::with_quicksort(&mut array);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    sample::write_to(array, "sorted.csv")
}
