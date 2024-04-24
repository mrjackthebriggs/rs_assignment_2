/*
These stand allone functions stay here due 
to their usefrulness with checking inputs
*/

//Word Type Enums for categorising words
#[derive(PartialEq)]    // SO == can be used on the wrapped enums
pub enum WordType{
Noun,
Verb,
Adv,
Adj,
Prep,
Misc,
PropNoun,
NounVerb,
}

//Converts a String to a WordType
pub fn string_to_type(str_in:impl ToString) -> Result<WordType,String>
{
    let input = str_in.to_string();

    if input == "n"{
        return Ok(WordType::Noun);
    }
    else if input == "v"{
        return Ok(WordType::Verb);
    }
    else if input == "adv"{
        return Ok(WordType::Adv);
    }
    else if input == "adj"{
        return Ok(WordType::Adj);
    }
    else if input == "prep"{
        return Ok(WordType::Prep);
    }
    else if input == "pn"{
        return Ok(WordType::PropNoun);
    }
    else if input == "n_and_v"{
        return Ok(WordType::NounVerb);
    }
    else if input == "misc"{
        return Ok(WordType::Misc);
    }
    else {
        return Err(String::from(format!("{} not valid, please input a valid type",input)));
    }

    

}

//Converts WordType to String
pub fn type_to_string(type_in:&WordType) -> String
{
    match type_in{
        WordType::Noun => String::from("noun"),
        WordType::Verb => String::from("verb"),
        WordType::Adv => String::from("adverb"),
        WordType::Adj => String::from("adjective"),
        WordType::Prep => String::from("preposition"),
        WordType::Misc => String::from("miscellaneous"),
        WordType::PropNoun => String::from("proper noun"),
        WordType::NounVerb => String::from("noun and verb"),
    }
}

/*
    Word structure for holding word information
    Holds Options because there are points the word object will be blank, needed for loading words.
*/
pub struct Word{
    pub word:Option<String>,
    pub word_type:Option<WordType>,
    pub def:Option<String>
}

impl Word{
    pub fn print_word(&self){
        if self.word.is_some(){
            println!("\n{}\n[{}]\n{}\n",self.word.as_ref().unwrap(),type_to_string(self.word_type.as_ref().unwrap()),self.def.as_ref().unwrap());
        }
    }

    pub fn new(word_in:impl ToString, word_type_in:impl ToString, def_in:impl ToString) -> Self{
        let str_word_in = word_in.to_string().to_lowercase();
        let str_word_type = word_type_in.to_string().to_lowercase();
        let str_word_def = def_in.to_string();
        Self{
            word:Some(str_word_in),
            word_type:Some(string_to_type(str_word_type).expect("Invalid input")),
            def:Some(str_word_def)
        }
    }

    pub fn new_blank() -> Self{
        Self{
            word:None,
            word_type:None,
            def:None
        }
    }
}

// Added to make handling Word easier
// Enables Word to use  .clone()
impl Clone for Word{
    fn clone(&self) -> Word{
        Self{
            word:self.word.clone(),
            word_type:{
                match self.word_type.as_ref(){
                    Some(srg) => match srg{
                        WordType::Adj => Some(WordType::Adj),
                        WordType::Misc => Some(WordType::Misc),
                        WordType::Adv => Some(WordType::Adv),
                        WordType::Noun => Some(WordType::Noun),
                        WordType::Verb => Some(WordType::Verb),
                        WordType::Prep => Some(WordType::Prep),
                        WordType::PropNoun => Some(WordType::PropNoun),
                        WordType::NounVerb => Some(WordType::NounVerb),
                    },
                    None => None
                }
            },
            def:self.def.clone()
        }
    }
}
