use std::cmp::Eq;

use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<F, A, R>
where
    F: Fn(A) -> R,
    A: Eq + Hash + Copy,
{
    calculation: F,
    cache: HashMap<A, R>,
}

impl<F, A, R> Cacher<F, A, R>
where
    F: Fn(A) -> R,
    A: Eq + Hash + Copy,
{
    fn new(calculation: F) -> Cacher<F, A, R> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> &R {
        self.cache.entry(arg).or_insert((self.calculation)(arg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|&a| a);

        let _v1 = c.value(&1);
        let v2 = c.value(&2);

        assert_eq!(*v2, 2);
    }

    #[test]
    fn slice_size() {
        let mut c = Cacher::new(|s: &str| s.len());
        assert_eq!(*c.value("asdf"), 4 as usize);
    }
}
