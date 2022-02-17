use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let answer = get_answer1(&buf);
    println!("{}", answer.unwrap());

    let answer = get_answer2(&buf);
    println!("{}", answer.unwrap());
}

fn get_answer1(input: &str) -> Option<i64> {
    let mut iter = input.lines();
    let mut p1_pos = iter.next()?.strip_prefix("Player 1 starting position: ")?.parse::<i64>().ok()?;
    let mut p2_pos = iter.next()?.strip_prefix("Player 2 starting position: ")?.parse::<i64>().ok()?;

    let mut r = Roller::new();
    let mut p1_score = 0;
    let mut p2_score = 0;

    let (loser, rolls) = loop {
    	p1_pos = (p1_pos + r.roll() + r.roll() + r.roll() - 1) % 10 + 1;
    	p1_score += p1_pos;
    	if p1_score >= 1000 {
    		break (p2_score, r.count());
    	}

    	p2_pos = (p2_pos + r.roll() + r.roll() + r.roll() - 1) % 10 + 1;
    	p2_score += p2_pos;
    	if p2_score >= 1000 {
    		break (p1_score, r.count());
    	}
    };

    Some(loser * rolls as i64)
}



fn get_answer2(input: &str) -> Option<usize> {
    let mut iter = input.lines();
    let p1_pos = iter.next()?.strip_prefix("Player 1 starting position: ")?.parse::<i8>().ok()?;
    let p2_pos = iter.next()?.strip_prefix("Player 2 starting position: ")?.parse::<i8>().ok()?;

    let init_state = State{
    	hi_score: 0,
    	lo_score: 0,
    	p1_winning: false,
    	p1_turn: true,
    	p1_pos,
    	p2_pos,
    };

    let mut game_states = BTreeMap::new();
    game_states.insert(init_state, 1);

    let rolls = [
    	(3_i8, 1_usize),
    	(4, 3),
    	(5, 6),
    	(6, 7),
    	(7, 6),
    	(8, 3),
    	(9, 1),
    ];

    loop {
    	let state = game_states.keys().next().unwrap().clone();

    	if state.hi_score >= 21 {
    		break;
    	}

    	let state_count = game_states.remove(&state).unwrap();

    	for (roll, roll_count) in rolls {
    		let (mut p1_score, mut p2_score) = if state.p1_winning {
    			(state.hi_score, state.lo_score)
    		} else {
    			(state.lo_score, state.hi_score)
    		};

    		let mut p1_pos: i8 = state.p1_pos;
    		let mut p2_pos: i8 = state.p2_pos;

    		if state.p1_turn {
    			p1_pos = (p1_pos + roll - 1) % 10 + 1;
    			p1_score += p1_pos as u8;
    		} else {
    			p2_pos = (p2_pos + roll - 1) % 10 + 1;
    			p2_score += p2_pos as u8;
    		}

    		let (hi_score, lo_score, p1_winning) = if p1_score > p2_score {
    			(p1_score, p2_score, true)
    		} else {
    			(p2_score, p1_score, false)
    		};

    		let new_state = State {
    			hi_score,
    			lo_score,
    			p1_winning,
    			p1_turn: !state.p1_turn,
    			p1_pos,
    			p2_pos,
    		};

    		*game_states.entry(new_state).or_default() += state_count * roll_count; 
    	}
    }

    let mut p1_count = 0;
    let mut p2_count = 0;

    for (state, count) in game_states.iter() {
    	if state.p1_winning {
    		p1_count += count;
    	} else {
    		p2_count += count;
    	}
    }

    Some(p1_count.max(p2_count))
}

struct Roller(i64, usize);

impl Roller {
	fn new() -> Self {
		Self(1, 0)
	}

	fn roll(&mut self) -> i64 {
		let out = self.0;
		self.0 = self.0 % 100 + 1;
		self.1 += 1;
		out
	}

	fn count(&self) -> usize {
		self.1
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
	hi_score: u8,
	lo_score: u8,
	p1_winning: bool,
	p1_turn: bool,
	p1_pos: i8,
	p2_pos: i8,
}
