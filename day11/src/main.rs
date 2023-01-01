mod monkey;
use monkey::*;

fn main() {
    let mut monkeys = MonkeyGroup::parse(include_str!("input.txt"));
    println!("{:?}", monkeys);

    (0..20).for_each(|_| monkeys.step_round());

    println!("The level of monkey business is {}", monkeys.monkey_business());
}
