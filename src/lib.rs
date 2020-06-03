use std::time::{Duration, Instant};

pub enum Move {
    Left,
    Right,
    Stay,
}

pub struct Action<S, V>(pub V, pub Move, pub S);

pub struct Tape<T> {
    pos: usize,
    tape: Vec<T>,
    blank: T,
}

impl<T> Tape<T>
where
    T: Copy + PartialEq,
{
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

    /// Returns the current value
    fn current(&self) -> &T {
        &self.tape[self.pos]
    }

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

    fn len(&self) -> usize {
        self.tape.len()
    }

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
    pub fn new(start: S, stop: S, tape: Tape<V>) -> Self {
        Self {
            state: start,
            stop,
            tape,
            steps: 0,
            elapsed: Duration::new(0, 0),
        }
    }

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
