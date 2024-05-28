
// -------------------------------------------
// 			Efficient Storage and Retrieval of Words 
// ------------------------------------------- 

use std::collections::HashMap;
#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            is_word: false,
            children: HashMap::new(),
        }
    }
}
#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &String) {
        let mut current = &mut self.root;
        for w in word.chars() {
            current = current.children.entry(w).or_insert(Node::new());
        }

        if !current.is_word {
            current.is_word = true;
        }
    }

    fn search(&self, word: &String) -> bool {
        let mut current = &self.root;
        for w in word.chars() {
            if current.children.get(&w).is_some() {
                current = current.children.get(&w).unwrap();
            } else {
                return false;
            }
        }
        current.is_word
    }
}

fn main() {
    let words = vec![
        "the", "a", "there", "answer", "any", "by", "bye", "their", "abc",
    ].into_iter().map(|x| String::from(x)).collect::<Vec<String>>();

    let mut d = WordDictionary::new();

    for i in 0..words.len() {
        d.insert(&words[i]);
    }
    println!(
        "Searching 'there' in the dictionary results: {}",
        d.search(&"ther".to_string())
    );
    // search for 'that' and finally ther
}
