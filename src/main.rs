extern crate lapp;

use std::process::exit;
use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, imageops};

mod batchtools;

fn verbose_out (name: String, img: &DynamicImage) {
    //Provides image information
    println!("{}","----Details of ".to_owned() + &name + ".----");
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
    println!("----");
}

fn main() {
    let args = lapp::parse_args("
Jason's Image Processor: a custom CLI image processor.
  -v, --verbose Output image and extra processing information
  -f, --file (default '') input file name
  -b, --batch (default '') list of file names for parallel batch processing
  -o, --output (default 'image.png') output file name, used as a suffix for batches
  --flipv Flip image vertically
	");

    //Turn input args into variables
    let verbose = args.get_bool("verbose");
    let file = args.get_string("file");
    let batch = args.get_string("batch");
    let output = args.get_string("output");
    let flipv = args.get_bool("flipv");

    //Single file configuration
    if !file.is_empty() {
        // Use the open function to load an image from a Path.
        // `open` returns a `DynamicImage` on success.
        let mut img = image::open(file.clone()).unwrap();

        // The dimensions method returns the images width and height.
        if verbose { verbose_out(file.clone(), &img) }

        if flipv {
            //changes img to a vertically flipped image by using method and reassigning
            println!("Flipping image vertically.");
            img = img.flipv();
        }

        // Write the contents of this image to the Writer in PNG format.
        println!("Processing Complete.");
        img.save(output.as_str()).unwrap();

        println!("{}", output + " saved.");
    }

    //Batch file configuration.
    // Yes, you can use single and batch at the same time.
    // No, duplicate support is not guaranteed.
    if !batch.is_empty() {
        let file_vec = batchtools::get_file_vector_from_batch(batch.clone(), verbose).unwrap();
        if flipv {

        }
    //TODO: Parallel batch processing
    }
}