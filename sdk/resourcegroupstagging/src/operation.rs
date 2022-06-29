// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DescribeReportCreation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_report_creation`](crate::client::Client::describe_report_creation).
///
/// See [`crate::client::fluent_builders::DescribeReportCreation`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeReportCreation {
    _private: (),
}
impl DescribeReportCreation {
    /// Creates a new builder-style object to manufacture [`DescribeReportCreationInput`](crate::input::DescribeReportCreationInput).
    pub fn builder() -> crate::input::describe_report_creation_input::Builder {
        crate::input::describe_report_creation_input::Builder::default()
    }
    /// Creates a new `DescribeReportCreation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeReportCreation {
    type Output = std::result::Result<
        crate::output::DescribeReportCreationOutput,
        crate::error::DescribeReportCreationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_report_creation_error(response)
        } else {
            crate::operation_deser::parse_describe_report_creation_response(response)
        }
    }
}

/// Operation shape for `GetComplianceSummary`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_compliance_summary`](crate::client::Client::get_compliance_summary).
///
/// See [`crate::client::fluent_builders::GetComplianceSummary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetComplianceSummary {
    _private: (),
}
impl GetComplianceSummary {
    /// Creates a new builder-style object to manufacture [`GetComplianceSummaryInput`](crate::input::GetComplianceSummaryInput).
    pub fn builder() -> crate::input::get_compliance_summary_input::Builder {
        crate::input::get_compliance_summary_input::Builder::default()
    }
    /// Creates a new `GetComplianceSummary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetComplianceSummary {
    type Output = std::result::Result<
        crate::output::GetComplianceSummaryOutput,
        crate::error::GetComplianceSummaryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_compliance_summary_error(response)
        } else {
            crate::operation_deser::parse_get_compliance_summary_response(response)
        }
    }
}

/// Operation shape for `GetResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resources`](crate::client::Client::get_resources).
///
/// See [`crate::client::fluent_builders::GetResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResources {
    _private: (),
}
impl GetResources {
    /// Creates a new builder-style object to manufacture [`GetResourcesInput`](crate::input::GetResourcesInput).
    pub fn builder() -> crate::input::get_resources_input::Builder {
        crate::input::get_resources_input::Builder::default()
    }
    /// Creates a new `GetResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResources {
    type Output =
        std::result::Result<crate::output::GetResourcesOutput, crate::error::GetResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resources_error(response)
        } else {
            crate::operation_deser::parse_get_resources_response(response)
        }
    }
}

/// Operation shape for `GetTagKeys`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_tag_keys`](crate::client::Client::get_tag_keys).
///
/// See [`crate::client::fluent_builders::GetTagKeys`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTagKeys {
    _private: (),
}
impl GetTagKeys {
    /// Creates a new builder-style object to manufacture [`GetTagKeysInput`](crate::input::GetTagKeysInput).
    pub fn builder() -> crate::input::get_tag_keys_input::Builder {
        crate::input::get_tag_keys_input::Builder::default()
    }
    /// Creates a new `GetTagKeys` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTagKeys {
    type Output =
        std::result::Result<crate::output::GetTagKeysOutput, crate::error::GetTagKeysError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_tag_keys_error(response)
        } else {
            crate::operation_deser::parse_get_tag_keys_response(response)
        }
    }
}

/// Operation shape for `GetTagValues`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_tag_values`](crate::client::Client::get_tag_values).
///
/// See [`crate::client::fluent_builders::GetTagValues`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTagValues {
    _private: (),
}
impl GetTagValues {
    /// Creates a new builder-style object to manufacture [`GetTagValuesInput`](crate::input::GetTagValuesInput).
    pub fn builder() -> crate::input::get_tag_values_input::Builder {
        crate::input::get_tag_values_input::Builder::default()
    }
    /// Creates a new `GetTagValues` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTagValues {
    type Output =
        std::result::Result<crate::output::GetTagValuesOutput, crate::error::GetTagValuesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_tag_values_error(response)
        } else {
            crate::operation_deser::parse_get_tag_values_response(response)
        }
    }
}

/// Operation shape for `StartReportCreation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_report_creation`](crate::client::Client::start_report_creation).
///
/// See [`crate::client::fluent_builders::StartReportCreation`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartReportCreation {
    _private: (),
}
impl StartReportCreation {
    /// Creates a new builder-style object to manufacture [`StartReportCreationInput`](crate::input::StartReportCreationInput).
    pub fn builder() -> crate::input::start_report_creation_input::Builder {
        crate::input::start_report_creation_input::Builder::default()
    }
    /// Creates a new `StartReportCreation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartReportCreation {
    type Output = std::result::Result<
        crate::output::StartReportCreationOutput,
        crate::error::StartReportCreationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_report_creation_error(response)
        } else {
            crate::operation_deser::parse_start_report_creation_response(response)
        }
    }
}

/// Operation shape for `TagResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resources`](crate::client::Client::tag_resources).
///
/// See [`crate::client::fluent_builders::TagResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResources {
    _private: (),
}
impl TagResources {
    /// Creates a new builder-style object to manufacture [`TagResourcesInput`](crate::input::TagResourcesInput).
    pub fn builder() -> crate::input::tag_resources_input::Builder {
        crate::input::tag_resources_input::Builder::default()
    }
    /// Creates a new `TagResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResources {
    type Output =
        std::result::Result<crate::output::TagResourcesOutput, crate::error::TagResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resources_error(response)
        } else {
            crate::operation_deser::parse_tag_resources_response(response)
        }
    }
}

/// Operation shape for `UntagResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resources`](crate::client::Client::untag_resources).
///
/// See [`crate::client::fluent_builders::UntagResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResources {
    _private: (),
}
impl UntagResources {
    /// Creates a new builder-style object to manufacture [`UntagResourcesInput`](crate::input::UntagResourcesInput).
    pub fn builder() -> crate::input::untag_resources_input::Builder {
        crate::input::untag_resources_input::Builder::default()
    }
    /// Creates a new `UntagResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResources {
    type Output =
        std::result::Result<crate::output::UntagResourcesOutput, crate::error::UntagResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resources_error(response)
        } else {
            crate::operation_deser::parse_untag_resources_response(response)
        }
    }
}
