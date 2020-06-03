use utm::{Action, Machine, Move, Tape};

// Busy beaver (5-state, 2-symbol)

#[derive(PartialEq, Debug)]
enum BusyBeaver5 {
    A,
    B,
    C,
    D,
    E,
    Halt,
}

impl BusyBeaver5 {
    fn rules(state: &Self, value: &u32) -> Action<Self, u32> {
        // | S0 | V0 | V1 | Dir   | S1   |
        // |----+----+----+-------+------|
        // | A  |  0 |  1 | Right | B    |
        // | A  |  1 |  1 | Left  | C    |
        // | B  |  0 |  1 | Right | C    |
        // | B  |  1 |  1 | Right | B    |
        // | C  |  0 |  1 | Right | D    |
        // | C  |  1 |  0 | Left  | E    |
        // | D  |  0 |  1 | Left  | A    |
        // | D  |  1 |  1 | Left  | D    |
        // | E  |  0 |  1 | Right | Halt |
        // | E  |  1 |  0 | Left  | A    |

        match (state, value) {
            (Self::A, 0) => Action(1, Move::Right, Self::B),
            (Self::A, 1) => Action(1, Move::Left, Self::C),
            (Self::B, 0) => Action(1, Move::Right, Self::C),
            (Self::B, 1) => Action(1, Move::Right, Self::B),
            (Self::C, 0) => Action(1, Move::Right, Self::D),
            (Self::C, 1) => Action(0, Move::Left, Self::E),
            (Self::D, 0) => Action(1, Move::Left, Self::A),
            (Self::D, 1) => Action(1, Move::Left, Self::D),
            (Self::E, 0) => Action(1, Move::Right, Self::Halt),
            (Self::E, 1) => Action(0, Move::Left, Self::A),
            _ => panic!("Invalid operation"),
        }
    }

    fn run() {
        let mut machine = Machine::new(Self::A, Self::Halt, Tape::new(0, vec![]));
        machine.run(Self::rules);
        machine.show_report(Some(1));
    }
}

fn main() {
    BusyBeaver5::run();
}
