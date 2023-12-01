use day_01::part2::process;
use miette::Context;

fn main() -> miette::Result<()> {
    let file = include_str!("../../inputs/input2.txt");

    let result = process(file).context("process part 2")?;
    println!("{}", result);

    Ok(())
}
