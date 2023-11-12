use tokio::time;

pub async fn sleep_for_secs(secs: u64) {
    println!("Sleeping for: {secs} seconds");
    time::sleep(time::Duration::from_secs(secs)).await;
    println!("Awake!");
}

pub async fn sleep_for_ms(ms: u64) {
    println!("Sleeping for: {ms} seconds");
    time::sleep(time::Duration::from_millis(ms)).await;
    println!("Awake!");
}
