# JIP - Jason's Image Processor
JIP is a CLI batch image processing tool written in Rust.
```
Jason's Image Processor: a custom CLI image processor.
  -v, --verbose Output image and extra processing information
  -f, --file (default '') input file name
  -b, --batch (default '') list of file names for parallel batch processing
  -o, --output (default 'image.png') output file name, used as a suffix for batches
  --flipv Flip image vertically
```

## Image Caching
Each threaded job opens, acts on, then writes a png file. These are temporary files initially created from the originals specified in the batch file. This is to give Rust a chance to release memory and presumably work more images without needing to keep them in memory...we go from an array keeping complete image data to an array of strings. All the output files are just copies made from the last cache images before they're deleted.

In the directory the command is run a ".JIPtemp" directory is created. This will contain the cached images. This is deleted upon completeion of the command.

Instead of just doing a file system copy, we load the image into memory the output the image as a new file in case the filetype(specified by extension) is something other than png.