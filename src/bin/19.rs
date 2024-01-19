use std::{collections::HashMap, ops::Range};

advent_of_code::solution!(19);

struct Part {
    looking: usize,
    musical: usize,
    aerodynamic: usize,
    shiny: usize,
}

#[derive(Copy, Clone)]
enum Category {
    Looking,
    Musical,
    Aerodynamic,
    Shiny,
}

enum Operator {
    Less,
    Greater,
}

struct Condition {
    category: Category,
    operator: Operator,
    rate: usize,
}

#[derive(PartialEq, Eq)]
enum Finished {
    Accepted,
    Rejected,
}

#[derive(PartialEq, Eq)]
enum Destination {
    Workflow(String),
    End(Finished),
}

struct Rule {
    condition: Option<Condition>,
    destination: Destination,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

struct Solver {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let rate_info = &value.replace(['{', '}'], "");
        let mut rates = rate_info.split(',');
        let looking = Part::extract_rate(rates.next().unwrap());
        let musical = Part::extract_rate(rates.next().unwrap());
        let aerodynamic = Part::extract_rate(rates.next().unwrap());
        let shiny = Part::extract_rate(rates.next().unwrap());
        Self {
            looking,
            musical,
            aerodynamic,
            shiny,
        }
    }
}

impl Part {
    fn extract_rate(value: &str) -> usize {
        value.split('=').last().unwrap().parse().unwrap()
    }

    fn rating(&self) -> usize {
        self.looking + self.musical + self.aerodynamic + self.shiny
    }

    fn get_rate(&self, category: &Category) -> usize {
        use Category::*;
        match category {
            Looking => self.looking,
            Musical => self.musical,
            Aerodynamic => self.aerodynamic,
            Shiny => self.shiny,
        }
    }
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::Looking,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Invalid category"),
        }
    }
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Less,
            '>' => Self::Greater,
            _ => panic!("Invalid operator"),
        }
    }
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let category = chars.next().unwrap().into();
        let operator = chars.next().unwrap().into();
        let rate = String::from_iter(chars).parse().unwrap();
        Self {
            category,
            operator,
            rate,
        }
    }
}

impl Condition {
    fn resolve(&self, part: &Part) -> bool {
        use Operator::*;
        let value = part.get_rate(&self.category);
        match self.operator {
            Less => value < self.rate,
            Greater => value > self.rate,
        }
    }
}

impl From<&str> for Finished {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => panic!("Invalid finished state"),
        }
    }
}

impl From<&str> for Destination {
    fn from(value: &str) -> Self {
        use Finished::*;
        match value {
            "A" => Self::End(Accepted),
            "R" => Self::End(Rejected),
            _ => Self::Workflow(value.to_string()),
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if value.contains(':') {
            let mut parts = value.split(':');
            let condition = Some(parts.next().unwrap().into());
            let destination = parts.next().unwrap().into();
            return Self {
                condition,
                destination,
            };
        }
        Self {
            condition: None,
            destination: value.into(),
        }
    }
}

impl Rule {
    fn resolve(&self, part: &Part) -> Option<&Destination> {
        match &self.condition {
            Some(condition) => match condition.resolve(part) {
                true => Some(&self.destination),
                false => None,
            },
            None => Some(&self.destination),
        }
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let mut value = value.to_string();
        value.pop();
        let mut parts = value.split('{');
        let name = parts.next().unwrap().to_string();
        let rules = parts.next().unwrap().split(',').map(Rule::from).collect();
        Self { name, rules }
    }
}

impl Workflow {
    fn resolve(&self, part: &Part) -> &Destination {
        for rule in &self.rules {
            match rule.resolve(part) {
                None => continue,
                Some(destination) => return destination,
            }
        }
        panic!("No destination found");
    }
}

impl From<&str> for Solver {
    fn from(value: &str) -> Self {
        let mut parts = value.split("\n\n");
        let workflows = HashMap::from_iter(
            parts
                .next()
                .unwrap()
                .lines()
                .map(Workflow::from)
                .map(|workflow| (workflow.name.clone(), workflow)),
        );
        let parts = parts.next().unwrap().lines().map(Part::from).collect();
        Self { workflows, parts }
    }
}

impl Solver {
    fn first_workflow(&self) -> &Workflow {
        &self.workflows["in"]
    }

