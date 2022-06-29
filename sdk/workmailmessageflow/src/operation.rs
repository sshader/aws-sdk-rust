// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `GetRawMessageContent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_raw_message_content`](crate::client::Client::get_raw_message_content).
///
/// See [`crate::client::fluent_builders::GetRawMessageContent`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRawMessageContent {
    _private: (),
}
impl GetRawMessageContent {
    /// Creates a new builder-style object to manufacture [`GetRawMessageContentInput`](crate::input::GetRawMessageContentInput).
    pub fn builder() -> crate::input::get_raw_message_content_input::Builder {
        crate::input::get_raw_message_content_input::Builder::default()
    }
    /// Creates a new `GetRawMessageContent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for GetRawMessageContent {
    type Output = std::result::Result<
        crate::output::GetRawMessageContentOutput,
        crate::error::GetRawMessageContentError,
    >;
    fn parse_unloaded(
        &self,
        response: &mut aws_smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_get_raw_message_content(
            response,
        ))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_get_raw_message_content_error(response)
    }
}

/// Operation shape for `PutRawMessageContent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_raw_message_content`](crate::client::Client::put_raw_message_content).
///
/// See [`crate::client::fluent_builders::PutRawMessageContent`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutRawMessageContent {
    _private: (),
}
impl PutRawMessageContent {
    /// Creates a new builder-style object to manufacture [`PutRawMessageContentInput`](crate::input::PutRawMessageContentInput).
    pub fn builder() -> crate::input::put_raw_message_content_input::Builder {
        crate::input::put_raw_message_content_input::Builder::default()
    }
    /// Creates a new `PutRawMessageContent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRawMessageContent {
    type Output = std::result::Result<
        crate::output::PutRawMessageContentOutput,
        crate::error::PutRawMessageContentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_raw_message_content_error(response)
        } else {
            crate::operation_deser::parse_put_raw_message_content_response(response)
        }
    }
}
