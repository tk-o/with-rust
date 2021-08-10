use anyhow::Result;
use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{borrow::Borrow, collections::BTreeMap};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args = App::new("grepy")
        .version(env!("CARGO_PKG_VERSION"))
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .about("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .about("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let arg_input = args.value_of("input").expect("arg must be provided");
    let input_file = File::open(arg_input)?;
    let reader = BufReader::new(input_file);

    let arg_pattern = args.value_of("pattern").expect("arg must be provided");
    let search_term = Regex::new(arg_pattern)?;
    let grepy_needle = GrepyNeedle::Regex(&search_term);

    for (line_idx, line_) in reader.lines().enumerate() {
        let line = line_?;
        let mut grepy = Grepy::new();
        let matches = grepy.find_matches(&grepy_needle, &line);

        if matches.is_empty() == false {
            println!("#{}: {}", line_idx, line);
        }
    }

    Ok(())
}

enum GrepyNeedle<'a> {
    PlainText(&'a str),
    Regex(&'a Regex),
}

#[derive(Default)]
struct Grepy<'a> {
    /// Matches: exact lines
    matches: BTreeMap<usize, &'a str>,
}

impl<'a> Grepy<'a> {
    fn new() -> Self {
        Default::default()
    }

    fn find_matches(
        &mut self,
        needle: &'a GrepyNeedle<'a>,
        haystack: &'a str,
    ) -> &BTreeMap<usize, &'a str> {
        self.find_matches_extended(needle, haystack, 0)
    }

    fn find_matches_extended(
        &mut self,
        needle: &'a GrepyNeedle<'a>,
        haystack: &'a str,
        surrounding_lines_count: usize,
    ) -> &BTreeMap<usize, &'a str> {
        let list_of_matches: BTreeMap<_, _> = haystack
            .lines()
            .enumerate()
            // first, let's see which lines are relevant for the matching operation
            .filter_map(|(line_idx, line)| {
                let no_match_in_current_line = match needle {
                    GrepyNeedle::PlainText(needle) => line.contains(needle) == false,
                    GrepyNeedle::Regex(regex) => (*regex).find(line).is_none(),
                };

                if no_match_in_current_line {
                    return None;
                }

                let matched_line = match surrounding_lines_count {
                    0 => vec![line_idx],
                    _ => {
                        let line_from_idx = line_idx.saturating_sub(surrounding_lines_count);
                        let line_to_idx = line_idx.saturating_add(surrounding_lines_count);

                        (line_from_idx..=line_to_idx).collect::<Vec<_>>()
                    }
                };

                Some(matched_line)
            })
            .flatten()
            // and then get the relevant lines from text
            .filter_map(|line_idx| match haystack.lines().nth(line_idx) {
                None => None,
                Some(line) => Some((line_idx, line)),
            })
            .collect();

        self.matches = list_of_matches;

        self.matches.borrow()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_finds_results_by_exact_match_when_available() {
        let mut grepy = Grepy::new();
        let grepy_needle = GrepyNeedle::PlainText("are");
        let matches = grepy.find_matches(&grepy_needle, THE_QUOTE);

        // check number of matches
        assert_eq!(matches.len(), 3);

        // check a match contents and line
        assert_eq!(
            matches.get(&8).unwrap(),
            &"the issue is a complex one, that there are many factors to be considered,",
        );

        let grepy_needle = GrepyNeedle::PlainText("ambitious");

        assert_eq!(grepy.find_matches(&grepy_needle, THE_QUOTE).len(), 1,);
    }

    #[test]
    fn it_finds_no_results_by_when_not_available() {
        let mut grepy = Grepy::new();
        let grepy_needle = GrepyNeedle::PlainText("are not");
        let matches = grepy.find_matches(&grepy_needle, THE_QUOTE);

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn it_includes_surrounding_lines_for_context_when_needed() {
        let mut grepy = Grepy::new();
        let surrounding_lines_count = 1;
        let grepy_needle = GrepyNeedle::PlainText("example");

        let matches =
            grepy.find_matches_extended(&grepy_needle, THE_QUOTE, surrounding_lines_count);

        let matched_lines: String = matches
            .iter()
            .map(|(_, s)| &**s)
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(
            matched_lines,
            "\
It's easy to make a statement correct by making it vague. That's a common flaw in academic writing,
for example. If you know nothing at all about an issue, you can't go wrong by saying that
the issue is a complex one, that there are many factors to be considered,

For example, it's more useful to say that Pike's Peak is near the middle of Colorado than
merely somewhere in Colorado. But if I say it's in the exact middle of Colorado,"
        );
    }

    #[test]
    fn it_finds_results_by_when_provided_with_regex() {
        let mut grepy = Grepy::new();
        let search_term = Regex::new(r"(?mUi)co[\w]+(n|r)").unwrap();
        let grepy_needle = GrepyNeedle::Regex(&search_term);
        let matches = grepy.find_matches(&grepy_needle, THE_QUOTE);

        assert_eq!(matches.len(), 9);
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
