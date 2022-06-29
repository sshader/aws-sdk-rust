// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteThingShadow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_thing_shadow`](crate::client::Client::delete_thing_shadow).
///
/// See [`crate::client::fluent_builders::DeleteThingShadow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteThingShadow {
    _private: (),
}
impl DeleteThingShadow {
    /// Creates a new builder-style object to manufacture [`DeleteThingShadowInput`](crate::input::DeleteThingShadowInput).
    pub fn builder() -> crate::input::delete_thing_shadow_input::Builder {
        crate::input::delete_thing_shadow_input::Builder::default()
    }
    /// Creates a new `DeleteThingShadow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteThingShadow {
    type Output = std::result::Result<
        crate::output::DeleteThingShadowOutput,
        crate::error::DeleteThingShadowError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_thing_shadow_error(response)
        } else {
            crate::operation_deser::parse_delete_thing_shadow_response(response)
        }
    }
}

/// Operation shape for `GetRetainedMessage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_retained_message`](crate::client::Client::get_retained_message).
///
/// See [`crate::client::fluent_builders::GetRetainedMessage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRetainedMessage {
    _private: (),
}
impl GetRetainedMessage {
    /// Creates a new builder-style object to manufacture [`GetRetainedMessageInput`](crate::input::GetRetainedMessageInput).
    pub fn builder() -> crate::input::get_retained_message_input::Builder {
        crate::input::get_retained_message_input::Builder::default()
    }
    /// Creates a new `GetRetainedMessage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRetainedMessage {
    type Output = std::result::Result<
        crate::output::GetRetainedMessageOutput,
        crate::error::GetRetainedMessageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_retained_message_error(response)
        } else {
            crate::operation_deser::parse_get_retained_message_response(response)
        }
    }
}

/// Operation shape for `GetThingShadow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_thing_shadow`](crate::client::Client::get_thing_shadow).
///
/// See [`crate::client::fluent_builders::GetThingShadow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetThingShadow {
    _private: (),
}
impl GetThingShadow {
    /// Creates a new builder-style object to manufacture [`GetThingShadowInput`](crate::input::GetThingShadowInput).
    pub fn builder() -> crate::input::get_thing_shadow_input::Builder {
        crate::input::get_thing_shadow_input::Builder::default()
    }
    /// Creates a new `GetThingShadow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetThingShadow {
    type Output =
        std::result::Result<crate::output::GetThingShadowOutput, crate::error::GetThingShadowError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_thing_shadow_error(response)
        } else {
            crate::operation_deser::parse_get_thing_shadow_response(response)
        }
    }
}

/// Operation shape for `ListNamedShadowsForThing`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_named_shadows_for_thing`](crate::client::Client::list_named_shadows_for_thing).
///
/// See [`crate::client::fluent_builders::ListNamedShadowsForThing`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListNamedShadowsForThing {
    _private: (),
}
impl ListNamedShadowsForThing {
    /// Creates a new builder-style object to manufacture [`ListNamedShadowsForThingInput`](crate::input::ListNamedShadowsForThingInput).
    pub fn builder() -> crate::input::list_named_shadows_for_thing_input::Builder {
        crate::input::list_named_shadows_for_thing_input::Builder::default()
    }
    /// Creates a new `ListNamedShadowsForThing` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListNamedShadowsForThing {
    type Output = std::result::Result<
        crate::output::ListNamedShadowsForThingOutput,
        crate::error::ListNamedShadowsForThingError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_named_shadows_for_thing_error(response)
        } else {
            crate::operation_deser::parse_list_named_shadows_for_thing_response(response)
        }
    }
}

/// Operation shape for `ListRetainedMessages`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_retained_messages`](crate::client::Client::list_retained_messages).
///
/// See [`crate::client::fluent_builders::ListRetainedMessages`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRetainedMessages {
    _private: (),
}
impl ListRetainedMessages {
    /// Creates a new builder-style object to manufacture [`ListRetainedMessagesInput`](crate::input::ListRetainedMessagesInput).
    pub fn builder() -> crate::input::list_retained_messages_input::Builder {
        crate::input::list_retained_messages_input::Builder::default()
    }
    /// Creates a new `ListRetainedMessages` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRetainedMessages {
    type Output = std::result::Result<
        crate::output::ListRetainedMessagesOutput,
        crate::error::ListRetainedMessagesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_retained_messages_error(response)
        } else {
            crate::operation_deser::parse_list_retained_messages_response(response)
        }
    }
}

/// Operation shape for `Publish`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`publish`](crate::client::Client::publish).
///
/// See [`crate::client::fluent_builders::Publish`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct Publish {
    _private: (),
}
impl Publish {
    /// Creates a new builder-style object to manufacture [`PublishInput`](crate::input::PublishInput).
    pub fn builder() -> crate::input::publish_input::Builder {
        crate::input::publish_input::Builder::default()
    }
    /// Creates a new `Publish` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Publish {
    type Output = std::result::Result<crate::output::PublishOutput, crate::error::PublishError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_publish_error(response)
        } else {
            crate::operation_deser::parse_publish_response(response)
        }
    }
}

/// Operation shape for `UpdateThingShadow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_thing_shadow`](crate::client::Client::update_thing_shadow).
///
/// See [`crate::client::fluent_builders::UpdateThingShadow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateThingShadow {
    _private: (),
}
impl UpdateThingShadow {
    /// Creates a new builder-style object to manufacture [`UpdateThingShadowInput`](crate::input::UpdateThingShadowInput).
    pub fn builder() -> crate::input::update_thing_shadow_input::Builder {
        crate::input::update_thing_shadow_input::Builder::default()
    }
    /// Creates a new `UpdateThingShadow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateThingShadow {
    type Output = std::result::Result<
        crate::output::UpdateThingShadowOutput,
        crate::error::UpdateThingShadowError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_thing_shadow_error(response)
        } else {
            crate::operation_deser::parse_update_thing_shadow_response(response)
        }
    }
}
