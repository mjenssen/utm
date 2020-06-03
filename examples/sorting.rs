use utm::{Action, Machine, Move, Tape};

// Sorting

#[derive(PartialEq, Debug)]
enum Sorting {
    A,
    B,
    C,
    D,
    E,
    Halt,
}

impl Sorting {
    fn rules(state: &Self, value: &u32) -> Action<Self, u32> {
        // | S0 | V0 | V1 | Dir   | S1   |
        // |----+----+----+-------+------|
        // | A  |  1 |  1 | Right | A    |
        // | A  |  2 |  3 | Right | B    |
        // | A  |  0 |  0 | Left  | E    |
        // | B  |  1 |  1 | Right | B    |
        // | B  |  2 |  2 | Right | B    |
        // | B  |  0 |  0 | Left  | C    |
        // | C  |  1 |  2 | Left  | D    |
        // | C  |  2 |  2 | Left  | C    |
        // | C  |  3 |  2 | Left  | E    |
        // | D  |  1 |  1 | Left  | D    |
        // | D  |  2 |  2 | Left  | D    |
        // | D  |  3 |  1 | Right | A    |
        // | E  |  1 |  1 | Left  | E    |
        // | E  |  0 |  0 | Right | Halt |

        match (state, value) {
            (Self::A, 1) => Action(1, Move::Right, Self::A),
            (Self::A, 2) => Action(3, Move::Right, Self::B),
            (Self::A, 0) => Action(0, Move::Left, Self::E),
            (Self::B, 1) => Action(1, Move::Right, Self::B),
            (Self::B, 2) => Action(2, Move::Right, Self::B),
            (Self::B, 0) => Action(0, Move::Left, Self::C),
            (Self::C, 1) => Action(2, Move::Left, Self::D),
            (Self::C, 2) => Action(2, Move::Left, Self::C),
            (Self::C, 3) => Action(2, Move::Left, Self::E),
            (Self::D, 1) => Action(1, Move::Left, Self::D),
            (Self::D, 2) => Action(2, Move::Left, Self::D),
            (Self::D, 3) => Action(1, Move::Right, Self::A),
            (Self::E, 1) => Action(1, Move::Left, Self::E),
            (Self::E, 0) => Action(0, Move::Right, Self::Halt),
            _ => panic!("Invalid operation"),
        }
    }

    fn run() {
        let mut machine = Machine::new(
            Self::A,
            Self::Halt,
            Tape::new(0, vec![2, 2, 2, 1, 2, 2, 1, 2, 1, 2, 1, 2, 1, 2]),
        );
        machine.run(Self::rules);
        machine.show_report(None);
    }
}

fn main() {
    Sorting::run();
}
