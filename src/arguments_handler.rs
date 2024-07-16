/*
    contains help message function to display and function to check the existence of the arguments
*/

mod options_functions;

/*
    Help Message to display
    
    Input --> None
    Return --> None
    Calls: 
        1) options_functions::get_exec_file_name() --> to get the exe file name
*/
pub fn help_message() {
    
    let exe_file_name: String = options_functions::get_exec_file_name();

    println!(r#"[+] Usage: {} <search-query> [-options]
    
    options:
        -h, --help                           print the help message
        -p, --print-search-query             will print the full search query instead of searching for it
        -l, --list-browsers                  list all the browsers which exist in the system
        -b, --browser <browser-name>         choose the browser you want to use for the search
        -u, --inurl <file>                   search for pages that have spicific word in the url
        -e, --exclude-domains <file>         get the excluded domains for the search from a text file
        -i, --intitle <file>                 search for pages that have spicific word
        -t, --intext <file>                  find pages where the specified word appears in the web page
        -f, --filetype <file>                search for specific file types = filetype:x or filetype:y
        -r, --rules <file>                   use a single file that has all the search operators you want
        -s, --search-engine <search-engine>  choose the search engine you want to use

        Available search engines:            _______________________________________
                                            |   google  |   duckduckgo  |   bing    |
        Made By:                            |___________|_______________|___________|
        ~ MuhammadMuazen ~                  |   yahoo   |     yandex    |   brave   |
        ~ github.com/MuhammadMuazen ~       |___________|_______________|___________|
        
        Note: All the Google Dorks operators use the or method to search
        e.g: operator:<line1> or operator:<line2> where the lines come from the provided files
        
[+] Example(1): {} "search_query" -b firefox -s brave -u inurl_file.txt -f filetypes.txt
[+] Example(2): {} "seach_query" -b firefox -s google -r rules_file.txt"#, 
    exe_file_name, exe_file_name, exe_file_name);
    
    println!();
}

/*
    The exposed function that is used to get the arguments and check if they exist
    
    Input --> arguments: which are passed from the main.rs and represt the stdin args
    Return --> Vec<String>: which holds the 9 values:
        0) search_query
        1) search_engine
        2) browser
        3) inurl
        4) exluded_domains
        5) intitle
        6) intext
        7) filetype
        8) full_rules
        9) print_search_query
    Calls:
        1) options_functions::list_available_browsers(): to check all the avaliable browsers on the system
        2) options_functions::check_existence_of_option(): to get the option value and check it is existence
        3) options_functions::search_engine_option(): to check if the provided search enigne exists or not

*/
pub fn handle_arguments(arguments: Vec<String>) -> Vec<String> {
    
    let mut search_query: String = String::new();
    // Value to check if the user want to print the search query
    let mut print_search_query: String = "no".to_string(); 

    // This vector holds the values of the options the user have passed like this
    // 0 --> search_query
    // 1 --> search_engine
    // 2 --> browser
    // 3 --> inurl
    // 4 --> exluded_domains
    // 5 --> intitle
    // 6 --> intext
    // 7 --> filetype
    // 8 --> full_rules
    // 9 --> print_search_query
    let mut returned_data_vec: Vec<String> = Vec::new();


    if arguments.len() <= 1 {
        
        println!("[!] No valid arguments provided!");
        help_message();
        std::process::exit(-1); 
    }
    
    // -h, --help option
    if arguments.iter().any(|s: &String| s.contains("-h") || s.contains("--help")) {
        
        help_message();
        std::process::exit(0);
    }

    // -l, --list-browser option
    if arguments.iter().any(|s: &String| s.contains("-l") || s.contains("--list-browsers")) {
        let avail_browsers: Vec<String> = options_functions::list_available_browsers();
        
        if avail_browsers.len() < 1 {
            println!("[!] No browser where found!");
            //TODO add the browser path to a file
            return Vec::new();
        } else {
            println!("[+] Found the following browsers:\n");

            for browser in avail_browsers {
                println!("\t{}", browser);
            }
        }

        std::process::exit(0);
    }

    // Get what the user is searching for 
    search_query.push_str(arguments[1].as_str());
    
    // -p, --print-search-query option
    if arguments.iter().any(|s: &String| s.contains("-p") || s.contains("--print-search-query")) {
        print_search_query = "yes".to_string();
    }

    // -s, --search-engine option
    let mut search_engine: String = options_functions::check_existence_of_option(
        &arguments, "-s", "--search-engine");
    
    // -b, --browser option
    let mut browser: String = options_functions::check_existence_of_option(
        &arguments, "-b", "--browser");

    // -u, --inurl option
    let mut inurl: String = options_functions::check_existence_of_option(
        &arguments, "-u", "--inurl");

    // -e, --exclude-domains option
    let mut excluded_domains: String = options_functions::check_existence_of_option(
        &arguments, "-e", "--exclude-domains");

    // -i, --intitle option
    let mut intitle: String = options_functions::check_existence_of_option(
        &arguments, "-i", "--intitle");

    // -t, --intext option
    let mut intext: String = options_functions::check_existence_of_option(
        &arguments, "-t", "--intext");

    // -f, --filetype option
    let mut filetype: String = options_functions::check_existence_of_option(
        &arguments, "-f", "--filetype");

    // -r, --rules option
    let full_rules: String = options_functions::check_existence_of_option(
        &arguments, "-r", "--rules");

    if search_query.is_empty() {
        
        println!("[!] Empty search query!");
        help_message();
        std::process::exit(-1);
    }

    // Set the default search engine to google if the search engine option is not set or search engine option is not available
    if search_engine.is_empty() || !options_functions::search_engine_option(&search_engine) {
        search_engine.push_str("google"); // Change the default here if you want
    }

    // Set the default browser to firefox
    if browser.is_empty() {

        let avail_browsers: Vec<String> = options_functions::list_available_browsers();
        
        if avail_browsers.iter().any(|s: &String | s.contains("firefox")) {
            browser.push_str("firefox");
        } else {
            println!("[!] No browser found in the system!");
            std::process::exit(-1);
        }
    }

    // If the full rules file is set igonre all other files
    if !full_rules.is_empty() {
        inurl = String::new();
        excluded_domains = String::new();
        intitle = String::new();
        intext = String::new();
        filetype = String::new();
    }

    // 0
    returned_data_vec.push(search_query);
    // 1
    returned_data_vec.push(search_engine);
    // 2
    returned_data_vec.push(browser);
    // 3
    returned_data_vec.push(inurl);
    // 4
    returned_data_vec.push(excluded_domains);
    // 5
    returned_data_vec.push(intitle);
    // 6
    returned_data_vec.push(intext);
    // 7
    returned_data_vec.push(filetype);
    // 8
    returned_data_vec.push(full_rules);
    // 9
    returned_data_vec.push(print_search_query);

    return returned_data_vec;
}
