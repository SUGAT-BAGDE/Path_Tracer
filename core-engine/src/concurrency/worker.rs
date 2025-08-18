use std::thread;
use std::thread::JoinHandle;

use crossbeam::channel::{Receiver, Sender};

use crate::accumulator::Accumulator;
use crate::concurrency::RenderJob;
use crate::sampler::Sampler;

pub struct RenderingWorker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl RenderingWorker {
    pub fn new(id: usize, job_rx: Receiver<RenderJob>, result_tx: Sender<Accumulator>) -> Self {
        let thread = thread::spawn(move || {
            let mut sampler = Sampler::new();
            loop {
                match job_rx.recv() {
                    Ok(job) => {
                        println!("Worker {id} got a job. Executing!!..");
                        let acc = job(&mut sampler);
                        match result_tx.send(acc) {
                            Ok(_) => (),
                            Err(e) => {
                                println!(
                                    "Worker {} failed to send result: receiver disconnected. Err : {:?}",
                                    id, e
                                );
                                break;
                            }
                        }
                        println!("Worker {id} job completed. Seeking another...");
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; Shutting down..");
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }

    pub fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
