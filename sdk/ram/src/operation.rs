// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AcceptResourceShareInvitation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`accept_resource_share_invitation`](crate::client::Client::accept_resource_share_invitation).
///
/// See [`crate::client::fluent_builders::AcceptResourceShareInvitation`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AcceptResourceShareInvitation {
    _private: (),
}
impl AcceptResourceShareInvitation {
    /// Creates a new builder-style object to manufacture [`AcceptResourceShareInvitationInput`](crate::input::AcceptResourceShareInvitationInput)
    pub fn builder() -> crate::input::accept_resource_share_invitation_input::Builder {
        crate::input::accept_resource_share_invitation_input::Builder::default()
    }
    /// Creates a new `AcceptResourceShareInvitation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AcceptResourceShareInvitation {
    type Output = std::result::Result<
        crate::output::AcceptResourceShareInvitationOutput,
        crate::error::AcceptResourceShareInvitationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_accept_resource_share_invitation_error(response)
        } else {
            crate::operation_deser::parse_accept_resource_share_invitation_response(response)
        }
    }
}

/// Operation shape for `AssociateResourceShare`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`associate_resource_share`](crate::client::Client::associate_resource_share).
///
/// See [`crate::client::fluent_builders::AssociateResourceShare`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateResourceShare {
    _private: (),
}
impl AssociateResourceShare {
    /// Creates a new builder-style object to manufacture [`AssociateResourceShareInput`](crate::input::AssociateResourceShareInput)
    pub fn builder() -> crate::input::associate_resource_share_input::Builder {
        crate::input::associate_resource_share_input::Builder::default()
    }
    /// Creates a new `AssociateResourceShare` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateResourceShare {
    type Output = std::result::Result<
        crate::output::AssociateResourceShareOutput,
        crate::error::AssociateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_resource_share_error(response)
        } else {
            crate::operation_deser::parse_associate_resource_share_response(response)
        }
    }
}

/// Operation shape for `AssociateResourceSharePermission`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`associate_resource_share_permission`](crate::client::Client::associate_resource_share_permission).
///
/// See [`crate::client::fluent_builders::AssociateResourceSharePermission`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateResourceSharePermission {
    _private: (),
}
impl AssociateResourceSharePermission {
    /// Creates a new builder-style object to manufacture [`AssociateResourceSharePermissionInput`](crate::input::AssociateResourceSharePermissionInput)
    pub fn builder() -> crate::input::associate_resource_share_permission_input::Builder {
        crate::input::associate_resource_share_permission_input::Builder::default()
    }
    /// Creates a new `AssociateResourceSharePermission` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateResourceSharePermission {
    type Output = std::result::Result<
        crate::output::AssociateResourceSharePermissionOutput,
        crate::error::AssociateResourceSharePermissionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_resource_share_permission_error(response)
        } else {
            crate::operation_deser::parse_associate_resource_share_permission_response(response)
        }
    }
}

/// Operation shape for `CreateResourceShare`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_resource_share`](crate::client::Client::create_resource_share).
///
/// See [`crate::client::fluent_builders::CreateResourceShare`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateResourceShare {
    _private: (),
}
impl CreateResourceShare {
    /// Creates a new builder-style object to manufacture [`CreateResourceShareInput`](crate::input::CreateResourceShareInput)
    pub fn builder() -> crate::input::create_resource_share_input::Builder {
        crate::input::create_resource_share_input::Builder::default()
    }
    /// Creates a new `CreateResourceShare` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateResourceShare {
    type Output = std::result::Result<
        crate::output::CreateResourceShareOutput,
        crate::error::CreateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_resource_share_error(response)
        } else {
            crate::operation_deser::parse_create_resource_share_response(response)
        }
    }
}

