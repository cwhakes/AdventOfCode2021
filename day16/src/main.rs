use std::fs::File;
use std::io::Read;

use nom::{
    bits::complete::take,
    multi::{many0, many_m_n},
    IResult,
};

fn main() {
    let mut buf = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let bin = convert_to_bin(&buf).unwrap();
    let (_, packet) = parse_packet((&bin, 0)).unwrap();

    println!("{}", packet.version_sum);
    println!("{}", packet.num);
}

fn convert_to_bin(input: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len() / 2);

    let mut iter = input.chars();
    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        let byte = a.to_digit(16)? * 16 + b.to_digit(16)?;
        out.push(byte as u8);
    }

    Some(out)
}

fn parse_packet(bits: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (bits, version): (_, i64) = take(3u8)(bits)?;
    let (bits, type_id): (_, u8) = take(3u8)(bits)?;

    let (bits, packet) = if type_id == 4 {
        let mut bits_group = (bits, 0i64);
        let mut num = 0i64;
        loop {
            bits_group = take(5u8)(bits_group.0)?;
            num = num * 0b10000 + bits_group.1 % 0b10000;
            if bits_group.1 < 0b10000 {
                break (bits_group.0, Packet::new(num));
            }
        }
    } else {
        let (bits, length_type): (_, u8) = take(1u8)(bits)?;
        if length_type == 0u8 {
            let (bits, len): (_, usize) = take(15u8)(bits)?;

            let offset = len + bits.1;
            let mut sub_bit_vec = bits.0[..(offset / 8)].to_owned();
            let (rem_bits, last_byte): (_, u8) = take(offset % 8)((&bits.0[(offset / 8)..], 0))?;
            if offset % 8 > 0 {
                sub_bit_vec.push(last_byte << (8 - offset % 8));
            }
            let sub_bits = (&*sub_bit_vec, bits.1);

            let (_, packets) = many0(parse_packet)(sub_bits).unwrap();

            (rem_bits, Packet::operation(type_id, &packets))
        } else {
            let (bits, count) = take(11u8)(bits)?;
            let (bits, packets) = many_m_n(count, count, parse_packet)(bits)?;

            (bits, Packet::operation(type_id, &packets))
        }
    };
    Ok((bits, packet.with_version(version)))
}

#[derive(Debug)]
struct Packet {
    version_sum: i64,
    num: i64,
}

impl Packet {
    fn new(num: i64) -> Self {
        Self {
            version_sum: 0,
            num,
        }
    }

    fn with_version(mut self, version: i64) -> Self {
        self.version_sum += version;
        self
    }

    fn operation(op: u8, packets: &[Self]) -> Self {
        let num = match op {
            0 => packets.iter().map(|p| p.num).sum(),
            1 => packets.iter().map(|p| p.num).product(),
            2 => packets.iter().map(|p| p.num).min().unwrap(),
            3 => packets.iter().map(|p| p.num).max().unwrap(),

            5 => {
                if packets[0].num > packets[1].num {
                    1
                } else {
                    0
                }
            }
            6 => {
                if packets[0].num < packets[1].num {
                    1
                } else {
                    0
                }
            }
            7 => {
                if packets[0].num == packets[1].num {
                    1
                } else {
                    0
                }
            }

            _ => 0,
        };

        Packet {
            version_sum: packets.iter().map(|p| p.version_sum).sum(),
            num,
        }
    }
}
