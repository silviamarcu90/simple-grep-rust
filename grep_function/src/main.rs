use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;
use clap::ArgAction;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The option -f, followed by the file path:
    /// the file to be read from and grep
    #[clap(short, parse(from_os_str))]
    fpath: std::path::PathBuf,
    
    /// The option -p, followed by the pattern to be searched in the file
    #[clap(short, value_parser)]
    pattern: String,

    /// The option -m, followed by a number to limit the maximum number of matched lines displayed
    #[clap(short = 'm', value_parser,  default_value_t = 0)]
    max_matches: i32,

    /// The option -n to show the line number
    #[clap(action = ArgAction::SetTrue, short = 'n')]
    show_line_number: bool,

    /// The option -I to allow case insensitive when searching
    #[clap(action = ArgAction::SetTrue, short = 'I')]
    case_insensitive: bool
}

fn main() {
    let args = Cli::parse();

    println!("pattern searched:{:?}\nfile path:{:?}", args.pattern, args.fpath.display());

    let mut found:bool = false; // flag to know if the pattern was found
    let mut matches:i32 = 0;


    if let Ok(lines) = read_lines(args.fpath) {
        let mut line_number:i32 = 0;
        for line in lines {
            line_number = line_number + 1;
            if let Ok(line_text) = line {
                if ( args.case_insensitive && line_text.to_ascii_lowercase().contains(&args.pattern.to_ascii_lowercase()) )
                || ( !args.case_insensitive && line_text.contains(&args.pattern) ) { // why I should use &args.pattern?? and not copy the value
                    
                    found = true;
                    if args.max_matches < 0 || (args.max_matches > 0 && matches >= args.max_matches)
                    {
                        break;
                    }
                    matches = matches + 1;

                    if args.show_line_number
                    {
                        println!("{}:{}", line_number, line_text);
                    }
                    else {
                        println!("{}", line_text);
                    }
                }
            }
        }
    }
    else {
        println!("File not found.");
    }

    if !found
    {
        println!("Not found.");
    }
}