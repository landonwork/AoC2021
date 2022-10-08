
type Algorithm = Vec<bool>;
type Image = Vec<Vec<bool>>;

fn main() {
    let (algo, image) = read_input("input.txt");
    println!("{}", part1(&algo, image.clone()));
    println!("{}", part2(&algo, image));
}

fn part1(algo: &Algorithm, image: Image) -> usize {
    let mut background = false;
    let image = enhance_image(algo, image, background);
    background = true;
    let image = enhance_image(algo, image, background);
    image.into_iter().flatten().map(|b| b as usize).sum()
}

fn part2(algo: &Algorithm, mut image: Image) -> usize {
    let mut background = false;
    for _ in 0..50 {
        image = enhance_image(&algo, image.clone(), background);
        background = !background;
    }
    image.into_iter().flatten().map(|b| b as usize).sum()
}

fn bools_to_u16(image: &Image, coords: (usize, usize)) -> u16 {
    let mut n = 0;
    let mut places = 8;

    for i in 0..3 {
        for j in 0..3 {
            n |= (image[coords.0 + i][coords.1 + j] as u16) << places;
            places -= 1;
        }
    }

    n
}

fn enhance_image(algo: &Algorithm, image: Image, background: bool) -> Image {
    let new_dims = (image.len() + 2, image[0].len() + 2);
    let padded = pad_image(image, background);
    let mut image = vec![vec![false; new_dims.1]; new_dims.0];
    for row in 0..new_dims.0 {
        for col in 0..new_dims.1 {
            let algo_index = bools_to_u16(&padded, (row, col));
            image[row][col] = enhance_pixel(algo_index, algo); // pfft
        }
    }
    image
}

fn enhance_pixel(n: u16, algo: &Algorithm) -> bool {
    algo[n as usize]
}

// My algorithm starts with a light pixel, so my infinitely sized background
// will alternate between light and dark pixels
fn pad_image(image: Image, background: bool) -> Image {
    let new_dims = (image.len() + 4, image[0].len() + 4); // rows and columns
    let mut new_image = Vec::with_capacity(new_dims.0);

    new_image.push(vec![background; new_dims.1]);
    new_image.push(vec![background; new_dims.1]);

    // Pad all the other rows...
    for mut row in image {
        let mut new_row = Vec::with_capacity(new_dims.1);
        new_row.extend(&[background, background]);
        new_row.append(&mut row);
        new_row.extend(&[background, background]);
        new_image.push(new_row);
    }

    new_image.push(vec![background; new_dims.1]);
    new_image.push(vec![background; new_dims.1]);

    new_image
}

fn read_input(path: &str) -> (Algorithm, Image) {
    let raw = std::fs::read_to_string(path).unwrap();
    let mut lines = raw.lines();
    let algo = lines.next().unwrap().chars().map(text_to_bool).collect();
    lines.next();

    let mut image = Vec::new();
    while let Some(line) = lines.next() {
        image.push(line.chars().map(text_to_bool).collect());
    }

    (algo, image)
}

fn text_to_bool(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let (algo, image) = read_input("input.txt");
        assert_eq!(algo.len(), 512);
        for row in &image {
            assert_eq!(image[0].len(), row.len());
        }
        assert_eq!(image.len(), 100);
    }

    #[test]
    fn test_padding() {
        let image1: Image = vec![vec![true]];
        let image2: Image = vec![vec![false]];

        let expected1 = vec![vec![false, false, false, false, false],
                             vec![false, false, false, false, false],
                             vec![false, false, true,  false, false],
                             vec![false, false, false, false, false],
                             vec![false, false, false, false, false]];
        let expected2 = vec![vec![true, true,  true, true, true],
                             vec![true, true,  true, true, true],
                             vec![true, true, false, true, true],
                             vec![true, true,  true, true, true],
                             vec![true, true,  true, true, true]];

        assert_eq!(pad_image(image1, false), expected1);
        assert_eq!(pad_image(image2, true), expected2);
    }

    #[test]
    fn test_to_u16() {
        let image = vec![vec![false, false, false],
                         vec![true,  false, false],
                         vec![true,  false, true]];
        let actual = bools_to_u16(&image, (0, 0));
        assert_eq!(actual, 37);
    }
}

