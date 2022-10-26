use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules: HashMap<Vec<char>, char> = HashMap::new();
    let mut polymer_template: String = String::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if index == 0 {
            polymer_template = line;
        } else if index > 1 {
            let (pair, element) = line.splitn(2, " -> ").collect_tuple().unwrap();
            rules.insert(pair.chars().collect(), element.chars().next().unwrap());
        }
    }

    let steps = 10;
    let mut template = polymer_template.clone();
    for _ in 1..=steps {
        let mut new_template = template.chars().next().unwrap().to_string();
        // 1. Get all pairs in the template
        for pair in template.chars().collect::<Vec<char>>().windows(2) {
            //let pair_str = String::from_iter(pair);
            let element = rules.get(pair).unwrap();
            let new_str = format!("{}{}", element, &pair.last().unwrap());
            new_template.push_str(&new_str);
        }

        template = new_template;
    }

    // Part 1:
    // take the quantity of the most common element and subtract
    // the quantity of the least common element after
    // 10 steps
    let mut counts = BTreeMap::new();
    for char in template.chars() {
        *counts.entry(char).or_insert(0) += 1;
    }

    let most_common = counts.iter().max_by_key(|&(_, count)| count).unwrap().1;
    let least_common = counts.iter().min_by_key(|&(_, count)| count).unwrap().1;
    println!("Part 1: {}", most_common - least_common);

    // Part 2: 40 steps - use dict counts instead of building polymer template to prevent memory allocations

    fn update_counts(pair: &[char], rules: &HashMap<Vec<char>, char>, num_steps: usize) -> HashMap<char, usize> {
        // Update counts with the pair and element recursively
        let element = rules.get(&pair.to_vec()).unwrap();
        if num_steps == 0 {
            // Update dict with first element
            let mut c: HashMap<char, usize> = HashMap::new();
            //*counts.entry(pair[0]).or_insert(0) += 1;
            *c.entry(pair[0]).or_insert(0) += 1;
            return c;
            // for item in pair {
            //     *counts.entry(*item).or_insert(0) += 1;
            // }
            //*counts.entry(*element).or_insert(0) += 1;
            //return;
        }
        let pair_1 = &[pair[0], *element];
        let pair_2 = &[*element, pair[1]];
        let mut counts_left = update_counts(pair_1, rules, num_steps - 1);
        let counts_right = update_counts(pair_2, rules, num_steps - 1);

        for (k, v)  in counts_right.iter() {
            *counts_left.entry(*k).or_insert(0) += v;
        }
        //counts_left.extend(counts_right);
        return counts_left;
    }

    let num_steps = 10;
    let mut counts: HashMap<char, usize> = HashMap::new();

    for initial_pair in polymer_template.chars().collect::<Vec<char>>().windows(2) {
        //println!("{:?}", counts);
        //println!("{:?}", initial_pair);
        let counts_for_initial_pair = update_counts(initial_pair, &rules, num_steps);
        // Update counts
        //counts.extend(counts_for_initial_pair);
        for (k, v)  in counts_for_initial_pair.iter() {
            *counts.entry(*k).or_insert(0) += v;
        }
    }
    //
    // Update dict with last element
    *counts.entry(polymer_template.chars().last().unwrap()).or_insert(0) += 1;
    println!("{:?}", counts);

    let most_common = counts.iter().max_by_key(|&(_, count)| count).unwrap().1;
    let least_common = counts.iter().min_by_key(|&(_, count)| count).unwrap().1;
    println!("Part 2: {}", most_common - least_common);
}
