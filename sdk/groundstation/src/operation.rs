// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_contact`](crate::client::Client::cancel_contact).
///
/// See [`crate::client::fluent_builders::CancelContact`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelContact {
    _private: (),
}
impl CancelContact {
    /// Creates a new builder-style object to manufacture [`CancelContactInput`](crate::input::CancelContactInput).
    pub fn builder() -> crate::input::cancel_contact_input::Builder {
        crate::input::cancel_contact_input::Builder::default()
    }
    /// Creates a new `CancelContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelContact {
    type Output =
        std::result::Result<crate::output::CancelContactOutput, crate::error::CancelContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_contact_error(response)
        } else {
            crate::operation_deser::parse_cancel_contact_response(response)
        }
    }
}

/// Operation shape for `CreateConfig`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_config`](crate::client::Client::create_config).
///
/// See [`crate::client::fluent_builders::CreateConfig`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateConfig {
    _private: (),
}
impl CreateConfig {
    /// Creates a new builder-style object to manufacture [`CreateConfigInput`](crate::input::CreateConfigInput).
    pub fn builder() -> crate::input::create_config_input::Builder {
        crate::input::create_config_input::Builder::default()
    }
    /// Creates a new `CreateConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateConfig {
    type Output =
        std::result::Result<crate::output::CreateConfigOutput, crate::error::CreateConfigError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_config_error(response)
        } else {
            crate::operation_deser::parse_create_config_response(response)
        }
    }
}

/// Operation shape for `CreateDataflowEndpointGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_dataflow_endpoint_group`](crate::client::Client::create_dataflow_endpoint_group).
///
/// See [`crate::client::fluent_builders::CreateDataflowEndpointGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDataflowEndpointGroup {
    _private: (),
}
impl CreateDataflowEndpointGroup {
    /// Creates a new builder-style object to manufacture [`CreateDataflowEndpointGroupInput`](crate::input::CreateDataflowEndpointGroupInput).
    pub fn builder() -> crate::input::create_dataflow_endpoint_group_input::Builder {
        crate::input::create_dataflow_endpoint_group_input::Builder::default()
    }
    /// Creates a new `CreateDataflowEndpointGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDataflowEndpointGroup {
    type Output = std::result::Result<
        crate::output::CreateDataflowEndpointGroupOutput,
        crate::error::CreateDataflowEndpointGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_dataflow_endpoint_group_error(response)
        } else {
            crate::operation_deser::parse_create_dataflow_endpoint_group_response(response)
        }
    }
}

/// Operation shape for `CreateMissionProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_mission_profile`](crate::client::Client::create_mission_profile).
///
/// See [`crate::client::fluent_builders::CreateMissionProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateMissionProfile {
    _private: (),
}
impl CreateMissionProfile {
    /// Creates a new builder-style object to manufacture [`CreateMissionProfileInput`](crate::input::CreateMissionProfileInput).
    pub fn builder() -> crate::input::create_mission_profile_input::Builder {
        crate::input::create_mission_profile_input::Builder::default()
    }
    /// Creates a new `CreateMissionProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateMissionProfile {
    type Output = std::result::Result<
        crate::output::CreateMissionProfileOutput,
        crate::error::CreateMissionProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_mission_profile_error(response)
        } else {
            crate::operation_deser::parse_create_mission_profile_response(response)
        }
    }
}

/// Operation shape for `DeleteConfig`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_config`](crate::client::Client::delete_config).
///
/// See [`crate::client::fluent_builders::DeleteConfig`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteConfig {
    _private: (),
}
impl DeleteConfig {
    /// Creates a new builder-style object to manufacture [`DeleteConfigInput`](crate::input::DeleteConfigInput).
    pub fn builder() -> crate::input::delete_config_input::Builder {
        crate::input::delete_config_input::Builder::default()
    }
    /// Creates a new `DeleteConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteConfig {
    type Output =
        std::result::Result<crate::output::DeleteConfigOutput, crate::error::DeleteConfigError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_config_error(response)
        } else {
            crate::operation_deser::parse_delete_config_response(response)
        }
    }
}

