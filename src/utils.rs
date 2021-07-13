use std::collections::HashMap;
use pad::PadStr;


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

pub fn print_as_lines_with_context(data: HashMap<String, Option<String>>, properties: &[&str], pad_length: Option<usize>) {
    // Determine the longest length of the properties given.
    let mut longest_property_length: usize = 0;

    if pad_length.is_none() {
        for property in properties {
            if property.len() > longest_property_length {
                longest_property_length = property.len()
            }
        }
    } else {
        longest_property_length = pad_length.unwrap();
    }

    for property in properties {
        let property_value = data.get(property.to_owned()).unwrap();
        println!("  {} :  {}", property.pad_to_width(longest_property_length), property_value.clone().unwrap_or_default());
    }
}