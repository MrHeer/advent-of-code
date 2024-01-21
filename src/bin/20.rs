use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

advent_of_code::solution!(20);

const BROADCAST_NAME: &str = "broadcaster";

#[derive(Copy, Clone, PartialEq)]
enum Pulse {
    Low,
    High,
}

enum Prefix {
    FlipFlop,
    Conjunction,
}

impl Pulse {
    fn flip(&self) -> Self {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
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
    fn add_destination(&mut self, other: Rc<RefCell<dyn Module>>);
    fn get_destinations(&self) -> &Vec<Rc<RefCell<dyn Module>>>;
    fn connect_from(&mut self, from: Address);
    fn receive(&mut self, from: Address, pulse: Pulse);
    fn get_pulse(&self) -> Option<Pulse>;

    fn connect_to(&mut self, to: Rc<RefCell<dyn Module>>) {
        self.add_destination(to.clone());
        to.borrow_mut().connect_from(self.get_address());
    }
    fn send(&mut self) {
        if let Some(pulse) = self.get_pulse() {
            self.get_destinations().iter().for_each(|m| {
                let from_address = self.get_address();
                let to_address = m.borrow().get_address();
                println!("{} -{}-> {}", from_address, pulse, to_address);
                m.borrow_mut().receive(from_address, pulse);
            });
            self.get_destinations().iter().for_each(|m| {
                m.borrow_mut().send();
            });
        }
    }
}

enum State {
    On,
    Off,
}

impl State {
    fn flip(&self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

struct FlipFlop {
    name: String,
    state: State,
    next_pulse: Option<Pulse>,
    destinations: Vec<Rc<RefCell<dyn Module>>>,
}

struct Conjunction {
    name: String,
    next_pulse: Option<Pulse>,
    last_pulse: HashMap<String, Pulse>,
    destinations: Vec<Rc<RefCell<dyn Module>>>,
}

struct Broadcast {
    name: String,
    next_pulse: Option<Pulse>,
    destinations: Vec<Rc<RefCell<dyn Module>>>,
}

struct Untyped {
    name: String,
    destinations: Vec<Rc<RefCell<dyn Module>>>,
}

struct Button {
    broadcaster: Rc<RefCell<dyn Module>>,
}

impl Module for FlipFlop {
    fn get_address(&self) -> Address {
        self.name.clone()
    }

    fn add_destination(&mut self, other: Rc<RefCell<dyn Module>>) {
        self.destinations.push(other);
    }

    fn get_destinations(&self) -> &Vec<Rc<RefCell<dyn Module>>> {
        &self.destinations
    }

    fn connect_from(&mut self, _from: Address) {}

    fn receive(&mut self, _from: Address, pulse: Pulse) {
        use Pulse::*;
        match pulse {
            Low => {
                self.state = self.state.flip();
                self.next_pulse = Some(pulse.flip());
            }
            High => (),
        }
    }

    fn get_pulse(&self) -> Option<Pulse> {
        self.next_pulse
    }
}

impl Module for Conjunction {
    fn get_address(&self) -> Address {
        self.name.clone()
    }

    fn add_destination(&mut self, other: Rc<RefCell<dyn Module>>) {
        self.destinations.push(other);
    }

    fn get_destinations(&self) -> &Vec<Rc<RefCell<dyn Module>>> {
        &self.destinations
    }

    fn connect_from(&mut self, from: Address) {
        self.last_pulse.insert(from, Pulse::Low);
    }

    fn receive(&mut self, from: Address, pulse: Pulse) {
        self.last_pulse.insert(from, pulse);
        self.next_pulse = match self.last_pulse.values().all(|p| *p == Pulse::High) {
            true => Some(Pulse::Low),
            false => Some(Pulse::High),
        };
    }

    fn get_pulse(&self) -> Option<Pulse> {
        self.next_pulse
    }
}

impl Module for Broadcast {
    fn get_address(&self) -> Address {
        self.name.clone()
    }

    fn add_destination(&mut self, other: Rc<RefCell<dyn Module>>) {
        self.destinations.push(other);
    }

    fn get_destinations(&self) -> &Vec<Rc<RefCell<dyn Module>>> {
        &self.destinations
    }

    fn connect_from(&mut self, _from: Address) {}

    fn receive(&mut self, _from: Address, pulse: Pulse) {
        self.next_pulse = Some(pulse);
    }

    fn get_pulse(&self) -> Option<Pulse> {
        self.next_pulse
    }
}

impl Module for Untyped {
    fn get_address(&self) -> Address {
        self.name.clone()
    }

    fn add_destination(&mut self, _other: Rc<RefCell<dyn Module>>) {}

    fn get_destinations(&self) -> &Vec<Rc<RefCell<dyn Module>>> {
        &self.destinations
    }

    fn connect_from(&mut self, _from: Address) {}

    fn receive(&mut self, _from: Address, _pulse: Pulse) {}

    fn get_pulse(&self) -> Option<Pulse> {
        None
    }
}

impl FlipFlop {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            next_pulse: None,
            state: State::Off,
            destinations: vec![],
        }
    }
}

impl Conjunction {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            next_pulse: None,
            last_pulse: HashMap::new(),
            destinations: vec![],
        }
    }
}

