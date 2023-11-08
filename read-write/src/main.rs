use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, BufReader};

fn get_args () -> io::Result<(String, String)> {
    let args: Vec<String> = env::args().collect();
    let open_path = String::from(args[1].clone());
    let write_path = String::from(args[2].clone());
    Ok((open_path, write_path))
}

fn copy (src: &str, dest: &str) -> io::Result<()> {

    let file = File::open(&src)?;
    let mut reader = BufReader::new(file);
    
    let new_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // Still create the file if it doesn't exist
        .open(&dest)?;

    let mut writer = BufWriter::new(new_file);
    io::copy(&mut reader, &mut writer)?;
    Ok(())
}

fn main () -> io::Result<()> {

    let (src, dest) = get_args()?;
    
    copy(&src, &dest)?;

    println!("{} copied to {}", src, dest);
    Ok(())
}