    fn resolve(&self, part: &Part) -> &Finished {
        let mut workflow = self.first_workflow();
        use Destination::*;
        loop {
            match workflow.resolve(part) {
                Workflow(name) => workflow = &self.workflows[name],
                End(result) => return result,
            }
        }
    }

    fn rating_sum(&self) -> usize {
        use Finished::*;
        self.parts
            .iter()
            .filter(|part| self.resolve(part) == &Accepted)
            .map(|part| part.rating())
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Solver::from(input).rating_sum())
}

#[derive(Clone)]
struct Segment {
    looking: Range<usize>,
    musical: Range<usize>,
    aerodynamic: Range<usize>,
    shiny: Range<usize>,
}

impl Default for Segment {
    fn default() -> Self {
        Self {
            looking: 1..4001,
            musical: 1..4001,
            aerodynamic: 1..4001,
            shiny: 1..4001,
        }
    }
}

impl Segment {
    fn distinct_combinations(&self) -> usize {
        self.looking.len() * self.musical.len() * self.aerodynamic.len() * self.shiny.len()
    }

    fn update_segment(mut self, condition: &Condition) -> Self {
        use Category::*;
        use Operator::*;
        let Condition {
            category,
            operator,
            rate,
        } = condition;
        let Segment {
            looking,
            musical,
            aerodynamic,
            shiny,
        } = &mut self;
        let rate = *rate;
        match (category, operator) {
            (Looking, Less) => looking.end = rate,
            (Looking, Greater) => looking.start = rate + 1,
            (Musical, Less) => musical.end = rate,
            (Musical, Greater) => musical.start = rate + 1,
            (Aerodynamic, Less) => aerodynamic.end = rate,
            (Aerodynamic, Greater) => aerodynamic.start = rate + 1,
            (Shiny, Less) => shiny.end = rate,
            (Shiny, Greater) => shiny.start = rate + 1,
        }
        self
    }
}

impl Condition {
    fn reverse(&self) -> Self {
        use Operator::*;
        let category = self.category;
        let operator = &self.operator;
        let rate = self.rate;
        match operator {
            Less => Self {
                category,
                operator: Greater,
                rate: rate - 1,
            },
            Greater => Self {
                category,
                operator: Less,
                rate: rate + 1,
            },
        }
    }
}

impl Solver {
    fn get_accepted_segments(
        &self,
        segment: Segment,
        work_flow: &Workflow,
        rule_index: usize,
    ) -> Vec<Segment> {
        use Destination::*;
        use Finished::*;
        let rule = &work_flow.rules[rule_index];
        match (&rule.condition, &rule.destination) {
            (None, End(Rejected)) => vec![],
            (None, End(Accepted)) => vec![segment],
            (None, Workflow(next_flow_name)) => {
                self.get_accepted_segments(segment, &self.workflows[next_flow_name], 0)
            }
            (Some(condition), End(Accepted)) => [
                vec![segment.clone().update_segment(condition)],
                self.get_accepted_segments(
                    segment.update_segment(&condition.reverse()),
                    work_flow,
                    rule_index + 1,
                ),
            ]
            .concat(),
            (Some(condition), End(Rejected)) => self.get_accepted_segments(
                segment.update_segment(&condition.reverse()),
                work_flow,
                rule_index + 1,
            ),
            (Some(condition), Workflow(next_flow_name)) => {
                let next_workflow_segments = self.get_accepted_segments(
                    segment.clone().update_segment(condition),
                    &self.workflows[next_flow_name],
                    0,
                );
                let next_rule_segments = self.get_accepted_segments(
                    segment.update_segment(&condition.reverse()),
                    work_flow,
                    rule_index + 1,
                );
                [next_workflow_segments, next_rule_segments].concat()
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let solver = Solver::from(input);
    let first_workflow = solver.first_workflow();
    let segments = solver.get_accepted_segments(Default::default(), first_workflow, 0);
    Some(
        segments
            .iter()
            .map(|segment| segment.distinct_combinations())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
