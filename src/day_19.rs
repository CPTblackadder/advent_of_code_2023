use std::{collections::HashMap, ops::Range};

use rayon::iter::Either;

use crate::TaskCompleter;

#[derive(Clone, Debug, PartialEq, Eq)]
enum InstructionResult<'a> {
    Reject,
    Accept,
    Rule(&'a str),
}

impl<'a> InstructionResult<'a> {
    fn new(input: &'a str) -> Self {
        match input {
            "A" => InstructionResult::Accept,
            "R" => InstructionResult::Reject,
            _ => InstructionResult::Rule(input),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Variables {
    X,
    M,
    A,
    S,
}
impl Variables {
    fn new(input: &str) -> Variables {
        match input {
            "x" => Variables::X,
            "m" => Variables::M,
            "a" => Variables::A,
            "s" => Variables::S,
            _ => panic!("Invalid variable {}", input),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Condition {
    GreaterThan,
    LessThan,
}
impl Condition {
    fn new(input: &str) -> Condition {
        match input {
            "<" => Condition::LessThan,
            ">" => Condition::GreaterThan,
            _ => panic!("Invalic Condition {}", input),
        }
    }
}

#[derive(Debug)]
struct Matcher<'a> {
    variable: Variables,
    condition: Condition,
    value: i64,
    result: InstructionResult<'a>,
}

impl<'a> Matcher<'a> {
    fn matches(&self, arg: &XMASObject) -> bool {
        let v;
        match self.variable {
            Variables::X => v = &arg.x,
            Variables::M => v = &arg.m,
            Variables::A => v = &arg.a,
            Variables::S => v = &arg.s,
        }
        match self.condition {
            Condition::GreaterThan => v > &self.value,
            Condition::LessThan => v < &self.value,
        }
    }

    fn matches_range(
        &self,
        unmatched_range: &XMASRange,
    ) -> (Option<XMASRange>, InstructionResult<'a>, Option<XMASRange>) {
        let (lower, higher) = unmatched_range.split_at(
            self.variable,
            self.value,
            self.condition == Condition::GreaterThan,
        );
        match self.condition {
            Condition::GreaterThan => (higher, self.result.clone(), lower),
            Condition::LessThan => (lower, self.result.clone(), higher),
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    rules: Vec<Matcher<'a>>,
    default: InstructionResult<'a>,
}

impl<'a> Rule<'a> {
    fn new(s: std::str::Split<'a, &[char]>) -> Self {
        let (rules, default) = s.filter(|x| !x.is_empty()).map(get_matcher).fold(
            (vec![], InstructionResult::Accept),
            |(mut v, r), x| match x {
                Either::Left(y) => {
                    v.push(y);
                    (v, r)
                }
                Either::Right(y) => (v, y),
            },
        );

        Self { rules, default }
    }

    fn run(&self, arg: &XMASObject) -> InstructionResult<'a> {
        for rule in &self.rules {
            if rule.matches(arg) {
                return rule.result.clone();
            }
        }
        self.default.clone()
    }

    fn run_range(&self, arg: &XMASRange) -> Vec<(XMASRange, InstructionResult<'a>)> {
        let mut ranges = vec![];
        let mut unmatched_range = arg.clone();
        for rule in &self.rules {
            let (matched_range, result, u) = rule.matches_range(&unmatched_range);
            if let Some(m) = matched_range {
                ranges.push((m, result));
            }
            if let Some(u) = u {
                unmatched_range = u.clone();
            } else {
                // No more unmatched ranges
                return ranges;
            }
        }
        ranges.push((unmatched_range, self.default.clone()));
        ranges
    }
}

fn parse_condition<'a>(input: &'a str) -> (Variables, Condition, i64) {
    let v = Variables::new(&input[0..1]);
    let c = Condition::new(&input[1..2]);
    let r = input[2..].parse::<i64>().unwrap();
    (v, c, r)
}

fn get_matcher<'a>(input: &'a str) -> Either<Matcher<'a>, InstructionResult<'a>> {
    if input.contains(":") {
        let mut s = input.split(':');
        let (variable, condition, value) = parse_condition(s.next().unwrap());
        let r = s.next().unwrap();
        let result = InstructionResult::new(r);

        Either::Left(Matcher {
            variable,
            condition,
            value,
            result,
        })
    } else {
        Either::Right(InstructionResult::new(input))
    }
}

struct XMASObject {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}
impl XMASObject {
    fn run_rules(&self, rules: &HashMap<&str, Rule<'_>>) -> bool {
        let mut current_rule = "in";
        loop {
            let rule = rules.get(current_rule).expect(&format!(
                "Couldn't find {} in {:?}",
                current_rule,
                rules.keys()
            ));
            match rule.run(&self) {
                InstructionResult::Reject => return false,
                InstructionResult::Accept => return true,
                InstructionResult::Rule(new_rule) => current_rule = new_rule,
            }
        }
    }

    fn sum_values(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone)]
struct XMASRange {
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>,
}

impl XMASRange {
    fn run_rules(&self, rules: &HashMap<&str, Rule<'_>>) -> Vec<XMASRange> {
        self.run_rule(rules, "in")
    }

    fn run_rule(&self, rules: &HashMap<&str, Rule<'_>>, rule: &str) -> Vec<XMASRange> {
        let rule = rules
            .get(rule)
            .expect(&format!("Couldn't find {} in {:?}", rule, rules.keys()));
        rule.run_range(&self)
            .into_iter()
            .filter(|(_, result)| result != &InstructionResult::Reject)
            .map(|(range, result)| {
                if let InstructionResult::Rule(r) = result {
                    range.run_rule(rules, r)
                } else {
                    vec![range]
                }
            })
            .flatten()
            .collect()
    }

    fn get_combinations(&self) -> i64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }

    fn split_at(
        &self,
        variable: Variables,
        value: i64,
        value_in_lower: bool,
    ) -> (Option<XMASRange>, Option<XMASRange>) {
        let pivot = if value_in_lower { value + 1 } else { value };
        let r = match variable {
            Variables::X => &self.x,
            Variables::M => &self.m,
            Variables::A => &self.a,
            Variables::S => &self.s,
        };
        if pivot < r.start {
            (None, Some(self.clone()))
        } else if pivot > r.end {
            (Some(self.clone()), None)
        } else {
            (
                Some(self.clone_with_value(variable, r.start..pivot)),
                Some(self.clone_with_value(variable, pivot..r.end)),
            )
        }
    }

    fn clone_with_value(&self, variable: Variables, pivot: Range<i64>) -> Self {
        match variable {
            Variables::X => Self {
                x: pivot,
                ..self.clone()
            },
            Variables::M => Self {
                m: pivot,
                ..self.clone()
            },
            Variables::A => Self {
                a: pivot,
                ..self.clone()
            },
            Variables::S => Self {
                s: pivot,
                ..self.clone()
            },
        }
    }
}

pub struct Task19;

impl TaskCompleter for Task19 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_19/input");
        let mut doing_rules = true;
        let mut rules = HashMap::new();
        let mut objects = vec![];
        for line in contents.lines() {
            if line.is_empty() {
                doing_rules = false;
                continue;
            }
            if doing_rules {
                let mut s = line.split(&['{', '}', ','][..]);
                let name = s.next().unwrap();
                let rule = Rule::new(s);
                rules.insert(name, rule);
            } else {
                let mut s = line.split(&['{', '}', ','][..]);
                // Everything before first {
                s.next();
                let x = s.next().unwrap()[2..].parse::<i64>().unwrap();
                let m = s.next().unwrap()[2..].parse::<i64>().unwrap();
                let a = s.next().unwrap()[2..].parse::<i64>().unwrap();
                let s = s.next().unwrap()[2..].parse::<i64>().unwrap();
                objects.push(XMASObject { x, m, a, s })
            }
        }

        objects
            .iter()
            .filter(|x| (*x).run_rules(&rules))
            .map(|x| x.sum_values())
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_19/input");
        let mut doing_rules = true;
        let mut rules = HashMap::new();
        for line in contents.lines() {
            if line.is_empty() {
                doing_rules = false;
                continue;
            }
            if doing_rules {
                let mut s = line.split(&['{', '}', ','][..]);
                let name = s.next().unwrap();
                let rule = Rule::new(s);
                rules.insert(name, rule);
            }
        }

        XMASRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
        .run_rules(&rules)
        .iter()
        .map(|x| x.get_combinations())
        .sum::<i64>()
        .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("398527".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("133973513090020".to_owned())
    }
}
