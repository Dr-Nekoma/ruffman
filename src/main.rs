mod heap;
mod io;

fn main() {
    let file_name = "test.txt";

    let mut file_content: Vec<u8> = Vec::new();

    let number_elements = io::read_file(file_name, &mut file_content).unwrap();

    println!("{:?}", file_content);

    let probabilities = heap::calculate_probability(&mut file_content, number_elements);

    println!("{:?}", probabilities);
}
