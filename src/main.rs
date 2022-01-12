use std::path::Path;

use clap::{ArgEnum, ArgGroup, Parser};
mod file;
mod sample;
mod sort;

/// Compare sort algorithms, sorted file will be prefixed with sorted in the same directory of the read file.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("file_mod")
        .args(&["generate"])
        .requires("size"),
    ))]
struct Args {
    /// Absolute .csv file path to be generated or read
    #[clap(short, long, default_value = "sample.csv")]
    path: String,

    /// Generate a sample file
    #[clap(short, long)]
    generate: bool,

    /// Size of the array to be written in the generated sample
    #[clap(short, long, default_value_t = 2_usize.pow(14))]
    size: usize,

    /// Sort using the algorithm chosen
    #[clap(arg_enum, default_value_t)]
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
    if args.generate {
        sample::write(&args.path, args.size)?;
    }
    let mut vec = sample::read(&args.path)?;

    let now = std::time::Instant::now();
    match args.sorter {
        Sorter::Selection => sort::with_selection(&mut vec),
        Sorter::Merge => sort::with_merge(&mut vec),
        Sorter::QuickSort => sort::with_quicksort(&mut vec),
    };
    let elapsed = now.elapsed();
    println!("Elapsed:\n{:.2?}", elapsed);

    let basename = Path::new(&args.path).file_stem().unwrap().to_str().unwrap();
    let sorted_file_name = format!("sorted-{}", &basename);
    let new_path = file::change_file_name(&args.path, &sorted_file_name);
    sample::write_to(vec, new_path.to_str().unwrap())
}
