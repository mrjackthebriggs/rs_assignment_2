use crate::word::Word;
use crate::word;
use std::{fs::File, io::{Read, Write}};

pub struct Dictionary{
    dict:Vec<Word>,
    file_loc:String,
    words_loaded:u32
}

impl Dictionary{
    /*
    Function for loading the words from a text document and turning it into a vector of Word objects
    takes a reference to the "dictionary" and the file location, returns a boolean.
    Notes:
    - Reads each word object
    - Had issues with reading each line and constructing an object
    Used Some/None and checked if it had None to assign a value
    */
    pub fn new(load_location:impl ToString) -> Dictionary{
       
        let mut temp_dict:Dictionary = Dictionary{
             dict: Vec::new(), 
             file_loc: load_location.to_string(), 
             words_loaded: 0 
            };

        match temp_dict.load_dictionary(load_location){
            Ok(count) => temp_dict.words_loaded = count,
            Err(msg) => println!("\nERROR: Dictionary load failed, {}\n", msg)
        };

        return temp_dict;
    }

    fn load_dictionary(& mut self, load_loc:impl ToString) -> Result<u32,String>
    {
        let load_str = load_loc.to_string();

        let mut file_ob = match File::open(load_str){
            Ok(the_file) => the_file,
            Err(_) => return Err(String::from("File could not be loaded"))
        };

        //Read lines
        let mut file_str:String = String::new();
        file_ob.read_to_string(& mut file_str);
        let mut word_buf:Word = Word::new_blank();
        for lin in file_str.lines()
        {     
            
            if lin == "<word>"
            {
                word_buf = Word::new_blank();
                continue;
            }

            else if lin == "</word>"
            {
                self.dict.push(word_buf);
                word_buf = Word::new_blank();   //needs to be here for memeory safety
                continue;
            }

            //checks if blank, if so, overwrites with actual value
            if word_buf.word == None
            {
                word_buf.word = Some(String::from(lin));
                continue;
            }

            else if word_buf.def == None
            {
                word_buf.def = Some(String::from(lin));
                continue;
            }  

            else if word_buf.word_type == None
            {
                word_buf.word_type = Some(word::string_to_type(lin).expect("Invalid input from .txt doc"));
                continue;
            }     
        }

        //if no words were loaded, it probably failed
        if self.dict.len() > 0 {
            Ok(self.dict.len().try_into().unwrap())
        }
        else {
            Err(String::from("No objects loaded"))
        }   
    }

    /* 
    Finds the word using linear searching, unless char is remapped, binary searching is not possible
    Returns a result
    Notes:
    - learned the use of as_ref, dunno why I can't just use '&' here
    */
    pub fn find_word(&self, search: impl ToString) -> Result<Word,String>
    {
        let search_str = search.to_string().to_lowercase();

        for dict_word in &self.dict
        {
            if dict_word.word.as_ref().unwrap() == &search_str
            {
                return Ok(dict_word.clone());
            }
        }

        return Err(String::from("Word not found"));
    }

    /*
    Returns a vector of words in the dictionary that contains more than three z's
    Notes:
    - Rusts built in methods are a life saver, so easy to work with!
    */
    pub fn mt3z_words(&self) -> Vec<String>
    {
        let mut ret_vec:Vec<String> = Vec::new();

        for dict_word in &self.dict
        {
            let mut z_count:u16 = 0;
            for str_char in dict_word.word.as_ref().unwrap().chars()
            {
                if str_char == 'z'
                {
                    z_count += 1;
                }
            }

            if z_count > 3
            {
                ret_vec.push(dict_word.word.as_ref().unwrap().clone());
            }
        }

        return ret_vec;
    }

    /*
    Adds a word object to the dictionary and saves it as a .txt doc
    Requires a mut ref for the dict, a word obj to add and a destination to save the .txt file.
    Notes:
    - as_ref.unwrap().clone().etc. These long method chains are cool, and efficient and all, wish there were better ways to do this
    - enuming the word types came in clutch here
    */
    pub fn add_word(& mut self, in_word:Word, dest: impl ToString) -> Result<String, String>
    {
        //does word already exist?
        match self.find_word(in_word.word.as_ref().unwrap())
        {
            Ok(_wrd) => return Err(String::from("Word already exists")),
            Err(_er) => self.dict.push(in_word)
        }

        //now new word added, prepare string to write
        let mut write_str:String = String::new();

        for wrd in &self.dict
        {
            write_str.push_str("<word>\n");

            write_str.push_str(wrd.word.as_ref().unwrap().clone().as_str());
            write_str.push_str("\n");

            write_str.push_str(wrd.def.as_ref().unwrap().clone().as_str());
            write_str.push_str("\n");

            let type_word:&str = match wrd.word_type.as_ref().unwrap()
            {
                word::WordType::Noun => "n",
                word::WordType::Verb => "v",
                word::WordType::Adv => "adv",
                word::WordType::Adj => "adj",
                word::WordType::Prep => "prep",
                word::WordType::Misc => "misc",
                word::WordType::PropNoun => "pn",
                word::WordType::NounVerb => "n_and_v",
            };
            write_str.push_str(type_word);
            write_str.push_str("\n");

            write_str.push_str("</word>\n");
        }

        let mut file_dest = dest.to_string();

        let mut file = match File::create(file_dest.clone())
        {
            Ok(fl) => fl,
            Err(_er_str) => return Err(String::from("Failed to create new dict file"))
        };

        file.write_all(write_str.as_bytes());

        Ok(file_dest)
    }
}