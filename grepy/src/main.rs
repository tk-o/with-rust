use std::collections::HashMap;

fn main() {
    let search_term = "are";
    let quote = THE_QUOTE;

    let matches = Grepy::find_matches(&search_term, &quote);

    println!("Matches: {:?}", matches);
}

struct Grepy;

impl Grepy {
    fn find_matches<'a>(needle: &'a str, haystack: &'a str) -> HashMap<u32, &'a str> {
        let mut matches = HashMap::new();

        for (i, line) in haystack.lines().enumerate() {
            if line.contains(&needle) {
                let line_number = i as u32 + 1;
                matches.insert(line_number, line);
            }
        }

        matches
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_finds_results_by_exact_match_when_available() {
        assert_eq!(
            Grepy::find_matches("are", THE_QUOTE).len(),
            3,
        );

        assert_eq!(
            Grepy::find_matches("ambitious", THE_QUOTE).len(),
            1,
        );
    }

    #[test]
    fn it_finds_no_results_by_when_not_available() {
        let matches = Grepy::find_matches("are not", THE_QUOTE);

        assert_eq!(matches.len(), 0);
    }
}

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
