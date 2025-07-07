use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

/// 手写一个线程池
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    join_handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let join_handle = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} executing job");
                    job()
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Self {
            id,
            join_handle: Some(join_handle),
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0, "ThreadPool size must greater than zero!");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone()))
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            sender.send(Box::new(job)).unwrap()
        } else {
            panic!("ThreadPool has closed")
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);
            if let Some(handle) = worker.join_handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

fn main() {
    // 创建包含4个线程的线程池
    let pool = ThreadPool::new(4);

    // 提交8个任务
    for i in 0..80 {
        pool.execute(move || {
            println!("Task {i} started");
            thread::sleep(std::time::Duration::from_secs(1));
            println!("Task {i} finished");
        });
    }

    // 等待所有任务完成（当pool离开作用域时自动关闭）
    println!("All tasks submitted");
    thread::sleep(std::time::Duration::from_secs(2));
}
