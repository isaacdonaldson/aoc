const INPUT_STR: &str = include_str!("../input_test.txt");

fn main() {
    let _map = INPUT_STR
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split("").filter(|c| *c != "").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("I can no longer finish before work ¯\\_(ツ)_/¯");
}
