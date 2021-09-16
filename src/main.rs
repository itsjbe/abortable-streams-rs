use std::error::Error;
use std::time::{Duration, Instant};

use async_std as a_std;
use futures::{StreamExt, TryStreamExt};
use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

const N: usize = 10;
const BF: usize = 2;

#[async_std::main]
async fn main() {
    //    println!(
    //        "First {} pages, buffered by {}:\n{:?}",
    //        N,
    //        BF,
    //        get_n_pages_buffer_unordered(N, BF).await
    //    );
    //    println!("\n\n");
    main_iter().await;
}

async fn main_iter() {
    let mut values = get_pages_futures().buffer_unordered(BF).take(N);

    while let Some(p) = values.next().await
    {
        match p
        {
            Ok(v) =>
            {
                println!("values: {:?}", v)
            }
            Err(e) =>
            {
                println!("error: {}", e)
            }
        }
    }
}

async fn get_n_pages_buffer_unordered(n: usize, buf_factor: usize) -> Vec<Result<Vec<usize>>> {
    get_pages_futures()
        .take(n)
        .buffer_unordered(buf_factor)
        .collect()
        .await
}

fn get_pages_futures(
) -> impl a_std::stream::Stream<Item = impl a_std::future::Future<Output = Result<Vec<usize>>>> {
    futures::stream::iter(0..).map(|i| get_page(i))
}

async fn get_page(i: usize) -> Result<Vec<usize>> {
    let millis = Uniform::from(0..10).sample(&mut rand::thread_rng());
    println!(
        "[{}] # get_page({}) will complete in {} ms",
        START_TIME.elapsed().as_millis(),
        i,
        millis
    );

    a_std::task::sleep(Duration::from_millis(millis)).await;

    if millis >= 5
    {
        let msg = format!("[{}] job timed out {}", START_TIME.elapsed().as_millis(), i,);
        println!("{}", msg);
        return Err(From::from(msg));
    }

    println!(
        "[{}] # get_page({}) completed",
        START_TIME.elapsed().as_millis(),
        i
    );

    Ok((10 * i..10 * (i + 1)).collect())
}
