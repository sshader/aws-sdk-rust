// Copied off of invoke

impl InvokeStreamedInput {}
/// Orchestration and serialization glue logic for `Invoke`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct InvokeStreamed;
impl InvokeStreamed {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::invoke_streamed::InvokeStreamedInput,
    ) -> ::std::result::Result<
        crate::operation::invoke::InvokeOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::invoke::InvokeError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>,
    > {
        let map_err =
            |err: ::aws_smithy_http::result::SdkError<
                ::aws_smithy_runtime_api::client::interceptors::context::Error,
                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
            >| { err.map_service_error(|err| err.downcast::<crate::operation::invoke_streamed::InvokeError>().expect("correct error type")) };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(output.downcast::<crate::operation::invoke_streamed::InvokeStreamedOutput>().expect("correct output type"))
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::invoke_streamed::InvokeStreamedInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("lambda", "InvokeStreamed", input, runtime_plugins, stop_point).await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());

        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for InvokeStreamed {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("Invoke");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            InvokeStreamedRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            InvokeStreamedResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new("Invoke", "lambda"));
        let mut signing_options = ::aws_runtime::auth::sigv4::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::sigv4::SigV4OperationSigningConfig {
            region: None,
            service: None,
            signing_options,
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(&self) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        // Retry classifiers are operation-specific because they need to downcast operation-specific error types.
        let retry_classifiers = ::aws_smithy_runtime_api::client::retries::RetryClassifiers::new()
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::SmithyErrorClassifier::<
                crate::operation::invoke::InvokeError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AmzRetryAfterHeaderClassifier)
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::ModeledAsRetryableClassifier::<
                crate::operation::invoke::InvokeError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AwsErrorCodeClassifier::<
                crate::operation::invoke::InvokeError,
            >::new())
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::HttpStatusCodeClassifier::default());

        ::std::borrow::Cow::Owned(
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("Invoke")
                .with_retry_classifiers(::std::option::Option::Some(retry_classifiers))
                .with_auth_scheme_option_resolver(::std::option::Option::Some(
                    ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(
                        ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver::new(vec![
                            ::aws_runtime::auth::sigv4::SCHEME_ID,
                        ]),
                    ),
                ))
                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(InvokeEndpointParamsInterceptor) as _),
        )
    }
}

#[derive(Debug)]
struct InvokeStreamedResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::ResponseDeserializer for InvokeStreamedResponseDeserializer {
    fn deserialize_streaming(
        &self,
        response: &mut ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::std::option::Option<::aws_smithy_runtime_api::client::interceptors::context::OutputOrError> {
        // If this is an error, defer to the non-streaming parser
        if !response.status().is_success() && response.status().as_u16() != 200 {
            return ::std::option::Option::None;
        }
        ::std::option::Option::Some(crate::protocol_serde::type_erase_result(
            crate::protocol_serde::shape_invoke::de_invoke_http_response_streamed(response),
        ))
    }

    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 {
            crate::protocol_serde::shape_invoke::de_invoke_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_invoke::de_invoke_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct InvokeStreamedRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::RequestSerializer for InvokeStreamedRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input.downcast::<crate::operation::invoke_streamed::InvokeStreamedInput>().expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::invoke_streamed::InvokeStreamedInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                use ::std::fmt::Write as _;
                let input_1 = &_input.function_name;
                let input_1 = input_1
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_http::operation::error::BuildError::missing_field("function_name", "cannot be empty or unset"))?;
                let function_name = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
                if function_name.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_http::operation::error::BuildError::missing_field(
                        "function_name",
                        "cannot be empty or unset",
                    ));
                }
                // #### Key difference from Invoke -- hitting the streaming API
                ::std::write!(output, "/2021-11-15/functions/{FunctionName}/response-streaming-invocations", FunctionName = function_name)
                    .expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            fn uri_query(
                _input: &crate::operation::invoke::InvokeInput,
                mut output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                let mut query = ::aws_smithy_http::query::Writer::new(output);
                if let ::std::option::Option::Some(inner_2) = &_input.qualifier {
                    {
                        query.push_kv("Qualifier", &::aws_smithy_http::query::fmt_string(&inner_2));
                    }
                }
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::invoke::InvokeInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder = crate::protocol_serde::shape_invoke::ser_invoke_headers(input, builder)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/octet-stream");
            builder
        };
        let body = ::aws_smithy_http::body::SdkBody::from(crate::protocol_serde::shape_invoke_input::ser_payload_http_payload(input.payload)?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request"))
    }
}
#[derive(Debug)]
struct InvokeEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Interceptor for InvokeEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "InvokeEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context.input().downcast_ref::<InvokeInput>().ok_or("failed to downcast to InvokeInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

use crate::operation::invoke::InvokeError;

pub use crate::operation::invoke_streamed::_invoke_streamed_output::InvokeStreamedOutput;

pub use crate::operation::invoke_streamed::_invoke_streamed_input::InvokeStreamedInput;

mod _invoke_streamed_input;

mod _invoke_streamed_output;

/// Builders
pub mod builders;
