use std::cmp::Ordering;
use std::fmt;
use std::iter::zip;
use serde_json::{Result, Value, json};
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn with_slice<T>(&self, f: impl FnOnce(&[Packet]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Number(n) => { write!(f, "{n}")}
            Packet::List(n) => { f.debug_list().entries(n).finish() }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (l, r) => Some({
                l.with_slice(|l| {
                    r.with_slice(|r| {
                        l.iter()
                            .zip(r.iter())
                            .map(|(aa,bb)| aa.cmp(bb))
                            .find(|&ord| ord != Ordering::Equal)
                            .unwrap_or_else(|| l.len().cmp(&r.len()))
                    })
                })
            })
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {

    let mut sum = 0;
    for (i, groups) in include_str!("input.txt").split("\r\n\r\n").enumerate() {
        let mut nodes = groups.lines().map(|line| serde_json::from_str::<Packet>(line).unwrap());
        let l = nodes.next().unwrap();
        let r = nodes.next().unwrap();

        if l < r {
            sum += i + 1;
        }

        println!("LEFT : {:?}, RIGHT : {:?}", l, r);
    }
    println!("{}", sum);
}

fn part2() {

    let divider_packets = vec![Packet::List(vec![Packet::Number(2)]), Packet::List(vec![Packet::Number(6)])];

    let mut packets = include_str!("input.txt").lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Packet>(line).unwrap())
        .chain(divider_packets.iter().cloned())
        .collect::<Vec<Packet>>();

    packets.sort();

    let decoder_key = divider_packets
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    println!("{:?}", decoder_key);
}
