use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use num::integer::lcm;

use crate::TaskCompleter;

pub struct Task20;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Low => f.write_str("low"),
            Pulse::High => f.write_str("high"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ModuleDestinations<'a>(Vec<&'a str>);

#[derive(Debug, PartialEq, Eq)]
enum Module<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcast(ModuleDestinations<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FlipFlop<'a> {
    name: &'a str,
    is_on: bool,
    destinations: ModuleDestinations<'a>,
}

#[derive(Debug, PartialEq, Eq)]
struct Conjunction<'a> {
    name: &'a str,
    inputs: HashMap<&'a str, Pulse>,
    destinations: ModuleDestinations<'a>,
}

impl<'a> Module<'a> {
    fn recieve_pulse(
        &mut self,
        from: &'a str,
        pulse: Pulse,
        pulse_queue: &mut VecDeque<(&'a str, Pulse, &'a str)>,
    ) {
        let name = self.name();
        match self {
            Module::FlipFlop(state) => {
                if pulse == Pulse::High {
                    return;
                }
                if state.is_on {
                    state.is_on = false;
                    send_pulses(pulse_queue, &state.destinations, Pulse::Low, name);
                } else {
                    state.is_on = true;
                    send_pulses(pulse_queue, &state.destinations, Pulse::High, name);
                }
            }
            Module::Conjunction(state) => {
                state.inputs.insert(from, pulse);
                let pulse_to_send = if state.inputs.values().all(|x| x == &Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                send_pulses(pulse_queue, &state.destinations, pulse_to_send, name);
            }
            Module::Broadcast(dests) => send_pulses(pulse_queue, dests, Pulse::Low, name),
        }
    }

    fn name(&self) -> &'a str {
        match self {
            Module::FlipFlop(state) => state.name,
            Module::Conjunction(state) => state.name,
            Module::Broadcast(_) => "broadcast",
        }
    }

    fn get_recipients(&self) -> &ModuleDestinations<'a> {
        match self {
            Module::FlipFlop(state) => &state.destinations,
            Module::Conjunction(state) => &state.destinations,
            Module::Broadcast(reciepients) => reciepients,
        }
    }
}

fn send_pulses<'a>(
    pulses_to_send: &mut VecDeque<(&'a str, Pulse, &'a str)>,
    destinations: &ModuleDestinations<'a>,
    pulse: Pulse,
    from: &'a str,
) {
    for d in &destinations.0 {
        pulses_to_send.push_front((from, pulse, d));
    }
}

fn create_module<'a>(modules: &mut HashMap<&'a str, Module<'a>>, line: &'a str) {
    let mut s = line.split("->");
    let first = s.next().unwrap();
    let second = s.next().unwrap();

    let recieveds = ModuleDestinations(second.split(",").map(|x| x.trim()).collect::<Vec<&str>>());

    if first.trim() == "broadcaster" {
        modules.insert("broadcast", Module::Broadcast(recieveds));
    } else {
        let module_type = &first[0..1];
        let module_name = first[1..].trim();
        let module: Module<'a> = match module_type {
            "&" => Module::Conjunction(Conjunction {
                name: module_name,
                inputs: HashMap::new(),
                destinations: recieveds,
            }),
            "%" => Module::FlipFlop(FlipFlop {
                name: module_name,
                is_on: false,
                destinations: recieveds,
            }),
            _ => panic!("Invalid module type {}", module_type),
        };
        modules.insert(module_name, module);
    }
}

// fn get_string<'a, 'b>(input: &HashMap<&'a str, Module<'a>>) -> &'b str {
//     let mut s = "";

//     input
//         .iter()
//         .map(|(name, module)| format!("{}{}", name, module.to_string()).as_str())
//         .collect()
// }

impl TaskCompleter for Task20 {
    fn do_task_1(&self) -> String {
        let contents: &str = include_str!("../input/day_20/input");
        let mut modules = HashMap::new();
        for line in contents.lines() {
            create_module(&mut modules, line);
        }

        let mut input_list = HashMap::<&str, HashSet<&str>>::new();

        for (name, module) in &modules {
            let recipients = module.get_recipients();
            for recipient in &recipients.0 {
                if let Some(inputs) = input_list.get_mut(recipient) {
                    inputs.insert(name);
                } else {
                    let mut new_set = HashSet::<&str>::new();
                    new_set.insert(name);
                    input_list.insert(recipient, new_set);
                }
            }
        }

        for (name, module) in &mut modules {
            match module {
                Module::Conjunction(state) => {
                    for name in input_list.get(name).unwrap() {
                        state.inputs.insert(name, Pulse::Low);
                    }
                }
                _ => (),
            }
        }

        let mut high_pulses = 0;
        let mut low_pulses = 0;

        let mut pulse_queue = VecDeque::new();
        for _ in 0..1000 {
            // println!("------RUN: {}---------", i);
            pulse_queue.push_front(("button", Pulse::Low, "broadcast"));

            while let Some((from, pulse, to)) = pulse_queue.pop_back() {
                // println!("{} -{}-> {}", from, pulse, to);
                match pulse {
                    Pulse::Low => low_pulses += 1,
                    Pulse::High => high_pulses += 1,
                }
                if let Some(m) = modules.get_mut(to) {
                    m.recieve_pulse(from, pulse, &mut pulse_queue);
                }
            }
            // dbg!(&modules);
        }
        (high_pulses * low_pulses).to_string()
    }

    fn do_task_2(&self) -> String {
        let contents: &str = include_str!("../input/day_20/input");
        let mut modules = HashMap::new();
        for line in contents.lines() {
            create_module(&mut modules, line);
        }

        let mut input_list = HashMap::<&str, HashSet<&str>>::new();

        for (name, module) in &modules {
            let recipients = module.get_recipients();
            for recipient in &recipients.0 {
                if let Some(inputs) = input_list.get_mut(recipient) {
                    inputs.insert(name);
                } else {
                    let mut new_set = HashSet::<&str>::new();
                    new_set.insert(name);
                    input_list.insert(recipient, new_set);
                }
            }
        }

        for (name, module) in &mut modules {
            match module {
                Module::Conjunction(state) => {
                    for name in input_list.get(name).unwrap() {
                        state.inputs.insert(name, Pulse::Low);
                    }
                }
                _ => (),
            }
        }

        let mut pulse_queue = VecDeque::new();
        let mut qz = i64::MAX;
        let mut cq = i64::MAX;
        let mut jx = i64::MAX;
        let mut tt = i64::MAX;

        for i in 1.. {
            pulse_queue.push_front(("button", Pulse::Low, "broadcast"));

            while let Some((from, pulse, to)) = pulse_queue.pop_back() {
                if from == "qz" && qz == i64::MAX && pulse == Pulse::High {
                    qz = i;
                } else if from == "cq" && cq == i64::MAX && pulse == Pulse::High {
                    cq = i;
                } else if from == "jx" && jx == i64::MAX && pulse == Pulse::High {
                    jx = i;
                } else if from == "tt" && tt == i64::MAX && pulse == Pulse::High {
                    tt = i;
                }

                if qz != i64::MAX && cq != i64::MAX && jx != i64::MAX && tt != i64::MAX {
                    return lcm(qz, lcm(cq, lcm(jx, tt))).to_string();
                }

                if let Some(m) = modules.get_mut(to) {
                    m.recieve_pulse(from, pulse, &mut pulse_queue);
                }
            }
        }
        panic!("No way to exit loop early");
    }

    fn task_1_result(&self) -> Option<String> {
        Some("925955316".to_owned())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("241528477694627".to_owned())
    }
}
