use std::sync::mpsc;
use std::thread;

fn send_messages() -> (mpsc::Receiver<String>, Vec<thread::JoinHandle<String>>) {
    let (tx, rx) = mpsc::channel();

    let mut handlers: Vec<thread::JoinHandle<String>> = vec![];

    for thread_number in 0..5 {
        let new_tx = tx.clone();
        let handle = thread::spawn(move || {
            let mut thread_message = String::from("IM A THREAD ");
            thread_message.push_str(&thread_number.to_string());

            let vals = vec![thread_message];

            for val in vals {
                println!("IM SENDING FROM THREAD {}", thread_number);
                new_tx.send(val).unwrap();
            }

            let mut finished_message = String::from("FINISHED");
            finished_message.push_str(&thread_number.to_string());
            finished_message
        });

        handlers.push(handle);
    }

    (rx, handlers)
}

fn receive_messages(rx: mpsc::Receiver<String>) {
    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    let (rx, handlers) = send_messages();
    receive_messages(rx);

    for handle in handlers {
        if let Ok(thread_message) = handle.join() {
            println!("THREAD MESSAGE: {}", thread_message);
        }
    }

    println!("AT THE END WHEN COMM ENDS");
}
