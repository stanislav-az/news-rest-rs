use std::vec::Vec;
use std::{thread, time};
use futures::future::join_all;

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let third_outcome = async {
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);
        futures_vec.push(future_four);
        futures_vec.push(future_five);

        // applies the spawn async tasks for all futures and collect them into a vector
        let handles = futures_vec
            .into_iter()
            .map(tokio::task::spawn)
            .collect::<Vec<_>>();
        let results = join_all(handles).await;
        return results;
    };

    // https://docs.rs/tokio/latest/tokio/runtime/index.html#multi-thread-scheduler
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap(); // Builds single threaded runtime
    // Elapsed time 4s

    // https://docs.rs/tokio/latest/tokio/runtime/index.html#current-thread-scheduler
    let rt = tokio::runtime::Runtime::new().unwrap(); // Builds multi-threaded runtime
    // Elapsed time 2s

    let now = time::Instant::now();
    let result = rt.block_on(third_outcome);
    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);
}
