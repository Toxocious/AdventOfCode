use crate::custom_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    println!("Advent of Code :: Day 01 - Part 1");

    // we need to iterate over every line
    let output = _input
        .lines()
        .map(|line| {
            let mut nums =
                // get only the numbers from each line
                line.chars().filter_map(|character| {
                    character.to_digit(10)
                });

            // get the first number in the line
            let first = nums.next().expect("Should be a number");

            // get the last number in the line if possible
            // if it exists, we concat the `let first` value with the {last} value
            // if it doesn't exist, we concat `let first` with itself
            match nums.last() {
                Some(last) => format!("{first}{last}"),
                None => format!("{first}{first}"),
            }
            // parse the result as a u32 integer
            .parse::<u32>()
            .expect("Should be a valid number")
        })
        // sum all of the returned u32 numbers
        .sum::<u32>();

    // return the output as a string
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../inputs/example1.txt");
        assert_eq!("142", process(input)?);

        Ok(())
    }
}
