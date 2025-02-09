// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_enabled_baseline::_get_enabled_baseline_output::GetEnabledBaselineOutputBuilder;

pub use crate::operation::get_enabled_baseline::_get_enabled_baseline_input::GetEnabledBaselineInputBuilder;

impl GetEnabledBaselineInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_enabled_baseline::GetEnabledBaselineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_enabled_baseline::GetEnabledBaselineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_enabled_baseline();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetEnabledBaseline`.
///
/// <p>Retrieve details of an <code>EnabledBaseline</code> resource by specifying its identifier.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEnabledBaselineFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_enabled_baseline::builders::GetEnabledBaselineInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_enabled_baseline::GetEnabledBaselineOutput,
        crate::operation::get_enabled_baseline::GetEnabledBaselineError,
    > for GetEnabledBaselineFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_enabled_baseline::GetEnabledBaselineOutput,
            crate::operation::get_enabled_baseline::GetEnabledBaselineError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetEnabledBaselineFluentBuilder {
    /// Creates a new `GetEnabledBaseline`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetEnabledBaseline as a reference.
    pub fn as_input(&self) -> &crate::operation::get_enabled_baseline::builders::GetEnabledBaselineInputBuilder {
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
        crate::operation::get_enabled_baseline::GetEnabledBaselineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_enabled_baseline::GetEnabledBaselineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_enabled_baseline::GetEnabledBaseline::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_enabled_baseline::GetEnabledBaseline::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_enabled_baseline::GetEnabledBaselineOutput,
        crate::operation::get_enabled_baseline::GetEnabledBaselineError,
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
    /// <p>Identifier of the <code>EnabledBaseline</code> resource to be retrieved, in ARN format.</p>
    pub fn enabled_baseline_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.enabled_baseline_identifier(input.into());
        self
    }
    /// <p>Identifier of the <code>EnabledBaseline</code> resource to be retrieved, in ARN format.</p>
    pub fn set_enabled_baseline_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_enabled_baseline_identifier(input);
        self
    }
    /// <p>Identifier of the <code>EnabledBaseline</code> resource to be retrieved, in ARN format.</p>
    pub fn get_enabled_baseline_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_enabled_baseline_identifier()
    }
}
