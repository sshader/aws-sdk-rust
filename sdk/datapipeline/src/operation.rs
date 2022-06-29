// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `ActivatePipeline`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`activate_pipeline`](crate::client::Client::activate_pipeline).
///
/// See [`crate::client::fluent_builders::ActivatePipeline`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ActivatePipeline {
    _private: (),
}
impl ActivatePipeline {
    /// Creates a new builder-style object to manufacture [`ActivatePipelineInput`](crate::input::ActivatePipelineInput).
    pub fn builder() -> crate::input::activate_pipeline_input::Builder {
        crate::input::activate_pipeline_input::Builder::default()
    }
    /// Creates a new `ActivatePipeline` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ActivatePipeline {
    type Output = std::result::Result<
        crate::output::ActivatePipelineOutput,
        crate::error::ActivatePipelineError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_activate_pipeline_error(response)
        } else {
            crate::operation_deser::parse_activate_pipeline_response(response)
        }
    }
}

/// Operation shape for `AddTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_tags`](crate::client::Client::add_tags).
///
/// See [`crate::client::fluent_builders::AddTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddTags {
    _private: (),
}
impl AddTags {
    /// Creates a new builder-style object to manufacture [`AddTagsInput`](crate::input::AddTagsInput).
    pub fn builder() -> crate::input::add_tags_input::Builder {
        crate::input::add_tags_input::Builder::default()
    }
    /// Creates a new `AddTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddTags {
    type Output = std::result::Result<crate::output::AddTagsOutput, crate::error::AddTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_tags_error(response)
        } else {
            crate::operation_deser::parse_add_tags_response(response)
        }
    }
}

/// Operation shape for `CreatePipeline`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_pipeline`](crate::client::Client::create_pipeline).
///
/// See [`crate::client::fluent_builders::CreatePipeline`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreatePipeline {
    _private: (),
}
impl CreatePipeline {
    /// Creates a new builder-style object to manufacture [`CreatePipelineInput`](crate::input::CreatePipelineInput).
    pub fn builder() -> crate::input::create_pipeline_input::Builder {
        crate::input::create_pipeline_input::Builder::default()
    }
    /// Creates a new `CreatePipeline` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePipeline {
    type Output =
        std::result::Result<crate::output::CreatePipelineOutput, crate::error::CreatePipelineError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_pipeline_error(response)
        } else {
            crate::operation_deser::parse_create_pipeline_response(response)
        }
    }
}

/// Operation shape for `DeactivatePipeline`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deactivate_pipeline`](crate::client::Client::deactivate_pipeline).
///
/// See [`crate::client::fluent_builders::DeactivatePipeline`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeactivatePipeline {
    _private: (),
}
impl DeactivatePipeline {
    /// Creates a new builder-style object to manufacture [`DeactivatePipelineInput`](crate::input::DeactivatePipelineInput).
    pub fn builder() -> crate::input::deactivate_pipeline_input::Builder {
        crate::input::deactivate_pipeline_input::Builder::default()
    }
    /// Creates a new `DeactivatePipeline` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeactivatePipeline {
    type Output = std::result::Result<
        crate::output::DeactivatePipelineOutput,
        crate::error::DeactivatePipelineError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deactivate_pipeline_error(response)
        } else {
            crate::operation_deser::parse_deactivate_pipeline_response(response)
        }
    }
}

/// Operation shape for `DeletePipeline`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_pipeline`](crate::client::Client::delete_pipeline).
///
/// See [`crate::client::fluent_builders::DeletePipeline`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeletePipeline {
    _private: (),
}
impl DeletePipeline {
    /// Creates a new builder-style object to manufacture [`DeletePipelineInput`](crate::input::DeletePipelineInput).
    pub fn builder() -> crate::input::delete_pipeline_input::Builder {
        crate::input::delete_pipeline_input::Builder::default()
    }
    /// Creates a new `DeletePipeline` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePipeline {
    type Output =
        std::result::Result<crate::output::DeletePipelineOutput, crate::error::DeletePipelineError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_pipeline_error(response)
        } else {
            crate::operation_deser::parse_delete_pipeline_response(response)
        }
    }
}

