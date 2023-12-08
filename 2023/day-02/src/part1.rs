use std::{collections::BTreeMap, ops::Not};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

/**
 * Determine's if a round is within the "winning" criteria.
 */
impl<'a> Game<'a> {
    fn valid_for_cube_set(&self, cube_limits: &BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            // Start an interator for our rounds data
            .iter()
            .any(|round| {
                // Iterate through each game in the current round
                round.iter().any(|shown_cube| {
                    // Get the amount of the current cube based on its color
                    shown_cube.amount > *cube_limits.get(shown_cube.color).expect("a valid cube")
                })
            })
            .not()
            .then_some(
                self.id
                    .parse::<u32>()
                    .expect("game id should a parsable u32"),
            )
    }
}

/**
 * Track the data of a cube.
 *
 * Input example:
 *      "3 blue"
 */
fn cube(input: &str) -> IResult<&str, Cube> {
    // Parses the input and returns a Cube, containing the color and amount of the cube
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;

    // Return the input and our filled Cube struct
    Ok((input, Cube { color, amount }))
}

/**
 * Handle calculating the cubes used in the round.
 *
 * Input example:
 *      "3 blue, 4 red"
 */
fn cubes(input: &str) -> IResult<&str, Vec<Cube>> {
    // Parses the input by splitting the input string into cube slices (3 blue OR 4 red)
    // Then passes the cube slice to `fn cube`
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;

    // Return the input and our cubes
    Ok((input, cubes))
}

/**
 * Parse a single game.
 *
 * Example input:
 *      "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
 */
fn game(input: &str) -> IResult<&str, Game> {
    // Strip the "Game " text from the string
    // Get the Game ID and store it in the id part of the tuple
    // Keep the rest of the input that we haven't parsed
    let (input, id) = preceded(tag("Game "), digit1)(input)?;

    // Gets all cubes from each round and sends the input data to `fn cubes`
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), cubes))(input)?;

    // Returns the input and a Game struct filled with our rounds data.
    Ok((input, Game { id, rounds }))
}

/**
 * Handles parsing all games from an input.
 */
fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    // get a list of games from the input?
    let (input, games) = separated_list1(line_ending, game)(input)?;

    Ok((input, games))
}

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    println!("Advent of Code :: Day 02 - Part 1");

    let cube_limits = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let games = parse_games(_input).expect("Should parse");

    Ok(games
        .1
        .iter()
        .filter_map(|game| game.valid_for_cube_set(&cube_limits))
        .sum::<u32>()
        .to_string())
}

/**
 * Testing the example input.
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../inputs/example1.txt");
        assert_eq!("8", process(input)?);

        Ok(())
    }
}
