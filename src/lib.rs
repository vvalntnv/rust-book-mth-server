use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create new ThreadPool
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panick if the size is 0.

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create threads
            workers.push(Worker::new(id, reciever.clone()));
        }

        ThreadPool { workers, sender } 
    }

    pub fn execute<F> (&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Terminating all the workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id:  usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(
        id: usize,
        reciever: Arc<Mutex<mpsc::Receiver<Message>>>
    ) -> Worker 
    {
        let thread = thread::spawn(move || loop {
            let message = reciever
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Terminating worker {}", id);
                    break;
                } 
            }
            
        }); 

        Worker { id, thread: Some(thread) }
    }
}