impl Broadcast {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            next_pulse: None,
            destinations: vec![],
        }
    }
}

impl Untyped {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            destinations: vec![],
        }
    }
}

impl Button {
    fn new(broadcaster: Rc<RefCell<dyn Module>>) -> Self {
        Self { broadcaster }
    }

    fn push(&mut self) {
        println!("button -low-> broadcaster");
        self.broadcaster
            .borrow_mut()
            .receive("button".to_string(), Pulse::Low);
        self.broadcaster.borrow_mut().send();
    }
}

impl ToString for Prefix {
    fn to_string(&self) -> String {
        use Prefix::*;
        match self {
            FlipFlop => "%".to_string(),
            Conjunction => "&".to_string(),
        }
    }
}

struct ModuleInfo {
    name: String,
    prefix: Option<Prefix>,
}

impl From<&str> for ModuleInfo {
    fn from(value: &str) -> Self {
        use Prefix::*;
        if value.starts_with(&FlipFlop.to_string()) {
            Self {
                name: value
                    .strip_prefix(&FlipFlop.to_string())
                    .unwrap()
                    .to_string(),
                prefix: Some(FlipFlop),
            }
        } else if value.starts_with(&Conjunction.to_string()) {
            Self {
                name: value
                    .strip_prefix(&Conjunction.to_string())
                    .unwrap()
                    .to_string(),
                prefix: Some(Conjunction),
            }
        } else {
            Self {
                name: value.to_string(),
                prefix: None,
            }
        }
    }
}

fn create_module(module_info: ModuleInfo) -> Rc<RefCell<dyn Module>> {
    use Prefix::*;
    match (&module_info.name, module_info.prefix) {
        (name, None) => {
            if name == BROADCAST_NAME {
                Rc::new(RefCell::new(Broadcast::new(name)))
            } else {
                Rc::new(RefCell::new(Untyped::new(name)))
            }
        }
        (name, Some(FlipFlop)) => Rc::new(RefCell::new(crate::FlipFlop::new(name))),
        (name, Some(Conjunction)) => Rc::new(RefCell::new(crate::Conjunction::new(name))),
    }
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        const BROADCAST_NAME: &str = "broadcaster";
        let mut modules: HashMap<Address, Rc<RefCell<dyn Module>>> = HashMap::new();

        // initialize modules
        value
            .lines()
            .map(|line| line.split(" -> ").next().unwrap())
            .for_each(|module_info| {
                let module = create_module(module_info.into());
                modules.insert(module.clone().borrow().get_address(), module);
            });

        let mut get_module = |module_info: ModuleInfo| {
            modules
                .entry(module_info.name.clone())
                .or_insert(create_module(module_info))
                .clone()
        };

        // connect the modules
        value.lines().for_each(|line| {
            let mut parts = line.split(" -> ");
            let module_info: ModuleInfo = parts.next().unwrap().into();
            let module = get_module(module_info);

            parts
                .next()
                .unwrap()
                .split(", ")
                .for_each(|connect_io_info| {
                    let to_module_info: ModuleInfo = connect_io_info.into();
                    let to_module = get_module(to_module_info);
                    module.borrow_mut().connect_to(to_module);
                });
        });

        Button::new(modules.get(BROADCAST_NAME).unwrap().clone())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Button::from(input).push();
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
