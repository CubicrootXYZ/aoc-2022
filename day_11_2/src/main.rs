mod monkeys;

use monkeys::{get_monkeys, get_test_monkeys};
use std::collections::VecDeque;

fn main() {
    let mut monkeys = get_test_monkeys();

    for i in 0..1000 * monkeys.len() {
        let monkey_num = i % monkeys.len();

        let mut mv = [0, 0];
        while mv[0] >= 0 {
            let m = monkeys.get_mut(monkey_num).unwrap(); // Outside of the loop it creates an error since we can not borrow a mut twice at the same time
            mv = m.get_move();
            if mv[0] == 0 {
                break;
            }

            let m_next = monkeys.get_mut(mv[1] as usize).unwrap();
            m_next.add_item(mv[0]);
            println!("Moved item {} from {} to {}", mv[0], monkey_num, mv[1])
        }
    }

    let mut max = 0;
    let mut max2 = 0;
    for monkey in monkeys {
        if monkey.inspected_items > max {
            max2 = max;
            max = monkey.inspected_items;
            continue;
        }
        if monkey.inspected_items > max2 {
            max2 = monkey.inspected_items;
        }
    }

    println!("Monkey businees is: {} * {} = {}", max, max2, max * max2)
}
