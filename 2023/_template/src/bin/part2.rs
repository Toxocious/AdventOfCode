use {{crate_name}}::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {

    let result = process(file).context("process part 2")?;
    println!("{}", result);

    Ok(())
}
