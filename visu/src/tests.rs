#![cfg(test)]

use anyhow::Result;

use crate::render_chart;

#[test]
#[ignore]
fn plot() -> Result<()> {
    render_chart("Test", [&[1, 2, 5], &[2, 3, 8], &[500, 551, 420]], "../test.png")?;

    Ok(())
}
