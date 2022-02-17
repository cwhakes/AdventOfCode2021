use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer1(&buf);
    println!("{}", answer.unwrap());

    let answer = get_answer2(&buf);
    println!("{}", answer);
}

fn get_answer1(input: &str) -> Option<usize> {
    let cuboids = input.lines().map(Cuboid::from_str).collect::<Option<Vec<_>>>()?;

    let mut reactor = BTreeSet::new();

    for (state, cuboid) in cuboids {
    	if cuboid.within_bounds() {
	    	if state {
	    		reactor.extend(cuboid.iter());
	    	} else {
	    		for coord in cuboid.iter() {
	    			reactor.remove(&coord);
	    		}
	    	}
	    }
    }

    Some(reactor.into_iter().filter(|(x, y, z)| (-50..=50).contains(x) && (-50..=50).contains(y) && (-50..=50).contains(z)).count())
}

fn get_answer2(input: &str) -> usize {
    let cuboids = input.lines().map(Cuboid::from_str).collect::<Option<Vec<_>>>().unwrap();

    let mut reactor_on: Vec<Cuboid> = Vec::new();
    let mut reactor_off: Vec<Cuboid> = Vec::new();

    for (state, cuboid) in cuboids {
    	if state {
    		let off_overlaps: Vec<_> = reactor_on.iter().map(|c| c.overlap(&cuboid)).filter(|o| o.size() > 0).collect();
    		let on_overlaps: Vec<_> = reactor_off.iter().map(|c| c.overlap(&cuboid)).filter(|o| o.size() > 0).collect();
    		reactor_off.extend(off_overlaps);
    		reactor_on.extend(on_overlaps);
    		reactor_on.push(cuboid);

    	} else {
    		let off_overlaps: Vec<_> = reactor_on.iter().map(|c| c.overlap(&cuboid)).filter(|o| o.size() > 0).collect();
    		let on_overlaps: Vec<_> = reactor_off.iter().map(|c| c.overlap(&cuboid)).filter(|o| o.size() > 0).collect();
    		reactor_off.extend(off_overlaps);
    		reactor_on.extend(on_overlaps);
    	}
    }

    reactor_on.iter().map(Cuboid::size).sum::<usize>() - reactor_off.iter().map(Cuboid::size).sum::<usize>() 
}

struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Cuboid {
    fn from_str(input: &str) -> Option<(bool, Self)> {
        let (state, input) = input.split_once(" ")?;
        let state = match state {
            "on" => true,
            "off" => false,
            _ => return None,
        };

        let mut iter = input.trim().split(",");
        let (x1, x2) = iter.next()?.strip_prefix("x=")?.split_once("..")?;
        let (y1, y2) = iter.next()?.strip_prefix("y=")?.split_once("..")?;
        let (z1, z2) = iter.next()?.strip_prefix("z=")?.split_once("..")?;

        Some((
            state,
            Self {
                x: x1.parse().ok()?..=x2.parse().ok()?,
                y: y1.parse().ok()?..=y2.parse().ok()?,
                z: z1.parse().ok()?..=z2.parse().ok()?,
            },
        ))
    }

    fn iter(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        self.x.clone().flat_map(move |x| {
            self.y
                .clone()
                .flat_map(move |y| self.z.clone().map(move |z| (x, y, z)))
        })
    }

    fn within_bounds(&self) -> bool {
    	(*self.x.start() <= 50 && *self.x.end() >= -50) &&
    	(*self.y.start() <= 50 && *self.y.end() >= -50) &&
    	(*self.z.start() <= 50 && *self.z.end() >= -50)
    }

    fn size(&self) -> usize {
    	self.x.clone().count() * self.y.clone().count() * self.z.clone().count()
    }

    fn overlap(&self, other: &Self) -> Self {
    	Self {
    		x: *(self.x.start().max(other.x.start()))..=*(self.x.end().min(other.x.end())),
    		y: *(self.y.start().max(other.y.start()))..=*(self.y.end().min(other.y.end())),
    		z: *(self.z.start().max(other.z.start()))..=*(self.z.end().min(other.z.end())),
    	}
    }
}
