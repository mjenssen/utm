use utm::{Action, Machine, Move, Tape};

// Busy beaver (3-state, 2-symbol)

#[derive(PartialEq, Debug)]
enum BusyBeaver3 {
    A,
    B,
    C,
    Halt,
}

impl BusyBeaver3 {
    fn rules(state: &Self, value: &u32) -> Action<Self, u32> {
        // | S0 | V0 | V1 | Dir   | S1   |
        // |----+----+----+-------+------|
        // | A  |  0 |  1 | Right | B    |
        // | A  |  1 |  1 | Left  | C    |
        // | B  |  0 |  1 | Left  | A    |
        // | B  |  1 |  1 | Right | B    |
        // | C  |  0 |  1 | Left  | B    |
        // | C  |  1 |  1 | Stay  | Halt |

        match (state, value) {
            (Self::A, 0) => Action(1, Move::Right, Self::B),
            (Self::A, 1) => Action(1, Move::Left, Self::C),
            (Self::B, 0) => Action(1, Move::Left, Self::A),
            (Self::B, 1) => Action(1, Move::Right, Self::B),
            (Self::C, 0) => Action(1, Move::Left, Self::B),
            (Self::C, 1) => Action(1, Move::Stay, Self::Halt),
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
    BusyBeaver3::run();
}
