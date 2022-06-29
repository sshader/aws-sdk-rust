// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_app_monitor`](crate::client::Client::create_app_monitor).
///
/// See [`crate::client::fluent_builders::CreateAppMonitor`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAppMonitor {
    _private: (),
}
impl CreateAppMonitor {
    /// Creates a new builder-style object to manufacture [`CreateAppMonitorInput`](crate::input::CreateAppMonitorInput).
    pub fn builder() -> crate::input::create_app_monitor_input::Builder {
        crate::input::create_app_monitor_input::Builder::default()
    }
    /// Creates a new `CreateAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAppMonitor {
    type Output = std::result::Result<
        crate::output::CreateAppMonitorOutput,
        crate::error::CreateAppMonitorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_create_app_monitor_response(response)
        }
    }
}

/// Operation shape for `DeleteAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_app_monitor`](crate::client::Client::delete_app_monitor).
///
/// See [`crate::client::fluent_builders::DeleteAppMonitor`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAppMonitor {
    _private: (),
}
impl DeleteAppMonitor {
    /// Creates a new builder-style object to manufacture [`DeleteAppMonitorInput`](crate::input::DeleteAppMonitorInput).
    pub fn builder() -> crate::input::delete_app_monitor_input::Builder {
        crate::input::delete_app_monitor_input::Builder::default()
    }
    /// Creates a new `DeleteAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAppMonitor {
    type Output = std::result::Result<
        crate::output::DeleteAppMonitorOutput,
        crate::error::DeleteAppMonitorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_delete_app_monitor_response(response)
        }
    }
}

/// Operation shape for `GetAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_app_monitor`](crate::client::Client::get_app_monitor).
///
/// See [`crate::client::fluent_builders::GetAppMonitor`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAppMonitor {
    _private: (),
}
impl GetAppMonitor {
    /// Creates a new builder-style object to manufacture [`GetAppMonitorInput`](crate::input::GetAppMonitorInput).
    pub fn builder() -> crate::input::get_app_monitor_input::Builder {
        crate::input::get_app_monitor_input::Builder::default()
    }
    /// Creates a new `GetAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAppMonitor {
    type Output =
        std::result::Result<crate::output::GetAppMonitorOutput, crate::error::GetAppMonitorError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_get_app_monitor_response(response)
        }
    }
}

/// Operation shape for `GetAppMonitorData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_app_monitor_data`](crate::client::Client::get_app_monitor_data).
///
/// See [`crate::client::fluent_builders::GetAppMonitorData`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAppMonitorData {
    _private: (),
}
impl GetAppMonitorData {
    /// Creates a new builder-style object to manufacture [`GetAppMonitorDataInput`](crate::input::GetAppMonitorDataInput).
    pub fn builder() -> crate::input::get_app_monitor_data_input::Builder {
        crate::input::get_app_monitor_data_input::Builder::default()
    }
    /// Creates a new `GetAppMonitorData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAppMonitorData {
    type Output = std::result::Result<
        crate::output::GetAppMonitorDataOutput,
        crate::error::GetAppMonitorDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_app_monitor_data_error(response)
        } else {
            crate::operation_deser::parse_get_app_monitor_data_response(response)
        }
    }
}

/// Operation shape for `ListAppMonitors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_app_monitors`](crate::client::Client::list_app_monitors).
///
/// See [`crate::client::fluent_builders::ListAppMonitors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAppMonitors {
    _private: (),
}
impl ListAppMonitors {
    /// Creates a new builder-style object to manufacture [`ListAppMonitorsInput`](crate::input::ListAppMonitorsInput).
    pub fn builder() -> crate::input::list_app_monitors_input::Builder {
        crate::input::list_app_monitors_input::Builder::default()
    }
    /// Creates a new `ListAppMonitors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAppMonitors {
    type Output = std::result::Result<
        crate::output::ListAppMonitorsOutput,
        crate::error::ListAppMonitorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_app_monitors_error(response)
        } else {
            crate::operation_deser::parse_list_app_monitors_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `PutRumEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_rum_events`](crate::client::Client::put_rum_events).
///
/// See [`crate::client::fluent_builders::PutRumEvents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutRumEvents {
    _private: (),
}
impl PutRumEvents {
    /// Creates a new builder-style object to manufacture [`PutRumEventsInput`](crate::input::PutRumEventsInput).
    pub fn builder() -> crate::input::put_rum_events_input::Builder {
        crate::input::put_rum_events_input::Builder::default()
    }
    /// Creates a new `PutRumEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRumEvents {
    type Output =
        std::result::Result<crate::output::PutRumEventsOutput, crate::error::PutRumEventsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_rum_events_error(response)
        } else {
            crate::operation_deser::parse_put_rum_events_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateAppMonitor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_app_monitor`](crate::client::Client::update_app_monitor).
///
/// See [`crate::client::fluent_builders::UpdateAppMonitor`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAppMonitor {
    _private: (),
}
impl UpdateAppMonitor {
    /// Creates a new builder-style object to manufacture [`UpdateAppMonitorInput`](crate::input::UpdateAppMonitorInput).
    pub fn builder() -> crate::input::update_app_monitor_input::Builder {
        crate::input::update_app_monitor_input::Builder::default()
    }
    /// Creates a new `UpdateAppMonitor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAppMonitor {
    type Output = std::result::Result<
        crate::output::UpdateAppMonitorOutput,
        crate::error::UpdateAppMonitorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_app_monitor_error(response)
        } else {
            crate::operation_deser::parse_update_app_monitor_response(response)
        }
    }
}
