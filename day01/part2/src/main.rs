fn main() {
    let entries: Vec<u32> = std::fs::read_to_string("../input")
        .expect("Unable to read input")
        .lines()
        .map(|e| e.parse::<u32>().unwrap())
        .collect();

    for (i, e) in entries.iter().enumerate() {
        for (j, f) in entries[i + 1..].iter().enumerate() {
            for g in entries[j + 1..].iter() {
                if e + f + g == 2020 {
                    println!("{}", e * f * g);
                    return;
                }
            }
        }
    }
}
