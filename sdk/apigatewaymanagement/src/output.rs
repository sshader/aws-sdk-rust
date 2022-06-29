// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PostToConnectionOutput {}
impl std::fmt::Debug for PostToConnectionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PostToConnectionOutput");
        formatter.finish()
    }
}
/// See [`PostToConnectionOutput`](crate::output::PostToConnectionOutput).
pub mod post_to_connection_output {

    /// A builder for [`PostToConnectionOutput`](crate::output::PostToConnectionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PostToConnectionOutput`](crate::output::PostToConnectionOutput).
        pub fn build(self) -> crate::output::PostToConnectionOutput {
            crate::output::PostToConnectionOutput {}
        }
    }
}
impl PostToConnectionOutput {
    /// Creates a new builder-style object to manufacture [`PostToConnectionOutput`](crate::output::PostToConnectionOutput).
    pub fn builder() -> crate::output::post_to_connection_output::Builder {
        crate::output::post_to_connection_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetConnectionOutput {
    /// <p>The time in ISO 8601 format for when the connection was established.</p>
    pub connected_at: std::option::Option<aws_smithy_types::DateTime>,
    #[allow(missing_docs)] // documentation missing in model
    pub identity: std::option::Option<crate::model::Identity>,
    /// <p>The time in ISO 8601 format for when the connection was last active.</p>
    pub last_active_at: std::option::Option<aws_smithy_types::DateTime>,
}
impl GetConnectionOutput {
    /// <p>The time in ISO 8601 format for when the connection was established.</p>
    pub fn connected_at(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.connected_at.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn identity(&self) -> std::option::Option<&crate::model::Identity> {
        self.identity.as_ref()
    }
    /// <p>The time in ISO 8601 format for when the connection was last active.</p>
    pub fn last_active_at(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_active_at.as_ref()
    }
}
impl std::fmt::Debug for GetConnectionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetConnectionOutput");
        formatter.field("connected_at", &self.connected_at);
        formatter.field("identity", &self.identity);
        formatter.field("last_active_at", &self.last_active_at);
        formatter.finish()
    }
}
/// See [`GetConnectionOutput`](crate::output::GetConnectionOutput).
pub mod get_connection_output {

    /// A builder for [`GetConnectionOutput`](crate::output::GetConnectionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connected_at: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) identity: std::option::Option<crate::model::Identity>,
        pub(crate) last_active_at: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>The time in ISO 8601 format for when the connection was established.</p>
        pub fn connected_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.connected_at = Some(input);
            self
        }
        /// <p>The time in ISO 8601 format for when the connection was established.</p>
        pub fn set_connected_at(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.connected_at = input;
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn identity(mut self, input: crate::model::Identity) -> Self {
            self.identity = Some(input);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_identity(mut self, input: std::option::Option<crate::model::Identity>) -> Self {
            self.identity = input;
            self
        }
        /// <p>The time in ISO 8601 format for when the connection was last active.</p>
        pub fn last_active_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.last_active_at = Some(input);
            self
        }
        /// <p>The time in ISO 8601 format for when the connection was last active.</p>
        pub fn set_last_active_at(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.last_active_at = input;
            self
        }
        /// Consumes the builder and constructs a [`GetConnectionOutput`](crate::output::GetConnectionOutput).
        pub fn build(self) -> crate::output::GetConnectionOutput {
            crate::output::GetConnectionOutput {
                connected_at: self.connected_at,
                identity: self.identity,
                last_active_at: self.last_active_at,
            }
        }
    }
}
impl GetConnectionOutput {
    /// Creates a new builder-style object to manufacture [`GetConnectionOutput`](crate::output::GetConnectionOutput).
    pub fn builder() -> crate::output::get_connection_output::Builder {
        crate::output::get_connection_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteConnectionOutput {}
impl std::fmt::Debug for DeleteConnectionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteConnectionOutput");
        formatter.finish()
    }
}
/// See [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
pub mod delete_connection_output {

    /// A builder for [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
        pub fn build(self) -> crate::output::DeleteConnectionOutput {
            crate::output::DeleteConnectionOutput {}
        }
    }
}
impl DeleteConnectionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
    pub fn builder() -> crate::output::delete_connection_output::Builder {
        crate::output::delete_connection_output::Builder::default()
    }
}
