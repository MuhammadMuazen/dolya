/*
    Responsible for building the full query which will look like
    http://<search_engine>/<path> <query> [options_content]
*/

mod full_query_helper_functions;
mod query_handler;

/*
    Function to build the full query
    Input: arguments Vec<String> which represent the user proivded arguments like this:
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
    Return --> String that represent the full query
    Calls:
        1) full_query_helper_functions::intilize_query() --> to make the query starts with the wanted search engine URI
        2) query_handler::rules_handler() --> handle the full rules file
        3) or_operator_handler() --> handle inurl, intitle, intext, filetype option
        4) excluded_domains_handler() --> handle the excluded domains option
        5) full_query_helper_functions::adding_operators_files() --> add the content of the files to the final query
*/
pub fn full_query(arguments: Vec<String>) -> String {

    // Choose the search engine
    let mut final_query: String = if !full_query_helper_functions::intilize_query(
        arguments[1].to_string()).is_empty() {
            full_query_helper_functions::intilize_query(arguments[1].to_string())
        } else {
            String::new()
        };
    
    if final_query.is_empty() {

        println!("[!] Something went wrong while building the full query!");
        return String::new();
    }

    // Add the search query to the full query
    final_query.push_str(&arguments[0].to_string());

    // Adding rules from the rules file content
    if !arguments[8].is_empty() {
        
        final_query.push_str(&" ".to_string());
        final_query.push_str(&query_handler::rules_handler(arguments[8].to_string()));
        
        return final_query;
    }

    // Adding inurl file content
    final_query.push_str(&full_query_helper_functions::adding_operators_files(
        query_handler::or_operator_handler(arguments[3].to_string(), "inurl".to_string())));
    // Adding intitle file content
    final_query.push_str(&full_query_helper_functions::adding_operators_files(
        query_handler::or_operator_handler(arguments[5].to_string(), "intitle".to_string())));
    // Adding intext file content
    final_query.push_str(&full_query_helper_functions::adding_operators_files(
        query_handler::or_operator_handler(arguments[6].to_string(), "intext".to_string())));
    // Adding filetype file content
    final_query.push_str(&full_query_helper_functions::adding_operators_files(
        query_handler::or_operator_handler(arguments[7].to_string(), "filetype".to_string())));
    // Adding the excluded domains file content
    final_query.push_str(&full_query_helper_functions::adding_operators_files(
        query_handler::excluded_domains_handler(arguments[4].to_string())));

    final_query.push_str(&"\'".to_string());

    return final_query;
}