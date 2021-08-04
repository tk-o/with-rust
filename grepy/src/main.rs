use std::collections::HashMap;
use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::ops::Deref;

fn main() {
    let search_term = "are";
    let quote = THE_QUOTE;

    let mut grepy = Grepy::new();
    let matches = grepy.find_matches(&search_term, &quote);

    println!("Matches: {:?}", matches);
}

#[derive(Default)]
struct Grepy<'a> {
    /// Matches: exact lines
    simple_matches: HashMap<u32, &'a str>,
    /// Matches: exact lines and their surrounding lines
    extended_matches: HashMap<u32, Vec<&'a str>>,
}

impl<'a> Grepy<'a> {
    fn new() -> Self {
        Default::default()
    }

    fn find_matches(&mut self, needle: &'a str, haystack: &'a str) -> &HashMap<u32, &'a str> {
        let mut simple_matches = HashMap::new();

        self.find_matches_extended(needle, haystack, 0).iter().for_each(|(k, v)| {
            // FIXME: sort out the type for value
            simple_matches.insert(k.to_owned(), v.get(0).unwrap());
        });

        self.simple_matches = simple_matches;

        self.simple_matches.borrow()
    }

    fn find_matches_extended(&mut self, needle: &'a str, haystack: &'a str, surrounding_lines_count: usize) -> &HashMap<u32, Vec<&'a str>> {
        let mut extended_matches = HashMap::new();
        let mut extended_line_ranges: Vec<(u32, u32)> = Vec::new();

        for (i, line) in haystack.lines().enumerate() {
            let line_extended_matched = Vec::with_capacity(surrounding_lines_count);

            if line.contains(&needle) {
                let line_number = i as u32 + 1;
                extended_matches.insert(line_number, line_extended_matched);
            }
        }

        self.extended_matches = extended_matches;

        self.extended_matches.borrow()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_finds_results_by_exact_match_when_available() {
        let mut grepy = Grepy::new();
        let matches = grepy.find_matches("are", THE_QUOTE);

        // check number of matches
        assert_eq!(matches.len(), 3);

        // check a match contents and line
        assert_eq!(
            matches.get(&9).unwrap(),
            &"the issue is a complex one, that there are many factors to be considered,",
        );

        assert_eq!(
            grepy.find_matches("ambitious", THE_QUOTE).len(),
            1,
        );
    }

    #[test]
    fn it_finds_no_results_by_when_not_available() {
        let mut grepy = Grepy::new();
        let matches = grepy.find_matches("are not", THE_QUOTE);

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn it_includes_surrounding_lines_for_context_when_needed() {
        let mut grepy = Grepy::new();

        let matches = grepy.find_matches_extended("example", THE_QUOTE);

        assert_eq!(matches.len(), 0);

    }
}

/// Source: http://www.paulgraham.com/useful.html
const THE_QUOTE: &str = "\
What should an essay be?

Many people would say persuasive. That's what a lot of us were taught essays should be.
But I think we can aim for something more ambitious: that an essay should be useful.

To start with, that means it should be correct. But it's not enough merely to be correct.
It's easy to make a statement correct by making it vague. That's a common flaw in academic writing,
for example. If you know nothing at all about an issue, you can't go wrong by saying that
the issue is a complex one, that there are many factors to be considered,
that it's a mistake to take too simplistic a view of it, and so on.

Though no doubt correct, such statements tell the reader nothing. Useful writing makes claims that
are as strong as they can be made without becoming false.

For example, it's more useful to say that Pike's Peak is near the middle of Colorado than
merely somewhere in Colorado. But if I say it's in the exact middle of Colorado,
I've now gone too far, because it's a bit east of the middle.

Precision and correctness are like opposing forces.
It's easy to satisfy one if you ignore the other.
The converse of vaporous academic writing is the bold, but false, rhetoric of demagogues.
Useful writing is bold, but true.";
