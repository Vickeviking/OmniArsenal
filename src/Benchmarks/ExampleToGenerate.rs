use omni_arsenal::containers::trees::red_black_tree::RedBlackTree;
use std::{boxed::Box, cell::RefCell, rc::Rc};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::time::Instant;
use plotters::prelude::*;

/***
 * 
 * Creates a plot of the time complexity of insertion in a Red-Black Tree.
 * 
 */


fn example() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Time Complexity of Insertion in Red-Black Tree", ("Arial", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10f32, 0f32..6f32)?;

    chart.configure_mesh().draw()?;

    let mut data = Vec::new();

    for size in (1..=100).map(|i| i * 10000) { // Adjusted the range here
        let mut rb_tree: RedBlackTree<i32, i32> = RedBlackTree::new_empty();
        let mut list: Vec<i32> = (0..size).collect();
        list.shuffle(&mut thread_rng());

        let start = Instant::now();

        for i in list {
            rb_tree.insert(i, i);
        }

        let duration = start.elapsed();
        let seconds = duration.as_secs_f32();
        data.push((size as f32 / 100000.0, seconds)); // Adjusted the x-value here
    }

    chart.draw_series(LineSeries::new(data, &RED))?;

    Ok(())
}
