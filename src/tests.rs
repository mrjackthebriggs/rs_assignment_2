#[cfg(test)]
mod tests{
    use crate::dictionary;
    use crate::word::{self, Word};
    
    #[test]
    fn word_colone(){
            let new_word:Word;
            {
                let test_word:Word = Word::new("pissing","v", "where you pee");
                new_word = test_word.clone();
            }

            assert_eq!(new_word.word.unwrap(),"pissing");       
    }

    #[test]
    fn dictionary_constrcut(){
        
    }
}
