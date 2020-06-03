use std::time::{Duration, Instant};

pub enum Move {
    Left,
    Right,
    Stay,
}

/// An action to take based on the current state and value.
///
/// This is generalized for the state type S and value type V.
pub struct Action<S, V>(pub V, pub Move, pub S);

/// The tape represents the memory of the machine.
///
/// This is generalized for the value type T.
pub struct Tape<T> {
    pos: usize,
    tape: Vec<T>,
    blank: T,
}

impl<T> Tape<T>
where
    T: Copy + PartialEq,
{
    /// Create a new tape. It must be given the value that represents
    /// a blank value, as well as an initial list of values for the
    /// tape.
    pub fn new(blank: T, tape: Vec<T>) -> Self {
        if tape.is_empty() {
            Self {
                pos: 0,
                tape: vec![blank],
                blank,
            }
        } else {
            Self {
                pos: 0,
                tape,
                blank,
            }
        }
    }

    /// Return the value at the current position.
    fn current(&self) -> &T {
        &self.tape[self.pos]
    }

    /// Update the tape by writing the given value to the current
    /// position and then moving the position in the direction given.
    fn update(&mut self, value: T, dir: Move) {
        self.tape[self.pos] = value;

        match dir {
            Move::Stay => {
                // Do nothing
            }
            Move::Left => {
                if self.pos == 0 {
                    self.tape.insert(0, self.blank);
                } else {
                    self.pos -= 1;
                }
            }
            Move::Right => {
                self.pos += 1;

                if self.pos == self.tape.len() {
                    self.tape.push(self.blank);
                }
            }
        }
    }

    /// Return the length of the tape.
    fn len(&self) -> usize {
        self.tape.len()
    }

    /// Count the number of times a given value is found in the tape.
    fn count(&self, value: T) -> usize {
        let mut length = 0;

        for v in self.tape.iter() {
            if *v == value {
                length += 1
            }
        }

        length
    }
}

/// The machine itself. It contains a tape, and some internal
/// information about the number of steps taken, timing, and the
/// current state.
///
/// It is generalized over the state type S and value type V.
pub struct Machine<S, V> {
    state: S,
    stop: S,
    tape: Tape<V>,
    steps: usize,
    elapsed: Duration,
}

impl<S, V> Machine<S, V>
where
    S: PartialEq,
    V: Copy + PartialEq,
{
    /// Create a new machine. It must be given the starting state and
    /// the state that will halt the machine.
    pub fn new(start: S, stop: S, tape: Tape<V>) -> Self {
        Self {
            state: start,
            stop,
            tape,
            steps: 0,
            elapsed: Duration::new(0, 0),
        }
    }

    /// Run the machine using the rules given. `rules` is a function
    /// that is given the current state and value and must return an
    /// `Action` defining a new value, a direction to take on the
    /// tape, and the new state.
    pub fn run(&mut self, rules: fn(&S, &V) -> Action<S, V>) {
        let start_time = Instant::now();

        self.steps = 0;

        loop {
            if self.state == self.stop {
                break;
            }

            let action = rules(&self.state, self.tape.current());

            self.tape.update(action.0, action.1);
            self.state = action.2;

            self.steps += 1;
        }

        self.elapsed = start_time.elapsed();
    }

    /// Helper function to show a simple report on how many steps it
    /// took and how long the machine run for.
    pub fn show_report(&self, count_values: Option<V>)
    where
        V: std::fmt::Display,
        S: std::fmt::Debug,
    {
        println!(
            "Steps: {}, final state: {:?}, final tape size: {}",
            self.steps,
            self.state,
            self.tape.len()
        );

        println!("That took {} µs to compute.", self.elapsed.as_micros());

        if let Some(value) = count_values {
            println!(
                "Count of the value '{}' in final tape: {}",
                value,
                self.tape.count(value)
            );
        }
    }
}
