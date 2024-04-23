use async_trait::async_trait;
use policy_mod_io::policy as io;
use tokio::time::{sleep, Duration, Instant};

/// The service to implement the API of Policy.
pub struct Service {
    cool_down_time: u64,
    allowed_requests: usize,
}

impl Service {
    pub fn new(cool_down_time: u64, allowed_requests: usize) -> Self {
        Self {
            cool_down_time,
            allowed_requests,
        }
    }
}

#[async_trait]
impl io::Api for Service {
    async fn handle_request_rate(&self, data: &mut io::Data) -> Result<(), io::Error> {
        let mut delay: Option<Duration> = None;
        let now = Instant::now();
        {
            // This scope is needed because MutexGuard is not Sent, therefore, sleep should happen
            // after dropping it!
            let mut last_requests_time = data
                .last_requests_time
                .lock()
                .map_err(|e| io::Error::GuardError(e.to_string()))?;
            dbg!(&last_requests_time);

            let cool_down_instant = now
                .checked_sub(Duration::new(self.cool_down_time, 0))
                .unwrap();

            while let Some(front) = last_requests_time.front() {
                if front < &cool_down_instant {
                    last_requests_time.pop_front();
                } else {
                    break;
                }
            }

            while last_requests_time.len() >= self.allowed_requests {
                if let Some(instant) = last_requests_time.pop_front() {
                    delay = Some(instant - cool_down_instant);
                }
            }

            last_requests_time.push_back(now);
        }

        if let Some(d) = delay {
            sleep(d).await;
        }
        Ok(())
    }
}
