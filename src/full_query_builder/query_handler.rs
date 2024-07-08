/*
    This file handle and format all the or operators, excluded domains, full rules options content
*/

mod query_helper_functions;

/* 
    Function to handle: intitle, inurl, intext and filetype operators file content
    Input --> provided file path as a String and the or operator as a String
    Return --> String: the fomated content of the file provided for the option
    Calls:
        1) query_helper_functions::read_file(): to read a file
        2) query_helper_functions::end_file_content_handler(): to handle the endline in each file
*/
pub fn or_operator_handler(file_path: String, operator: String) -> String {

    if file_path.is_empty() {

        return String::new();
    }

    let mut operator_file_content: String = query_helper_functions::read_file(file_path.clone());

    if operator_file_content.is_empty() {

        println!("[!] {} is empty", file_path);
        return String::new();
    }

    let mut operator_query: String = operator;
    
    operator_query.push_str(":\"");

    operator_file_content = query_helper_functions::end_file_content_handler(operator_file_content);

    operator_file_content = operator_file_content.replace("\n", "\" or \"");
    // add the last " in the inurl content
    operator_file_content.push_str("\"");

    operator_query.push_str(&operator_file_content.to_string());

    return operator_query; 
}

/*
    Function that handle and format the excluded domains option provided file content
    Input --> the file path as String
    Return --> String: which holds the formated file content
    Calls:
        1) query_helper_functions::read_file(): to read a file
        2) query_helper_functions::end_file_content_handler(): to handle the endline in each file
*/
pub fn excluded_domains_handler(file_path: String) -> String {

    if file_path.is_empty() {

        return String::new();
    }

    let mut excluded_domains_file_content: String = query_helper_functions::read_file(file_path.clone());

    if excluded_domains_file_content.is_empty() {
        
        println!("[!] {} is empty", file_path);
        return String::new();
    }

    let mut excluded_domains_query: String = "-".to_string();

    excluded_domains_file_content = query_helper_functions::end_file_content_handler(excluded_domains_file_content);

    excluded_domains_file_content = excluded_domains_file_content.replace("\n", " -");

    excluded_domains_query.push_str(&excluded_domains_file_content.to_string());

    return excluded_domains_query;
}

/*
    Function to handle and format the provided rules file content
    Input --> the file path as a String
    Return --> String: holds the formated file content
    Calls:
        1) query_helper_functions::read_file(): to read a file
        2) query_helper_functions::end_file_content_handler(): to handle the endline in each file
        3) query_helper_functions::rearrange_rules_file_elements(): used to put the excluded domains at the end of the returned String
*/
pub fn rules_handler(file_path: String) -> String {

    if file_path.is_empty() {

        return String::new();
    }

    let mut rules_file_content: String = query_helper_functions::read_file(file_path.clone());
    
    // Check if the rules file is empty
    if rules_file_content.is_empty() {

        println!("[!] {} is empty!", file_path);
        return String::new();
    }

    let mut rules_query: String = String::new();

    rules_file_content = query_helper_functions::end_file_content_handler(rules_file_content);

    rules_file_content = rules_file_content.replace("\n", " ");
    // Put the excluded domains operators in the end of the query string
    rules_file_content = query_helper_functions::rearrange_rules_file_elements(rules_file_content.clone());
    
    rules_query.push_str(&rules_file_content.to_string());

    return rules_query;
}