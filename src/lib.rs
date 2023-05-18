use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

pub fn find_matches(content: String, pattern: &str) -> Vec<String> {
    // progress bar
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{prefix} {spinner:.green} [{elapsed}] [{bar:40.green/cyan}] {bytes}/{total_bytes})")
        .unwrap().progress_chars("||="));
    pb.set_prefix("Searching: ");

    let mut matches :Vec<String> = Vec::new();

    for line in content.lines().progress_with(pb) {
        if line.contains(pattern) {
            matches.push(line.to_string());
        }

        // progress bar
        thread::sleep(Duration::from_millis(5));
    }

    return matches;
}