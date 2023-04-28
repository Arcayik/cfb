mod parse;
mod record;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("USAGE: cfb (record|compile) <FILE>");
        std::process::exit(2);
    }
    let query = &args[1];
    let outpath = &args[2];

    let fbdev = "/dev/fb0";
    record::capture(fbdev, outpath);

    let mut capture = parse::CaptureFile::from_path(outpath);

    capture.output_video();

    Ok(())
}

