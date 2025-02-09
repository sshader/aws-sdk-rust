// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_source_server::_delete_source_server_output::DeleteSourceServerOutputBuilder;

pub use crate::operation::delete_source_server::_delete_source_server_input::DeleteSourceServerInputBuilder;

impl DeleteSourceServerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_source_server::DeleteSourceServerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_source_server::DeleteSourceServerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_source_server();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteSourceServer`.
///
/// <p>Deletes a single Source Server by ID. The Source Server must be disconnected first.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteSourceServerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_source_server::builders::DeleteSourceServerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_source_server::DeleteSourceServerOutput,
        crate::operation::delete_source_server::DeleteSourceServerError,
    > for DeleteSourceServerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_source_server::DeleteSourceServerOutput,
            crate::operation::delete_source_server::DeleteSourceServerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteSourceServerFluentBuilder {
    /// Creates a new `DeleteSourceServer`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteSourceServer as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_source_server::builders::DeleteSourceServerInputBuilder {
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
        crate::operation::delete_source_server::DeleteSourceServerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_source_server::DeleteSourceServerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_source_server::DeleteSourceServer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_source_server::DeleteSourceServer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_source_server::DeleteSourceServerOutput,
        crate::operation::delete_source_server::DeleteSourceServerError,
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
    /// <p>The ID of the Source Server to be deleted.</p>
    pub fn source_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_server_id(input.into());
        self
    }
    /// <p>The ID of the Source Server to be deleted.</p>
    pub fn set_source_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_server_id(input);
        self
    }
    /// <p>The ID of the Source Server to be deleted.</p>
    pub fn get_source_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_server_id()
    }
}
