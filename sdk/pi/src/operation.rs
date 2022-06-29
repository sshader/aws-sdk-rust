// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DescribeDimensionKeys`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_dimension_keys`](crate::client::Client::describe_dimension_keys).
///
/// See [`crate::client::fluent_builders::DescribeDimensionKeys`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDimensionKeys {
    _private: (),
}
impl DescribeDimensionKeys {
    /// Creates a new builder-style object to manufacture [`DescribeDimensionKeysInput`](crate::input::DescribeDimensionKeysInput).
    pub fn builder() -> crate::input::describe_dimension_keys_input::Builder {
        crate::input::describe_dimension_keys_input::Builder::default()
    }
    /// Creates a new `DescribeDimensionKeys` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDimensionKeys {
    type Output = std::result::Result<
        crate::output::DescribeDimensionKeysOutput,
        crate::error::DescribeDimensionKeysError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_dimension_keys_error(response)
        } else {
            crate::operation_deser::parse_describe_dimension_keys_response(response)
        }
    }
}

/// Operation shape for `GetDimensionKeyDetails`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_dimension_key_details`](crate::client::Client::get_dimension_key_details).
///
/// See [`crate::client::fluent_builders::GetDimensionKeyDetails`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDimensionKeyDetails {
    _private: (),
}
impl GetDimensionKeyDetails {
    /// Creates a new builder-style object to manufacture [`GetDimensionKeyDetailsInput`](crate::input::GetDimensionKeyDetailsInput).
    pub fn builder() -> crate::input::get_dimension_key_details_input::Builder {
        crate::input::get_dimension_key_details_input::Builder::default()
    }
    /// Creates a new `GetDimensionKeyDetails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDimensionKeyDetails {
    type Output = std::result::Result<
        crate::output::GetDimensionKeyDetailsOutput,
        crate::error::GetDimensionKeyDetailsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_dimension_key_details_error(response)
        } else {
            crate::operation_deser::parse_get_dimension_key_details_response(response)
        }
    }
}

/// Operation shape for `GetResourceMetadata`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_metadata`](crate::client::Client::get_resource_metadata).
///
/// See [`crate::client::fluent_builders::GetResourceMetadata`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceMetadata {
    _private: (),
}
impl GetResourceMetadata {
    /// Creates a new builder-style object to manufacture [`GetResourceMetadataInput`](crate::input::GetResourceMetadataInput).
    pub fn builder() -> crate::input::get_resource_metadata_input::Builder {
        crate::input::get_resource_metadata_input::Builder::default()
    }
    /// Creates a new `GetResourceMetadata` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourceMetadata {
    type Output = std::result::Result<
        crate::output::GetResourceMetadataOutput,
        crate::error::GetResourceMetadataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_metadata_error(response)
        } else {
            crate::operation_deser::parse_get_resource_metadata_response(response)
        }
    }
}

/// Operation shape for `GetResourceMetrics`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_metrics`](crate::client::Client::get_resource_metrics).
///
/// See [`crate::client::fluent_builders::GetResourceMetrics`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceMetrics {
    _private: (),
}
impl GetResourceMetrics {
    /// Creates a new builder-style object to manufacture [`GetResourceMetricsInput`](crate::input::GetResourceMetricsInput).
    pub fn builder() -> crate::input::get_resource_metrics_input::Builder {
        crate::input::get_resource_metrics_input::Builder::default()
    }
    /// Creates a new `GetResourceMetrics` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourceMetrics {
    type Output = std::result::Result<
        crate::output::GetResourceMetricsOutput,
        crate::error::GetResourceMetricsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_metrics_error(response)
        } else {
            crate::operation_deser::parse_get_resource_metrics_response(response)
        }
    }
}

/// Operation shape for `ListAvailableResourceDimensions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_available_resource_dimensions`](crate::client::Client::list_available_resource_dimensions).
///
/// See [`crate::client::fluent_builders::ListAvailableResourceDimensions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAvailableResourceDimensions {
    _private: (),
}
impl ListAvailableResourceDimensions {
    /// Creates a new builder-style object to manufacture [`ListAvailableResourceDimensionsInput`](crate::input::ListAvailableResourceDimensionsInput).
    pub fn builder() -> crate::input::list_available_resource_dimensions_input::Builder {
        crate::input::list_available_resource_dimensions_input::Builder::default()
    }
    /// Creates a new `ListAvailableResourceDimensions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAvailableResourceDimensions {
    type Output = std::result::Result<
        crate::output::ListAvailableResourceDimensionsOutput,
        crate::error::ListAvailableResourceDimensionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_available_resource_dimensions_error(response)
        } else {
            crate::operation_deser::parse_list_available_resource_dimensions_response(response)
        }
    }
}

/// Operation shape for `ListAvailableResourceMetrics`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_available_resource_metrics`](crate::client::Client::list_available_resource_metrics).
///
/// See [`crate::client::fluent_builders::ListAvailableResourceMetrics`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAvailableResourceMetrics {
    _private: (),
}
impl ListAvailableResourceMetrics {
    /// Creates a new builder-style object to manufacture [`ListAvailableResourceMetricsInput`](crate::input::ListAvailableResourceMetricsInput).
    pub fn builder() -> crate::input::list_available_resource_metrics_input::Builder {
        crate::input::list_available_resource_metrics_input::Builder::default()
    }
    /// Creates a new `ListAvailableResourceMetrics` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAvailableResourceMetrics {
    type Output = std::result::Result<
        crate::output::ListAvailableResourceMetricsOutput,
        crate::error::ListAvailableResourceMetricsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_available_resource_metrics_error(response)
        } else {
            crate::operation_deser::parse_list_available_resource_metrics_response(response)
        }
    }
}
