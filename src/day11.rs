use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day11;

pub struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    true_dest: Option<Rc<RefCell<Monkey>>>,
    false_dest: Option<Rc<RefCell<Monkey>>>,
    inspections: u32,
}

impl Monkey {
    fn new_rc(
        items: Vec<u64>,
        operation: Box<dyn Fn(u64) -> u64>,
        test: Box<dyn Fn(u64) -> bool>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(
            Monkey {
                items: VecDeque::try_from(items).unwrap(),
                operation,
                test,
                true_dest: None,
                false_dest: None,
                inspections: 0,
            }
            .into(),
        )
    }
}

impl Solution for Day11 {
    type ParsedInput = String;

    fn parse_input(_input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        0.to_string()
    }

    fn part_one(_input: &mut Self::ParsedInput) -> String {
        let mut monkeys = create_monkeys();
        for _round in 0..20 {
            for monkey in monkeys.iter_mut() {
                (**monkey).borrow_mut().handle_round();
            }
        }
        let mut inspections = monkeys
            .iter()
            .map(|x| (**x).borrow().inspections)
            .collect::<Vec<u32>>();
        inspections.sort();

        (inspections[inspections.len() - 1] * inspections[inspections.len() - 2]).to_string()
    }

    fn part_two(_input: &mut Self::ParsedInput) -> String {
        let mut monkeys = create_monkeys();
        for _round in 0..10000 {
            for monkey in monkeys.iter_mut() {
                (**monkey).borrow_mut().handle_round_p2();
            }
        }
        let mut inspections = monkeys
            .iter()
            .map(|x| (**x).borrow().inspections)
            .collect::<Vec<u32>>();
        // println!("{:?}", inspections);
        inspections.sort();
        // println!("{:?}", inspections);
        // println!(
        //     "{:?}, {:?}",
        //     inspections[inspections.len() - 1],
        //     inspections[inspections.len() - 2]
        // );
        ((inspections[inspections.len() - 1] as u64) * (inspections[inspections.len() - 2] as u64))
            .to_string()
    }
}

impl Monkey {
    fn inspect(&mut self) {
        self.items[0] = (self.operation)(self.items[0]) / 3;
        self.inspections += 1;
    }

    fn inspect_p2(&mut self) {
        self.items[0] = (self.operation)(self.items[0]) % (2 * 17 * 19 * 3 * 5 * 13 * 7 * 11);
        self.inspections += 1;
    }

    fn test(&mut self) -> bool {
        (self.test)(self.items[0])
    }

    fn post_inspection(&mut self) {
        let test_passed = self.test();
        let item_to_throw = self.items.pop_front().expect("No item to throw");
        if test_passed {
            self.true_dest
                .clone()
                .map(|inner| (*inner).borrow_mut().catch_item(item_to_throw));
        } else {
            self.false_dest
                .clone()
                .map(|inner| (*inner).borrow_mut().catch_item(item_to_throw));
        }
    }

    fn handle_next_item(&mut self) {
        self.inspect();
        self.post_inspection();
    }

    fn handle_next_item_p2(&mut self) {
        self.inspect_p2();
        self.post_inspection();
    }

    fn handle_round(&mut self) {
        for _i in 0..self.items.len() {
            self.handle_next_item();
        }
    }

    fn handle_round_p2(&mut self) {
        for _i in 0..self.items.len() {
            self.handle_next_item_p2();
        }
    }

    fn set_true_mk(&mut self, monkey: Rc<RefCell<Monkey>>) {
        self.true_dest = Some(monkey);
    }

    fn set_false_mk(&mut self, monkey: Rc<RefCell<Monkey>>) {
        self.false_dest = Some(monkey);
    }

    fn catch_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn create_monkeys() -> Vec<Rc<RefCell<Monkey>>> {
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    //I'm building my monkeys manually because there aren't many and parsing that looks like it would suck.
    monkeys.push(Monkey::new_rc(
        vec![83, 62, 93],
        Box::new(|x| x.checked_mul(17).unwrap()),
        Box::new(|x| x % 2 == 0),
    ));
    monkeys.push(Monkey::new_rc(
        vec![90, 55],
        Box::new(|x| x.checked_add(1).unwrap()),
        Box::new(|x| x % 17 == 0),
    ));
    monkeys.push(Monkey::new_rc(
        vec![91, 78, 80, 97, 79, 88],
        Box::new(|x| x.checked_add(3).unwrap()),
        Box::new(|x| x % 19 == 0),
    ));
    monkeys.push(Monkey::new_rc(
        vec![64, 80, 83, 89, 59],
        Box::new(|x| x.checked_add(5).unwrap()),
        Box::new(|x| x % 3 == 0),
    ));
    monkeys.push(Monkey::new_rc(
        vec![98, 92, 99, 51],
        Box::new(|x| x.checked_mul(x).unwrap()),
        Box::new(|x| x % 5 == 0),
    ));
    monkeys.push(Monkey::new_rc(
        vec![68, 57, 95, 85, 98, 75, 98, 75],
        Box::new(|x| x.checked_add(2).unwrap()),
        Box::new(|x| x % 13 == 0),
    ));
    monkeys.push(Monkey::new_rc(
        vec![74],
        Box::new(|x| x.checked_add(4).unwrap()),
        Box::new(|x| x % 7 == 0),
    ));
    monkeys.push(Monkey::new_rc(
        vec![68, 64, 60, 68, 87, 80, 82],
        Box::new(|x| x.checked_mul(19).unwrap()),
        Box::new(|x| x % 11 == 0),
    ));
    (*monkeys[0]).borrow_mut().set_true_mk(monkeys[1].clone());
    (*monkeys[0]).borrow_mut().set_false_mk(monkeys[6].clone());
    (*monkeys[1]).borrow_mut().set_true_mk(monkeys[6].clone());
    (*monkeys[1]).borrow_mut().set_false_mk(monkeys[3].clone());
    (*monkeys[2]).borrow_mut().set_true_mk(monkeys[7].clone());
    (*monkeys[2]).borrow_mut().set_false_mk(monkeys[5].clone());
    (*monkeys[3]).borrow_mut().set_true_mk(monkeys[7].clone());
    (*monkeys[3]).borrow_mut().set_false_mk(monkeys[2].clone());
    (*monkeys[4]).borrow_mut().set_true_mk(monkeys[0].clone());
    (*monkeys[4]).borrow_mut().set_false_mk(monkeys[1].clone());
    (*monkeys[5]).borrow_mut().set_true_mk(monkeys[4].clone());
    (*monkeys[5]).borrow_mut().set_false_mk(monkeys[0].clone());
    (*monkeys[6]).borrow_mut().set_true_mk(monkeys[3].clone());
    (*monkeys[6]).borrow_mut().set_false_mk(monkeys[2].clone());
    (*monkeys[7]).borrow_mut().set_true_mk(monkeys[4].clone());
    (*monkeys[7]).borrow_mut().set_false_mk(monkeys[5].clone());
    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day11_part1_case1() {
        assert_eq!(Day11::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day11_part2_case1() {
        assert_eq!(Day11::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(Day11::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
