use std::error::Error;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
    .version("0.1.0")
    .author("sutonchoko")
    .about("Rust cat")
    // ここで引数を定義する
    .arg(
      Arg::with_name("files")
          .value_name("FILE")
          .help("Input file(s) [default: -]")
          .multiple(true)
          .default_value("-"),
    )
    .arg(
        Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
            .conflicts_with("number_nonblank"),
    )
    .arg(
        Arg::with_name("number_nonblank")
            .short("b")
            .long("number-nonblank")
            .help("Number nonblank lines")
            .takes_value(false),
    )
    .get_matches();

  Ok(Config {
    files: matches.values_of_lossy("files").unwrap(),
    number_lines: matches.is_present("number"),
    number_nonblank_lines: matches.is_present("number_nonblank"),
  })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}