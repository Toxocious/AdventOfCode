use crate::custom_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    println!("Advent of Code :: Day 00 - Part 2");

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../inputs/example2.txt");
        assert_eq!("", process(input)?);

        Ok(())
    }
}
