use pipeline::input::csv::CsvReader;
use pipeline::input::Reader;


fn main() {
    let input = CsvReader::with_headers("datasets/car.data");
    if let Ok(input) = input {
        println!("{:?}", input.get_column_idx(0));
    }
}
