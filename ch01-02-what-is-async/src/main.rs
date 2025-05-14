use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();

    let (_, _, _, _) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
    );

    let elapsed_time = start_time.elapsed();
    println!("Request took {} ms", elapsed_time.as_millis());

    Ok(())
}
