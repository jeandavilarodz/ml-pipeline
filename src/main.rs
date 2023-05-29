use pipeline::data::column::Column;

fn main() {
    let mut column = Column::<f64>::new();
    column.set_name(String::from("Test"));
    column.append(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    println!("{}", column);
}
