use rand;

const SIZE: usize = 2_usize.pow(12);
const SAMPLE_PATH: &str = "./samples/sample.csv";

fn generate_sample() -> [isize; SIZE] {
    let mut my_array: [isize; SIZE] = [0; SIZE];
    for i in 0..my_array.len() {
        my_array[i] = rand::random();
    }
    return my_array;
}

pub fn write() -> Result<(), csv::Error> {
    let my_array = generate_sample();
    std::fs::create_dir_all("./samples")?;
    let mut writer = csv::Writer::from_path(SAMPLE_PATH)?;

    for i in 0..my_array.len() {
        writer.write_record(&[my_array[i].to_string()])?;
    }

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;
    Ok(())
}

pub fn read() -> Result<[isize; SIZE], csv::Error> {
    let mut reader = csv::Reader::from_path(SAMPLE_PATH)?;
    let mut my_array: [isize; SIZE] = [0; SIZE];

    let mut i: usize = 0;
    for result in reader.records() {
        my_array[i] = result?.deserialize::<isize>(None)?;
        i = i + 1;
    }
    Ok(my_array)
}
