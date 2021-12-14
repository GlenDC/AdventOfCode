use std::collections::HashMap;

fn main() {
    println!("++++++++++++");
    println!("PART#1");
    println!("++++++++++++");

    part1(TEST_INPUT);

    println!("-----");

    part1(INPUT);

    // TODO: figure out a better way to do this,
    // as there is no way we're going to manage part#2 with this naive algorithm

    // println!("++++++++++++");
    // println!("PART#2");
    // println!("++++++++++++");

    // part2(TEST_INPUT);

    // println!("-----");

    // part2(INPUT);
}

fn part1(input: &str) {
    run(input, 10);
}

fn part2(input: &str) {
    run(input, 40);
}

fn run(input: &str, steps: usize) {
    let mut it = input.split("\n");
    let mut polymer: Vec<u8> = it.next().unwrap().bytes().collect();
    assert!(it.next().unwrap().is_empty());
    let mut rules: HashMap<(u8, u8), u8> = HashMap::new();
    for line in it {
        let mut it = line.split(" -> ");
        let pair = it.next().unwrap().as_bytes();
        assert!(pair.len() == 2);
        let results = it.next().unwrap().as_bytes();
        assert!(results.len() == 1);
        assert!(rules.insert((pair[0], pair[1]), results[0]).is_none());
    }

    // println_polymer(&polymer[..]);
    for _step in 0..steps {
        let (p, last) = polymer[1..]
            .iter()
            .fold((Vec::new(), polymer[0]), |(mut v, p), c| {
                v.push(p);
                if let Some(e) = rules.get(&(p, *c)) {
                    v.push(*e);
                }
                (v, *c)
            });
        polymer = p;
        polymer.push(last);
        // println_polymer(&polymer[..]);
    }

    let mut quantities: HashMap<u8, usize> = HashMap::new();
    for b in polymer {
        *quantities.entry(b).or_default() += 1;
    }
    let mut quantities: Vec<(u8, usize)> = quantities.into_iter().collect();
    quantities.sort_by(|(_, n), (_, m)| n.partial_cmp(m).unwrap());
    let lc = quantities[0];
    let mc = quantities[quantities.len() - 1];
    println!(
        "mc({} => {}) - lc({} => {}) = {}",
        std::str::from_utf8(&[mc.0]).unwrap(),
        mc.1,
        std::str::from_utf8(&[lc.0]).unwrap(),
        lc.1,
        mc.1 - lc.1
    );
}

// fn println_polymer(b: &[u8]) {
//     println!("{}", std::str::from_utf8(b).unwrap());
// }

const TEST_INPUT: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

const INPUT: &'static str = "VHCKBFOVCHHKOHBPNCKO

SO -> F
OP -> V
NF -> F
BO -> V
BH -> S
VB -> B
SV -> B
BK -> S
KC -> N
SP -> O
CP -> O
VN -> O
HO -> S
PC -> B
CS -> O
PO -> K
KF -> B
BP -> K
VO -> O
HB -> N
PH -> O
FF -> O
FB -> K
CC -> H
FK -> F
HV -> P
CO -> S
OC -> N
KV -> V
SS -> O
FC -> O
NP -> B
OH -> B
OF -> K
KB -> K
BN -> C
OK -> C
NC -> O
NO -> O
FS -> C
VP -> K
KP -> S
VS -> B
VV -> N
NN -> P
KH -> P
OB -> H
HP -> H
KK -> H
FH -> F
KS -> V
BS -> V
SN -> H
CB -> B
HN -> K
SB -> O
OS -> K
BC -> H
OV -> N
PN -> B
VH -> N
SK -> C
PV -> K
VC -> N
PF -> S
NB -> B
PP -> S
NS -> F
PB -> B
CV -> C
HK -> P
PK -> S
NH -> B
SH -> V
KO -> H
NV -> B
HH -> V
FO -> O
CK -> O
VK -> F
HF -> O
BF -> C
BV -> P
KN -> K
VF -> C
FN -> V
ON -> C
SF -> F
SC -> C
OO -> S
FP -> K
PS -> C
NK -> O
BB -> V
HC -> H
FV -> V
CH -> N
HS -> V
CF -> F
CN -> S";