/// Operation shape for `DescribeObjects`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_objects`](crate::client::Client::describe_objects).
///
/// See [`crate::client::fluent_builders::DescribeObjects`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeObjects {
    _private: (),
}
impl DescribeObjects {
    /// Creates a new builder-style object to manufacture [`DescribeObjectsInput`](crate::input::DescribeObjectsInput).
    pub fn builder() -> crate::input::describe_objects_input::Builder {
        crate::input::describe_objects_input::Builder::default()
    }
    /// Creates a new `DescribeObjects` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeObjects {
    type Output = std::result::Result<
        crate::output::DescribeObjectsOutput,
        crate::error::DescribeObjectsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_objects_error(response)
        } else {
            crate::operation_deser::parse_describe_objects_response(response)
        }
    }
}

/// Operation shape for `DescribePipelines`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_pipelines`](crate::client::Client::describe_pipelines).
///
/// See [`crate::client::fluent_builders::DescribePipelines`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribePipelines {
    _private: (),
}
impl DescribePipelines {
    /// Creates a new builder-style object to manufacture [`DescribePipelinesInput`](crate::input::DescribePipelinesInput).
    pub fn builder() -> crate::input::describe_pipelines_input::Builder {
        crate::input::describe_pipelines_input::Builder::default()
    }
    /// Creates a new `DescribePipelines` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribePipelines {
    type Output = std::result::Result<
        crate::output::DescribePipelinesOutput,
        crate::error::DescribePipelinesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_pipelines_error(response)
        } else {
            crate::operation_deser::parse_describe_pipelines_response(response)
        }
    }
}

/// Operation shape for `EvaluateExpression`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`evaluate_expression`](crate::client::Client::evaluate_expression).
///
/// See [`crate::client::fluent_builders::EvaluateExpression`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct EvaluateExpression {
    _private: (),
}
impl EvaluateExpression {
    /// Creates a new builder-style object to manufacture [`EvaluateExpressionInput`](crate::input::EvaluateExpressionInput).
    pub fn builder() -> crate::input::evaluate_expression_input::Builder {
        crate::input::evaluate_expression_input::Builder::default()
    }
    /// Creates a new `EvaluateExpression` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for EvaluateExpression {
    type Output = std::result::Result<
        crate::output::EvaluateExpressionOutput,
        crate::error::EvaluateExpressionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_evaluate_expression_error(response)
        } else {
            crate::operation_deser::parse_evaluate_expression_response(response)
        }
    }
}

/// Operation shape for `GetPipelineDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_pipeline_definition`](crate::client::Client::get_pipeline_definition).
///
/// See [`crate::client::fluent_builders::GetPipelineDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetPipelineDefinition {
    _private: (),
}
impl GetPipelineDefinition {
    /// Creates a new builder-style object to manufacture [`GetPipelineDefinitionInput`](crate::input::GetPipelineDefinitionInput).
    pub fn builder() -> crate::input::get_pipeline_definition_input::Builder {
        crate::input::get_pipeline_definition_input::Builder::default()
    }
    /// Creates a new `GetPipelineDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPipelineDefinition {
    type Output = std::result::Result<
        crate::output::GetPipelineDefinitionOutput,
        crate::error::GetPipelineDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_pipeline_definition_error(response)
        } else {
            crate::operation_deser::parse_get_pipeline_definition_response(response)
        }
    }
}

/// Operation shape for `ListPipelines`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_pipelines`](crate::client::Client::list_pipelines).
///
/// See [`crate::client::fluent_builders::ListPipelines`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPipelines {
    _private: (),
}
impl ListPipelines {
    /// Creates a new builder-style object to manufacture [`ListPipelinesInput`](crate::input::ListPipelinesInput).
    pub fn builder() -> crate::input::list_pipelines_input::Builder {
        crate::input::list_pipelines_input::Builder::default()
    }
    /// Creates a new `ListPipelines` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPipelines {
    type Output =
        std::result::Result<crate::output::ListPipelinesOutput, crate::error::ListPipelinesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pipelines_error(response)
        } else {
            crate::operation_deser::parse_list_pipelines_response(response)
        }
    }
}