/// Operation shape for `DeleteResourceShare`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_resource_share`](crate::client::Client::delete_resource_share).
///
/// See [`crate::client::fluent_builders::DeleteResourceShare`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResourceShare {
    _private: (),
}
impl DeleteResourceShare {
    /// Creates a new builder-style object to manufacture [`DeleteResourceShareInput`](crate::input::DeleteResourceShareInput)
    pub fn builder() -> crate::input::delete_resource_share_input::Builder {
        crate::input::delete_resource_share_input::Builder::default()
    }
    /// Creates a new `DeleteResourceShare` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteResourceShare {
    type Output = std::result::Result<
        crate::output::DeleteResourceShareOutput,
        crate::error::DeleteResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_resource_share_error(response)
        } else {
            crate::operation_deser::parse_delete_resource_share_response(response)
        }
    }
}

/// Operation shape for `DisassociateResourceShare`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disassociate_resource_share`](crate::client::Client::disassociate_resource_share).
///
/// See [`crate::client::fluent_builders::DisassociateResourceShare`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateResourceShare {
    _private: (),
}
impl DisassociateResourceShare {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceShareInput`](crate::input::DisassociateResourceShareInput)
    pub fn builder() -> crate::input::disassociate_resource_share_input::Builder {
        crate::input::disassociate_resource_share_input::Builder::default()
    }
    /// Creates a new `DisassociateResourceShare` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateResourceShare {
    type Output = std::result::Result<
        crate::output::DisassociateResourceShareOutput,
        crate::error::DisassociateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_resource_share_error(response)
        } else {
            crate::operation_deser::parse_disassociate_resource_share_response(response)
        }
    }
}

/// Operation shape for `DisassociateResourceSharePermission`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disassociate_resource_share_permission`](crate::client::Client::disassociate_resource_share_permission).
///
/// See [`crate::client::fluent_builders::DisassociateResourceSharePermission`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateResourceSharePermission {
    _private: (),
}
impl DisassociateResourceSharePermission {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceSharePermissionInput`](crate::input::DisassociateResourceSharePermissionInput)
    pub fn builder() -> crate::input::disassociate_resource_share_permission_input::Builder {
        crate::input::disassociate_resource_share_permission_input::Builder::default()
    }
    /// Creates a new `DisassociateResourceSharePermission` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateResourceSharePermission {
    type Output = std::result::Result<
        crate::output::DisassociateResourceSharePermissionOutput,
        crate::error::DisassociateResourceSharePermissionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_resource_share_permission_error(response)
        } else {
            crate::operation_deser::parse_disassociate_resource_share_permission_response(response)
        }
    }
}

/// Operation shape for `EnableSharingWithAwsOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`enable_sharing_with_aws_organization`](crate::client::Client::enable_sharing_with_aws_organization).
///
/// See [`crate::client::fluent_builders::EnableSharingWithAwsOrganization`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct EnableSharingWithAwsOrganization {
    _private: (),
}
impl EnableSharingWithAwsOrganization {
    /// Creates a new builder-style object to manufacture [`EnableSharingWithAwsOrganizationInput`](crate::input::EnableSharingWithAwsOrganizationInput)
    pub fn builder() -> crate::input::enable_sharing_with_aws_organization_input::Builder {
        crate::input::enable_sharing_with_aws_organization_input::Builder::default()
    }
    /// Creates a new `EnableSharingWithAwsOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for EnableSharingWithAwsOrganization {
    type Output = std::result::Result<
        crate::output::EnableSharingWithAwsOrganizationOutput,
        crate::error::EnableSharingWithAwsOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_enable_sharing_with_aws_organization_error(response)
        } else {
            crate::operation_deser::parse_enable_sharing_with_aws_organization_response(response)
        }
    }
}

/// Operation shape for `GetPermission`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_permission`](crate::client::Client::get_permission).
///
/// See [`crate::client::fluent_builders::GetPermission`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetPermission {
    _private: (),
}
impl GetPermission {
    /// Creates a new builder-style object to manufacture [`GetPermissionInput`](crate::input::GetPermissionInput)
    pub fn builder() -> crate::input::get_permission_input::Builder {
        crate::input::get_permission_input::Builder::default()
    }
    /// Creates a new `GetPermission` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPermission {
    type Output =
        std::result::Result<crate::output::GetPermissionOutput, crate::error::GetPermissionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_permission_error(response)
        } else {
            crate::operation_deser::parse_get_permission_response(response)
        }
    }
}