/// Operation shape for `DeleteDataflowEndpointGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_dataflow_endpoint_group`](crate::client::Client::delete_dataflow_endpoint_group).
///
/// See [`crate::client::fluent_builders::DeleteDataflowEndpointGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDataflowEndpointGroup {
    _private: (),
}
impl DeleteDataflowEndpointGroup {
    /// Creates a new builder-style object to manufacture [`DeleteDataflowEndpointGroupInput`](crate::input::DeleteDataflowEndpointGroupInput).
    pub fn builder() -> crate::input::delete_dataflow_endpoint_group_input::Builder {
        crate::input::delete_dataflow_endpoint_group_input::Builder::default()
    }
    /// Creates a new `DeleteDataflowEndpointGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDataflowEndpointGroup {
    type Output = std::result::Result<
        crate::output::DeleteDataflowEndpointGroupOutput,
        crate::error::DeleteDataflowEndpointGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_dataflow_endpoint_group_error(response)
        } else {
            crate::operation_deser::parse_delete_dataflow_endpoint_group_response(response)
        }
    }
}

/// Operation shape for `DeleteMissionProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_mission_profile`](crate::client::Client::delete_mission_profile).
///
/// See [`crate::client::fluent_builders::DeleteMissionProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteMissionProfile {
    _private: (),
}
impl DeleteMissionProfile {
    /// Creates a new builder-style object to manufacture [`DeleteMissionProfileInput`](crate::input::DeleteMissionProfileInput).
    pub fn builder() -> crate::input::delete_mission_profile_input::Builder {
        crate::input::delete_mission_profile_input::Builder::default()
    }
    /// Creates a new `DeleteMissionProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteMissionProfile {
    type Output = std::result::Result<
        crate::output::DeleteMissionProfileOutput,
        crate::error::DeleteMissionProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_mission_profile_error(response)
        } else {
            crate::operation_deser::parse_delete_mission_profile_response(response)
        }
    }
}

/// Operation shape for `DescribeContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_contact`](crate::client::Client::describe_contact).
///
/// See [`crate::client::fluent_builders::DescribeContact`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeContact {
    _private: (),
}
impl DescribeContact {
    /// Creates a new builder-style object to manufacture [`DescribeContactInput`](crate::input::DescribeContactInput).
    pub fn builder() -> crate::input::describe_contact_input::Builder {
        crate::input::describe_contact_input::Builder::default()
    }
    /// Creates a new `DescribeContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeContact {
    type Output = std::result::Result<
        crate::output::DescribeContactOutput,
        crate::error::DescribeContactError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_contact_error(response)
        } else {
            crate::operation_deser::parse_describe_contact_response(response)
        }
    }
}

/// Operation shape for `GetConfig`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_config`](crate::client::Client::get_config).
///
/// See [`crate::client::fluent_builders::GetConfig`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetConfig {
    _private: (),
}
impl GetConfig {
    /// Creates a new builder-style object to manufacture [`GetConfigInput`](crate::input::GetConfigInput).
    pub fn builder() -> crate::input::get_config_input::Builder {
        crate::input::get_config_input::Builder::default()
    }
    /// Creates a new `GetConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetConfig {
    type Output = std::result::Result<crate::output::GetConfigOutput, crate::error::GetConfigError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_config_error(response)
        } else {
            crate::operation_deser::parse_get_config_response(response)
        }
    }
}

/// Operation shape for `GetDataflowEndpointGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_dataflow_endpoint_group`](crate::client::Client::get_dataflow_endpoint_group).
///
/// See [`crate::client::fluent_builders::GetDataflowEndpointGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDataflowEndpointGroup {
    _private: (),
}
impl GetDataflowEndpointGroup {
    /// Creates a new builder-style object to manufacture [`GetDataflowEndpointGroupInput`](crate::input::GetDataflowEndpointGroupInput).
    pub fn builder() -> crate::input::get_dataflow_endpoint_group_input::Builder {
        crate::input::get_dataflow_endpoint_group_input::Builder::default()
    }
    /// Creates a new `GetDataflowEndpointGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDataflowEndpointGroup {
    type Output = std::result::Result<
        crate::output::GetDataflowEndpointGroupOutput,
        crate::error::GetDataflowEndpointGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_dataflow_endpoint_group_error(response)
        } else {
            crate::operation_deser::parse_get_dataflow_endpoint_group_response(response)
        }
    }
}

/// Operation shape for `GetMinuteUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_minute_usage`](crate::client::Client::get_minute_usage).
///
/// See [`crate::client::fluent_builders::GetMinuteUsage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMinuteUsage {
    _private: (),
}
impl GetMinuteUsage {
    /// Creates a new builder-style object to manufacture [`GetMinuteUsageInput`](crate::input::GetMinuteUsageInput).
    pub fn builder() -> crate::input::get_minute_usage_input::Builder {
        crate::input::get_minute_usage_input::Builder::default()
    }
    /// Creates a new `GetMinuteUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetMinuteUsage {
    type Output =
        std::result::Result<crate::output::GetMinuteUsageOutput, crate::error::GetMinuteUsageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_minute_usage_error(response)
        } else {
            crate::operation_deser::parse_get_minute_usage_response(response)
        }
    }
}