/// Operation shape for `PollForTask`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`poll_for_task`](crate::client::Client::poll_for_task).
///
/// See [`crate::client::fluent_builders::PollForTask`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PollForTask {
    _private: (),
}
impl PollForTask {
    /// Creates a new builder-style object to manufacture [`PollForTaskInput`](crate::input::PollForTaskInput).
    pub fn builder() -> crate::input::poll_for_task_input::Builder {
        crate::input::poll_for_task_input::Builder::default()
    }
    /// Creates a new `PollForTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PollForTask {
    type Output =
        std::result::Result<crate::output::PollForTaskOutput, crate::error::PollForTaskError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_poll_for_task_error(response)
        } else {
            crate::operation_deser::parse_poll_for_task_response(response)
        }
    }
}

/// Operation shape for `PutPipelineDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_pipeline_definition`](crate::client::Client::put_pipeline_definition).
///
/// See [`crate::client::fluent_builders::PutPipelineDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutPipelineDefinition {
    _private: (),
}
impl PutPipelineDefinition {
    /// Creates a new builder-style object to manufacture [`PutPipelineDefinitionInput`](crate::input::PutPipelineDefinitionInput).
    pub fn builder() -> crate::input::put_pipeline_definition_input::Builder {
        crate::input::put_pipeline_definition_input::Builder::default()
    }
    /// Creates a new `PutPipelineDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutPipelineDefinition {
    type Output = std::result::Result<
        crate::output::PutPipelineDefinitionOutput,
        crate::error::PutPipelineDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_pipeline_definition_error(response)
        } else {
            crate::operation_deser::parse_put_pipeline_definition_response(response)
        }
    }
}

/// Operation shape for `QueryObjects`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`query_objects`](crate::client::Client::query_objects).
///
/// See [`crate::client::fluent_builders::QueryObjects`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct QueryObjects {
    _private: (),
}
impl QueryObjects {
    /// Creates a new builder-style object to manufacture [`QueryObjectsInput`](crate::input::QueryObjectsInput).
    pub fn builder() -> crate::input::query_objects_input::Builder {
        crate::input::query_objects_input::Builder::default()
    }
    /// Creates a new `QueryObjects` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for QueryObjects {
    type Output =
        std::result::Result<crate::output::QueryObjectsOutput, crate::error::QueryObjectsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_query_objects_error(response)
        } else {
            crate::operation_deser::parse_query_objects_response(response)
        }
    }
}

/// Operation shape for `RemoveTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`remove_tags`](crate::client::Client::remove_tags).
///
/// See [`crate::client::fluent_builders::RemoveTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RemoveTags {
    _private: (),
}
impl RemoveTags {
    /// Creates a new builder-style object to manufacture [`RemoveTagsInput`](crate::input::RemoveTagsInput).
    pub fn builder() -> crate::input::remove_tags_input::Builder {
        crate::input::remove_tags_input::Builder::default()
    }
    /// Creates a new `RemoveTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RemoveTags {
    type Output =
        std::result::Result<crate::output::RemoveTagsOutput, crate::error::RemoveTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_remove_tags_error(response)
        } else {
            crate::operation_deser::parse_remove_tags_response(response)
        }
    }
}

/// Operation shape for `ReportTaskProgress`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`report_task_progress`](crate::client::Client::report_task_progress).
///
/// See [`crate::client::fluent_builders::ReportTaskProgress`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ReportTaskProgress {
    _private: (),
}
impl ReportTaskProgress {
    /// Creates a new builder-style object to manufacture [`ReportTaskProgressInput`](crate::input::ReportTaskProgressInput).
    pub fn builder() -> crate::input::report_task_progress_input::Builder {
        crate::input::report_task_progress_input::Builder::default()
    }
    /// Creates a new `ReportTaskProgress` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ReportTaskProgress {
    type Output = std::result::Result<
        crate::output::ReportTaskProgressOutput,
        crate::error::ReportTaskProgressError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_report_task_progress_error(response)
        } else {
            crate::operation_deser::parse_report_task_progress_response(response)
        }
    }
}

