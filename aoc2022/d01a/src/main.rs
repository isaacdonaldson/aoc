const CALORIE_STR : &str = include_str!("../input.txt");

fn main() {
    let max = CALORIE_STR
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();

    println!("Total max: {}", max);
    
}
