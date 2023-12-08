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

impl<'a> Game<'a> {
    /**
     * Determine if the minimum amount of cubes needed of each color for the game.
     */
    fn minimum_cube_set(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();

        self.rounds
            .iter()
            .fold(map, |mut acc, round| {
                for cube in round.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| {
                            *v = (*v).max(cube.amount);
                        })
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
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
    println!("Advent of Code :: Day 02 - Part 2");

    let games = parse_games(_input).expect("Should parse");

    Ok(games
        .1
        .iter()
        .map(|game| game.minimum_cube_set())
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../inputs/example2.txt");
        assert_eq!("2286", process(input)?);

        Ok(())
    }
}
