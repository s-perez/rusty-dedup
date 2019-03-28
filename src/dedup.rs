use crate::parser::Parser;
use log::{info, error};
use radix_trie::{Trie};


pub fn deduplicate(parser: &Parser) -> Result<bool, &'static str> {
    let mut trie: Trie<String, bool> = Trie::new();

    while !parser.depleted() {
        let element = parser.next();
        match trie.get(&(&element).hash.to_string()) {
            Some(_) => (),
            None => {
                parser.notify(&element);
                trie.insert((&element).hash.to_string(), true);
                ()
            }
        };
    };

    Ok(true)
}
