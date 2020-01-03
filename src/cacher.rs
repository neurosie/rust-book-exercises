use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

/*
Try modifying Cacher to hold a hash map rather than a single value. The keys of
the hash map will be the arg values that are passed in, and the values of the
hash map will be the result of calling the closure on that key. Instead of
looking at whether self.value directly has a Some or a None value, the value
function will look up the arg in the hash map and return the value if it’s
present. If it’s not present, the Cacher will call the closure and save the
resulting value in the hash map associated with its arg value.

The second problem with the current Cacher implementation is that it only
accepts closures that take one parameter of type u32 and return a u32. We might
want to cache the results of closures that take a string slice and return usize
values, for example. To fix this issue, try introducing more generic parameters
to increase the flexibility of the Cacher functionality.
*/

pub struct Cacher<F, A, R>
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
    pub fn new(calculation: F) -> Cacher<F, A, R> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: A) -> &R {
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

    #[test]
    fn tuple_argument() {
        let mut c =
            Cacher::new(|(x, b): (i32, bool)| if b { x + 2 } else { x + 1 });
        assert_eq!(*c.value((1, false)), 2);
    }
}
