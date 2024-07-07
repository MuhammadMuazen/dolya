pub fn intilize_query(search_engine_name: String) -> String {
    
    let init_full_query: String = match search_engine_name.as_str() {
        "google" => "\'https://www.google.com/search?q=".to_string(),
        "duckduckgo" => "\'https://duckduckgo.com/?q=".to_string(),
        "bing" => "\'https://www.bing.com/search?q=".to_string(),
        "yahoo" => "\'https://search.yahoo.com/search?p=".to_string(),
        "yandex" => "\'https://yandex.com/search/?text=".to_string(),
        "brave" => "\'https://search.brave.com/search?q=".to_string(),
        _ => String::new()
    };

    return init_full_query;
}

pub fn adding_operators_files(rules_file_conent: String) -> String {

    let mut content_start_with_space: String = " ".to_string();

    if rules_file_conent.is_empty() {
        
        return String::new();
    }

    content_start_with_space.push_str(&rules_file_conent.to_string());

    return content_start_with_space;
}