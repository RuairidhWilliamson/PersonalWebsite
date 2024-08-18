use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct JobId {
    pub name: &'static str,
    pub args_hash: u64,
}

impl JobId {
    pub fn new(name: &'static str, args_hash: u64) -> Self {
        Self { name, args_hash }
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { name, args_hash } = self;
        f.write_fmt(format_args!("{name}/{args_hash:x}"))
    }
}

pub struct JobIdBuilder {
    pub name: &'static str,
    pub hasher: DefaultHasher,
}

impl JobIdBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            hasher: DefaultHasher::default(),
        }
    }

    #[must_use]
    pub fn arg<A>(mut self, arg: A) -> Self
    where
        A: Hash,
    {
        arg.hash(&mut self.hasher);
        self
    }

    pub fn build(self) -> JobId {
        JobId {
            name: self.name,
            args_hash: self.hasher.finish(),
        }
    }
}
