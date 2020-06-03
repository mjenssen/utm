use utm::{Action, Machine, Move, Tape};

// Increment

#[derive(PartialEq, Debug)]
enum Increment {
    Q0,
    QF,
}

impl Increment {
    fn rules(state: &Self, value: &u32) -> Action<Self, u32> {
        match (state, value) {
            (Self::Q0, 1) => Action(1, Move::Right, Self::Q0),
            (Self::Q0, 0) => Action(1, Move::Stay, Self::QF),
            _ => panic!("Invalid operation"),
        }
    }

    fn run() {
        let mut machine = Machine::new(Self::Q0, Self::QF, Tape::new(0, vec![1, 1, 1]));
        machine.run(Self::rules);
        machine.show_report(Some(1));
    }
}

fn main() {
    Increment::run();
}
