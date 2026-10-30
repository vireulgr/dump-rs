use std::collections::HashMap;
use std::iter;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

fn main() -> io::Result<()> {

    let known_formats = HashMap::from([
        ("PNG", vec![0x89,  0x50,  0x4e,  0x47,  0x0d,  0x0a,  0x1a,  0x0a]),
        ("JPEG", vec![0xFF, 0xD8, 0xFF]),
        ("GIF", vec![0x47, 0x49, 0x46, 0x38])
    ]);

    const BUF_SIZE: usize = 1024;

    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() < 2 {
        println!("Must be at least one argument: path to file to print content");
        std::process::exit(0);
    }
    let (file_offset, file_path) = if cli_args.len() == 3 {
        (&cli_args[1], &cli_args[2])
    }
    else {
        (&"--start".to_string(), &cli_args[1])
    };

    if !file_offset.starts_with("--start")
        && !file_offset.starts_with("--end")
    {
        eprintln!("File print location not equal neither '--start' nor '--end'");
        return Ok(());
    }


    // Open the binary file
    let mut file = File::open(file_path)?;
    if file_offset.starts_with("--end") {
        file.seek(SeekFrom::End(-(BUF_SIZE as i64)))?;
    }

    // Wrap in a buffered reader for efficient streaming
    let mut reader = BufReader::new(file);

    // Buffer to hold chunks
    let mut buffer = [0u8; BUF_SIZE]; // 1 KB buffer

    // Read a chunk
    let bytes_read = reader.read(&mut buffer)?;

    println!("Contents of file {}", file_path);

    if bytes_read == 0 {
        println!("file is empty");
        // EOF reached
        return Ok(());
    }

    for item in known_formats.iter() {
        let found = iter::zip(item.1, buffer)
            .all(|char| *char.0 == char.1);

        if found {
            break;
        }
    }
    let maybe_format = known_formats
        .iter()
        .find(|record| {
            iter::zip(record.1, buffer).all(|char| *char.0 == char.1)
        });

    if let Some(format) = maybe_format {
        println!("File is {}", format.0);
    }

    print!("row | 00             05             0A             0F                15             1A             1F");

    let mut col_number = 0;
    let mut row_number = 0;
    loop {
        // Process the chunk (example: print hex values)
        for byte in &buffer[..bytes_read] {
            if col_number % 32 == 0 {
                println!();
                print!("{:02x}  | ", row_number);
                col_number = 0;
                row_number += 1;
            }
            col_number += 1;
            print!("{:02X} ", byte);
        }

        // Read a chunk
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            // EOF reached
            break;
        }

        println!();
        break;
    }

    Ok(())
}
