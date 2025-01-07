use crate::screen::word as palabra;
use crate::screen::loader as loader;
use colored::*;


pub fn init() {
    println!("{}","Present items".underline());
    compare_word_with_element();
}

fn get_elements() -> String {
    let loaded_content = match loader::load_elements() {
       Ok(content) => content, 
       Err(_) => String::from("Test,test,when,is,not,found,the,file"),
    }; 
    String::from(loaded_content)
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
