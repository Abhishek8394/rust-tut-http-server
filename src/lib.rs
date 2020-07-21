use std::thread;
use std::{error::Error, fmt};
use std::sync::{mpsc,Arc,Mutex};

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Error creating pool. Size must be > 1")
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker{
    id: String,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: String, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let id_copy = String::from(&id[..]);
        let thread = thread::spawn(move||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got job.", id_copy);
                job();
            }
            /// Below will make our server serial!
            /// This is because the mutex lock obtained will remain in scope for duration of block 
            /// which is when the job runs. So we would have to lock for entire duration of job!
            /// But we only need to pop job from queue, dont need lock later. Hence the `loop`
            /// block above, it releases lock as soon as the statement finishes executing.
            /// Try using below block as main loop, requests will be served serially!
            // while let Ok(job) = receiver.lock().unwrap().recv(){
            //     println!("Worker {} got job.", id_copy);
            //     job();
            // }
        });
        Worker{id, thread}
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
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size{
            workers.push(Worker::new(format!("worker-{}", i), Arc::clone(&receiver)));
        }
        return Result::Ok(ThreadPool{workers, sender});
    }
    
    pub fn execute<F>(&self, f: F)  where F: FnOnce() + Send + 'static{
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
