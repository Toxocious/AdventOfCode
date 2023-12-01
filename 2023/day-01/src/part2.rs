use crate::custom_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    println!("Advent of Code :: Day 01 - Part 2");

    // we need to iterate over every line
    let output = _input
        .lines()
        .map(process_string)
        // sum all of the returned u32 numbers
        .sum::<u32>();

    // return the output as a string
    Ok(output.to_string())
}

/**
 * Takes a string from the input file and calcs the number value for that line
 */
fn process_string(line: &str) -> u32 {
    // normalize the string by
    // replacing number words with the integer value
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let line_mod = &line[index..];

        let result = if line_mod.starts_with("one") {
            Some('1')
        } else if line_mod.starts_with("two") {
            Some('2')
        } else if line_mod.starts_with("three") {
            Some('3')
        } else if line_mod.starts_with("four") {
            Some('4')
        } else if line_mod.starts_with("five") {
            Some('5')
        } else if line_mod.starts_with("six") {
            Some('6')
        } else if line_mod.starts_with("seven") {
            Some('7')
        } else if line_mod.starts_with("eight") {
            Some('8')
        } else if line_mod.starts_with("nine") {
            Some('9')
        } else {
            let result = line_mod.chars().next();
            result
        };

        index += 1;

        result
    });

    // get only the numbers from each line
    let mut nums = line_iter.filter_map(|character| character.to_digit(10));

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../inputs/example2.txt");
        assert_eq!("281", process(input)?);

        Ok(())
    }
}