/// Operation shape for `ReportTaskRunnerHeartbeat`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`report_task_runner_heartbeat`](crate::client::Client::report_task_runner_heartbeat).
///
/// See [`crate::client::fluent_builders::ReportTaskRunnerHeartbeat`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ReportTaskRunnerHeartbeat {
    _private: (),
}
impl ReportTaskRunnerHeartbeat {
    /// Creates a new builder-style object to manufacture [`ReportTaskRunnerHeartbeatInput`](crate::input::ReportTaskRunnerHeartbeatInput).
    pub fn builder() -> crate::input::report_task_runner_heartbeat_input::Builder {
        crate::input::report_task_runner_heartbeat_input::Builder::default()
    }
    /// Creates a new `ReportTaskRunnerHeartbeat` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ReportTaskRunnerHeartbeat {
    type Output = std::result::Result<
        crate::output::ReportTaskRunnerHeartbeatOutput,
        crate::error::ReportTaskRunnerHeartbeatError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_report_task_runner_heartbeat_error(response)
        } else {
            crate::operation_deser::parse_report_task_runner_heartbeat_response(response)
        }
    }
}

/// Operation shape for `SetStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`set_status`](crate::client::Client::set_status).
///
/// See [`crate::client::fluent_builders::SetStatus`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SetStatus {
    _private: (),
}
impl SetStatus {
    /// Creates a new builder-style object to manufacture [`SetStatusInput`](crate::input::SetStatusInput).
    pub fn builder() -> crate::input::set_status_input::Builder {
        crate::input::set_status_input::Builder::default()
    }
    /// Creates a new `SetStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SetStatus {
    type Output = std::result::Result<crate::output::SetStatusOutput, crate::error::SetStatusError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_set_status_error(response)
        } else {
            crate::operation_deser::parse_set_status_response(response)
        }
    }
}

/// Operation shape for `SetTaskStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`set_task_status`](crate::client::Client::set_task_status).
///
/// See [`crate::client::fluent_builders::SetTaskStatus`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SetTaskStatus {
    _private: (),
}
impl SetTaskStatus {
    /// Creates a new builder-style object to manufacture [`SetTaskStatusInput`](crate::input::SetTaskStatusInput).
    pub fn builder() -> crate::input::set_task_status_input::Builder {
        crate::input::set_task_status_input::Builder::default()
    }
    /// Creates a new `SetTaskStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SetTaskStatus {
    type Output =
        std::result::Result<crate::output::SetTaskStatusOutput, crate::error::SetTaskStatusError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_set_task_status_error(response)
        } else {
            crate::operation_deser::parse_set_task_status_response(response)
        }
    }
}

/// Operation shape for `ValidatePipelineDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`validate_pipeline_definition`](crate::client::Client::validate_pipeline_definition).
///
/// See [`crate::client::fluent_builders::ValidatePipelineDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ValidatePipelineDefinition {
    _private: (),
}
impl ValidatePipelineDefinition {
    /// Creates a new builder-style object to manufacture [`ValidatePipelineDefinitionInput`](crate::input::ValidatePipelineDefinitionInput).
    pub fn builder() -> crate::input::validate_pipeline_definition_input::Builder {
        crate::input::validate_pipeline_definition_input::Builder::default()
    }
    /// Creates a new `ValidatePipelineDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ValidatePipelineDefinition {
    type Output = std::result::Result<
        crate::output::ValidatePipelineDefinitionOutput,
        crate::error::ValidatePipelineDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_validate_pipeline_definition_error(response)
        } else {
            crate::operation_deser::parse_validate_pipeline_definition_response(response)
        }
    }
}
