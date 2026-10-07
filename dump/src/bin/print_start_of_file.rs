fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() < 2 {
        println!("Must be at least one argument: path to file to print content");
        std::process::exit(0);
    }

    let file_path = &cli_args[1];
    println!("{}", file_path.to_string());
    let file_content = std::fs::read_to_string(file_path).expect("File reading error");
    
    let content_size = file_content.len();

    let slice_size = if content_size > 31 { 31 } else { content_size };
    let slice = &file_content[0..slice_size];

    println!("{:?}", slice);
}
