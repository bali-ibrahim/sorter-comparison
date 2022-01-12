mod sample;
mod sort;

fn main() -> Result<(), csv::Error> {
    sample::write(2_usize.pow(1))?;
    let mut vec = sample::read()?;

    let now = std::time::Instant::now();
    sort::with_selection(&mut vec);
    // sort::with_merge(&mut vec);
    // sort::with_quicksort(&mut vec);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    sample::write_to(vec, "sorted.csv")
}
