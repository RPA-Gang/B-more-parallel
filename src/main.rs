mod struct_parser;

use indicatif::{ProgressBar, ProgressDrawTarget};
use rayon::prelude::*;
use struct_parser::Cli;

fn main() -> std::io::Result<()> {
    let args = Cli::new();

    let pb = ProgressBar::new(100);
    pb.set_draw_target(ProgressDrawTarget::stdout());

    rayon::scope(|s| {
        s.spawn(|_| {
            (0..args.value).into_par_iter().for_each(|n| {
                let value = fibo_par(n);
                update_progress_bar(&pb, n, value);
            });
        });
    });

    pb.finish_and_clear();
    let n_threads = rayon::current_num_threads();
    println!("Number of threads used: {}", n_threads);
    println!("Fibbonaci number given: {}", args.value);

    // pause();
    Ok(())
}

fn fibo_par(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (a, b) = rayon::join(|| fibo_par(n - 1), || fibo_par(n - 2));
            a + b
        }
    }
}

fn update_progress_bar(pb: &ProgressBar, n: u32, value: u32) {
    pb.println(format!("[+] fibo({}) | value currently is: {}", n, value));
    pb.eta();
    pb.inc(1);
}
