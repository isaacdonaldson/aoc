const CALORIE_STR : &str = include_str!("../input.txt");

fn main() {
    let mut maxes: Vec<u32> = CALORIE_STR
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    maxes.sort_unstable();


    let top_three_sum: u32 = maxes.iter().rev().take(3).sum();

    println!("Total of top three: {top_three_sum}");
    
}
