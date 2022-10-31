use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type PairStep = ((char, char), usize);

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let mut polymer_template: String = String::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if index == 0 {
            polymer_template = line;
        } else if index > 1 {
            let (pair, element) = line.splitn(2, " -> ").collect_tuple().unwrap();
            let pair_tuple: (char, char) = pair.chars().collect_tuple().unwrap();
            rules.insert(pair_tuple, element.chars().next().unwrap());
        }
    }

    // Part 1: take the quantity of the most common element and subtract the quantity of the least common element after 10 steps
    let steps = 10;
    let mut template = polymer_template.clone();
    for _ in 1..=steps {
        // Add the first char to the new template
        let mut new_template = template.chars().next().unwrap().to_string();
        // 1. Iterate all possible pairs in the current template
        for pair in template.chars().collect::<Vec<char>>().windows(2) {
            let element = rules.get(&(pair[0], pair[1])).unwrap();
            let polymer_part = format!("{}{}", element, &pair.last().unwrap());
            new_template.push_str(&polymer_part);
        }
        // Update the current template
        template = new_template;
    }

    let counts = template.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let most_common = counts.iter().max_by_key(|&(_, count)| count).unwrap().1;
    let least_common = counts.iter().min_by_key(|&(_, count)| count).unwrap().1;
    println!("Part 1: {}", most_common - least_common);

    // Part 2: 40 steps - use dict counts and caching instead of building polymer templates to prevent memory allocations
    fn update_counts(
        pair: &(char, char),
        rules: &HashMap<(char, char), char>,
        num_steps: usize,
        memo: &mut HashMap<PairStep, HashMap<char, usize>>,
    ) -> HashMap<char, usize> {
        // Update counts with the pair and element recursively
        let element = rules.get(pair).unwrap();
        if num_steps == 0 {
            // Update dict with first element - the other elements will be added in the following iterations
            let mut c: HashMap<char, usize> = HashMap::new();
            *c.entry(pair.0).or_insert(0) += 1;
            return c;
        }

        // For each new pair, check if counts already in memo, otherwise add to memo then merge the each pair's counts for that step
        let mut new_counts = HashMap::new();
        let pair_1 = (pair.0, *element);
        let pair_2 = (*element, pair.1);
        for pair in [pair_1, pair_2] {
            match memo.get(&(pair, num_steps)) {
                Some(counts) => {
                    counts
                        .iter()
                        .for_each(|(k, v)| *new_counts.entry(*k).or_insert(0) += v);
                }
                None => {
                    let counts = update_counts(&pair, rules, num_steps - 1, memo);
                    memo.insert((pair, num_steps), counts.clone());
                    counts
                        .iter()
                        .for_each(|(k, v)| *new_counts.entry(*k).or_insert(0) += v);
                }
            };
        }
        new_counts
    }

    let num_steps = 40;
    let mut all_counts = HashMap::new();
    let mut memo = HashMap::new();

    for pair in polymer_template.chars().collect::<Vec<char>>().windows(2) {
        let counts_for_initial_pair =
            update_counts(&(pair[0], pair[1]), &rules, num_steps, &mut memo);
        // Update counts
        counts_for_initial_pair
            .iter()
            .for_each(|(k, v)| *all_counts.entry(*k).or_insert(0) += v);
    }

    // Update dict with last element
    *all_counts
        .entry(polymer_template.chars().last().unwrap())
        .or_insert(0) += 1;

    let most_common = all_counts.iter().max_by_key(|&(_, count)| count).unwrap().1;
    let least_common = all_counts.iter().min_by_key(|&(_, count)| count).unwrap().1;
    println!("Part 2: {}", most_common - least_common);
}
