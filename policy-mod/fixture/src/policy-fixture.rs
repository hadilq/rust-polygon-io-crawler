use async_trait::async_trait;
use policy_mod_io::policy as io;

pub struct ServiceFake;

#[async_trait]
impl io::Api for ServiceFake {
    async fn handle_request_rate(&self, _data: &mut io::Data) -> io::Result<()> {
        Ok(())
    }
}
