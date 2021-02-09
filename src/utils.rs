use std::collections::HashMap;


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

fn get_tabs_for_key<'a>(len: usize) -> &'a str {
    if len < 8 {
        return "\t\t\t\t";
    } else if len > 16 {
        return "\t\t";
    }

    return "\t\t\t";
}