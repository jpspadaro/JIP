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
  -b, --batch (default '') batch file name
  -o, --output (default 'image.png') output file name, used as a suffix for batches
  -g, --grayscale Change colors to greyscale
  -i, --invert inverts colors
  -C, --leave-cache Leave cache directory after operation
  --flipv Flip image vertically
  --fliph Flip image horizontally
  --brighten (default 0) brightens or darkens an image. Negatives darken. Values range from -256 to 256.
	");

    //Turn input args into variables
    let verbose = args.get_bool("verbose");
    let file = args.get_string("file");
    let batch = args.get_string("batch");
    let brighten = args.get_integer("brighten");
    let output = args.get_string("output");
    let grayscale = args.get_bool("grayscale");
    let invert = args.get_bool("invert");

    let flipv = args.get_bool("flipv");
    let fliph = args.get_bool("fliph");
    let leave_cache = args.get_bool("leave-cache");


    //Batch file configuration.
    let mut file_vec;

    if !batch.is_empty() {
        file_vec = batchtools::get_file_vector_from_batch(batch.clone(), verbose).unwrap();
    } else {
        file_vec = vec![];
    }
    if !file.is_empty() {
        let mut temp_file_vec: Vec<String> = vec![file];
        file_vec.append(&mut temp_file_vec);
    }

    batchtools::create_cache_images(file_vec.clone(), verbose);

    if flipv {
        batchtools::batch_flipv(file_vec.clone(), verbose);
    }
    if fliph {
        batchtools::batch_fliph(file_vec.clone(), verbose);
    }
    if grayscale {
        batchtools::batch_grayscale(file_vec.clone(), verbose);
    }
    if invert {
        batchtools::batch_invert(file_vec.clone(), verbose);
    }
    if brighten!=0 {
        batchtools::batch_brighten(file_vec.clone(), brighten,verbose);
    }
    if !output.is_empty() {
        batchtools::batch_output(file_vec.clone(), output, verbose)
    }

    //TODO: Parallel batch processing
    if !leave_cache {
        batchtools::remove_cache_images(verbose).expect("Problem removing cache. Delete \".JIPcache\" in this directory to clean up.");
    }
}