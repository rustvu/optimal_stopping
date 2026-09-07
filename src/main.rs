//! Modeling the [Optimal Stopping problem](https://en.wikipedia.org/wiki/Optimal_stopping)
//! Note: this code is not optimal nor most idiomatic Rust. The motivation is to
//! use basic language concepts for a non-trivial example early in the class.
use fastrand;

const N_SUITORS: usize = 100;
const N_EXPERIMENTS: usize = 100_000;

/// Suitors are represented as random integer numbers (full range)
/// where a higher numbers mean more desirable suitor
fn generate_suitors() -> [i32; N_SUITORS] {
    let mut suitors = [0; N_SUITORS];
    for i in 0..N_SUITORS {
        suitors[i] = fastrand::i32(..);
    }
    suitors
}

/// Pick a prince (best suitor) using a simple exploration/exploitation strategy
/// The `n_explore` parameter controls the exploration size
fn pick_a_prince(suitors: [i32; N_SUITORS], n_explore: usize) -> i32 {
    assert!(n_explore <= N_SUITORS);

    let mut bar = i32::MIN;
    for i in 0..N_SUITORS {
        if i < n_explore {
            bar = if suitors[i] > bar { suitors[i] } else { bar };
        } else {
            if suitors[i] >= bar {
                return suitors[i];
            }
        }
    }

    suitors[N_SUITORS - 1]
}

fn main() {
    let mut optimum = (0, 0); // (score, explore)
    let mut scores = [0; N_SUITORS];

    for n_explore in 0..N_SUITORS {
        let mut score = 0;
        for _ in 0..N_EXPERIMENTS {
            let suitors = generate_suitors();
            let best_suitor = *suitors.iter().max().unwrap();
            let prince = pick_a_prince(suitors, n_explore);

            // The goal is to pick the absolute best suitor
            if prince == best_suitor {
                score += 1;
            }
        }
        scores[n_explore] = score;
        if score > optimum.0 {
            optimum = (score, n_explore);
        }
    }

    println!("Optimal exploration threshold: {}", optimum.1);

    // Optional Plotting
    #[cfg(feature = "plotly")]
    {
        use plotly::{
            common::Mode,
            layout::{Axis, Shape, ShapeLine, ShapeType},
            Layout, Plot, Scatter,
        };

        let mut plot = Plot::new();
        let trace = Scatter::new((0..scores.len()).collect(), Vec::from(scores));
        plot.add_trace(trace);

        let optimum_marker = Scatter::new(vec![optimum.1], vec![optimum.0]).mode(Mode::Markers);
        plot.add_trace(optimum_marker);

        let layout = Layout::new()
            //.width(800)
            .height(600)
            .show_legend(false)
            .title("Optimal Stopping")
            .x_axis(
                Axis::new()
                    .title("n_explore")
                    .tick_values(vec![optimum.1 as f64])
                    .tick_text(vec![optimum.1.to_string()]),
            )
            .y_axis(Axis::new().title("score"))
            .shapes(vec![Shape::new()
                .shape_type(ShapeType::Line)
                .x0(optimum.1 as f64)
                .x1(optimum.1 as f64)
                .y0(0.0)
                .y1(optimum.0 as f64)
                .line(ShapeLine::new().color("red"))]);
        plot.set_layout(layout);
        plot.show();
    }
}
