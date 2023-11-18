#![allow(unused)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    num::NonZeroUsize,
};

use nlprule::{rules_filename, tokenizer_filename, Rules, Tokenizer};

pub struct GrammarChecker {
    tokenizer: Tokenizer,
    rules: Rules,
    cache: lru::LruCache<u64, Vec<nlprule::types::Suggestion>>,
}

impl std::fmt::Debug for GrammarChecker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("GrammarChecker")
    }
}

impl GrammarChecker {
    pub fn new() -> Self {
        let mut tokenizer_bytes: &'static [u8] =
            include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("en")));
        let mut rules_bytes: &'static [u8] =
            include_bytes!(concat!(env!("OUT_DIR"), "/", rules_filename!("en")));

        let tokenizer =
            Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is valid");
        let rules = Rules::from_reader(&mut rules_bytes).expect("rules binary is valid");
        let cache = lru::LruCache::new(NonZeroUsize::new(32).unwrap());
        Self {
            tokenizer,
            rules,
            cache,
        }
    }

    pub fn suggest(&mut self, text: &str) -> Vec<nlprule::types::Suggestion> {
        let text_hash = hash_str(text);
        if let Some(suggestions) = self.cache.get(&text_hash) {
            return suggestions.clone();
        }
        let suggestions = self.rules.suggest(text, &self.tokenizer);
        self.cache.put(text_hash, suggestions.clone());
        suggestions
    }
}

fn hash_str(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}
