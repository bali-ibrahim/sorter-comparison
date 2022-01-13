use std::time::Duration;

use clap::{ArgEnum, ArgGroup, Parser};
use sort::NumberOf;
mod file;
mod sample;
mod sort;

const SAMPLE_FILE_NAME: &str = "sample.csv";

/// Compare sort algorithms, sorted file will be prefixed with sorted in the same directory of the read file.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("file_mod")
        .args(&["generate"])
        .requires("path")
        .requires("size"),
    ))]
#[clap(group(
    ArgGroup::new("sort_mod")
        .args(&["no_sorting"])
        .conflicts_with_all(&["write_mod"]),
    ))]
#[clap(group(
    ArgGroup::new("write_mod")
        .args(&["output"])
        .conflicts_with_all(&["sort_mod"]),
    ))]
struct Args {
    /// .csv file path to be generated or read
    #[clap(short, long, default_value = SAMPLE_FILE_NAME)]
    path: String,

    /// Generate a sample file
    #[clap(short, long)]
    generate: bool,

    /// Perform no sorting
    #[clap(short, long)]
    no_sorting: bool,

    /// Sorted list output path
    #[clap(short, long,
        // default_value_t = file::add_prefix("sorted-", SAMPLE_FILE_NAME).into_os_string().into_string().unwrap()
    )]
    output: Option<String>,

    /// Size of the array to be written in the generated sample
    #[clap(short, long, default_value_t = 2_usize.pow(16))]
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

#[derive(Debug)]
struct Results {
    number_of: NumberOf,
    elapsed: Duration,
}

fn main() {
    let args = Args::parse();
    if args.generate {
        sample::write(&args.path, args.size).unwrap();
    }

    let mut vec = Vec::<isize>::new();
    if !args.no_sorting {
        vec = read_n_sort(&args).unwrap();
    }

    if args.output.is_some() {
        let o = args.output.unwrap();
        if !o.trim().is_empty() {
            sample::write_to(vec, &o).unwrap();
        }
    }
}

fn read_n_sort(args: &Args) -> Result<Vec<isize>, csv::Error> {
    let mut results = Results {
        number_of: NumberOf::default(),
        elapsed: Duration::default(),
    };
    let mut vec = sample::read(&args.path)?;
    let now = std::time::Instant::now();
    match args.sorter {
        Sorter::Selection => results.number_of = sort::with_selection(&mut vec),
        Sorter::Merge => results.number_of = sort::with_merge(&mut vec),
        Sorter::QuickSort => results.number_of = sort::with_quicksort(&mut vec),
    };
    results.elapsed = now.elapsed();
    println!("{:?}", results);
    Ok(vec)
}
