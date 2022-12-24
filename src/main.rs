use anyhow::bail;
use ascii_art::image_to_ascii;
use clap::Parser;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image to convert.
    #[arg(short, long)]
    input: PathBuf,
    /// Location of where to save the text file.
    #[arg(short, long)]
    output: PathBuf,
    /// Width to resize before processing.
    #[arg(short, long)]
    target_width: Option<u32>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let img = image::open(args.input);
    let Ok(img) = img else{
        bail!("Image could not be read!")
    };
    let Some(extension) = args.output.extension() else {
        bail!("Output file's extension could not be read!")
    };
    if extension != "txt" {
        bail!("Output has to be a .txt file!")
    }
    if let Some(parent_dir) = args.output.parent() {
        create_dir_all(parent_dir)?;
    }

    let mut target_file = File::create(args.output)?;
    let buf = image_to_ascii(&img, args.target_width);
    target_file.write_all(buf.as_bytes())?;

    println!("Completed");
    Ok(())
}
