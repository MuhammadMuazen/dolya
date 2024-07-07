pub fn print_query(mut query: String) {
    
    if query.starts_with('\'')  && query.ends_with('\''){

        query = query.trim_matches('\'').to_string();
    }

    println!("[+] The full query is:\n");
    println!("\t{}", query);
}


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