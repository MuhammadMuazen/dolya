/*
    Helper functions that are called in full_query_builder.rs
*/


/*
    Function that initilize the query String with the wanted search engine
    Input --> String: search_engine_name
    Return --> String that is equal to the wanted search engine URI
*/
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

/*
    Function used to concatenate all the operators of the Google dorks
    Input --> String: content of the file that holds the dorks
    Return --> String: the full rules in one String
*/
pub fn adding_operators_files(rules_file_conent: String) -> String {

    let mut content_start_with_space: String = " ".to_string();

    if rules_file_conent.is_empty() {
        
        return String::new();
    }

    content_start_with_space.push_str(&rules_file_conent.to_string());

    return content_start_with_space;
}