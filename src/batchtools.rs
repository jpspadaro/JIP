use std::fmt::Error;
use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, imageops};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn open_image(file: String, verbose: bool) -> DynamicImage {
    //TODO: open image
}

fn write_image(file_out: String, verbose: bool) {

}

fn create_cache_images (file_vec: Vec<String>, verbose: bool) -> Result<u32, Error>{
    //TODO: make cache images for internal use
}

fn remove_cache_images (verbose: bool) -> Result<u32, Error> {
    //TODO: Remove cache images
}

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



pub fn batch_flipv(file_vec: Vec<String>, verbose: bool) {
    //TODO: work on this after you have a good way of managing image arrays
}