#![cfg(test)]

use anyhow::Result;

use crate::{render::Graph, render_chart};

#[test]
#[ignore]
fn plot() -> Result<()> {
    render_chart(Graph {
        title: "Test Graph",
        data: [&[1, 2, 5], &[2, 3, 8], &[500, 551, 420]],
        legend: ["data", "some data", "data too"],
        output_file: "../test.png",
        ..Default::default()
    })?;

    Ok(())
}
