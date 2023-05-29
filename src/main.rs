use pipeline::input::csv::CsvReader;
use pipeline::input::Reader;


fn main() {
    let input = CsvReader::with_headers("datasets/test.csv");
    if let Ok(input) = input {
        for col in input.columns() {
            println!("{}", col);   
        }
    }
}
