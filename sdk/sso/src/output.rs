// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct LogoutOutput {}
impl std::fmt::Debug for LogoutOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LogoutOutput");
        formatter.finish()
    }
}
/// See [`LogoutOutput`](crate::output::LogoutOutput).
pub mod logout_output {

    /// A builder for [`LogoutOutput`](crate::output::LogoutOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`LogoutOutput`](crate::output::LogoutOutput).
        pub fn build(self) -> crate::output::LogoutOutput {
            crate::output::LogoutOutput {}
        }
    }
}
impl LogoutOutput {
    /// Creates a new builder-style object to manufacture [`LogoutOutput`](crate::output::LogoutOutput).
    pub fn builder() -> crate::output::logout_output::Builder {
        crate::output::logout_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAccountsOutput {
    /// <p>The page token client that is used to retrieve the list of accounts.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
    pub account_list: std::option::Option<std::vec::Vec<crate::model::AccountInfo>>,
}
impl ListAccountsOutput {
    /// <p>The page token client that is used to retrieve the list of accounts.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
    pub fn account_list(&self) -> std::option::Option<&[crate::model::AccountInfo]> {
        self.account_list.as_deref()
    }
}
impl std::fmt::Debug for ListAccountsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAccountsOutput");
        formatter.field("next_token", &self.next_token);
        formatter.field("account_list", &self.account_list);
        formatter.finish()
    }
}
/// See [`ListAccountsOutput`](crate::output::ListAccountsOutput).
pub mod list_accounts_output {

    /// A builder for [`ListAccountsOutput`](crate::output::ListAccountsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) account_list: std::option::Option<std::vec::Vec<crate::model::AccountInfo>>,
    }
    impl Builder {
        /// <p>The page token client that is used to retrieve the list of accounts.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The page token client that is used to retrieve the list of accounts.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Appends an item to `account_list`.
        ///
        /// To override the contents of this collection use [`set_account_list`](Self::set_account_list).
        ///
        /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
        pub fn account_list(mut self, input: crate::model::AccountInfo) -> Self {
            let mut v = self.account_list.unwrap_or_default();
            v.push(input);
            self.account_list = Some(v);
            self
        }
        /// <p>A paginated response with the list of account information and the next token if more results are available.</p>
        pub fn set_account_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::AccountInfo>>,
        ) -> Self {
            self.account_list = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAccountsOutput`](crate::output::ListAccountsOutput).
        pub fn build(self) -> crate::output::ListAccountsOutput {
            crate::output::ListAccountsOutput {
                next_token: self.next_token,
                account_list: self.account_list,
            }
        }
    }
}
impl ListAccountsOutput {
    /// Creates a new builder-style object to manufacture [`ListAccountsOutput`](crate::output::ListAccountsOutput).
    pub fn builder() -> crate::output::list_accounts_output::Builder {
        crate::output::list_accounts_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListAccountRolesOutput {
    /// <p>The page token client that is used to retrieve the list of accounts.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>A paginated response with the list of roles and the next token if more results are available.</p>
    pub role_list: std::option::Option<std::vec::Vec<crate::model::RoleInfo>>,
}
impl ListAccountRolesOutput {
    /// <p>The page token client that is used to retrieve the list of accounts.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A paginated response with the list of roles and the next token if more results are available.</p>
    pub fn role_list(&self) -> std::option::Option<&[crate::model::RoleInfo]> {
        self.role_list.as_deref()
    }
}
impl std::fmt::Debug for ListAccountRolesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListAccountRolesOutput");
        formatter.field("next_token", &self.next_token);
        formatter.field("role_list", &self.role_list);
        formatter.finish()
    }
}
/// See [`ListAccountRolesOutput`](crate::output::ListAccountRolesOutput).
pub mod list_account_roles_output {

    /// A builder for [`ListAccountRolesOutput`](crate::output::ListAccountRolesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) role_list: std::option::Option<std::vec::Vec<crate::model::RoleInfo>>,
    }
    impl Builder {
        /// <p>The page token client that is used to retrieve the list of accounts.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The page token client that is used to retrieve the list of accounts.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Appends an item to `role_list`.
        ///
        /// To override the contents of this collection use [`set_role_list`](Self::set_role_list).
        ///
        /// <p>A paginated response with the list of roles and the next token if more results are available.</p>
        pub fn role_list(mut self, input: crate::model::RoleInfo) -> Self {
            let mut v = self.role_list.unwrap_or_default();
            v.push(input);
            self.role_list = Some(v);
            self
        }
        /// <p>A paginated response with the list of roles and the next token if more results are available.</p>
        pub fn set_role_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::RoleInfo>>,
        ) -> Self {
            self.role_list = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAccountRolesOutput`](crate::output::ListAccountRolesOutput).
        pub fn build(self) -> crate::output::ListAccountRolesOutput {
            crate::output::ListAccountRolesOutput {
                next_token: self.next_token,
                role_list: self.role_list,
            }
        }
    }
}
impl ListAccountRolesOutput {
    /// Creates a new builder-style object to manufacture [`ListAccountRolesOutput`](crate::output::ListAccountRolesOutput).
    pub fn builder() -> crate::output::list_account_roles_output::Builder {
        crate::output::list_account_roles_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRoleCredentialsOutput {
    /// <p>The credentials for the role that is assigned to the user.</p>
    pub role_credentials: std::option::Option<crate::model::RoleCredentials>,
}
impl GetRoleCredentialsOutput {
    /// <p>The credentials for the role that is assigned to the user.</p>
    pub fn role_credentials(&self) -> std::option::Option<&crate::model::RoleCredentials> {
        self.role_credentials.as_ref()
    }
}
impl std::fmt::Debug for GetRoleCredentialsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRoleCredentialsOutput");
        formatter.field("role_credentials", &self.role_credentials);
        formatter.finish()
    }
}
/// See [`GetRoleCredentialsOutput`](crate::output::GetRoleCredentialsOutput).
pub mod get_role_credentials_output {

    /// A builder for [`GetRoleCredentialsOutput`](crate::output::GetRoleCredentialsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) role_credentials: std::option::Option<crate::model::RoleCredentials>,
    }
    impl Builder {
        /// <p>The credentials for the role that is assigned to the user.</p>
        pub fn role_credentials(mut self, input: crate::model::RoleCredentials) -> Self {
            self.role_credentials = Some(input);
            self
        }
        /// <p>The credentials for the role that is assigned to the user.</p>
        pub fn set_role_credentials(
            mut self,
            input: std::option::Option<crate::model::RoleCredentials>,
        ) -> Self {
            self.role_credentials = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRoleCredentialsOutput`](crate::output::GetRoleCredentialsOutput).
        pub fn build(self) -> crate::output::GetRoleCredentialsOutput {
            crate::output::GetRoleCredentialsOutput {
                role_credentials: self.role_credentials,
            }
        }
    }
}
impl GetRoleCredentialsOutput {
    /// Creates a new builder-style object to manufacture [`GetRoleCredentialsOutput`](crate::output::GetRoleCredentialsOutput).
    pub fn builder() -> crate::output::get_role_credentials_output::Builder {
        crate::output::get_role_credentials_output::Builder::default()
    }
}
