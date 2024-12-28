use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left_words, right_word) = parse_equation(input)?;

    let all_letters = gather_letters(&left_words, right_word);

    if all_letters.len() > 10 {
        return None;
    }

    let leading = leading_letters(&left_words, right_word);

    let used_digits = HashSet::new();
    let assignments = HashMap::new();

    backtrack(
        &all_letters,
        0,
        &leading,
        &used_digits,
        &assignments,
        &left_words,
        right_word,
    )
}

fn backtrack(
    letters: &[char],
    index: usize,
    leading: &HashSet<char>,
    used: &HashSet<u8>,
    current_assignment: &HashMap<char, u8>,
    left_words: &[String],
    right_word: &str,
) -> Option<HashMap<char, u8>> {
    match index == letters.len() {
        true => match check_sum(left_words, right_word, current_assignment) {
            true => Some(current_assignment.clone()),
            false => None,
        },
        false => {
            let letter = letters[index];

            for digit in 0..=9 {
                if used.contains(&digit) || (leading.contains(&letter) && digit == 0) {
                    continue;
                }

                let mut new_assignment = current_assignment.clone();
                new_assignment.insert(letter, digit);

                let mut new_used = used.clone();
                new_used.insert(digit);

                if let Some(solution) = backtrack(
                    letters,
                    index + 1,
                    leading,
                    &new_used,
                    &new_assignment,
                    left_words,
                    right_word,
                ) {
                    return Some(solution);
                }
            }

            None
        }
    }
}

fn check_sum(left_words: &[String], right_word: &str, assignments: &HashMap<char, u8>) -> bool {
    left_words
        .iter()
        .map(|w| word_value(w, assignments))
        .sum::<u64>()
        == word_value(right_word, assignments)
}

fn word_value(word: &str, assignments: &HashMap<char, u8>) -> u64 {
    word.chars().fold(0_u64, |acc, c| {
        acc * 10 + *assignments.get(&c).expect("Unexpected letter encountered") as u64
    })
}

fn leading_letters(left_words: &[String], right_word: &str) -> HashSet<char> {
    let mut leading = left_words
        .iter()
        .filter_map(|w| w.chars().next())
        .collect::<HashSet<char>>();

    if let Some(ch) = right_word.chars().next() {
        leading.insert(ch);
    }

    leading
}

fn gather_letters(left_words: &[String], right_word: &str) -> Vec<char> {
    let mut letters = HashSet::new();

    for word in left_words {
        letters.extend(word.chars());
    }
    letters.extend(right_word.chars());

    letters.into_iter().collect()
}

fn parse_equation(input: &str) -> Option<(Vec<String>, &str)> {
    let (lhs, rhs) = input.split_once("==")?;
    let right_word = rhs.trim();

    let left_words = lhs
        .split('+')
        .map(|chunk| chunk.trim().to_string())
        .collect::<Vec<String>>();

    match left_words.is_empty() || right_word.is_empty() {
        true => None,
        false => Some((left_words, right_word)),
    }
}
