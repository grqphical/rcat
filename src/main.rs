use std::env;
use std::fs;
use std::process;

fn main() {
    // get args from cmd line
    let mut args = env::args();

    // we don't need the first argument which is the file path
    args.next();

    // check if the user wants to have numbered lines
    let number_lines = env::var("NUMBERED_LINES").is_ok();

    // read the files data or throw an error if it couldn't
    let file = args.next().expect("Could not parse string");
    let contents = fs::read_to_string(&file).unwrap_or_else(|_err| {
        eprintln!("Could not open file: '{file}' because it does not exist");
        process::exit(1);
    });

    // print a nice title
    println!("Contents of '{file}':\n");

    if number_lines {
        // loop through every line, print the line number and the line data
        // do this if the user wants to have numbered lines
        let mut index = 1;

        for line in contents.lines()
        {
            println!("{index} {line}");
            index += 1;
        }
    }
    else
    {
        // otherwise print the whole thing
        println!("{contents}");
    }
    
}
