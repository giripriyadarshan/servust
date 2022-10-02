use indicatif::{ProgressBar, ProgressStyle};

pub fn progress_bar() -> ProgressBar {
    let pb = ProgressBar::new(1);
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_strings(&["-", "\\", "|", "/"]);
    pb.set_style(spinner_style);
    pb.enable_steady_tick(std::time::Duration::from_millis(50));
    pb.set_prefix("  ");
    pb
}
