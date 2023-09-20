use async_std::{stream::StreamExt, task::Builder};

use super::{stream::AsyncStream, task::Task};

#[derive(Clone)]
pub struct TaskQueue {
    pub(crate) stream: AsyncStream<Task>,
}

impl TaskQueue {
    pub(crate) fn new() -> Self {
        Self {
            stream: AsyncStream::new(),
        }
    }

    pub fn push(&mut self, runnable: Task) {
        self.stream.insert_item(runnable);
    }

    pub fn pop(&mut self) -> Option<Task> {
        Builder::new().blocking(async move { self.stream.next().await })
    }
}
