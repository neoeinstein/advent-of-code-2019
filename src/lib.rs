use std::{env, fs, io, path::PathBuf};

fn get_input_filename() -> Option<PathBuf> {
    let in_file = env::args().skip(1).next()?;
    if in_file == "-" {
        None
    } else {
        Some(PathBuf::from(in_file))
    }
}

/// Creates an input reader from the file specified on the command line (STDIN if not provided) 
pub fn get_input_reader() -> Box<dyn io::BufRead> {
    match get_input_filename() {
        Some(in_file) => Box::new(io::BufReader::new(fs::File::open(&in_file).expect("file should be openable"))),
        None => Box::new(io::BufReader::new(io::stdin())),
    }
}