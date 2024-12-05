use std::io::stdin;

fn main() {
    'input: loop {
        println!("Enter a positive whole number to factor or [Q]uit:");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input");
        input = input.trim().to_uppercase();

        if input == "Q" {
            println!("Goodbye");
            break 'input;
        } else if input.parse::<u64>().is_ok() {
            let number = input.parse::<i64>().unwrap();

            let mut factors: Vec<i64> = vec![];
            let upper_limit = (number as f64).sqrt() as i64 + 1;

            for n in 1..upper_limit {
                if number % n == 0 {
                    factors.push(n);
                    factors.push(number / n);
                }
            }

            factors.sort();
            factors.dedup();

            let display_string = factors
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            println!("{}", display_string);
        } else {
            println!("Invalid entry. Please try again.");
            continue 'input;
        }
    }
}
