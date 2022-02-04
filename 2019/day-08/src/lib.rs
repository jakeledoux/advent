use itertools::Itertools;

type Resolution = (usize, usize);

#[derive(Default)]
struct Layer {
    pixels: Vec<u32>,
}

pub fn part_one(input: &'static str, resolution: Resolution) -> usize {
    parse_input(input, resolution)
        .into_iter()
        .map(|layer| layer.pixels.iter().copied().counts())
        .min_by_key(|counts| *counts.get(&0).unwrap_or(&0))
        .map(|counts| counts.get(&1).unwrap_or(&0) * counts.get(&2).unwrap_or(&0))
        .unwrap()
}

pub fn part_two(input: &'static str, resolution: Resolution) -> String {
    let layers = parse_input(input, resolution);
    let mut image = vec![' '; resolution.0 * resolution.1];
    for (i, pixel) in image.iter_mut().enumerate() {
        *pixel = layers
            .iter()
            .map(|layer| layer.pixels[i])
            .find(|&p| p != 2)
            .map(|p| match p {
                0 => ' ',
                1 => '█',
                _ => unreachable!(),
            })
            .unwrap();
    }
    image
        .chunks(resolution.0)
        .map(|chunk| chunk.iter().collect::<String>())
        .join("\n")
}

fn parse_input(input: &'static str, resolution: Resolution) -> Vec<Layer> {
    let flat_pixels: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    flat_pixels
        .chunks(resolution.0 * resolution.1)
        .map(|chunk| Layer {
            pixels: chunk.into(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("123456789012", (3, 2)), 1);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("0222112222120000", (3, 2)), "   \n ██".to_string());
    }
}
