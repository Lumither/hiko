use async_trait::async_trait;

mod tasks;

#[async_trait]
pub trait Task {
    // do the task
    async fn exec(&mut self) -> Result<(), String>;

    fn fail_count(&self) -> u32;
}
