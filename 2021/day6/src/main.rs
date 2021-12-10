fn main() {
    println!("++++++++++++");
    println!("PART#1: Ok");
    println!("++++++++++++");
    part1(TEST_INPUT, 18);
    part1(TEST_INPUT, 80);

    println!("-----");

    part1(INPUT, 80);

    println!("++++++++++++");
    println!("PART#2: too slow and even wrong");
    println!("++++++++++++");

    part2_slow_and_wrong(TEST_INPUT, 18);
    part2_slow_and_wrong(TEST_INPUT, 80);

    println!("-----");

    part2_slow_and_wrong(INPUT, 80);

    println!("-----");

    part2_slow_and_wrong(TEST_INPUT, 140);

    println!("++++++++++++");
    println!("PART#2: Ok");
    println!("++++++++++++");

    part2(TEST_INPUT, 140);
}

fn part1(input: &str, days: usize) {
    let mut school: Vec<LanternFish> = input
        .split(",")
        .map(|s| {
            let counter: u8 = s.parse().unwrap();
            LanternFish::new(Some(counter))
        })
        .collect();
    for _i in 0..days {
        let mut n = 0;
        for fish in school.iter_mut() {
            n += fish.age() as usize;
        }
        for _ in 0..n {
            school.push(LanternFish::new(None));
        }
        // let ages: Vec<u8> = school.iter().map(|fish| fish.counter()).collect();
        // println!("After {} Day(s): {:?}", i + 1, ages);
    }
    println!("{}", school.len());
}

fn part2_slow_and_wrong(input: &str, days: usize) {
    let mut school: Vec<LanternFishSnapshot> = input
        .split(",")
        .map(|s| {
            let counter: u8 = s.parse().unwrap();
            LanternFishSnapshot::new(Some(counter), 0)
        })
        .collect();
    let mut total_fish_count = school.len();
    while !school.is_empty() {
        school = school
            .into_iter()
            .map(|snapshot| snapshot.into_snapshots(days))
            .fold(Vec::new(), |mut school, snapshots| {
                school.extend(snapshots);
                school
            });
        total_fish_count += school.len();
    }
    println!("{}", total_fish_count);
}

fn part2(_input: &str, _days: usize) {
    println!("TODO");
}

struct LanternFish {
    cycle: u8,
    counter: u8,
}

impl LanternFish {
    pub fn new(counter: Option<u8>) -> LanternFish {
        match counter {
            None => LanternFish {
                cycle: 9,
                counter: 8,
            },
            Some(counter) => LanternFish { cycle: 7, counter },
        }
    }

    pub fn age(&mut self) -> bool {
        if self.counter == 0 {
            self.cycle = 7; // only first cycle takes 9 days
            self.counter = self.cycle - 1;
            return true;
        }
        self.counter -= 1;
        false
    }

    // pub fn counter(&self) -> u8 {
    //     self.counter
    // }
}

struct LanternFishSnapshot {
    day: usize,
    cycle: u8,
    counter: u8,
}

impl LanternFishSnapshot {
    pub fn new(counter: Option<u8>, day: usize) -> LanternFishSnapshot {
        match counter {
            None => LanternFishSnapshot {
                day,
                cycle: 9,
                counter: 0,
            },
            Some(counter) => LanternFishSnapshot {
                day,
                cycle: 7,
                counter,
            },
        }
    }

    pub fn into_snapshots(mut self, max_day: usize) -> Vec<LanternFishSnapshot> {
        let mut children = Vec::new();
        if self.counter > 0 {
            self.day += (self.counter + 1) as usize;
            if self.day >= max_day {
                return children;
            }
            children.push(LanternFishSnapshot::new(None, self.day));
        } else if self.cycle == 9 {
            self.day += 9;
            if self.day >= max_day {
                return children;
            }
            children.push(LanternFishSnapshot::new(None, self.day));
        }
        // create all remaining
        for offset in 0..((max_day - self.day) / 7) {
            children.push(LanternFishSnapshot::new(None, self.day + (offset + 1) * 7));
        }
        children
    }
}

const TEST_INPUT: &'static str = "3,4,3,1,2";

const INPUT: &'static str = "1,2,1,3,2,1,1,5,1,4,1,2,1,4,3,3,5,1,1,3,5,3,4,5,5,4,3,1,1,4,3,1,5,2,5,2,4,1,1,1,1,1,1,1,4,1,4,4,4,1,4,4,1,4,2,1,1,1,1,3,5,4,3,3,5,4,1,3,1,1,2,1,1,1,4,1,2,5,2,3,1,1,1,2,1,5,1,1,1,4,4,4,1,5,1,2,3,2,2,2,1,1,4,3,1,4,4,2,1,1,5,1,1,1,3,1,2,1,1,1,1,4,5,5,2,3,4,2,1,1,1,2,1,1,5,5,3,5,4,3,1,3,1,1,5,1,1,4,2,1,3,1,1,4,3,1,5,1,1,3,4,2,2,1,1,2,1,1,2,1,3,2,3,1,4,5,1,1,4,3,3,1,1,2,2,1,5,2,1,3,4,5,4,5,5,4,3,1,5,1,1,1,4,4,3,2,5,2,1,4,3,5,1,3,5,1,3,3,1,1,1,2,5,3,1,1,3,1,1,1,2,1,5,1,5,1,3,1,1,5,4,3,3,2,2,1,1,3,4,1,1,1,1,4,1,3,1,5,1,1,3,1,1,1,1,2,2,4,4,4,1,2,5,5,2,2,4,1,1,4,2,1,1,5,1,5,3,5,4,5,3,1,1,1,2,3,1,2,1,1";
