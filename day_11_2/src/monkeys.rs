use std::collections::{vec_deque, VecDeque};

pub struct Monkey {
    items: VecDeque<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> bool>,
    throw_true: i64,
    throw_false: i64,
    pub inspected_items: i64,
}

impl Monkey {
    // worry level, next monkey id
    pub fn get_move(&mut self) -> [i64; 2] {
        if self.items.len() == 0 {
            return [-1; 2];
        }

        self.inspected_items += 1;
        let item = self.items.pop_front().unwrap();
        let op = &self.operation;
        let t = &self.test;
        let mut worry_level = op(item); // TODO avoid overflow

        let mut next_monkey = self.throw_false;
        if t(worry_level) {
            next_monkey = self.throw_true;
        }

        // we search n so that
        // (n * x) % a == o
        // (n * x) % b == p
        // therefore
        // n % a == o
        // n % b == p
        // must be true too, this is given if n is a common denominator of a and b, now do the same for all 8 monkeys :)
        return [worry_level % 9699690, next_monkey];
    }

    pub fn add_item(&mut self, item: i64) {
        self.items.push_back(item)
    }
}

pub fn get_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    // Monkey 0
    let m_13 = |x: i64| x * 13;
    let d_11 = |x: i64| x % 11 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(m_13),
        test: Box::new(d_11),
        throw_true: 4,
        throw_false: 7,
        inspected_items: 0,
    };
    m.items.push_back(98);
    m.items.push_back(97);
    m.items.push_back(98);
    m.items.push_back(55);
    m.items.push_back(56);
    m.items.push_back(72);
    monkeys.push(m);

    // Monkey 1
    let a_4 = |x: i64| x + 4;
    let d_17 = |x: i64| x % 17 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(a_4),
        test: Box::new(d_17),
        throw_true: 2,
        throw_false: 6,
        inspected_items: 0,
    };
    m.items.push_back(73);
    m.items.push_back(99);
    m.items.push_back(55);
    m.items.push_back(54);
    m.items.push_back(88);
    m.items.push_back(50);
    m.items.push_back(55);
    monkeys.push(m);

    // Monkey 2
    let m_11 = |x: i64| x * 11;
    let d_5 = |x: i64| x % 5 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(m_11),
        test: Box::new(d_5),
        throw_true: 6,
        throw_false: 5,
        inspected_items: 0,
    };
    m.items.push_back(67);
    m.items.push_back(98);
    monkeys.push(m);

    // Monkey 3
    let a_8 = |x: i64| x + 8;
    let d_13 = |x: i64| x % 13 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(a_8),
        test: Box::new(d_13),
        throw_true: 1,
        throw_false: 2,
        inspected_items: 0,
    };
    m.items.push_back(82);
    m.items.push_back(91);
    m.items.push_back(92);
    m.items.push_back(53);
    m.items.push_back(99);
    monkeys.push(m);

    // Monkey 4
    let m_s = |x: i64| x * x;
    let d_19 = |x: i64| x % 19 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(m_s),
        test: Box::new(d_19),
        throw_true: 3,
        throw_false: 1,
        inspected_items: 0,
    };
    m.items.push_back(52);
    m.items.push_back(62);
    m.items.push_back(94);
    m.items.push_back(96);
    m.items.push_back(52);
    m.items.push_back(87);
    m.items.push_back(53);
    m.items.push_back(60);
    monkeys.push(m);

    // Monkey 5
    let a_5 = |x: i64| x + 5;
    let d_2 = |x: i64| x % 2 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(a_5),
        test: Box::new(d_2),
        throw_true: 7,
        throw_false: 0,
        inspected_items: 0,
    };
    m.items.push_back(94);
    m.items.push_back(80);
    m.items.push_back(84);
    m.items.push_back(79);
    monkeys.push(m);

    // Monkey 6
    let a_1 = |x: i64| x + 1;
    let d_3 = |x: i64| x % 3 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(a_1),
        test: Box::new(d_3),
        throw_true: 0,
        throw_false: 5,
        inspected_items: 0,
    };
    m.items.push_back(89);
    monkeys.push(m);

    // Monkey 7
    let a_3 = |x: i64| x + 3;
    let d_7 = |x: i64| x % 7 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(a_3),
        test: Box::new(d_7),
        throw_true: 4,
        throw_false: 3,
        inspected_items: 0,
    };
    m.items.push_back(70);
    m.items.push_back(59);
    m.items.push_back(63);
    monkeys.push(m);

    return monkeys;
}

pub fn get_test_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    // Monkey 0
    let m_19 = |x: i64| x * 19;
    let d_23 = |x: i64| x % 23 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(m_19),
        test: Box::new(d_23),
        throw_true: 2,
        throw_false: 3,
        inspected_items: 0,
    };
    m.items.push_back(79);
    m.items.push_back(98);
    monkeys.push(m);

    // Monkey 1
    let a_6 = |x: i64| x + 6;
    let d_19 = |x: i64| x % 19 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(a_6),
        test: Box::new(d_19),
        throw_true: 2,
        throw_false: 0,
        inspected_items: 0,
    };
    m.items.push_back(54);
    m.items.push_back(65);
    m.items.push_back(75);
    m.items.push_back(74);
    monkeys.push(m);

    // Monkey 2
    let m_s = |x: i64| x * x;
    let d_13 = |x: i64| x % 13 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(m_s),
        test: Box::new(d_13),
        throw_true: 1,
        throw_false: 3,
        inspected_items: 0,
    };
    m.items.push_back(79);
    m.items.push_back(60);
    m.items.push_back(97);
    monkeys.push(m);

    // Monkey 3
    let a_3 = |x: i64| x + 3;
    let d_17 = |x: i64| x % 17 == 0;
    let mut m = Monkey {
        items: VecDeque::new(),
        operation: Box::new(a_3),
        test: Box::new(d_17),
        throw_true: 0,
        throw_false: 1,
        inspected_items: 0,
    };
    m.items.push_back(74);
    monkeys.push(m);

    return monkeys;
}
