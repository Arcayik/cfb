mod record;
mod compile;

fn usage() { println!("USAGE: cfb (record|compile) <FILE>"); }

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        usage();
        std::process::exit(2);
    }
    let query = &args[1];
    let outpath = &args[2];

    let fbdev = "/dev/fb0";

    match query.as_str() {
        "record"  => record::capture(fbdev, &outpath),
        "compile" => Ok(compile::CaptureFile::from_path(&outpath).output_video()),
        _ => { usage(); std::process::exit(2); }
    }?;

    Ok(())
}