/// Operation shape for `GetMissionProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_mission_profile`](crate::client::Client::get_mission_profile).
///
/// See [`crate::client::fluent_builders::GetMissionProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMissionProfile {
    _private: (),
}
impl GetMissionProfile {
    /// Creates a new builder-style object to manufacture [`GetMissionProfileInput`](crate::input::GetMissionProfileInput).
    pub fn builder() -> crate::input::get_mission_profile_input::Builder {
        crate::input::get_mission_profile_input::Builder::default()
    }
    /// Creates a new `GetMissionProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetMissionProfile {
    type Output = std::result::Result<
        crate::output::GetMissionProfileOutput,
        crate::error::GetMissionProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_mission_profile_error(response)
        } else {
            crate::operation_deser::parse_get_mission_profile_response(response)
        }
    }
}

/// Operation shape for `GetSatellite`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_satellite`](crate::client::Client::get_satellite).
///
/// See [`crate::client::fluent_builders::GetSatellite`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetSatellite {
    _private: (),
}
impl GetSatellite {
    /// Creates a new builder-style object to manufacture [`GetSatelliteInput`](crate::input::GetSatelliteInput).
    pub fn builder() -> crate::input::get_satellite_input::Builder {
        crate::input::get_satellite_input::Builder::default()
    }
    /// Creates a new `GetSatellite` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSatellite {
    type Output =
        std::result::Result<crate::output::GetSatelliteOutput, crate::error::GetSatelliteError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_satellite_error(response)
        } else {
            crate::operation_deser::parse_get_satellite_response(response)
        }
    }
}

/// Operation shape for `ListConfigs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_configs`](crate::client::Client::list_configs).
///
/// See [`crate::client::fluent_builders::ListConfigs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListConfigs {
    _private: (),
}
impl ListConfigs {
    /// Creates a new builder-style object to manufacture [`ListConfigsInput`](crate::input::ListConfigsInput).
    pub fn builder() -> crate::input::list_configs_input::Builder {
        crate::input::list_configs_input::Builder::default()
    }
    /// Creates a new `ListConfigs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConfigs {
    type Output =
        std::result::Result<crate::output::ListConfigsOutput, crate::error::ListConfigsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_configs_error(response)
        } else {
            crate::operation_deser::parse_list_configs_response(response)
        }
    }
}

/// Operation shape for `ListContacts`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_contacts`](crate::client::Client::list_contacts).
///
/// See [`crate::client::fluent_builders::ListContacts`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListContacts {
    _private: (),
}
impl ListContacts {
    /// Creates a new builder-style object to manufacture [`ListContactsInput`](crate::input::ListContactsInput).
    pub fn builder() -> crate::input::list_contacts_input::Builder {
        crate::input::list_contacts_input::Builder::default()
    }
    /// Creates a new `ListContacts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListContacts {
    type Output =
        std::result::Result<crate::output::ListContactsOutput, crate::error::ListContactsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_contacts_error(response)
        } else {
            crate::operation_deser::parse_list_contacts_response(response)
        }
    }
}

/// Operation shape for `ListDataflowEndpointGroups`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_dataflow_endpoint_groups`](crate::client::Client::list_dataflow_endpoint_groups).
///
/// See [`crate::client::fluent_builders::ListDataflowEndpointGroups`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDataflowEndpointGroups {
    _private: (),
}
impl ListDataflowEndpointGroups {
    /// Creates a new builder-style object to manufacture [`ListDataflowEndpointGroupsInput`](crate::input::ListDataflowEndpointGroupsInput).
    pub fn builder() -> crate::input::list_dataflow_endpoint_groups_input::Builder {
        crate::input::list_dataflow_endpoint_groups_input::Builder::default()
    }
    /// Creates a new `ListDataflowEndpointGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDataflowEndpointGroups {
    type Output = std::result::Result<
        crate::output::ListDataflowEndpointGroupsOutput,
        crate::error::ListDataflowEndpointGroupsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_dataflow_endpoint_groups_error(response)
        } else {
            crate::operation_deser::parse_list_dataflow_endpoint_groups_response(response)
        }
    }
}

