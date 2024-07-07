fn check_os_type() -> &'static str{

    if cfg!(target_os = "windows") {
        return "windows";
    } else if cfg!(target_os = "linux") {
        return "linux";
    } else {
        return "Other";
    }

}

pub fn get_exec_file_name() -> String {
    
    let exe_path: std::path::PathBuf = std::env::current_exe().expect("Failed to get current executable path");
    let exe_name: &std::ffi::OsStr = exe_path.file_name().expect("Failed to get executable name");
    let str_exe_name: &str = exe_name.to_str().expect("Failed to convert executable name to string");

    return str_exe_name.to_owned();
}


pub fn check_existence_of_option(arguments: &Vec<String>, option1: &str, option2: &str) -> String {
    
    let mut option_choice_value: String = String::new();

    if arguments.iter().any(|s: &String| s == option1 || s == option2) {
        
        let mut option_choice_index: usize = 0;
        

        if let Some(option_index) = arguments.iter().position(|value: &String| value == option1 || value == option2) {
            option_choice_index = option_index + 1;
        }

        if option_choice_index != 0 && option_choice_index < arguments.len() {
            option_choice_value.push_str(arguments[option_choice_index].as_str());
        }

    }

    return option_choice_value;
}

pub fn list_available_browsers() -> Vec<String> {
    
    let mut available_browsers: Vec<String> = Vec::new();

    // Directories to search in for the browser binary
    let directories_to_search: Vec<&str> = if check_os_type() == "linux" {
        vec!["/usr/bin/", "/usr/local/bin/", "/snap/bin/"]
    } else if check_os_type() == "windows" {
        println!("[!] Sorry listing the browsers or running the browser with the query on windows are not avaliable right now!");
        println!("[!] You can use the -p option which will print the query instead.\n");
        std::process::exit(-1);
    } else {
        vec!["None"]
    };

    // Vector that holds the names of the browsers the program will look for
    let browsers_names: Vec<&str> = vec!["firefox", "chrome", "google-chrome", "brave", "chromium"];

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
        //TODO add the browser full path and store the path of the browser in a file to use it
    } else {
        return available_browsers;
    }

}

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