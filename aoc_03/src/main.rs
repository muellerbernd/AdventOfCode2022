use std::fs::read_to_string;
fn main() {
    let file_path = "../inputs/aoc_03.txt";
    // let file_path = "test_input.txt";

    let contents: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let mut prios: Vec<u32> = Vec::new();
    for rucksack in contents.lines() {
        let compartment_size: usize = rucksack.len() / 2;
        let compartment_1 = &rucksack[..compartment_size];
        let compartment_2 = &rucksack[compartment_size..];
        let mut failed_item: char = ' ';
        for c in compartment_1.chars() {
            if compartment_2.contains(&c.to_string()) {
                failed_item = c;
                break;
            }
        }
        // ascii A = 65, Z=90, a=97, z=122
        // prios A = 27, Z=52, a=1, z=26
        if failed_item.is_lowercase() {
            prios.push(failed_item as u32 - 96)
        } else {
            prios.push(failed_item as u32 - 64 + 26)
        }
    }
    println!("task 1: {}", prios.iter().sum::<u32>());
}
