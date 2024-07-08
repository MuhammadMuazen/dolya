/*
    This file is called main.rs and hold the functions to -p option and the function to run the command of the query
*/

/*
    Function that prints the full query
    Input --> the query as a String
    Return --> None
*/
pub fn print_query(mut query: String) {
    
    if query.starts_with('\'')  && query.ends_with('\''){

        query = query.trim_matches('\'').to_string();
    }

    println!("[+] The full query is:\n");
    println!("\t{}", query);
}

/*
    Function that return the wanted query with a provided browser
    Input:
        1) the query as a String
        2) the browser name that would launch and search for the query as a String
    Return --> Vec<String>: which is length of two and holds the values:
            1) [0]: The browser name
            2) [1]: the query String
*/
pub fn browser_query_command_line(query: String, browser_name: String) -> Vec<String> {
    
    if browser_name.is_empty() {

        println!("[!] Sorry Could not find any browser!");
        print_query(query);
        std::process::exit(-1);
    }

    let mut command: Vec<String> = Vec::new();

    command.push(browser_name);
    command.push(query);

    return command;
}