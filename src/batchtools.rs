use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, imageops};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn get_file_vector_from_batch (batch_file: String, verbose: bool) -> io::Result<Vec<String>> {
    //Makes a vector of strings from a list in a file.
    let mut file_vec_out: Vec<String> = vec![];
    let file = File::open(batch_file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_vec:Vec<String> = Vec::from([line?]);
        file_vec_out.append(&mut line_vec);
    }

    if verbose {
        println!("Included files:");
        println!("{:?}", file_vec_out);
    }

    //TODO: parse file config info

    Ok(file_vec_out)
}