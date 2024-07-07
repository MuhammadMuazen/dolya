// Function to read a file from the system
pub fn read_file(file_path: String) -> String {

    let file_content: String = std::fs::read_to_string(file_path)
        .expect("[!] File path is invalid!");

    return file_content;
}

// Function to put the excluded domains operators in the end of the query string in the rules_handler function
pub fn rearrange_rules_file_elements(rules_query: String) -> String {
    
    let elements: Vec<&str> = rules_query.split_whitespace().collect();
    let mut starts_with_dash: Vec<&str> = vec![];
    let mut other_elements: Vec<&str> = vec![];

    for element in elements.iter() {
        
        if element.starts_with('-') {
            
            starts_with_dash.push(*element);
        } 
        else {
            
            other_elements.push(*element);
        }
    }

    other_elements.extend(starts_with_dash);
    
    return other_elements.join(" ");
}

// Function to handle the \n conditions in the end of every file
pub fn end_file_content_handler(mut content: String) -> String {
    // If the file ends with two \n\n remove them but if it ends with one \n remove it only
    if content.len() >= 2 {
    
        let mut reversed_content: std::iter::Rev<std::str::Chars> = content.chars().rev();
        let last_char: char = reversed_content.next().unwrap();
        let second_to_last_char: char = reversed_content.next().unwrap();
        
        if last_char == '\n' && second_to_last_char == '\n' {
            content.pop();
            content.pop();
        }

        if last_char == '\n' && second_to_last_char != '\n' {
            content.pop();
        }
    }

    return content;
}