/// Operation shape for `ListGroundStations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_ground_stations`](crate::client::Client::list_ground_stations).
///
/// See [`crate::client::fluent_builders::ListGroundStations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListGroundStations {
    _private: (),
}
impl ListGroundStations {
    /// Creates a new builder-style object to manufacture [`ListGroundStationsInput`](crate::input::ListGroundStationsInput).
    pub fn builder() -> crate::input::list_ground_stations_input::Builder {
        crate::input::list_ground_stations_input::Builder::default()
    }
    /// Creates a new `ListGroundStations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListGroundStations {
    type Output = std::result::Result<
        crate::output::ListGroundStationsOutput,
        crate::error::ListGroundStationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_ground_stations_error(response)
        } else {
            crate::operation_deser::parse_list_ground_stations_response(response)
        }
    }
}

/// Operation shape for `ListMissionProfiles`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_mission_profiles`](crate::client::Client::list_mission_profiles).
///
/// See [`crate::client::fluent_builders::ListMissionProfiles`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListMissionProfiles {
    _private: (),
}
impl ListMissionProfiles {
    /// Creates a new builder-style object to manufacture [`ListMissionProfilesInput`](crate::input::ListMissionProfilesInput).
    pub fn builder() -> crate::input::list_mission_profiles_input::Builder {
        crate::input::list_mission_profiles_input::Builder::default()
    }
    /// Creates a new `ListMissionProfiles` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListMissionProfiles {
    type Output = std::result::Result<
        crate::output::ListMissionProfilesOutput,
        crate::error::ListMissionProfilesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_mission_profiles_error(response)
        } else {
            crate::operation_deser::parse_list_mission_profiles_response(response)
        }
    }
}

/// Operation shape for `ListSatellites`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_satellites`](crate::client::Client::list_satellites).
///
/// See [`crate::client::fluent_builders::ListSatellites`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSatellites {
    _private: (),
}
impl ListSatellites {
    /// Creates a new builder-style object to manufacture [`ListSatellitesInput`](crate::input::ListSatellitesInput).
    pub fn builder() -> crate::input::list_satellites_input::Builder {
        crate::input::list_satellites_input::Builder::default()
    }
    /// Creates a new `ListSatellites` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSatellites {
    type Output =
        std::result::Result<crate::output::ListSatellitesOutput, crate::error::ListSatellitesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_satellites_error(response)
        } else {
            crate::operation_deser::parse_list_satellites_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `ReserveContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`reserve_contact`](crate::client::Client::reserve_contact).
///
/// See [`crate::client::fluent_builders::ReserveContact`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ReserveContact {
    _private: (),
}
impl ReserveContact {
    /// Creates a new builder-style object to manufacture [`ReserveContactInput`](crate::input::ReserveContactInput).
    pub fn builder() -> crate::input::reserve_contact_input::Builder {
        crate::input::reserve_contact_input::Builder::default()
    }
    /// Creates a new `ReserveContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ReserveContact {
    type Output =
        std::result::Result<crate::output::ReserveContactOutput, crate::error::ReserveContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_reserve_contact_error(response)
        } else {
            crate::operation_deser::parse_reserve_contact_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateConfig`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_config`](crate::client::Client::update_config).
///
/// See [`crate::client::fluent_builders::UpdateConfig`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateConfig {
    _private: (),
}
impl UpdateConfig {
    /// Creates a new builder-style object to manufacture [`UpdateConfigInput`](crate::input::UpdateConfigInput).
    pub fn builder() -> crate::input::update_config_input::Builder {
        crate::input::update_config_input::Builder::default()
    }
    /// Creates a new `UpdateConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateConfig {
    type Output =
        std::result::Result<crate::output::UpdateConfigOutput, crate::error::UpdateConfigError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_config_error(response)
        } else {
            crate::operation_deser::parse_update_config_response(response)
        }
    }
}

/// Operation shape for `UpdateMissionProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_mission_profile`](crate::client::Client::update_mission_profile).
///
/// See [`crate::client::fluent_builders::UpdateMissionProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateMissionProfile {
    _private: (),
}
impl UpdateMissionProfile {
    /// Creates a new builder-style object to manufacture [`UpdateMissionProfileInput`](crate::input::UpdateMissionProfileInput).
    pub fn builder() -> crate::input::update_mission_profile_input::Builder {
        crate::input::update_mission_profile_input::Builder::default()
    }
    /// Creates a new `UpdateMissionProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateMissionProfile {
    type Output = std::result::Result<
        crate::output::UpdateMissionProfileOutput,
        crate::error::UpdateMissionProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_mission_profile_error(response)
        } else {
            crate::operation_deser::parse_update_mission_profile_response(response)
        }
    }
}
