use std::collections::HashMap;

use crate::TaskCompleter;

pub struct Task7;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(hand: &[u32; 5]) -> Self {
        let mut count = [0; 13];
        for card in hand {
            count[*card as usize] += 1;
        }
        if count.contains(&5) {
            return HandType::FiveOfAKind;
        } else if count.contains(&4) {
            return HandType::FourOfAKind;
        } else if count.contains(&3) {
            if count.contains(&2) {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }
        match count.iter().filter(|x| x == &&2).count() {
            0 => HandType::HighCard,
            1 => HandType::OnePair,
            2 => HandType::TwoPair,
            _ => panic!("Can't have more than two pairs from 5 cards"),
        }
    }

    fn new_2(hand: &[u32; 5]) -> Self {
        let mut count = [0; 13];
        for card in hand {
            count[*card as usize] += 1;
        }
        let joker_count = count[0];
        let count = &count[1..13];

        if count.contains(&(5 - joker_count)) {
            return HandType::FiveOfAKind;
        } else if count.contains(&(4 - joker_count)) {
            return HandType::FourOfAKind;
        }

        // here can have at most 2 joker, as 3 or more will make four of a kind
        assert!(joker_count <= 2);
        if joker_count == 2 {
            // Can't have any counts of two as that would have returned four of a kind
            HandType::ThreeOfAKind
        } else if joker_count == 1 {
            match count.iter().filter(|x| x == &&2).count() {
                0 => HandType::OnePair,
                1 => HandType::ThreeOfAKind,
                2 => HandType::FullHouse,
                _ => panic!("Can't have more than two pairs from 5 cards"),
            }
        } else {
            if count.contains(&3) {
                if count.contains(&2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            } else {
                match count.iter().filter(|x| x == &&2).count() {
                    0 => HandType::HighCard,
                    1 => HandType::OnePair,
                    2 => HandType::TwoPair,
                    _ => panic!("Can't have more than two pairs from 5 cards"),
                }
            }
        }
    }
}

pub struct Hand {
    hand: [u32; 5],
    hand_type: HandType,
    bid: u32,
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.hand.cmp(&other.hand)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.hand.partial_cmp(&other.hand)
    }
}

impl Hand {
    fn new(x: &str) -> Self {
        let mut s = x.split(" ");
        let hand = s
            .next()
            .unwrap()
            .chars()
            .map(|x| match x {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => panic!("Invalid Card"),
            })
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();
        let bid = s.next().unwrap().parse::<u32>().unwrap();
        Self {
            hand,
            bid,
            hand_type: HandType::new(&hand),
        }
    }
}

pub struct Hand2 {
    hand: [u32; 5],
    hand_type: HandType,
    bid: u32,
}

impl Eq for Hand2 {}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.hand.cmp(&other.hand)
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.hand_type == other.hand_type
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.hand.partial_cmp(&other.hand)
    }
}

impl Hand2 {
    fn new(x: &str) -> Self {
        let mut s = x.split(" ");
        let hand = s
            .next()
            .unwrap()
            .chars()
            .map(|x| match x {
                'J' => 0,
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                '7' => 6,
                '8' => 7,
                '9' => 8,
                'T' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => panic!("Invalid Card"),
            })
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();
        let bid = s.next().unwrap().parse::<u32>().unwrap();
        Self {
            hand,
            bid,
            hand_type: HandType::new_2(&hand),
        }
    }
}

impl TaskCompleter for Task7 {
    fn get_name(&self) -> String {
        "7".to_owned()
    }

    fn do_task_1(&self) -> String {
        let mut c: Vec<Hand> = include_str!("../input/seven/input")
            .lines()
            .map(|x| Hand::new(x))
            .collect();
        c.sort();
        let mut sum = 0;
        for i in 0..c.len() {
            sum += (i + 1) * c[i].bid as usize;
        }
        sum.to_string()
    }

    fn do_task_2(&self) -> String {
        let mut c: Vec<Hand2> = include_str!("../input/seven/input")
            .lines()
            .map(|x| Hand2::new(x))
            .collect();
        c.sort();
        let mut sum = 0;
        for i in 0..c.len() {
            sum += (i + 1) * c[i].bid as usize;
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::seven::{Hand2, HandType};

    #[test]
    fn new_2_tests() {
        assert_eq!(HandType::FiveOfAKind, Hand2::new("AAAAA 0").hand_type);
        assert_eq!(HandType::FiveOfAKind, Hand2::new("JAAAA 0").hand_type);
        assert_eq!(HandType::FiveOfAKind, Hand2::new("JJAAA 0").hand_type);
        assert_eq!(HandType::FiveOfAKind, Hand2::new("JJJAA 0").hand_type);
        assert_eq!(HandType::FiveOfAKind, Hand2::new("JJJJA 0").hand_type);
        assert_eq!(HandType::FiveOfAKind, Hand2::new("JJJJJ 0").hand_type);

        assert_eq!(HandType::FullHouse, Hand2::new("AAJ22 0").hand_type);
        assert_eq!(HandType::FourOfAKind, Hand2::new("AJJ22 0").hand_type);
        assert_eq!(HandType::FiveOfAKind, Hand2::new("JJJ22 0").hand_type);
        assert_eq!(HandType::OnePair, Hand2::new("J2345 0").hand_type);
        assert_eq!(HandType::ThreeOfAKind, Hand2::new("J2245 0").hand_type);
    }

    #[test]
    fn run_2_on_example() {
        let mut c: Vec<Hand2> = include_str!("../input/seven/example")
            .lines()
            .map(|x| Hand2::new(x))
            .collect();
        c.sort();
        let mut sum = 0;
        for i in 0..c.len() {
            sum += (i + 1) * c[i].bid as usize;
        }
        assert_eq!(sum, 5905);
    }
}
