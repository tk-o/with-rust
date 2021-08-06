use std::borrow::Borrow;
use std::collections::HashMap;

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
    simple_matches: HashMap<usize, &'a str>,
    /// Matches: exact lines and their surrounding lines
    extended_matches: HashMap<usize, Vec<&'a str>>,
}

impl<'a> Grepy<'a> {
    fn new() -> Self {
        Default::default()
    }

    fn find_matches(&mut self, needle: &'a str, haystack: &'a str) -> &HashMap<usize, &'a str> {
        let mut simple_matches = HashMap::new();

        self.find_matches_extended(needle, haystack, 0)
            .iter()
            .for_each(|(k, v)| {
                simple_matches.insert(k.to_owned(), *v.get(0).unwrap());
            });

        self.simple_matches = simple_matches;

        self.simple_matches.borrow()
    }

    fn find_matches_extended(
        &mut self,
        needle: &'a str,
        haystack: &'a str,
        surrounding_lines_count: usize,
    ) -> &HashMap<usize, &'a str> {
        let mut matched_lines: Vec<GrepyMatch> = Vec::new();

        // first, let's see which lines are relevant for the matching operation
        for (i, line) in haystack.lines().enumerate() {
            if line.contains(&needle) {
                let matched_line = match surrounding_lines_count {
                    0 => GrepyMatch::SimpleMatch(i),
                    _ => GrepyMatch::ExtendedMatch(
                        i.saturating_sub(surrounding_lines_count),
                        i.saturating_add(surrounding_lines_count),
                    ),
                };

                matched_lines.push(matched_line);
            }
        }

        let lines_per_match = surrounding_lines_count * 2 + 1;

        let all_matching_lines: Vec<usize> = matched_lines
            .iter()
            .map(|matched_line| match *matched_line {
                GrepyMatch::SimpleMatch(line_idx) => {
                    vec![line_idx]
                }
                GrepyMatch::ExtendedMatch(line_idx_from, line_idx_to) => {
                    (line_idx_from..=line_idx_to).collect::<Vec<_>>()
                }
            })
            .flatten()
            .map()
            .collect();

        // let mut extended_matches: HashMap<usize, Vec<String>> =
        //     haystack.lines().enumerate().filter(|&(idx, _)| {});
        //     for matched_line in matched_lines.iter() {
        //         let mut output_lines = Vec::with_capacity(lines_per_match);
        //
        //         match *matched_line {
        //             GrepyMatch::SimpleMatch(line_idx) => {
        //                 if i == line_idx {
        //                     output_lines.push(line);
        //                     extended_matches.insert(line_idx, output_lines);
        //                 }
        //             }
        //             GrepyMatch::ExtendedMatch(line_idx_from, line_idx_to) => {
        //                 if i >= line_idx_from || i <= line_idx_to {
        //                     output_lines.push(line)
        //                 }
        //
        //                 if i == line_idx_to {
        //                     extended_matches
        //                         .insert(line_idx_to - surrounding_lines_count, output_lines);
        //                 }
        //             }
        //         }
        //     }
        // }

        // self.extended_matches = extended_matches;

        self.extended_matches.borrow()
    }
}

enum GrepyMatch {
    SimpleMatch(usize),
    ExtendedMatch(usize, usize),
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
            matches.get(&8).unwrap(),
            &"the issue is a complex one, that there are many factors to be considered,",
        );

        assert_eq!(grepy.find_matches("ambitious", THE_QUOTE).len(), 1,);
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
        let surrounding_lines_count = 1;

        let matches = grepy
            .find_matches_extended("example", THE_QUOTE, surrounding_lines_count)
            .clone();

        assert_eq!(matches.len(), 2);

        let lines_per_match = surrounding_lines_count * 2 + 1;

        matches.iter().for_each(|(_, v)| {
            assert!(v.len() <= lines_per_match);
            assert_eq!(v.capacity(), lines_per_match);
        });

        let first_match = matches.get(&8).unwrap();

        assert_eq!(
            format!("s1{:?}", first_match.join("")),
            "\
It's easy to make a statement correct by making it vague. That's a common flaw in academic writing,
for example. If you know nothing at all about an issue, you can't go wrong by saying that
the issue is a complex one, that there are many factors to be considered,"
        );
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
