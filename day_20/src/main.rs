use ndarray::{s, Array, Array2, Ix};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // Parse the 2 sections
    let (image_enhancement_algorithm, input_image) = parse_input(&input);

    // Part 1: How many pixels are lit in the resulting image
    // after applying the image enhancement algorithm twice
    let n_tries = 2;
    let output_image = get_output_image(input_image.clone(), &image_enhancement_algorithm, n_tries);
    let pixel_count = output_image.into_iter().filter(|&x| x == 1).count();
    println!("Part 1: {}", pixel_count);

    // Part 1: Run 50 times
    let n_tries = 50;
    let output_image = get_output_image(input_image, &image_enhancement_algorithm, n_tries);
    let pixel_count = output_image.into_iter().filter(|&x| x == 1).count();
    println!("Part 2: {}", pixel_count);
}

fn get_output_image(input_image: Array2<u8>, algo: &str, n_tries: usize) -> Array2<u8> {
    if n_tries == 0 {
        return input_image;
    }
    let mut output_image: Array2<u8> =
        Array::zeros((input_image.nrows() + 2, input_image.ncols() + 2));

    let algo_0 = algo.chars().next().unwrap();
    let algo_last =  algo.chars().last().unwrap();
    // flashing infinity pixels: None char changes if every 2nd attempt
    // Note, if the last item was not different than the first, the count would be infinite
    let none_char = {
        if algo_0 == '#' && algo_last == '.' && n_tries % 2 != 0 {
            '1'
        } else {
            '0'
        }
    };
    for (idx, value) in output_image.indexed_iter_mut() {
        let tx_idx = (idx.0 as i32 - 1, idx.1 as i32 - 1);
        let s = get_binary_string(&input_image, tx_idx, none_char);
        let number = get_number(&s);
        let pixel: char = algo.chars().nth(number).unwrap();
        *value = u8::from(pixel == '#');
    }
    get_output_image(output_image, algo, n_tries - 1)
}

fn get_binary_string(input_image: &Array2<u8>, idx: (i32, i32), none_char: char) -> String {
    let mut s: String = String::new();
    // Todo use slices, pad image instead?
    for i in [-1, 0, 1] {
        for j in [-1, 0, 1] {
            let new_index = (idx.0 + i, idx.1 + j);
            // Check if inside bounds
            if new_index.0 < 0
                || new_index.0 >= input_image.nrows() as i32
                || new_index.1 < 0
                || new_index.1 >= input_image.ncols() as i32
            {
                s.push(none_char);
            } else {
                let input_pixel = input_image[(new_index.0 as Ix, new_index.1 as Ix)];
                s.push_str(&input_pixel.to_string())
            }
        }
    }
    s
}

/// Return binary string as number
fn get_number(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn parse_input(input: &str) -> (String, Array2<u8>) {
    let mut image_enhancement_algorithm: String = String::new();

    // Get the image as bool matrix
    let mut input_image: Vec<Vec<u8>> = Vec::new();
    let mut algo_section = true;
    for line in input.lines() {
        if line.is_empty() {
            algo_section = false;
            continue;
        }
        if algo_section {
            image_enhancement_algorithm.push_str(line);
        } else {
            let values: Vec<u8> = line.chars().map(|x| u8::from(x == '#')).collect();
            input_image.push(values)
        }
    }
    let mut grid: Array2<u8> = Array::default((input_image.len(), input_image.len()));
    for (index, row) in input_image.into_iter().enumerate() {
        let mut row_at_index = grid.slice_mut(s![index, ..]);
        row_at_index.assign(&Array::from(row));
    }
    (image_enhancement_algorithm, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number() {
        let number = get_number(&"000100010");
        assert_eq!(number, 34);
    }
}
