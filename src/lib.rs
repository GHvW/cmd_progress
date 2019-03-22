use std::io;
use std::io::prelude::*;

pub fn print_progress(percent: usize) -> io::Result<()> {
  let num_bars = percent / 5;
  let num_spaces = 20 - num_bars;


  let mut progress = String::new();
  for _ in 0..num_bars {
    progress.push('|');
  }

  for _ in 0..num_spaces {
    progress.push(' ');
  }

  print!("\r[{}] {}% complete", progress, percent);
  io::stdout().flush()?;
  Ok(())
}