use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub enum PoolError {
    LockPoisoned,
}

// J = Job. The type of work item you feed in.
// O = Output. The type you get back on success.
// E = Error.
// F = the actual function/closure that does the work — turns one J into a Result<O, E>.
pub fn spawn_worker_pool<J, O, E, F>(
    num_workers: usize,
    process: F,
) -> (Sender<J>, Receiver<Result<O, E>>, Vec<JoinHandle<()>>)
where
    J: Send + 'static,
    O: Send + 'static,
    E: From<PoolError> + Send + 'static,
    F: Fn(J) -> Result<O, E> + Send + Sync + 'static,
{
    let (job_tx, job_rx) = mpsc::channel::<J>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    let (out_tx, out_rx) = mpsc::channel::<Result<O, E>>();
    let process = Arc::new(process);

    let mut handles = Vec::with_capacity(num_workers);

    for _ in 0..num_workers {
        let job_rx = Arc::clone(&job_rx);
        let out_tx = out_tx.clone();
        let process = Arc::clone(&process);

        let handle = thread::spawn(move || {
            loop {
                let job = {
                    let locked = match job_rx.lock() {
                        Ok(l) => l,
                        Err(_) => {
                            // Report the failure through the same channel as
                            // everything else, then stop this worker. If the
                            // send itself fails, the caller already dropped
                            // the receiver, so there's nothing left to do.
                            let _ = out_tx.send(Err(E::from(PoolError::LockPoisoned)));
                            break;
                        }
                    };
                    locked.recv()
                };

                match job {
                    Ok(job) => {
                        let result = process(job);
                        if out_tx.send(result).is_err() {
                            break; // caller dropped the receiver, nobody's listening
                        }
                    }
                    Err(_) => break, // job channel closed, no more work
                }
            }
        });
        handles.push(handle);
    }

    (job_tx, out_rx, handles)
}
