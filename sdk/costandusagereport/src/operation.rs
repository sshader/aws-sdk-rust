// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteReportDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_report_definition`](crate::client::Client::delete_report_definition).
///
/// See [`crate::client::fluent_builders::DeleteReportDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteReportDefinition {
    _private: (),
}
impl DeleteReportDefinition {
    /// Creates a new builder-style object to manufacture [`DeleteReportDefinitionInput`](crate::input::DeleteReportDefinitionInput).
    pub fn builder() -> crate::input::delete_report_definition_input::Builder {
        crate::input::delete_report_definition_input::Builder::default()
    }
    /// Creates a new `DeleteReportDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteReportDefinition {
    type Output = std::result::Result<
        crate::output::DeleteReportDefinitionOutput,
        crate::error::DeleteReportDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_report_definition_error(response)
        } else {
            crate::operation_deser::parse_delete_report_definition_response(response)
        }
    }
}

/// Operation shape for `DescribeReportDefinitions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_report_definitions`](crate::client::Client::describe_report_definitions).
///
/// See [`crate::client::fluent_builders::DescribeReportDefinitions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeReportDefinitions {
    _private: (),
}
impl DescribeReportDefinitions {
    /// Creates a new builder-style object to manufacture [`DescribeReportDefinitionsInput`](crate::input::DescribeReportDefinitionsInput).
    pub fn builder() -> crate::input::describe_report_definitions_input::Builder {
        crate::input::describe_report_definitions_input::Builder::default()
    }
    /// Creates a new `DescribeReportDefinitions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeReportDefinitions {
    type Output = std::result::Result<
        crate::output::DescribeReportDefinitionsOutput,
        crate::error::DescribeReportDefinitionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_report_definitions_error(response)
        } else {
            crate::operation_deser::parse_describe_report_definitions_response(response)
        }
    }
}

/// Operation shape for `ModifyReportDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`modify_report_definition`](crate::client::Client::modify_report_definition).
///
/// See [`crate::client::fluent_builders::ModifyReportDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ModifyReportDefinition {
    _private: (),
}
impl ModifyReportDefinition {
    /// Creates a new builder-style object to manufacture [`ModifyReportDefinitionInput`](crate::input::ModifyReportDefinitionInput).
    pub fn builder() -> crate::input::modify_report_definition_input::Builder {
        crate::input::modify_report_definition_input::Builder::default()
    }
    /// Creates a new `ModifyReportDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ModifyReportDefinition {
    type Output = std::result::Result<
        crate::output::ModifyReportDefinitionOutput,
        crate::error::ModifyReportDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_modify_report_definition_error(response)
        } else {
            crate::operation_deser::parse_modify_report_definition_response(response)
        }
    }
}

/// Operation shape for `PutReportDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_report_definition`](crate::client::Client::put_report_definition).
///
/// See [`crate::client::fluent_builders::PutReportDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutReportDefinition {
    _private: (),
}
impl PutReportDefinition {
    /// Creates a new builder-style object to manufacture [`PutReportDefinitionInput`](crate::input::PutReportDefinitionInput).
    pub fn builder() -> crate::input::put_report_definition_input::Builder {
        crate::input::put_report_definition_input::Builder::default()
    }
    /// Creates a new `PutReportDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutReportDefinition {
    type Output = std::result::Result<
        crate::output::PutReportDefinitionOutput,
        crate::error::PutReportDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_report_definition_error(response)
        } else {
            crate::operation_deser::parse_put_report_definition_response(response)
        }
    }
}
