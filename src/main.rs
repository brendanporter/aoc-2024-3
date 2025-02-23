use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read the file");

    let mut product: i32 = 0;

    contents =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

    for line in contents.lines() {
        product += multiply(line);
    }

    println!("product is: {}", product)
}

fn multiply(input: &str) -> i32 {
    // find all regex matches for
    let re = Regex::new(r"mut\((\d{1,3})\,(\d{1,3})\)").unwrap();

    let mut results = vec![];
    for (_, [num1, num2]) in re.captures_iter(input).map(|c| c.extract()) {
        dbg!(num1);
        dbg!(num2);
        results.push((num1, num2));
    }

    dbg!(&results);

    for result in results {
        dbg!(result);
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe() {
        assert_eq!(
            multiply("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }
}
