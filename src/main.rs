//bring hashmap in the scope
use std::collections::{HashMap};

fn vowel_to_hash(strg: &str) -> HashMap<char,bool>{

// create an array of vowels
    let vowel = ['a','e','i','o','u', 'A','E','I','O','U'];
   

    let mut map = HashMap::with_capacity(vowel.len());
    for v in vowel{
        map.insert(v,true);
    }
    map
}

fn pig_latin_word(word: &str, vowel_map:HashMap<char,bool>) -> String{
    //get the first letter
    let mut chars = word.chars();
    let first_word = match chars.next() {
        Some(c) => c,
        None => return String::new(),      
    };

    //if it doesnt start with a string leave at it is
    if !first_word.is_alphabetic(){
        return word.to_string();
    }

    //if first word is vowel
    if vowel_map.contains_key(&first_word){
        return format!("{word}-hay");
    } else {
        let first_len = first_word.len_utf8();
        let rest_word = &word[first_len..];
        return format!("{rest_word}-{}ay", first_len);
    }

}


fn main() {
    // practice the function
    // let word = "manana";
    
    // let first = word.chars().next().unwrap();
    // let first_word = first.len_utf8();
    // println!("{:?} {}",first,first_word);


}
