/*
    Holds helper functions for the arguments_handler.rs file
*/

/*
    Get the exe file name

    Input --> None
    Return --> String that holds the exe file name
*/
pub fn get_exec_file_name() -> String {
    
    let exe_path: std::path::PathBuf = std::env::current_exe().expect("Failed to get current executable path");
    let exe_name: &std::ffi::OsStr = exe_path.file_name().expect("Failed to get executable name");
    let str_exe_name: &str = exe_name.to_str().expect("Failed to convert executable name to string");

    return str_exe_name.to_owned();
}

/*
    Helper fucntion to check the existence of an option and if it exists it check the option passed value

    Input:
        1) arguments: arguments from the stdin command line
        2) option1: represt the -<char> option alias
        3) option2: represt the --<option-name> alias
    Return --> the value of the option the user have entered
*/
pub fn check_existence_of_option(arguments: &Vec<String>, option1: &str, option2: &str) -> String {
    
    let mut option_choice_value: String = String::new();

    // Check the existence of the option alias
    if arguments.iter().any(|s: &String| s == option1 || s == option2) {
        
        let mut option_choice_index: usize = 0;
        
        // Get the index of the user choice of the option
        if let Some(option_index) = arguments.iter().position(|value: &String| value == option1 || value == option2) {
            option_choice_index = option_index + 1;
        }

        if option_choice_index != 0 && option_choice_index < arguments.len() {
            option_choice_value.push_str(arguments[option_choice_index].as_str());
        }

    }

    return option_choice_value;
}

/*
    List all the available browsers on the system which are located in:
        1) /usr/bin/
        2) /usr/local/bin
        3) /snap/bin/
    Input --> None
    Return --> Vec<String> which holds all the available browsers full path of the system
*/
pub fn list_available_browsers() -> Vec<String> {
    
    let mut available_browsers: Vec<String> = Vec::new();

    // Directories to search in for the browser binary
    let directories_to_search: Vec<&str> = vec!["/usr/bin/", "/usr/local/bin/", "/snap/bin/"];

    // Vector that holds the names of the browsers the program will look for
    let browsers_names: Vec<&str> = vec!["firefox", "chrome", "brave", "chromium"];

    for browser in browsers_names {
        for dir in &directories_to_search {
            let path: String = format!("{}{}", dir, browser);
            // Check if the browser binary exists
            if std::fs::metadata(&path).is_ok() {
                available_browsers.push(path)
            }
        }
    }

    if available_browsers.len() < 1 {
        println!("[!] No Browsers where found!");
        return Vec::new();
    } else {
        return available_browsers;
    }

}

/*
    Function to check if the search engine that the user provided exists in the available_search_engines array
    Input --> search_engine: name of the search engine provided by the user
    Return --> true if found the search engine or false if not
*/
pub fn search_engine_option(search_engine: &String) -> bool {
    // Available search engines vector
    let available_search_engines: [String; 6] = ["google".to_string(), "duckduckgo".to_string(), 
                                  "bing".to_string(), "yahoo".to_string(), 
                                  "yandex".to_string(), "brave".to_string()];

    if !available_search_engines.contains(search_engine) {

        println!("[!] Sorry {} search engine is not supported! google search engine will be used instead.\n", search_engine);
        return false;
    }

    return true;
}