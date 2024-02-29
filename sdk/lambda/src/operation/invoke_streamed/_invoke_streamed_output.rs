// Copied from sdk/lambda/src/operation/invoke/_invoke_output.rs but with a `event_stream`
// property


#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
pub struct InvokeStreamedOutput {
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub status_code: i32,
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub function_error: ::std::option::Option<::std::string::String>,
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub log_result: ::std::option::Option<::std::string::String>,
    /// <p>The response from the function, or an error object.</p>
    pub event_stream: ::std::option::Option<::aws_smithy_http::event_stream::Receiver<
            crate::types::LambdaInvokeStream,
            ::aws_smithy_types::error::ErrorMetadata,
        >,
    >,
    pub payload: ::std::option::Option<::aws_smithy_types::Blob>,
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub executed_version: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl InvokeStreamedOutput {
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub fn status_code(&self) -> i32 {
        self.status_code
    }
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub fn function_error(&self) -> ::std::option::Option<&str> {
        self.function_error.as_deref()
    }
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub fn log_result(&self) -> ::std::option::Option<&str> {
        self.log_result.as_deref()
    }
    // /// <p>The response from the function, or an error object.</p>
    // pub fn payload(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
    //     self.payload.as_ref()
    // }
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub fn executed_version(&self) -> ::std::option::Option<&str> {
        self.executed_version.as_deref()
    }
}
impl ::std::fmt::Debug for InvokeStreamedOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("InvokeOutput");
        formatter.field("status_code", &self.status_code);
        formatter.field("function_error", &self.function_error);
        formatter.field("log_result", &self.log_result);
        formatter.field("payload", &"*** Sensitive Data Redacted ***");
        formatter.field("executed_version", &self.executed_version);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_http::request_id::RequestId for InvokeStreamedOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}

impl InvokeStreamedOutput {
    /// Creates a new builder-style object to manufacture [`InvokeOutput`](crate::operation::invoke::InvokeOutput).
    pub fn builder() -> InvokeStreamedOutputBuilder {
        InvokeStreamedOutputBuilder::default()
    }
}



/// A builder for [`InvokeOutput`](crate::operation::invoke::InvokeOutput).
#[non_exhaustive]
#[derive(::std::default::Default)]
pub struct InvokeStreamedOutputBuilder {
    pub(crate) status_code: ::std::option::Option<i32>,
    pub(crate) function_error: ::std::option::Option<::std::string::String>,
    pub(crate) log_result: ::std::option::Option<::std::string::String>,
    pub(crate) event_stream: ::std::option::Option<::aws_smithy_http::event_stream::Receiver<
        crate::types::LambdaInvokeStream,
        ::aws_smithy_types::error::ErrorMetadata,
        >,
    >,
    pub(crate) payload: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) executed_version: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl InvokeStreamedOutputBuilder {
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub fn status_code(mut self, input: i32) -> Self {
        self.status_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub fn set_status_code(mut self, input: ::std::option::Option<i32>) -> Self {
        self.status_code = input;
        self
    }
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub fn get_status_code(&self) -> &::std::option::Option<i32> {
        &self.status_code
    }
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub fn function_error(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.function_error = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub fn set_function_error(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.function_error = input;
        self
    }
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub fn get_function_error(&self) -> &::std::option::Option<::std::string::String> {
        &self.function_error
    }
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub fn log_result(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_result = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub fn set_log_result(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_result = input;
        self
    }
    /// <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    pub fn get_log_result(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_result
    }
    /// <p>A stream of Lambda response chunks</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/streaming.html">Transcribing streaming audio</a>.</p>
    pub fn set_event_stream(
        mut self,
        input: ::std::option::Option<::aws_smithy_http::event_stream::Receiver<
        crate::types::LambdaInvokeStream,
        ::aws_smithy_types::error::ErrorMetadata,
        >,
    >,
    ) -> Self {
        self.event_stream = input;
        self
    }
    /// <p>An encoded stream of audio blobs. Audio streams are encoded as either HTTP/2 or WebSocket data frames.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/streaming.html">Transcribing streaming audio</a>.</p>
    pub fn event_stream(
        &self,
        input: ::std::option::Option<
            ::aws_smithy_http::event_stream::EventStreamSender<crate::types::LambdaInvokeStream, ::aws_smithy_types::error::ErrorMetadata>,
        >,
    ) -> &::std::option::Option<::aws_smithy_http::event_stream::Receiver<
        crate::types::LambdaInvokeStream,
        ::aws_smithy_types::error::ErrorMetadata,
        >,
    > {
        &self.event_stream
    }
    /// <p>The response from the function, or an error object.</p>
    pub fn payload(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.payload = ::std::option::Option::Some(input);
        self
    }
    /// <p>The response from the function, or an error object.</p>
    pub fn set_payload(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.payload = input;
        self
    }
    
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub fn executed_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.executed_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub fn set_executed_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.executed_version = input;
        self
    }
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub fn get_executed_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.executed_version
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`InvokeOutput`](crate::operation::invoke::InvokeOutput).
    pub fn build(self) -> crate::operation::invoke_streamed::InvokeStreamedOutput {
        crate::operation::invoke_streamed::InvokeStreamedOutput {
            status_code: self.status_code.unwrap_or_default(),
            function_error: self.function_error,
            log_result: self.log_result,
            event_stream: self.event_stream,
            executed_version: self.executed_version,
            payload: self.payload,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for InvokeStreamedOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("InvokeStreamedOutputBuilder");
        formatter.field("status_code", &self.status_code);
        formatter.field("function_error", &self.function_error);
        formatter.field("log_result", &self.log_result);
        formatter.field("payload", &"*** Sensitive Data Redacted ***");
        formatter.field("executed_version", &self.executed_version);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
