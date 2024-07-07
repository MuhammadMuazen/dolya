mod query_helper_functions;

// Function to handle: intitle, inurl, intext and filetype operators
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