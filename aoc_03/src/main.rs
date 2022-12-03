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

    let mut task2_prios: Vec<u32> = Vec::new();
    let lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    for i in (0..lines.len() - 2).step_by(3) {
        let elf1 = lines.get(i).unwrap();
        let elf2 = lines.get(i + 1).unwrap();
        let elf3 = lines.get(i + 2).unwrap();
        for v in b'a'..=b'z' {
            let val: char = v as char;
            if elf1.contains(&val.to_string())
                && elf2.contains(&val.to_string())
                && elf3.contains(&val.to_string())
            {
                task2_prios.push(val as u32 - 96);
                break;
            }
        }

        for v in b'A'..=b'Z' {
            let val: char = v as char;
            if elf1.contains(&val.to_string())
                && elf2.contains(&val.to_string())
                && elf3.contains(&val.to_string())
            {
                task2_prios.push(val as u32 - 64 + 26);
                break;
            }
        }
    }
    println!("task 2: {}", task2_prios.iter().sum::<u32>());
}
