// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::check_no_new_access::_check_no_new_access_output::CheckNoNewAccessOutputBuilder;

pub use crate::operation::check_no_new_access::_check_no_new_access_input::CheckNoNewAccessInputBuilder;

impl CheckNoNewAccessInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::check_no_new_access::CheckNoNewAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::check_no_new_access::CheckNoNewAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.check_no_new_access();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CheckNoNewAccess`.
///
/// <p>Checks whether new access is allowed for an updated policy when compared to the existing policy.</p>
/// <p>You can find examples for reference policies and learn how to set up and run a custom policy check for new access in the <a href="https://github.com/aws-samples/iam-access-analyzer-custom-policy-check-samples">IAM Access Analyzer custom policy checks samples</a> repository on GitHub. The reference policies in this repository are meant to be passed to the <code>existingPolicyDocument</code> request parameter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CheckNoNewAccessFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::check_no_new_access::builders::CheckNoNewAccessInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::check_no_new_access::CheckNoNewAccessOutput,
        crate::operation::check_no_new_access::CheckNoNewAccessError,
    > for CheckNoNewAccessFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::check_no_new_access::CheckNoNewAccessOutput,
            crate::operation::check_no_new_access::CheckNoNewAccessError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CheckNoNewAccessFluentBuilder {
    /// Creates a new `CheckNoNewAccess`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CheckNoNewAccess as a reference.
    pub fn as_input(&self) -> &crate::operation::check_no_new_access::builders::CheckNoNewAccessInputBuilder {
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
        crate::operation::check_no_new_access::CheckNoNewAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::check_no_new_access::CheckNoNewAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::check_no_new_access::CheckNoNewAccess::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::check_no_new_access::CheckNoNewAccess::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::check_no_new_access::CheckNoNewAccessOutput,
        crate::operation::check_no_new_access::CheckNoNewAccessError,
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
    /// <p>The JSON policy document to use as the content for the updated policy.</p>
    pub fn new_policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.new_policy_document(input.into());
        self
    }
    /// <p>The JSON policy document to use as the content for the updated policy.</p>
    pub fn set_new_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_new_policy_document(input);
        self
    }
    /// <p>The JSON policy document to use as the content for the updated policy.</p>
    pub fn get_new_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_new_policy_document()
    }
    /// <p>The JSON policy document to use as the content for the existing policy.</p>
    pub fn existing_policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.existing_policy_document(input.into());
        self
    }
    /// <p>The JSON policy document to use as the content for the existing policy.</p>
    pub fn set_existing_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_existing_policy_document(input);
        self
    }
    /// <p>The JSON policy document to use as the content for the existing policy.</p>
    pub fn get_existing_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_existing_policy_document()
    }
    /// <p>The type of policy to compare. Identity policies grant permissions to IAM principals. Identity policies include managed and inline policies for IAM roles, users, and groups.</p>
    /// <p>Resource policies grant permissions on Amazon Web Services resources. Resource policies include trust policies for IAM roles and bucket policies for Amazon S3 buckets. You can provide a generic input such as identity policy or resource policy or a specific input such as managed policy or Amazon S3 bucket policy.</p>
    pub fn policy_type(mut self, input: crate::types::AccessCheckPolicyType) -> Self {
        self.inner = self.inner.policy_type(input);
        self
    }
    /// <p>The type of policy to compare. Identity policies grant permissions to IAM principals. Identity policies include managed and inline policies for IAM roles, users, and groups.</p>
    /// <p>Resource policies grant permissions on Amazon Web Services resources. Resource policies include trust policies for IAM roles and bucket policies for Amazon S3 buckets. You can provide a generic input such as identity policy or resource policy or a specific input such as managed policy or Amazon S3 bucket policy.</p>
    pub fn set_policy_type(mut self, input: ::std::option::Option<crate::types::AccessCheckPolicyType>) -> Self {
        self.inner = self.inner.set_policy_type(input);
        self
    }
    /// <p>The type of policy to compare. Identity policies grant permissions to IAM principals. Identity policies include managed and inline policies for IAM roles, users, and groups.</p>
    /// <p>Resource policies grant permissions on Amazon Web Services resources. Resource policies include trust policies for IAM roles and bucket policies for Amazon S3 buckets. You can provide a generic input such as identity policy or resource policy or a specific input such as managed policy or Amazon S3 bucket policy.</p>
    pub fn get_policy_type(&self) -> &::std::option::Option<crate::types::AccessCheckPolicyType> {
        self.inner.get_policy_type()
    }
}
