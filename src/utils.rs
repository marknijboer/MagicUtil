use std::{collections::HashMap, iter::FromIterator, process::exit};


pub fn print_as_json(data: HashMap<String, Option<String>>) {
    let json = serde_json::ser::to_string(&data).unwrap();
    println!("{}", json);
}

pub fn print_as_lines(data: HashMap<String, Option<String>>, properties: &[&str]) {
    for property in properties {
        let property_value = data.get(property.to_owned()).unwrap();
        println!("{}", property_value.clone().unwrap_or_default());
    }
}

pub fn print_as_lines_with_context(data: HashMap<String, Option<String>>, properties: &[&str]) {
    for property in properties {
        let property_value = data.get(property.to_owned()).unwrap();
        let tabs = get_tabs_for_key(property.len());
        println!("{}{}{}", property, tabs, property_value.clone().unwrap_or_default());
    }
}

pub fn get_wmic_output_as_list(wmic_output: Vec<u8>) -> Vec<String> {
    let output_string = String::from_utf8(wmic_output).unwrap();
    let output_option = output_string.split_once("\n");

    if output_option.is_none() {
        eprintln!("Could not find the status of the MagicINFO service");
        exit(1);
    }

    let output_line = output_option.unwrap().1;

    let mut match_list = Vec::new();
    let mut current_word: Vec<char> = Vec::new();

    // Iterate over the output letter by letter. Split the output in pairs of
    // characters separated by at least two spaces.
    for letter in output_line.chars() {

        // Leading whitespaces are ignored
        if letter.is_whitespace() && current_word.is_empty() {
            continue;
        }

        // If we encounter a whitespace while the last recorded character in the
        // current_word is also a whitespace, then we know we found a two-space
        // delimiter.
        if letter.is_whitespace() && current_word.last().unwrap().is_whitespace() {
            let match_string = String::from_iter(current_word.clone()).trim().to_owned();
            match_list.push(match_string);
            current_word.clear();
            continue
        }

        current_word.push(letter);
    }

    // Make sure we also get the last word if it contains something.
    if !current_word.is_empty() {
        let match_string = String::from_iter(current_word).trim().to_owned();
        if !match_string.is_empty() {
            match_list.push(match_string);
        }
    }

    return match_list;
}

fn get_tabs_for_key<'a>(len: usize) -> &'a str {
    if len < 8 {
        return "\t\t\t\t";
    } else if len > 16 {
        return "\t\t";
    }

    return "\t\t\t";
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_if_splitting_wmic_output_works() {
        use super::get_wmic_output_as_list;

        let output_string = "StartMode  StartName    State  \nAuto       LocalSystem  Running";
        let output_vector = Vec::from(output_string);

        let wmic_fields = get_wmic_output_as_list(output_vector);
        println!("{:#?}", wmic_fields);
        
        assert_eq!(wmic_fields[0], "Auto");
        assert_eq!(wmic_fields[1], "LocalSystem");
        assert_eq!(wmic_fields[2], "Running");
    }
}
