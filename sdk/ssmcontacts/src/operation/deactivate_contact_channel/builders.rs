// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deactivate_contact_channel::_deactivate_contact_channel_output::DeactivateContactChannelOutputBuilder;

pub use crate::operation::deactivate_contact_channel::_deactivate_contact_channel_input::DeactivateContactChannelInputBuilder;

impl DeactivateContactChannelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deactivate_contact_channel::DeactivateContactChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deactivate_contact_channel::DeactivateContactChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deactivate_contact_channel();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeactivateContactChannel`.
///
/// <p>To no longer receive Incident Manager engagements to a contact channel, you can deactivate the channel.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeactivateContactChannelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deactivate_contact_channel::builders::DeactivateContactChannelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::deactivate_contact_channel::DeactivateContactChannelOutput,
        crate::operation::deactivate_contact_channel::DeactivateContactChannelError,
    > for DeactivateContactChannelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::deactivate_contact_channel::DeactivateContactChannelOutput,
            crate::operation::deactivate_contact_channel::DeactivateContactChannelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeactivateContactChannelFluentBuilder {
    /// Creates a new `DeactivateContactChannel`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeactivateContactChannel as a reference.
    pub fn as_input(&self) -> &crate::operation::deactivate_contact_channel::builders::DeactivateContactChannelInputBuilder {
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
        crate::operation::deactivate_contact_channel::DeactivateContactChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deactivate_contact_channel::DeactivateContactChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deactivate_contact_channel::DeactivateContactChannel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deactivate_contact_channel::DeactivateContactChannel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::deactivate_contact_channel::DeactivateContactChannelOutput,
        crate::operation::deactivate_contact_channel::DeactivateContactChannelError,
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
    /// <p>The Amazon Resource Name (ARN) of the contact channel you're deactivating.</p>
    pub fn contact_channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.contact_channel_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the contact channel you're deactivating.</p>
    pub fn set_contact_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_contact_channel_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the contact channel you're deactivating.</p>
    pub fn get_contact_channel_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_contact_channel_id()
    }
}
