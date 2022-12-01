use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, imageops};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn get_file_vector_from_batch (batch_file: String, verbose: bool) -> io::Result<Vec<String>> {
    //Makes a vector of strings from a list in a file.
    let mut file_vec_out: Vec<String>;
    let file = File::open(batch_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let &mut line_vec = vec![line?];
        file_vec_out.append(line_vec);
    }

    if verbose {
        println!("Included files:");
        println!("{:?}", file_vec_out);
    }

    //TODO: file read
    //TODO: parse file config info

    Ok(file_vec_out)
}