/// Operation shape for `GetResourcePolicies`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_policies`](crate::client::Client::get_resource_policies).
///
/// See [`crate::client::fluent_builders::GetResourcePolicies`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourcePolicies {
    _private: (),
}
impl GetResourcePolicies {
    /// Creates a new builder-style object to manufacture [`GetResourcePoliciesInput`](crate::input::GetResourcePoliciesInput)
    pub fn builder() -> crate::input::get_resource_policies_input::Builder {
        crate::input::get_resource_policies_input::Builder::default()
    }
    /// Creates a new `GetResourcePolicies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourcePolicies {
    type Output = std::result::Result<
        crate::output::GetResourcePoliciesOutput,
        crate::error::GetResourcePoliciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_policies_error(response)
        } else {
            crate::operation_deser::parse_get_resource_policies_response(response)
        }
    }
}

/// Operation shape for `GetResourceShareAssociations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_share_associations`](crate::client::Client::get_resource_share_associations).
///
/// See [`crate::client::fluent_builders::GetResourceShareAssociations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceShareAssociations {
    _private: (),
}
impl GetResourceShareAssociations {
    /// Creates a new builder-style object to manufacture [`GetResourceShareAssociationsInput`](crate::input::GetResourceShareAssociationsInput)
    pub fn builder() -> crate::input::get_resource_share_associations_input::Builder {
        crate::input::get_resource_share_associations_input::Builder::default()
    }
    /// Creates a new `GetResourceShareAssociations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourceShareAssociations {
    type Output = std::result::Result<
        crate::output::GetResourceShareAssociationsOutput,
        crate::error::GetResourceShareAssociationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_share_associations_error(response)
        } else {
            crate::operation_deser::parse_get_resource_share_associations_response(response)
        }
    }
}

/// Operation shape for `GetResourceShareInvitations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_share_invitations`](crate::client::Client::get_resource_share_invitations).
///
/// See [`crate::client::fluent_builders::GetResourceShareInvitations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceShareInvitations {
    _private: (),
}
impl GetResourceShareInvitations {
    /// Creates a new builder-style object to manufacture [`GetResourceShareInvitationsInput`](crate::input::GetResourceShareInvitationsInput)
    pub fn builder() -> crate::input::get_resource_share_invitations_input::Builder {
        crate::input::get_resource_share_invitations_input::Builder::default()
    }
    /// Creates a new `GetResourceShareInvitations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourceShareInvitations {
    type Output = std::result::Result<
        crate::output::GetResourceShareInvitationsOutput,
        crate::error::GetResourceShareInvitationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_share_invitations_error(response)
        } else {
            crate::operation_deser::parse_get_resource_share_invitations_response(response)
        }
    }
}

/// Operation shape for `GetResourceShares`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_resource_shares`](crate::client::Client::get_resource_shares).
///
/// See [`crate::client::fluent_builders::GetResourceShares`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceShares {
    _private: (),
}
impl GetResourceShares {
    /// Creates a new builder-style object to manufacture [`GetResourceSharesInput`](crate::input::GetResourceSharesInput)
    pub fn builder() -> crate::input::get_resource_shares_input::Builder {
        crate::input::get_resource_shares_input::Builder::default()
    }
    /// Creates a new `GetResourceShares` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourceShares {
    type Output = std::result::Result<
        crate::output::GetResourceSharesOutput,
        crate::error::GetResourceSharesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_shares_error(response)
        } else {
            crate::operation_deser::parse_get_resource_shares_response(response)
        }
    }
}

/// Operation shape for `ListPendingInvitationResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_pending_invitation_resources`](crate::client::Client::list_pending_invitation_resources).
///
/// See [`crate::client::fluent_builders::ListPendingInvitationResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPendingInvitationResources {
    _private: (),
}
impl ListPendingInvitationResources {
    /// Creates a new builder-style object to manufacture [`ListPendingInvitationResourcesInput`](crate::input::ListPendingInvitationResourcesInput)
    pub fn builder() -> crate::input::list_pending_invitation_resources_input::Builder {
        crate::input::list_pending_invitation_resources_input::Builder::default()
    }
    /// Creates a new `ListPendingInvitationResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPendingInvitationResources {
    type Output = std::result::Result<
        crate::output::ListPendingInvitationResourcesOutput,
        crate::error::ListPendingInvitationResourcesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pending_invitation_resources_error(response)
        } else {
            crate::operation_deser::parse_list_pending_invitation_resources_response(response)
        }
    }
}

