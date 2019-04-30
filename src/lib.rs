use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub enum LoopState {
    Continue,
    Break,
}

pub trait Cancellable {
    type Error;

    fn for_each(&mut self) -> Result<LoopState, Self::Error>;

    fn run(&mut self) -> Result<(), Self::Error> {
        loop {
            match self.for_each() {
                Ok(LoopState::Continue) => {}
                Ok(LoopState::Break) => break,
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }


    fn spawn(mut self) -> Handle<Self::Error>
    where
        Self: Sized + Send + 'static,
        Self::Error: Send + 'static,
    {
        // thread-safe multiple reference
        let keep_running = Arc::new(AtomicBool::new(true));
        let join_handle = {
            let keep_running = keep_running.clone();
            thread::spawn(move || {
                // ========================================================== //
                while keep_running.load(Ordering::SeqCst) {
                    match self.for_each() {
                        Ok(LoopState::Continue) => {}
                        Ok(LoopState::Break) => break,
                        Err(e) => return Err(e),
                    }
                }
                // same return type as the Error branch
                Ok(())
                // ========================================================== //
            })
        };

        Handle {
            keep_running,
            executor: join_handle,
        }
    }
}

pub struct Handle<E> {
    keep_running: Arc<AtomicBool>,
    executor: thread::JoinHandle<Result<(), E>>,
}

#[derive(Clone)]
pub struct Canceller {
    keep_running: Arc<AtomicBool>,
}

impl<E> Handle<E> {
    // what's the purpose of this?
    pub fn Canceller(&self) -> Canceller {
        Canceller {
            keep_running: self.keep_running.clone(),
        }
    }

    pub fn cancel(&self) {
        self.keep_running.store(false, Ordering::SeqCst);
    }

    pub fn wait(self) -> Result<(), E> {
        match self.executor.join() {
            Ok(r) => r,
            Err(e) => {
                // propagate the panic
                panic!(e)
            }
        }
    }
}