use std::fs::read_to_string;
fn main() {
    let file_path = "../inputs/aoc_06.txt";
    // let file_path = "test_input.txt";

    let raw_input: String = read_to_string(file_path)
        .expect("Should have been able to read the file")
        .replace("\n", "");
    println!("{}", raw_input);
    // let window_size: usize = 4;
    let window_size: usize = 14; // task2
    for (i, _) in raw_input.chars().enumerate() {
        if i >= window_size-1 {
            let slice: String = raw_input[i - (window_size-1)..=i].to_string().to_owned();
            println!("{:?}", slice);
            let dublicates: Vec<char> = slice.chars().filter(|c| slice.matches(&c.to_string()).count() > 1).collect();
            println!("{:?}", dublicates);
            if dublicates.is_empty()
            {
                println!("first marker after character {:?}", i+1);
                break;
            }
        }
    }
}
