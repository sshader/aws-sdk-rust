// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Contains the information of an agent
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct Agent {
    /// Identifier for a resource.
    pub agent_id: ::std::string::String,
    /// Name for a resource.
    pub agent_name: ::std::string::String,
    /// Arn representation of the Agent.
    pub agent_arn: ::std::string::String,
    /// Draft Agent Version.
    pub agent_version: ::std::string::String,
    /// Client specified token used for idempotency checks
    pub client_token: ::std::option::Option<::std::string::String>,
    /// Instruction for the agent.
    pub instruction: ::std::option::Option<::std::string::String>,
    /// Schema Type for Action APIs.
    pub agent_status: crate::types::AgentStatus,
    /// ARN or name of a Bedrock model.
    pub foundation_model: ::std::option::Option<::std::string::String>,
    /// Description of the Resource.
    pub description: ::std::option::Option<::std::string::String>,
    /// Max Session Time.
    pub idle_session_ttl_in_seconds: i32,
    /// ARN of a IAM role.
    pub agent_resource_role_arn: ::std::string::String,
    /// A KMS key ARN
    pub customer_encryption_key_arn: ::std::option::Option<::std::string::String>,
    /// Time Stamp.
    pub created_at: ::aws_smithy_types::DateTime,
    /// Time Stamp.
    pub updated_at: ::aws_smithy_types::DateTime,
    /// Time Stamp.
    pub prepared_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// Failure Reasons for Error.
    pub failure_reasons: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The recommended actions users can take to resolve an error in failureReasons.
    pub recommended_actions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// Configuration for prompt override.
    pub prompt_override_configuration: ::std::option::Option<crate::types::PromptOverrideConfiguration>,
}
impl Agent {
    /// Identifier for a resource.
    pub fn agent_id(&self) -> &str {
        use std::ops::Deref;
        self.agent_id.deref()
    }
    /// Name for a resource.
    pub fn agent_name(&self) -> &str {
        use std::ops::Deref;
        self.agent_name.deref()
    }
    /// Arn representation of the Agent.
    pub fn agent_arn(&self) -> &str {
        use std::ops::Deref;
        self.agent_arn.deref()
    }
    /// Draft Agent Version.
    pub fn agent_version(&self) -> &str {
        use std::ops::Deref;
        self.agent_version.deref()
    }
    /// Client specified token used for idempotency checks
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// Instruction for the agent.
    pub fn instruction(&self) -> ::std::option::Option<&str> {
        self.instruction.as_deref()
    }
    /// Schema Type for Action APIs.
    pub fn agent_status(&self) -> &crate::types::AgentStatus {
        &self.agent_status
    }
    /// ARN or name of a Bedrock model.
    pub fn foundation_model(&self) -> ::std::option::Option<&str> {
        self.foundation_model.as_deref()
    }
    /// Description of the Resource.
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// Max Session Time.
    pub fn idle_session_ttl_in_seconds(&self) -> i32 {
        self.idle_session_ttl_in_seconds
    }
    /// ARN of a IAM role.
    pub fn agent_resource_role_arn(&self) -> &str {
        use std::ops::Deref;
        self.agent_resource_role_arn.deref()
    }
    /// A KMS key ARN
    pub fn customer_encryption_key_arn(&self) -> ::std::option::Option<&str> {
        self.customer_encryption_key_arn.as_deref()
    }
    /// Time Stamp.
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// Time Stamp.
    pub fn updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.updated_at
    }
    /// Time Stamp.
    pub fn prepared_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.prepared_at.as_ref()
    }
    /// Failure Reasons for Error.
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.failure_reasons.is_none()`.
    pub fn failure_reasons(&self) -> &[::std::string::String] {
        self.failure_reasons.as_deref().unwrap_or_default()
    }
    /// The recommended actions users can take to resolve an error in failureReasons.
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.recommended_actions.is_none()`.
    pub fn recommended_actions(&self) -> &[::std::string::String] {
        self.recommended_actions.as_deref().unwrap_or_default()
    }
    /// Configuration for prompt override.
    pub fn prompt_override_configuration(&self) -> ::std::option::Option<&crate::types::PromptOverrideConfiguration> {
        self.prompt_override_configuration.as_ref()
    }
}
impl ::std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("Agent");
        formatter.field("agent_id", &self.agent_id);
        formatter.field("agent_name", &self.agent_name);
        formatter.field("agent_arn", &self.agent_arn);
        formatter.field("agent_version", &self.agent_version);
        formatter.field("client_token", &self.client_token);
        formatter.field("instruction", &"*** Sensitive Data Redacted ***");
        formatter.field("agent_status", &self.agent_status);
        formatter.field("foundation_model", &self.foundation_model);
        formatter.field("description", &self.description);
        formatter.field("idle_session_ttl_in_seconds", &self.idle_session_ttl_in_seconds);
        formatter.field("agent_resource_role_arn", &self.agent_resource_role_arn);
        formatter.field("customer_encryption_key_arn", &self.customer_encryption_key_arn);
        formatter.field("created_at", &self.created_at);
        formatter.field("updated_at", &self.updated_at);
        formatter.field("prepared_at", &self.prepared_at);
        formatter.field("failure_reasons", &self.failure_reasons);
        formatter.field("recommended_actions", &self.recommended_actions);
        formatter.field("prompt_override_configuration", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl Agent {
    /// Creates a new builder-style object to manufacture [`Agent`](crate::types::Agent).
    pub fn builder() -> crate::types::builders::AgentBuilder {
        crate::types::builders::AgentBuilder::default()
    }
}

/// A builder for [`Agent`](crate::types::Agent).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AgentBuilder {
    pub(crate) agent_id: ::std::option::Option<::std::string::String>,
    pub(crate) agent_name: ::std::option::Option<::std::string::String>,
    pub(crate) agent_arn: ::std::option::Option<::std::string::String>,
    pub(crate) agent_version: ::std::option::Option<::std::string::String>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) instruction: ::std::option::Option<::std::string::String>,
    pub(crate) agent_status: ::std::option::Option<crate::types::AgentStatus>,
    pub(crate) foundation_model: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) idle_session_ttl_in_seconds: ::std::option::Option<i32>,
    pub(crate) agent_resource_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) customer_encryption_key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) prepared_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) failure_reasons: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) recommended_actions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) prompt_override_configuration: ::std::option::Option<crate::types::PromptOverrideConfiguration>,
}
impl AgentBuilder {
    /// Identifier for a resource.
    /// This field is required.
    pub fn agent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier for a resource.
    pub fn set_agent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_id = input;
        self
    }
    /// Identifier for a resource.
    pub fn get_agent_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_id
    }
    /// Name for a resource.
    /// This field is required.
    pub fn agent_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_name = ::std::option::Option::Some(input.into());
        self
    }
    /// Name for a resource.
    pub fn set_agent_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_name = input;
        self
    }
    /// Name for a resource.
    pub fn get_agent_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_name
    }
    /// Arn representation of the Agent.
    /// This field is required.
    pub fn agent_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// Arn representation of the Agent.
    pub fn set_agent_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_arn = input;
        self
    }
    /// Arn representation of the Agent.
    pub fn get_agent_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_arn
    }
    /// Draft Agent Version.
    /// This field is required.
    pub fn agent_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_version = ::std::option::Option::Some(input.into());
        self
    }
    /// Draft Agent Version.
    pub fn set_agent_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_version = input;
        self
    }
    /// Draft Agent Version.
    pub fn get_agent_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_version
    }
    /// Client specified token used for idempotency checks
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Client specified token used for idempotency checks
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Client specified token used for idempotency checks
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// Instruction for the agent.
    pub fn instruction(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instruction = ::std::option::Option::Some(input.into());
        self
    }
    /// Instruction for the agent.
    pub fn set_instruction(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instruction = input;
        self
    }
    /// Instruction for the agent.
    pub fn get_instruction(&self) -> &::std::option::Option<::std::string::String> {
        &self.instruction
    }
    /// Schema Type for Action APIs.
    /// This field is required.
    pub fn agent_status(mut self, input: crate::types::AgentStatus) -> Self {
        self.agent_status = ::std::option::Option::Some(input);
        self
    }
    /// Schema Type for Action APIs.
    pub fn set_agent_status(mut self, input: ::std::option::Option<crate::types::AgentStatus>) -> Self {
        self.agent_status = input;
        self
    }
    /// Schema Type for Action APIs.
    pub fn get_agent_status(&self) -> &::std::option::Option<crate::types::AgentStatus> {
        &self.agent_status
    }
    /// ARN or name of a Bedrock model.
    pub fn foundation_model(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.foundation_model = ::std::option::Option::Some(input.into());
        self
    }
    /// ARN or name of a Bedrock model.
    pub fn set_foundation_model(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.foundation_model = input;
        self
    }
    /// ARN or name of a Bedrock model.
    pub fn get_foundation_model(&self) -> &::std::option::Option<::std::string::String> {
        &self.foundation_model
    }
    /// Description of the Resource.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// Description of the Resource.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Description of the Resource.
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Max Session Time.
    /// This field is required.
    pub fn idle_session_ttl_in_seconds(mut self, input: i32) -> Self {
        self.idle_session_ttl_in_seconds = ::std::option::Option::Some(input);
        self
    }
    /// Max Session Time.
    pub fn set_idle_session_ttl_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.idle_session_ttl_in_seconds = input;
        self
    }
    /// Max Session Time.
    pub fn get_idle_session_ttl_in_seconds(&self) -> &::std::option::Option<i32> {
        &self.idle_session_ttl_in_seconds
    }
    /// ARN of a IAM role.
    /// This field is required.
    pub fn agent_resource_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_resource_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// ARN of a IAM role.
    pub fn set_agent_resource_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_resource_role_arn = input;
        self
    }
    /// ARN of a IAM role.
    pub fn get_agent_resource_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_resource_role_arn
    }
    /// A KMS key ARN
    pub fn customer_encryption_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customer_encryption_key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// A KMS key ARN
    pub fn set_customer_encryption_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customer_encryption_key_arn = input;
        self
    }
    /// A KMS key ARN
    pub fn get_customer_encryption_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.customer_encryption_key_arn
    }
    /// Time Stamp.
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// Time Stamp.
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// Time Stamp.
    /// This field is required.
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// Time Stamp.
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    /// Time Stamp.
    pub fn prepared_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.prepared_at = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_prepared_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.prepared_at = input;
        self
    }
    /// Time Stamp.
    pub fn get_prepared_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.prepared_at
    }
    /// Appends an item to `failure_reasons`.
    ///
    /// To override the contents of this collection use [`set_failure_reasons`](Self::set_failure_reasons).
    ///
    /// Failure Reasons for Error.
    pub fn failure_reasons(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.failure_reasons.unwrap_or_default();
        v.push(input.into());
        self.failure_reasons = ::std::option::Option::Some(v);
        self
    }
    /// Failure Reasons for Error.
    pub fn set_failure_reasons(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.failure_reasons = input;
        self
    }
    /// Failure Reasons for Error.
    pub fn get_failure_reasons(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.failure_reasons
    }
    /// Appends an item to `recommended_actions`.
    ///
    /// To override the contents of this collection use [`set_recommended_actions`](Self::set_recommended_actions).
    ///
    /// The recommended actions users can take to resolve an error in failureReasons.
    pub fn recommended_actions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.recommended_actions.unwrap_or_default();
        v.push(input.into());
        self.recommended_actions = ::std::option::Option::Some(v);
        self
    }
    /// The recommended actions users can take to resolve an error in failureReasons.
    pub fn set_recommended_actions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.recommended_actions = input;
        self
    }
    /// The recommended actions users can take to resolve an error in failureReasons.
    pub fn get_recommended_actions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.recommended_actions
    }
    /// Configuration for prompt override.
    pub fn prompt_override_configuration(mut self, input: crate::types::PromptOverrideConfiguration) -> Self {
        self.prompt_override_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Configuration for prompt override.
    pub fn set_prompt_override_configuration(mut self, input: ::std::option::Option<crate::types::PromptOverrideConfiguration>) -> Self {
        self.prompt_override_configuration = input;
        self
    }
    /// Configuration for prompt override.
    pub fn get_prompt_override_configuration(&self) -> &::std::option::Option<crate::types::PromptOverrideConfiguration> {
        &self.prompt_override_configuration
    }
    /// Consumes the builder and constructs a [`Agent`](crate::types::Agent).
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](crate::types::builders::AgentBuilder::agent_id)
    /// - [`agent_name`](crate::types::builders::AgentBuilder::agent_name)
    /// - [`agent_arn`](crate::types::builders::AgentBuilder::agent_arn)
    /// - [`agent_version`](crate::types::builders::AgentBuilder::agent_version)
    /// - [`agent_status`](crate::types::builders::AgentBuilder::agent_status)
    /// - [`idle_session_ttl_in_seconds`](crate::types::builders::AgentBuilder::idle_session_ttl_in_seconds)
    /// - [`agent_resource_role_arn`](crate::types::builders::AgentBuilder::agent_resource_role_arn)
    /// - [`created_at`](crate::types::builders::AgentBuilder::created_at)
    /// - [`updated_at`](crate::types::builders::AgentBuilder::updated_at)
    pub fn build(self) -> ::std::result::Result<crate::types::Agent, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Agent {
            agent_id: self.agent_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_id",
                    "agent_id was not specified but it is required when building Agent",
                )
            })?,
            agent_name: self.agent_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_name",
                    "agent_name was not specified but it is required when building Agent",
                )
            })?,
            agent_arn: self.agent_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_arn",
                    "agent_arn was not specified but it is required when building Agent",
                )
            })?,
            agent_version: self.agent_version.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_version",
                    "agent_version was not specified but it is required when building Agent",
                )
            })?,
            client_token: self.client_token,
            instruction: self.instruction,
            agent_status: self.agent_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_status",
                    "agent_status was not specified but it is required when building Agent",
                )
            })?,
            foundation_model: self.foundation_model,
            description: self.description,
            idle_session_ttl_in_seconds: self.idle_session_ttl_in_seconds.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "idle_session_ttl_in_seconds",
                    "idle_session_ttl_in_seconds was not specified but it is required when building Agent",
                )
            })?,
            agent_resource_role_arn: self.agent_resource_role_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_resource_role_arn",
                    "agent_resource_role_arn was not specified but it is required when building Agent",
                )
            })?,
            customer_encryption_key_arn: self.customer_encryption_key_arn,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building Agent",
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "updated_at",
                    "updated_at was not specified but it is required when building Agent",
                )
            })?,
            prepared_at: self.prepared_at,
            failure_reasons: self.failure_reasons,
            recommended_actions: self.recommended_actions,
            prompt_override_configuration: self.prompt_override_configuration,
        })
    }
}
impl ::std::fmt::Debug for AgentBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AgentBuilder");
        formatter.field("agent_id", &self.agent_id);
        formatter.field("agent_name", &self.agent_name);
        formatter.field("agent_arn", &self.agent_arn);
        formatter.field("agent_version", &self.agent_version);
        formatter.field("client_token", &self.client_token);
        formatter.field("instruction", &"*** Sensitive Data Redacted ***");
        formatter.field("agent_status", &self.agent_status);
        formatter.field("foundation_model", &self.foundation_model);
        formatter.field("description", &self.description);
        formatter.field("idle_session_ttl_in_seconds", &self.idle_session_ttl_in_seconds);
        formatter.field("agent_resource_role_arn", &self.agent_resource_role_arn);
        formatter.field("customer_encryption_key_arn", &self.customer_encryption_key_arn);
        formatter.field("created_at", &self.created_at);
        formatter.field("updated_at", &self.updated_at);
        formatter.field("prepared_at", &self.prepared_at);
        formatter.field("failure_reasons", &self.failure_reasons);
        formatter.field("recommended_actions", &self.recommended_actions);
        formatter.field("prompt_override_configuration", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
