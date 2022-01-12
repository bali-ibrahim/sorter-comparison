use std::isize;

use rand;

const SAMPLE_PATH: &str = "./samples/sample.csv";

fn generate_sample(size: usize) -> Vec<isize> {
    let mut vec = Vec::<isize>::new();
    for _ in 0..size {
        vec.push(rand::random());
    }
    return vec;
}

pub fn write(size: usize) -> Result<(), csv::Error> {
    let vec = generate_sample(size);
    std::fs::create_dir_all("./samples")?;
    write_to(vec, SAMPLE_PATH)
}

pub fn write_to(vec: Vec<isize>, path: &str) -> Result<(), csv::Error> {
    let mut writer = csv::Writer::from_path(path)?;

    for i in 0..vec.len() {
        writer.write_record(&[vec[i].to_string()])?;
    }

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;
    Ok(())
}

pub fn read() -> Result<Vec<isize>, csv::Error> {
    let mut reader = csv::Reader::from_path(SAMPLE_PATH)?;
    let mut my_vec = Vec::<isize>::new();
    for result in reader.records() {
        let val = result?.deserialize::<isize>(None)?;
        my_vec.push(val);
    }
    Ok(my_vec)
}
