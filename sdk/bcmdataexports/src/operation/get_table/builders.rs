// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_table::_get_table_output::GetTableOutputBuilder;

pub use crate::operation::get_table::_get_table_input::GetTableInputBuilder;

impl GetTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_table::GetTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_table::GetTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetTable`.
///
/// <p>Returns the metadata for the specified table and table properties. This includes the list of columns in the table schema, their data types, and column descriptions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_table::builders::GetTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::get_table::GetTableOutput, crate::operation::get_table::GetTableError>
    for GetTableFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::get_table::GetTableOutput, crate::operation::get_table::GetTableError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetTableFluentBuilder {
    /// Creates a new `GetTable`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetTable as a reference.
    pub fn as_input(&self) -> &crate::operation::get_table::builders::GetTableInputBuilder {
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
        crate::operation::get_table::GetTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_table::GetTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_table::GetTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_table::GetTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::get_table::GetTableOutput, crate::operation::get_table::GetTableError, Self>
    {
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
    /// <p>The name of the table.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the table.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
    /// Adds a key-value pair to `TableProperties`.
    ///
    /// To override the contents of this collection use [`set_table_properties`](Self::set_table_properties).
    ///
    /// <p>TableProperties are additional configurations you can provide to change the data and schema of a table. Each table can have different TableProperties. Tables are not required to have any TableProperties. Each table property has a default value that it assumes if not specified.</p>
    pub fn table_properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.table_properties(k.into(), v.into());
        self
    }
    /// <p>TableProperties are additional configurations you can provide to change the data and schema of a table. Each table can have different TableProperties. Tables are not required to have any TableProperties. Each table property has a default value that it assumes if not specified.</p>
    pub fn set_table_properties(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_table_properties(input);
        self
    }
    /// <p>TableProperties are additional configurations you can provide to change the data and schema of a table. Each table can have different TableProperties. Tables are not required to have any TableProperties. Each table property has a default value that it assumes if not specified.</p>
    pub fn get_table_properties(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_table_properties()
    }
}
