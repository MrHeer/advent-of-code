use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    rc::{Rc, Weak},
    vec,
};

use num::integer::lcm;

advent_of_code::solution!(20);

const BROADCASTER_NAME: &str = "broadcaster";
const BUTTON_NAME: &str = "button";
const FLIP_FLOP_PREFIX: &str = "%";
const CONJUNCTION_PREFIX: &str = "&";

#[derive(Copy, Clone, PartialEq, Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    On,
    Off,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ModuleKind {
    Untyped,
    FlipFlop,
    Conjunction,
    Broadcast,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
        }
    }
}

type Address = String;

trait Module {
    fn get_address(&self) -> Address;
    fn add_destination(&mut self, other: Weak<RefCell<dyn Module>>);
    fn get_destinations(&self) -> Vec<Weak<RefCell<dyn Module>>>;
    fn connect_from(&mut self, from: Address);
    fn get_connected(&self) -> Vec<Address>;
    fn receive(&mut self, from: Address, pulse: Pulse);
    fn get_output_pulse(&self) -> Option<Pulse>;
    fn is_init_state(&self) -> bool;
    fn push_history(&mut self, pulse: Pulse);
    fn get_history(&self) -> Vec<Pulse>;
    fn is_enable(&self) -> bool;
    fn get_kind(&self) -> ModuleKind;

    fn connect_to(&mut self, to: Weak<RefCell<dyn Module>>) {
        self.add_destination(to.clone());
        to.upgrade()
            .unwrap()
            .borrow_mut()
            .connect_from(self.get_address());
    }
    fn send(&mut self, pulse: Pulse) {
        self.get_destinations().into_iter().for_each(|module| {
            self.push_history(pulse);
            let from_address = self.get_address();
            module
                .upgrade()
                .unwrap()
                .borrow_mut()
                .receive(from_address, pulse);
        });
    }
}

