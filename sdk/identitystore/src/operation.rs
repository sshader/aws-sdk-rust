// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DescribeGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_group`](crate::client::Client::describe_group).
///
/// See [`crate::client::fluent_builders::DescribeGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeGroup {
    _private: (),
}
impl DescribeGroup {
    /// Creates a new builder-style object to manufacture [`DescribeGroupInput`](crate::input::DescribeGroupInput).
    pub fn builder() -> crate::input::describe_group_input::Builder {
        crate::input::describe_group_input::Builder::default()
    }
    /// Creates a new `DescribeGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeGroup {
    type Output =
        std::result::Result<crate::output::DescribeGroupOutput, crate::error::DescribeGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_group_error(response)
        } else {
            crate::operation_deser::parse_describe_group_response(response)
        }
    }
}

/// Operation shape for `DescribeUser`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_user`](crate::client::Client::describe_user).
///
/// See [`crate::client::fluent_builders::DescribeUser`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeUser {
    _private: (),
}
impl DescribeUser {
    /// Creates a new builder-style object to manufacture [`DescribeUserInput`](crate::input::DescribeUserInput).
    pub fn builder() -> crate::input::describe_user_input::Builder {
        crate::input::describe_user_input::Builder::default()
    }
    /// Creates a new `DescribeUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeUser {
    type Output =
        std::result::Result<crate::output::DescribeUserOutput, crate::error::DescribeUserError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_user_error(response)
        } else {
            crate::operation_deser::parse_describe_user_response(response)
        }
    }
}

/// Operation shape for `ListGroups`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_groups`](crate::client::Client::list_groups).
///
/// See [`crate::client::fluent_builders::ListGroups`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListGroups {
    _private: (),
}
impl ListGroups {
    /// Creates a new builder-style object to manufacture [`ListGroupsInput`](crate::input::ListGroupsInput).
    pub fn builder() -> crate::input::list_groups_input::Builder {
        crate::input::list_groups_input::Builder::default()
    }
    /// Creates a new `ListGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListGroups {
    type Output =
        std::result::Result<crate::output::ListGroupsOutput, crate::error::ListGroupsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_groups_error(response)
        } else {
            crate::operation_deser::parse_list_groups_response(response)
        }
    }
}

/// Operation shape for `ListUsers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_users`](crate::client::Client::list_users).
///
/// See [`crate::client::fluent_builders::ListUsers`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListUsers {
    _private: (),
}
impl ListUsers {
    /// Creates a new builder-style object to manufacture [`ListUsersInput`](crate::input::ListUsersInput).
    pub fn builder() -> crate::input::list_users_input::Builder {
        crate::input::list_users_input::Builder::default()
    }
    /// Creates a new `ListUsers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListUsers {
    type Output = std::result::Result<crate::output::ListUsersOutput, crate::error::ListUsersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_users_error(response)
        } else {
            crate::operation_deser::parse_list_users_response(response)
        }
    }
}
