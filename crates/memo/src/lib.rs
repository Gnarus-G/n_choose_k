use std::{collections::HashMap, hash::Hash};

pub struct Memo<F: Fn(I) -> O, I, O>
where
    I: Eq + Hash + Clone,
    O: Clone,
{
    func: F,
    map: HashMap<I, O>,
}

impl<F: Fn(I) -> O, I, O> Memo<F, I, O>
where
    I: Eq + Hash + Clone,
    O: Clone,
{
    pub fn new(func: F) -> Memo<F, I, O> {
        Memo {
            func,
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, input: I) -> O {
        match self.map.get(&input) {
            Some(value) => value.clone(),
            None => {
                let value = (self.func)(input.clone());
                self.map.insert(input, value.clone());
                value
            }
        }
    }
}