impl State {
    fn flip(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

impl Pulse {
    fn flip(&self) -> Self {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

struct Untyped {
    name: String,
    output_pulse: Option<Pulse>,
    destinations: Vec<Weak<RefCell<dyn Module>>>,
    connected: Vec<Address>,
    history: Vec<Pulse>,
}

struct FlipFlop {
    base: Untyped,
    state: State,
    enabled: bool,
}

struct Conjunction {
    base: Untyped,
    last_pulse: HashMap<String, Pulse>,
}

struct Broadcast {
    base: Untyped,
}

impl Untyped {
    fn set_output_pulse(&mut self, pulse: Pulse) {
        self.output_pulse = Some(pulse);
    }
}

impl Module for Untyped {
    fn get_address(&self) -> Address {
        self.name.clone()
    }

    fn add_destination(&mut self, other: Weak<RefCell<dyn Module>>) {
        self.destinations.push(other);
    }

    fn get_destinations(&self) -> Vec<Weak<RefCell<dyn Module>>> {
        self.destinations.clone()
    }

    fn connect_from(&mut self, from: Address) {
        self.connected.push(from)
    }

    fn get_connected(&self) -> Vec<Address> {
        self.connected.clone()
    }

    fn receive(&mut self, _from: Address, _pulse: Pulse) {}

    fn get_output_pulse(&self) -> Option<Pulse> {
        self.output_pulse
    }

    fn is_init_state(&self) -> bool {
        true
    }

    fn push_history(&mut self, pulse: Pulse) {
        self.history.push(pulse);
    }

    fn get_history(&self) -> Vec<Pulse> {
        self.history.clone()
    }

    fn is_enable(&self) -> bool {
        true
    }

    fn get_kind(&self) -> ModuleKind {
        ModuleKind::Untyped
    }
}

impl Module for FlipFlop {
    fn get_address(&self) -> Address {
        self.base.get_address()
    }

    fn add_destination(&mut self, other: Weak<RefCell<dyn Module>>) {
        self.base.add_destination(other);
    }

    fn get_destinations(&self) -> Vec<Weak<RefCell<dyn Module>>> {
        self.base.get_destinations()
    }

    fn connect_from(&mut self, from: Address) {
        self.base.connect_from(from);
    }

    fn get_connected(&self) -> Vec<Address> {
        self.base.get_connected()
    }

    fn receive(&mut self, _from: Address, pulse: Pulse) {
        use Pulse::*;
        use State::*;
        match (pulse, &self.state) {
            (High, _) => self.enabled = false,
            (Low, On) => {
                self.state = self.state.flip();
                self.base.set_output_pulse(Low);
                self.enabled = true;
            }
            (Low, Off) => {
                self.state = self.state.flip();
                self.base.set_output_pulse(High);
                self.enabled = true;
            }
        }
    }

    fn get_output_pulse(&self) -> Option<Pulse> {
        self.base.get_output_pulse()
    }

    fn is_init_state(&self) -> bool {
        self.state == State::Off
    }

    fn push_history(&mut self, pulse: Pulse) {
        self.base.push_history(pulse);
    }

    fn get_history(&self) -> Vec<Pulse> {
        self.base.get_history()
    }

    fn is_enable(&self) -> bool {
        self.enabled
    }

    fn get_kind(&self) -> ModuleKind {
        ModuleKind::FlipFlop
    }
}

impl Module for Conjunction {
    fn get_address(&self) -> Address {
        self.base.get_address()
    }

    fn add_destination(&mut self, other: Weak<RefCell<dyn Module>>) {
        self.base.add_destination(other);
    }

    fn get_destinations(&self) -> Vec<Weak<RefCell<dyn Module>>> {
        self.base.get_destinations()
    }

    fn connect_from(&mut self, from: Address) {
        self.base.connect_from(from.clone());
        self.last_pulse.insert(from, Pulse::Low);
    }

    fn get_connected(&self) -> Vec<Address> {
        self.base.get_connected()
    }

    fn receive(&mut self, from: Address, pulse: Pulse) {
        self.last_pulse.insert(from, pulse);
        use Pulse::*;
        let output_pulse = match self.last_pulse.values().all(|pulse| *pulse == High) {
            true => Low,
            false => High,
        };
        self.base.set_output_pulse(output_pulse);
    }

    fn get_output_pulse(&self) -> Option<Pulse> {
        self.base.get_output_pulse()
    }

    fn is_init_state(&self) -> bool {
        self.last_pulse.values().all(|pulse| *pulse == Pulse::Low)
    }

    fn push_history(&mut self, pulse: Pulse) {
        self.base.push_history(pulse);
    }

    fn get_history(&self) -> Vec<Pulse> {
        self.base.get_history()
    }

    fn is_enable(&self) -> bool {
        self.base.is_enable()
    }

    fn get_kind(&self) -> ModuleKind {
        ModuleKind::Conjunction
    }
}

impl Module for Broadcast {
    fn get_address(&self) -> Address {
        self.base.get_address()
    }

    fn add_destination(&mut self, other: Weak<RefCell<dyn Module>>) {
        self.base.add_destination(other);
    }

    fn get_destinations(&self) -> Vec<Weak<RefCell<dyn Module>>> {
        self.base.get_destinations()
    }

    fn connect_from(&mut self, from: Address) {
        self.base.connect_from(from);
    }

    fn get_connected(&self) -> Vec<Address> {
        self.base.get_connected()
    }

    fn receive(&mut self, _from: Address, pulse: Pulse) {
        self.base.set_output_pulse(pulse);
    }

    fn get_output_pulse(&self) -> Option<Pulse> {
        self.base.get_output_pulse()
    }

    fn is_init_state(&self) -> bool {
        self.base.is_init_state()
    }

    fn push_history(&mut self, pulse: Pulse) {
        self.base.push_history(pulse);
    }

    fn get_history(&self) -> Vec<Pulse> {
        self.base.get_history()
    }

    fn is_enable(&self) -> bool {
        self.base.is_enable()
    }

    fn get_kind(&self) -> ModuleKind {
        ModuleKind::FlipFlop
    }
}

impl Untyped {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            output_pulse: None,
            destinations: vec![],
            connected: vec![],
            history: vec![],
        }
    }
}

impl FlipFlop {
    fn new(name: &str) -> Self {
        Self {
            state: State::Off,
            base: Untyped::new(name),
            enabled: false,
        }
    }
}

impl Conjunction {
    fn new(name: &str) -> Self {
        Self {
            base: Untyped::new(name),
            last_pulse: HashMap::new(),
        }
    }
}

impl Broadcast {
    fn new(name: &str) -> Self {
        Self {
            base: Untyped::new(name),
        }
    }
}

struct Solver {
    modules: HashMap<Address, Rc<RefCell<dyn Module>>>,
}

impl Solver {
    fn new(modules: HashMap<Address, Rc<RefCell<dyn Module>>>) -> Self {
        Self { modules }
    }

    fn get_module(&self, address: &Address) -> Rc<RefCell<dyn Module>> {
        self.modules.get(address).unwrap().clone()
    }

    fn get_button(&self) -> Rc<RefCell<dyn Module>> {
        self.get_module(&BUTTON_NAME.to_string())
    }

    fn get_output_pulse(&self, address: &Address) -> Option<Pulse> {
        self.get_module(address).borrow().get_output_pulse()
    }

    fn get_connected(&self, address: &Address) -> Vec<Address> {
        self.get_module(address).borrow().get_connected()
    }

    fn get_all_connected(&self, address: &Address) -> HashSet<Address> {
        let mut all_connected = HashSet::new();
        let mut queue = VecDeque::from_iter(self.get_connected(address));
        while let Some(address) = queue.pop_front() {
            if !all_connected.contains(&address) {
                all_connected.insert(address.clone());
                queue.append(&mut VecDeque::from_iter(
                    self.get_connected(&address).into_iter(),
                ));
            }
        }
        all_connected
    }

    fn is_all_connected_init_state(&self, address: &Address) -> bool {
        self.get_all_connected(address)
            .iter()
            .all(|address| self.get_module(address).borrow().is_init_state())
    }

    fn get_kind(&self, address: &Address) -> ModuleKind {
        self.get_module(address).borrow().get_kind()
    }

    fn is_connected_all_conjunction(&self, address: &Address) -> bool {
        self.get_connected(address)
            .iter()
            .all(|address| self.get_kind(address) == ModuleKind::Conjunction)
    }

    fn split_conditions(&self, address: &Address, pulse: Pulse) -> Option<HashMap<Address, Pulse>> {
        if self.is_connected_all_conjunction(address) {
            let mut map = HashMap::new();
            self.get_connected(address).into_iter().for_each(|address| {
                map.insert(address, pulse.flip());
            });

            return Some(map);
        }

        None
    }

    fn get_conditions(&self, address: &Address, pulse: Pulse) -> Option<HashMap<Address, Pulse>> {
        match self.split_conditions(address, pulse) {
            Some(mut conditions) => {
                let mut stack: Vec<Address> = conditions.keys().cloned().collect();
                while let Some(address) = stack.pop() {
                    let flattened = self.split_conditions(&address, conditions[&address]);
                    if let Some(flattened) = flattened {
                        conditions.remove(&address);
                        stack.append(&mut flattened.keys().cloned().collect());
                        flattened.into_iter().for_each(|(address, pulse)| {
                            conditions.insert(address, pulse);
                        });
                    }
                }

                Some(conditions)
            }
            None => None,
        }
    }

    fn push_button(&self) {
        self.get_button().borrow_mut().send(Pulse::Low);
        let mut modules = VecDeque::from_iter(self.get_button().borrow().get_destinations());
        while let Some(module) = modules.pop_front() {
            if let Some(pulse) = {
                let upgrade = module.upgrade().unwrap();
                let borrowed = upgrade.borrow();
                borrowed.get_output_pulse()
            } {
                module.upgrade().unwrap().borrow_mut().send(pulse);
                modules.append(&mut VecDeque::from_iter(
                    module
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .get_destinations()
                        .into_iter()
                        .filter(|module| module.upgrade().unwrap().borrow().is_enable()),
                ));
            }
        }
    }

    fn solve_part_one(&self) -> Option<usize> {
        use Pulse::*;
        (0..1000).for_each(|_| self.push_button());
        let (low_count, hight_count) = self
            .modules
            .values()
            .flat_map(|module| module.borrow().get_history())
            .fold((0, 0), |mut acc, pulse| {
                match pulse {
                    Low => acc.0 += 1,
                    High => acc.1 += 1,
                }
                acc
            });
        Some(low_count * hight_count)
    }

    fn solve_part_two_by_conditions(&self, conditions: HashMap<Address, Pulse>) -> Option<usize> {
        let mut count = 0;
        let mut cycles: HashMap<Address, usize> = HashMap::new();
        loop {
            self.push_button();
            count += 1;
            conditions.iter().for_each(|(address, pulse)| {
                if self.get_output_pulse(address) == Some(*pulse)
                    && !cycles.contains_key(address)
                    && self.is_all_connected_init_state(address)
                {
                    cycles.insert(address.clone(), count);
                }
            });

            if cycles.len() == conditions.len() {
                return cycles.values().cloned().reduce(lcm);
            }
        }
    }

    fn solve_part_two_brute(&self, target: &Address, pulse: Pulse) -> Option<usize> {
        let mut count = 0;
        loop {
            self.push_button();
            count += 1;
            if self.get_output_pulse(target) == Some(pulse) {
                return Some(count);
            }
        }
    }

    fn solve_part_two(&self) -> Option<usize> {
        let target: Address = "rx".to_string();
        let pulse = Pulse::Low;
        let conditions = self.get_conditions(&target, Pulse::Low);
        match conditions {
            Some(conditions) => self.solve_part_two_by_conditions(conditions),
            None => self.solve_part_two_brute(&target, pulse),
        }
    }
}

struct ModuleInfo {
    name: String,
    kind: ModuleKind,
}

impl From<&str> for ModuleInfo {
    fn from(value: &str) -> Self {
        use ModuleKind::*;
        if value == BROADCASTER_NAME {
            Self {
                name: value.to_string(),
                kind: Broadcast,
            }
        } else if value.starts_with(FLIP_FLOP_PREFIX) {
            Self {
                name: value.strip_prefix(FLIP_FLOP_PREFIX).unwrap().to_string(),
                kind: FlipFlop,
            }
        } else if value.starts_with(CONJUNCTION_PREFIX) {
            Self {
                name: value.strip_prefix(CONJUNCTION_PREFIX).unwrap().to_string(),
                kind: Conjunction,
            }
        } else {
            Self {
                name: value.to_string(),
                kind: Untyped,
            }
        }
    }
}

fn create_module(module_info: ModuleInfo) -> Rc<RefCell<dyn Module>> {
    match module_info.kind {
        ModuleKind::Untyped => Rc::new(RefCell::new(Untyped::new(&module_info.name))),
        ModuleKind::FlipFlop => Rc::new(RefCell::new(FlipFlop::new(&module_info.name))),
        ModuleKind::Conjunction => Rc::new(RefCell::new(Conjunction::new(&module_info.name))),
        ModuleKind::Broadcast => Rc::new(RefCell::new(Broadcast::new(&module_info.name))),
    }
}

impl From<&str> for Solver {
    fn from(value: &str) -> Self {
        let mut modules: HashMap<Address, Rc<RefCell<dyn Module>>> = HashMap::new();

        // initialize modules
        value
            .lines()
            .map(|line| line.split(" -> ").next().unwrap())
            .for_each(|module_info| {
                let module = create_module(module_info.into());
                modules.insert(module.clone().borrow().get_address(), module);
            });

        let mut get_or_create_module = |module_info: ModuleInfo| {
            modules
                .entry(module_info.name.clone())
                .or_insert(create_module(module_info))
                .clone()
        };

        // connect the modules
        value.lines().for_each(|line| {
            let mut parts = line.split(" -> ");
            let module_info: ModuleInfo = parts.next().unwrap().into();
            let module = get_or_create_module(module_info);

            parts
                .next()
                .unwrap()
                .split(", ")
                .for_each(|connect_io_info| {
                    let to_module_info: ModuleInfo = connect_io_info.into();
                    let to_module = Rc::downgrade(&get_or_create_module(to_module_info));
                    module.borrow_mut().connect_to(to_module);
                });
        });

        // connect button to broadcaster
        let button = get_or_create_module(ModuleInfo::from(BUTTON_NAME));
        let broadcaster = get_or_create_module(ModuleInfo::from(BROADCASTER_NAME));
        button.borrow_mut().connect_to(Rc::downgrade(&broadcaster));

        Self::new(modules)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Solver::from(input).solve_part_one()
}

pub fn part_two(input: &str) -> Option<usize> {
    Solver::from(input).solve_part_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, None);
    }
}
