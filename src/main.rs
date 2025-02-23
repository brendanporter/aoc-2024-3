use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read the file");

    let mut product: i64 = 0;

    // contents =
    // "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
    let mut execute: bool = true;
    for line in contents.lines() {
        println!("before mult, product is {}", product);
        let rs = multiply(line, execute);
        product += rs.0;
        execute = rs.1
    }

    println!("product is: {}", product)
}

fn multiply(input: &str, ex: bool) -> (i64, bool) {
    // find all regex matches for
    let re = Regex::new(r"(don't\(\)|do\(\))()|mul\((\d{1,3})\,(\d{1,3})\)").unwrap();

    // let mut results = vec![];
    let mut execute: bool = ex;
    let mut product: i64 = 0;
    let mut dos: i32 = 0;
    let mut donts: i32 = 0;
    for (_, [m1, m2]) in re.captures_iter(input).map(|c| c.extract()) {
        // dbg!(m1);
        // dbg!(m2);
        if m1 == "don't()" {
            donts += 1;
            execute = false
        }
        if m1 == "do()" {
            dos += 1;
            execute = true
        }
        // if !execute && m1 != "do" && m1 != "don't" {
        //     println!(
        //         "not adding {} * {}",
        //         m1.parse::<i64>().unwrap(),
        //         m2.parse::<i64>().unwrap(),
        //         m1.parse::<i64>().unwrap() * m2.parse::<i64>().unwrap()
        //     );
        // }
        if execute && m1 != "do()" && m1 != "don't()" {
            product += m1.parse::<i64>().unwrap() * m2.parse::<i64>().unwrap();
            println!(
                "adding {} * {}: ({}): (product is currently {})",
                m1.parse::<i64>().unwrap(),
                m2.parse::<i64>().unwrap(),
                m1.parse::<i64>().unwrap() * m2.parse::<i64>().unwrap(),
                product
            );
        }
        // results.push((num1, num2));
    }

    println!("dos: {}, don'ts: {}", dos, donts);

    // dbg!(&results);

    return (product, execute);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe() {
        // assert_eq!(
        //     multiply("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        //     161
        // );
        assert_eq!(
            multiply("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
