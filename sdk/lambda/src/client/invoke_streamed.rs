impl super::Client {
    pub fn invoke_streamed(&self) -> crate::operation::invoke_streamed::builders::InvokeStreamedFluentBuilder {
        crate::operation::invoke_streamed::builders::InvokeStreamedFluentBuilder::new(self.handle.clone())
    }
}
