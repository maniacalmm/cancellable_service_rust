use std::thread;

fn main() {

    let mut join_handles: Vec<thread::JoinHandle<Result<u32, String>>> = Vec::new();
    for _ in 0..10 {
        let jh = thread::spawn(move || {
            let mut acc = 0;
            for j in 2..12 {
                acc += j;
            }
            Ok(acc)
        });
        join_handles.push(jh);
    }

    for jh in join_handles.into_iter() {
        let value: u32 = jh.join().unwrap().unwrap();
        println!("join! {}", value);
    }

    println!("in return join handle examples")
}