/// Operation shape for `ListPermissions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_permissions`](crate::client::Client::list_permissions).
///
/// See [`crate::client::fluent_builders::ListPermissions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPermissions {
    _private: (),
}
impl ListPermissions {
    /// Creates a new builder-style object to manufacture [`ListPermissionsInput`](crate::input::ListPermissionsInput)
    pub fn builder() -> crate::input::list_permissions_input::Builder {
        crate::input::list_permissions_input::Builder::default()
    }
    /// Creates a new `ListPermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPermissions {
    type Output = std::result::Result<
        crate::output::ListPermissionsOutput,
        crate::error::ListPermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_permissions_error(response)
        } else {
            crate::operation_deser::parse_list_permissions_response(response)
        }
    }
}

/// Operation shape for `ListPermissionVersions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_permission_versions`](crate::client::Client::list_permission_versions).
///
/// See [`crate::client::fluent_builders::ListPermissionVersions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPermissionVersions {
    _private: (),
}
impl ListPermissionVersions {
    /// Creates a new builder-style object to manufacture [`ListPermissionVersionsInput`](crate::input::ListPermissionVersionsInput)
    pub fn builder() -> crate::input::list_permission_versions_input::Builder {
        crate::input::list_permission_versions_input::Builder::default()
    }
    /// Creates a new `ListPermissionVersions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPermissionVersions {
    type Output = std::result::Result<
        crate::output::ListPermissionVersionsOutput,
        crate::error::ListPermissionVersionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_permission_versions_error(response)
        } else {
            crate::operation_deser::parse_list_permission_versions_response(response)
        }
    }
}

/// Operation shape for `ListPrincipals`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_principals`](crate::client::Client::list_principals).
///
/// See [`crate::client::fluent_builders::ListPrincipals`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPrincipals {
    _private: (),
}
impl ListPrincipals {
    /// Creates a new builder-style object to manufacture [`ListPrincipalsInput`](crate::input::ListPrincipalsInput)
    pub fn builder() -> crate::input::list_principals_input::Builder {
        crate::input::list_principals_input::Builder::default()
    }
    /// Creates a new `ListPrincipals` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPrincipals {
    type Output =
        std::result::Result<crate::output::ListPrincipalsOutput, crate::error::ListPrincipalsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_principals_error(response)
        } else {
            crate::operation_deser::parse_list_principals_response(response)
        }
    }
}

/// Operation shape for `ListResources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_resources`](crate::client::Client::list_resources).
///
/// See [`crate::client::fluent_builders::ListResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResources {
    _private: (),
}
impl ListResources {
    /// Creates a new builder-style object to manufacture [`ListResourcesInput`](crate::input::ListResourcesInput)
    pub fn builder() -> crate::input::list_resources_input::Builder {
        crate::input::list_resources_input::Builder::default()
    }
    /// Creates a new `ListResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListResources {
    type Output =
        std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resources_error(response)
        } else {
            crate::operation_deser::parse_list_resources_response(response)
        }
    }
}

/// Operation shape for `ListResourceSharePermissions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_resource_share_permissions`](crate::client::Client::list_resource_share_permissions).
///
/// See [`crate::client::fluent_builders::ListResourceSharePermissions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResourceSharePermissions {
    _private: (),
}
impl ListResourceSharePermissions {
    /// Creates a new builder-style object to manufacture [`ListResourceSharePermissionsInput`](crate::input::ListResourceSharePermissionsInput)
    pub fn builder() -> crate::input::list_resource_share_permissions_input::Builder {
        crate::input::list_resource_share_permissions_input::Builder::default()
    }
    /// Creates a new `ListResourceSharePermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListResourceSharePermissions {
    type Output = std::result::Result<
        crate::output::ListResourceSharePermissionsOutput,
        crate::error::ListResourceSharePermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resource_share_permissions_error(response)
        } else {
            crate::operation_deser::parse_list_resource_share_permissions_response(response)
        }
    }
}

