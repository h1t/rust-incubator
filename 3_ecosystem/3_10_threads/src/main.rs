use crossbeam::channel::{self, Receiver, Sender};
use rand::RngCore;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::thread::{self, JoinHandle};

const RANK: usize = 64;
type Matrix = [[u8; RANK]; RANK];

struct Producer(JoinHandle<()>);

impl Producer {
    fn spawn(tx: Sender<Matrix>) -> Self {
        Self(thread::spawn(move || loop {
            let m: Matrix = {
                let mut m = [[0; RANK]; RANK];
                let mut rng = rand::thread_rng();

                for row in &mut m {
                    rng.fill_bytes(row);
                }
                m
            };
            tx.send(m).unwrap();
        }))
    }

    fn join(self) {
        let _ = self.0.join();
    }
}

struct Consumer(JoinHandle<()>);

impl Consumer {
    fn spawn(rx: Receiver<Matrix>) -> Self {
        Self(thread::spawn(move || loop {
            let m: Matrix = rx.recv().unwrap();
            let sum: u64 = m
                .into_par_iter()
                .map(|s| s.iter().copied().map(u64::from).sum::<u64>())
                .sum();

            println!("{sum}");
        }))
    }

    fn join(self) {
        let _ = self.0.join();
    }
}

fn main() {
    let (tx, rx) = channel::unbounded();

    let _ = Consumer::spawn(rx.clone());
    let _ = Consumer::spawn(rx);
    Producer::spawn(tx).join();
}
