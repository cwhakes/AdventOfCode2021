use std::collections::BTreeSet;
use std::fmt;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer(&buf, 2);
    println!("{}", answer);

    let answer = get_answer(&buf, 50);
    println!("{}", answer);
}

fn get_answer(input: &str, max_iters: usize) -> usize {
    let mut image = Image::from_str(input);
    for i in 0..max_iters {
    	println!("{}", i);
        image.enhance(i);
    }

    image.data.len()
}

struct Image {
    enhancer: Vec<bool>,
    data: BTreeSet<(i32, i32)>,
}

impl Image {
    fn from_str(input: &str) -> Self {
        let mut iter = input.lines();

        let enhancer: Vec<_> = iter
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|c| c == '#')
            .collect();
        assert_eq!(512, enhancer.len());

        let data = iter
            .skip(1)
            .enumerate()
            .flat_map(|(y, line)| line.trim().chars().enumerate().map(move |(x, c)| (x, y, c)))
            .filter_map(|(x, y, c)| (c == '#').then(|| (x as i32, y as i32)))
            .collect();

        Self { enhancer, data }
    }

    fn enhance(&mut self, i: usize) {
        let Self { enhancer, data } = self;
        let min_x = data.iter().map(|c| c.0).min().unwrap() - 1;
        let max_x = data.iter().map(|c| c.0).max().unwrap() + 1;
        let min_y = data.iter().map(|c| c.1).min().unwrap() - 1;
        let max_y = data.iter().map(|c| c.1).max().unwrap() + 1;

        let mut new_data = BTreeSet::new();
        let offsets = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let index = Self::translate(
                    offsets
                        .into_iter()
                        .map(|(a, b)| data.contains(&(a + x, b + y)) ^ (0!=i%2 && enhancer[0])),
                );
                if enhancer[index] ^ (0==i%2 && enhancer[0]) {
                    new_data.insert((x, y));
                }
            }
        }

        self.data = new_data;
    }

    fn translate(pixels: impl Iterator<Item = bool>) -> usize {
        pixels.fold(0, |acc, pix| acc * 2 + pix as usize)
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x = self.data.iter().map(|c| c.0).min().unwrap() - 1;
        let max_x = self.data.iter().map(|c| c.0).max().unwrap() + 1;
        let min_y = self.data.iter().map(|c| c.1).min().unwrap() - 1;
        let max_y = self.data.iter().map(|c| c.1).max().unwrap() + 1;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.data.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
