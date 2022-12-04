use std::fmt::Error;
use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage, imageops};
use rayon::prelude::*;
use std::fs::File;
use std::fs::create_dir;
use std::io::{self, prelude::*, BufReader};
use std::ops::Add;

fn open_image(file: String, verbose: bool) -> Result<DynamicImage, Error> {
    let mut img = image::open(file.clone()).unwrap();
    Ok(img)
}

fn write_image(file_out: String, img: DynamicImage, verbose: bool) -> Result<u32, Error>{
    img.save(file_out.as_str()).unwrap();
    if verbose{println!("{}", "Writing ".to_owned() + file_out.as_str())};
    Ok(0)
}

fn create_cache_image (file: String, verbose: bool) -> Result<(), Error>{
    //Opens image using image crate
    let img: DynamicImage = open_image(file.clone(), verbose).unwrap();
    if verbose {println!("{}", "File opened: ".to_owned() + file.clone().as_str())};

    //Generates file with a known name
    let cache_file = String::from(".JIPcache/".to_owned() + file.clone().as_str() + ".temp.png");

    //Converts/writes it using image crate.
    write_image(cache_file.clone(), img, verbose).expect("Image write error:");
    if verbose {println!("{}", "File created: ".to_owned() + cache_file.clone().as_str())}

    Ok(())
}
pub fn create_cache_images (file_vec: Vec<String>, verbose: bool) -> Result<u32, Error>{
    //Makes cache images for internal use

    create_dir(".JIPcache");
    file_vec.into_par_iter().for_each(|x| create_cache_image(x.clone(), verbose).unwrap());
    Ok(0)
}

pub fn remove_cache_images (verbose: bool) -> Result<u32, Error> {
    //Delete the cache directory.
    std::fs::remove_dir_all(".JIPcache/").unwrap();
    if verbose {println!("Cache directory removed.")}
    Ok(0)
}

pub fn get_file_vector_from_batch (batch_file: String, verbose: bool) -> io::Result<Vec<String>> {
    //Makes a vector of strings from a list in a file.
    let mut file_vec_out: Vec<String> = vec![];
    let file = File::open(batch_file)?;
    let reader = BufReader::new(file);

    //TODO: Replace with map? May be more efficient.
    for line in reader.lines() {
        let mut line_vec:Vec<String> = Vec::from([line?]);
        file_vec_out.append(&mut line_vec);
    }

    if verbose {
        println!("Included files:");
        println!(" {:?}", file_vec_out);
    }

    //TODO: parse file config info

    create_cache_images(file_vec_out.clone(), verbose).expect("Error initializing cache.");

    Ok(file_vec_out)
}

fn single_output(file: String, out_suffix: String, verbose: bool) -> Result<(), Error>{
    if verbose {println!("Start file output: {}", file.clone());}
    let cache_file: String = String::from(".JIPcache/".to_owned() + file.as_str() + ".temp.png");
    let mut img: DynamicImage = open_image(cache_file.clone(), verbose).unwrap();

    if verbose {println!("  Using  cache file: {}", cache_file.clone())}

    let out_file_name: String = String::from(file.split(".").collect::<Vec<&str>>()[0].to_owned() + out_suffix.as_str());

    write_image(out_file_name.clone(), img, verbose).expect("Image write error:");
    if verbose {println!("File output finished: {}", out_file_name);}

    Ok(())

}

pub fn batch_output (file_vec: Vec<String>, out_suffix: String, verbose: bool) {
    file_vec.into_par_iter().for_each(|x| single_output(x.clone(), out_suffix.clone(), verbose).unwrap());
}

pub fn single_flipv(file: String, verbose: bool) -> Result<(), Error> {
    if verbose {println!("Start vertical flip: {}", file.clone());}
    let cache_file: String = String::from(".JIPcache/".to_owned() + file.as_str() + ".temp.png");
    let mut img: DynamicImage = open_image(cache_file.clone(), verbose).unwrap();

    img = img.flipv();

    write_image(cache_file.clone(), img, verbose).expect("Image write error:");
    if verbose {println!("Vertical Flip finished.");}

    Ok(())
}

pub fn batch_flipv(file_vec: Vec<String>, verbose: bool) {
    //TODO: work on this after you have a good way of managing image arrays (NOT...USING A FILE CACHE INSTEAD)
    file_vec.into_par_iter().for_each(|x| single_flipv(x.clone(), verbose).unwrap());
}