// Made with haterd and vengence by: MuhammadMuazen ;> // Just Kidding
// This is my first battle with the rust programming language don't judge XD
// dolya was made as learning project for me to help me and others to use google dorks faster
// https://github.com/MuhammadMuazen/dolya

mod full_query_builder;
mod arguments_handler;
mod browser_command_builder;

fn main() {

    // Get the command line args
    let args: Vec<String> = std::env::args().collect();
    
    // Check the arguments and format them
    let handled_args: Vec<String> = arguments_handler::handle_arguments(args);
    
    // Get the browser name
    let browser_name: String = handled_args[2].clone();
    // Get the user choice for printing the query or running the browser
    let print_query: String = handled_args[9].clone();

    // Build the full searching query
    let query: String = full_query_builder::full_query(handled_args);

    // Check if the user wants to print the query or running the browser with the query
    if print_query == "yes".to_string() {
        
        browser_command_builder::print_query(query);

    } else {

        // Get the full running command
        let mut browser_command: Vec<String> = browser_command_builder::browser_query_command_line(query, browser_name);

        browser_command[1] = browser_command[1].trim_matches('\'').to_string();

        println!("\n{} {}\n", browser_command[0].to_string(), browser_command[1].to_string());

        // Running the command like this <browser> <query>
        std::process::Command::new(browser_command[0].to_string())
            .arg(browser_command[1].to_string())
            .spawn()
            .expect("[!] Failed to run the browser!");
    
    }
}
