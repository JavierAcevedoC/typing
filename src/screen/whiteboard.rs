use crate::screen::word as palabra;
use colored::*;


pub fn init() {
    println!("{}","Present items".underline());
    compare_word_with_element();
}

fn get_elements() -> String {
    String::from("Hola,mi,nombre,es,javier")
}

fn compare_word_with_element() {
    let elements = get_elements();
    let elm = elements.split(',');

    let total_word = String::from(elm.count().to_string());

    println!("Total Words - {}", total_word.blue().bold());

    for element in elements.split(',') {
        println!("-> {}", element.yellow().bold());
        let word = palabra::read_word_line();
        palabra::get_diff(word.as_str(), element);
    }
}
