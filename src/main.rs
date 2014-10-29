extern crate image;
extern crate getopts;
use std::os;
use std::io::{File, BufferedWriter};
use image::GenericImage;
use getopts::{optopt,optflag,getopts};

fn print_help(program: &str) {
    println!("Convert image file loadable by rust-image to raw headerless rgb565.");
    println!("Origin is top left pixel.");
    println!("Usage: {} -i infile -o outfile [OPTIONS]", program);
    println!("-h --help\tDisplay this help");
}

fn main()
{
    let args: Vec<String> = os::args();
    
    let opts = [
        optopt("i", "", "set input file name", ""),
        optopt("o", "", "set output file name", ""),
        optflag("h", "help", "print help")
            ];

    let program = args[0].clone();

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };
    
    if matches.opt_present("h") || args.len() == 1 {
        print_help(program.as_slice());
        return;
    }

    let input = matches.opt_str("i");
    if input.is_none() {
        println!("Must specify input file with -i");
        return;
    }
    
    let output = matches.opt_str("o");
    if output.is_none() {
        println!("Must specify output file with -o");
        return;
    }
    
    let img = image::open(&Path::new(input.unwrap())).unwrap();
    let fout = File::create(&Path::new(output.unwrap())).unwrap();
    let mut writer = BufferedWriter::new(fout);
    
    println!("Converting image of size: {}", img.dimensions());
    
    for pixel in img.to_rgb().pixels()
    {
        let (_, _, pixeldata) = pixel;
        let (r8, g8, b8) = pixeldata.channels();
        let r5 = ((r8 as u16) >> 3) as u8;
        let g6 = ((g8 as u16) >> 2) as u8;
        let b5 = ((b8 as u16) >> 3) as u8;

        let _ = writer.write_u8 (((g6 & 0b000111) << 5) | b5);
        let _ = writer.write_u8 ((r5 << 3) | ((g6 & 0b111000) >> 3));

    }
    let result = writer.flush();
    if result.is_err()
    {
        fail!(result.err().unwrap());
    }
}