/// Operation shape for `ListResourceTypes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_resource_types`](crate::client::Client::list_resource_types).
///
/// See [`crate::client::fluent_builders::ListResourceTypes`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResourceTypes {
    _private: (),
}
impl ListResourceTypes {
    /// Creates a new builder-style object to manufacture [`ListResourceTypesInput`](crate::input::ListResourceTypesInput)
    pub fn builder() -> crate::input::list_resource_types_input::Builder {
        crate::input::list_resource_types_input::Builder::default()
    }
    /// Creates a new `ListResourceTypes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListResourceTypes {
    type Output = std::result::Result<
        crate::output::ListResourceTypesOutput,
        crate::error::ListResourceTypesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resource_types_error(response)
        } else {
            crate::operation_deser::parse_list_resource_types_response(response)
        }
    }
}

/// Operation shape for `PromoteResourceShareCreatedFromPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`promote_resource_share_created_from_policy`](crate::client::Client::promote_resource_share_created_from_policy).
///
/// See [`crate::client::fluent_builders::PromoteResourceShareCreatedFromPolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PromoteResourceShareCreatedFromPolicy {
    _private: (),
}
impl PromoteResourceShareCreatedFromPolicy {
    /// Creates a new builder-style object to manufacture [`PromoteResourceShareCreatedFromPolicyInput`](crate::input::PromoteResourceShareCreatedFromPolicyInput)
    pub fn builder() -> crate::input::promote_resource_share_created_from_policy_input::Builder {
        crate::input::promote_resource_share_created_from_policy_input::Builder::default()
    }
    /// Creates a new `PromoteResourceShareCreatedFromPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PromoteResourceShareCreatedFromPolicy {
    type Output = std::result::Result<
        crate::output::PromoteResourceShareCreatedFromPolicyOutput,
        crate::error::PromoteResourceShareCreatedFromPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_promote_resource_share_created_from_policy_error(response)
        } else {
            crate::operation_deser::parse_promote_resource_share_created_from_policy_response(
                response,
            )
        }
    }
}

/// Operation shape for `RejectResourceShareInvitation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`reject_resource_share_invitation`](crate::client::Client::reject_resource_share_invitation).
///
/// See [`crate::client::fluent_builders::RejectResourceShareInvitation`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RejectResourceShareInvitation {
    _private: (),
}
impl RejectResourceShareInvitation {
    /// Creates a new builder-style object to manufacture [`RejectResourceShareInvitationInput`](crate::input::RejectResourceShareInvitationInput)
    pub fn builder() -> crate::input::reject_resource_share_invitation_input::Builder {
        crate::input::reject_resource_share_invitation_input::Builder::default()
    }
    /// Creates a new `RejectResourceShareInvitation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RejectResourceShareInvitation {
    type Output = std::result::Result<
        crate::output::RejectResourceShareInvitationOutput,
        crate::error::RejectResourceShareInvitationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_reject_resource_share_invitation_error(response)
        } else {
            crate::operation_deser::parse_reject_resource_share_invitation_response(response)
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
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
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
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
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

/// Operation shape for `UpdateResourceShare`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_resource_share`](crate::client::Client::update_resource_share).
///
/// See [`crate::client::fluent_builders::UpdateResourceShare`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateResourceShare {
    _private: (),
}
impl UpdateResourceShare {
    /// Creates a new builder-style object to manufacture [`UpdateResourceShareInput`](crate::input::UpdateResourceShareInput)
    pub fn builder() -> crate::input::update_resource_share_input::Builder {
        crate::input::update_resource_share_input::Builder::default()
    }
    /// Creates a new `UpdateResourceShare` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateResourceShare {
    type Output = std::result::Result<
        crate::output::UpdateResourceShareOutput,
        crate::error::UpdateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_resource_share_error(response)
        } else {
            crate::operation_deser::parse_update_resource_share_response(response)
        }
    }
}
