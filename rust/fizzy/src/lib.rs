/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    matcher: Box<dyn Fn(T) -> bool + 'a>,
    subs: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F: Fn(T) -> bool + 'a, S: ToString>(matcher: F, subs: S) -> Matcher<'a, T> {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T: 'a> Fizzy<'a, T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }
}

impl<'a, T> Fizzy<'a, T>
    where
        T: ToString + Copy + 'a,
{
    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: IntoIterator<Item=T> + 'a>(self, iter: I) -> impl Iterator<Item=String> + 'a {
        iter.into_iter().map(move |item| {
            let res: String = self.matchers
                .iter()
                .filter(|m| (m.matcher)(item))
                .map(|m| m.subs.as_str())
                .collect();
            if res.is_empty() {
                item.to_string()
            } else {
                res
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T: PartialEq + std::ops::Rem<Output=T> + From<u8> + 'a>() -> Fizzy<'a, T> {
    Fizzy::new().add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}

impl<'a, T: 'a> Default for Fizzy<'a, T> {
    fn default() -> Self {
        Fizzy::new()
    }
}
