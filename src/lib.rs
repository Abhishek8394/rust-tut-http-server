use std::thread;
use std::{error::Error, fmt};
use std::sync::mpsc;

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Error creating pool. Size must be > 1")
    }
}

struct Job;

struct Worker{
    id: String,
    thread: thread::JoinHandle<()>,
    recv: mpsc::Receiver<Job>,
}

impl Worker{
    fn new(id: String, receiver: mpsc::Receiver<Job>) -> Worker{
        Worker{id, thread: thread::spawn(||{}), recv: Receiver}
    }
}


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool{
    /// Create a new threadpool
    ///
    ///
    /// The size is number of workers in pool
    /// 
    /// # Panics
    /// 
    /// `new` will panic if size is 0
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>{
        if size == 0 {
            return Result::Err(PoolCreationError);
        }
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        for i in 0..size{
            workers.push(Worker::new(format!("worker-{}", i), receiver));
        }
        return Result::Ok(ThreadPool{workers, sender});
    }
    
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static
    {    
                
    }
}
