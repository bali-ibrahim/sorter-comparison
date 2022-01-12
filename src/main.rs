use clap::{ArgEnum, Parser};
mod sample;
mod sort;

/// Compare sort algorithms
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: usize,

    /// Sort using the algorithm chosen
    #[clap(arg_enum, default_value_t)]
    // #[clap(short, long, default_value_t = Sorter::QuickSort)]
    sorter: Sorter,
}

#[derive(ArgEnum, Debug, Clone)]
enum Sorter {
    Selection,
    Merge,
    QuickSort,
}

impl Default for Sorter {
    fn default() -> Self {
        Self::QuickSort
    }
}

fn main() -> Result<(), csv::Error> {
    let args = Args::parse();
    sample::write(2_usize.pow(14))?;
    let mut vec = sample::read()?;

    let now = std::time::Instant::now();
    match args.sorter {
        Sorter::Selection => sort::with_selection(&mut vec),
        Sorter::Merge => sort::with_merge(&mut vec),
        Sorter::QuickSort => sort::with_quicksort(&mut vec),
    };
    let elapsed = now.elapsed();
    println!("Elapsed:\n{:.2?}", elapsed);
    sample::write_to(vec, "sorted.csv")
}
