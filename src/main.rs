extern crate glob;
extern crate indicatif;
extern crate walkdir;

use std::fs;
use std::path::Path;

use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::greed::Cli;
use crate::logger::*;

mod greed;
mod logger;

fn main() {

    let greed = Cli::parse();
    let path = Path::new(greed.path.as_str());

    logger_debug!(format!("Looking for files with name `{}` at `{}`", &greed.name, &greed.path));

    // Counter
    let mut file_count: u64 = 0;
    let mut dir_count: u64 = 0;
    let mut err_count: u64 = 0;
    let _ = visit_dirs(path, &mut file_count, &mut dir_count, &mut err_count);
    logger_debug!(format!("Looking on {} directories and {} files.", dir_count, file_count));
    if err_count > 0 {
        logger_warn!(format!("Found {} folders with no permission", err_count));
    }
    // let len = number_of_files(&greed.path);
    let mut results: Vec<String> = Vec::new();
    let mut pb = ProgressBar::new(file_count + dir_count);

    pb.set_style(ProgressStyle::default_bar()
        // .template(" [{spinner}] elapsed in {eta_precise}. {pos}/{len} - {msg}")
        .template(" [{spinner}] elapsed in {eta_precise}. Found {msg}.")
        .progress_chars("=> "));
    pb.set_message(results.len().to_string());

    let _ = find_by_name(&path, &greed.name, &mut results, &mut pb);

    // Finish and clear the progress bar
    pb.finish_and_clear();

    if results.is_empty() {
        logger_warn!("Nothing found.")
    } else {
        for result in results {
            logger_info!(format!("{}", result));
        }
    }

}

fn visit_dirs(dir: &Path, file_count: &mut u64, dir_count: &mut u64, err_count: &mut u64) -> std::io::Result<()> {
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(read_dir) => {
                for entry in read_dir {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_dir() {
                        // If it's a directory, recursively visit it
                        visit_dirs(&path, file_count, dir_count, err_count)?;
                        *dir_count += 1;
                    } else {
                        // If it's a file, you can process it here
                        // println!("{}", path.display());
                        *file_count += 1;
                    }
                }
            }
            Err(_) => {
                *err_count += 1;
            }
        }
    }
    Ok(())
}


fn find_by_name(dir: &Path, name: &String, results: &mut Vec<String>, pb: &mut ProgressBar) -> std::io::Result<()> {
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(read_dir) => {
                for entry in read_dir {
                    let entry = entry?;
                    let path = entry.path();
                    pb.inc(1);

                    if path.to_str().unwrap().contains(name) {
                        pb.set_message(results.len().to_string());
                        results.push(path.to_str().unwrap().to_string().replace(name, name.green().to_string().as_str()));
                    }

                    if path.is_dir() {
                        // If it's a directory, recursively visit it
                        find_by_name(&path, name, results, pb)?;
                    } else {
                        // If it's a file, you can process it here
                    }
                }
            }
            Err(_) => {}
        }
    }
    Ok(())
}