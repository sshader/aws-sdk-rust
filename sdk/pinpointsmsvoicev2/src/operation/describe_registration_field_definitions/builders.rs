// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_registration_field_definitions::_describe_registration_field_definitions_output::DescribeRegistrationFieldDefinitionsOutputBuilder;

pub use crate::operation::describe_registration_field_definitions::_describe_registration_field_definitions_input::DescribeRegistrationFieldDefinitionsInputBuilder;

impl DescribeRegistrationFieldDefinitionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_registration_field_definitions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeRegistrationFieldDefinitions`.
///
/// <p>Retrieves the specified registration type field definitions. You can use DescribeRegistrationFieldDefinitions to view the requirements for creating, filling out, and submitting each registration type.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeRegistrationFieldDefinitionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_registration_field_definitions::builders::DescribeRegistrationFieldDefinitionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsOutput,
        crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsError,
    > for DescribeRegistrationFieldDefinitionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsOutput,
            crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeRegistrationFieldDefinitionsFluentBuilder {
    /// Creates a new `DescribeRegistrationFieldDefinitions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeRegistrationFieldDefinitions as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_registration_field_definitions::builders::DescribeRegistrationFieldDefinitionsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitions::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsOutput,
        crate::operation::describe_registration_field_definitions::DescribeRegistrationFieldDefinitionsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_registration_field_definitions::paginator::DescribeRegistrationFieldDefinitionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_registration_field_definitions::paginator::DescribeRegistrationFieldDefinitionsPaginator {
        crate::operation::describe_registration_field_definitions::paginator::DescribeRegistrationFieldDefinitionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The type of registration form. The list of <b>RegistrationTypes</b> can be found using the <code>DescribeRegistrationTypeDefinitions</code> action.</p>
    pub fn registration_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.registration_type(input.into());
        self
    }
    /// <p>The type of registration form. The list of <b>RegistrationTypes</b> can be found using the <code>DescribeRegistrationTypeDefinitions</code> action.</p>
    pub fn set_registration_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_registration_type(input);
        self
    }
    /// <p>The type of registration form. The list of <b>RegistrationTypes</b> can be found using the <code>DescribeRegistrationTypeDefinitions</code> action.</p>
    pub fn get_registration_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_registration_type()
    }
    /// <p>The path to the section of the registration.</p>
    pub fn section_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.section_path(input.into());
        self
    }
    /// <p>The path to the section of the registration.</p>
    pub fn set_section_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_section_path(input);
        self
    }
    /// <p>The path to the section of the registration.</p>
    pub fn get_section_path(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_section_path()
    }
    /// Appends an item to `FieldPaths`.
    ///
    /// To override the contents of this collection use [`set_field_paths`](Self::set_field_paths).
    ///
    /// <p>An array of paths to the registration form field.</p>
    pub fn field_paths(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.field_paths(input.into());
        self
    }
    /// <p>An array of paths to the registration form field.</p>
    pub fn set_field_paths(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_field_paths(input);
        self
    }
    /// <p>An array of paths to the registration form field.</p>
    pub fn get_field_paths(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_field_paths()
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
