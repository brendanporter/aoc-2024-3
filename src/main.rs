use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read the file");

    let mut product: i32 = 0;

    for line in contents.lines() {
        product += multiply(line)?;
    }

    println!("product is: {}", product)
}

fn multiply(input: &str) -> Option<i32> {
    // find all regex matches for
    let re = Regex::new(r"mut\((\d{1,3})\,(\d{1,3})\)").unwrap();

    let mut results = vec![];
    for (_, [path, line_no, line]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((path, line_no.parse::<u64>()?, line));
    }

    for result in results {
        dbg!(result)
    }

    return Option<0>;
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
