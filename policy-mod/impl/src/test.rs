use crate::policy_impl::Service;
use io::Api;
use policy_mod_io::policy as io;
use tokio::time::Instant;

#[tokio::test]
async fn handle_request_rate_success() {
    let policy = Service::new(2, 5);
    let mut data = io::Data::default();

    let now = Instant::now();
    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;

    let new_now = Instant::now();
    let duration = new_now.checked_duration_since(now).unwrap().as_secs();
    assert!(duration < 2, "duration {:?}", duration);
}

#[tokio::test]
async fn handle_request_rate_block_success() {
    let policy = Service::new(2, 5);
    let mut data = io::Data::default();
    let now = Instant::now();

    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;
    let _ = policy.handle_request_rate(&mut data).await;

    let new_now = Instant::now();
    let duration = new_now.checked_duration_since(now).unwrap().as_secs();

    assert!(duration >= 2, "duration  {:?}", duration);
}
