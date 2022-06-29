// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendSshPublicKeyOutput {
    /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    pub request_id: std::option::Option<std::string::String>,
    /// <p>Is true if the request succeeds and an error otherwise.</p>
    pub success: bool,
}
impl SendSshPublicKeyOutput {
    /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    pub fn request_id(&self) -> std::option::Option<&str> {
        self.request_id.as_deref()
    }
    /// <p>Is true if the request succeeds and an error otherwise.</p>
    pub fn success(&self) -> bool {
        self.success
    }
}
impl std::fmt::Debug for SendSshPublicKeyOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendSshPublicKeyOutput");
        formatter.field("request_id", &self.request_id);
        formatter.field("success", &self.success);
        formatter.finish()
    }
}
/// See [`SendSshPublicKeyOutput`](crate::output::SendSshPublicKeyOutput).
pub mod send_ssh_public_key_output {

    /// A builder for [`SendSshPublicKeyOutput`](crate::output::SendSshPublicKeyOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) request_id: std::option::Option<std::string::String>,
        pub(crate) success: std::option::Option<bool>,
    }
    impl Builder {
        /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
        pub fn request_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.request_id = Some(input.into());
            self
        }
        /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
        pub fn set_request_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.request_id = input;
            self
        }
        /// <p>Is true if the request succeeds and an error otherwise.</p>
        pub fn success(mut self, input: bool) -> Self {
            self.success = Some(input);
            self
        }
        /// <p>Is true if the request succeeds and an error otherwise.</p>
        pub fn set_success(mut self, input: std::option::Option<bool>) -> Self {
            self.success = input;
            self
        }
        /// Consumes the builder and constructs a [`SendSshPublicKeyOutput`](crate::output::SendSshPublicKeyOutput).
        pub fn build(self) -> crate::output::SendSshPublicKeyOutput {
            crate::output::SendSshPublicKeyOutput {
                request_id: self.request_id,
                success: self.success.unwrap_or_default(),
            }
        }
    }
}
impl SendSshPublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`SendSshPublicKeyOutput`](crate::output::SendSshPublicKeyOutput).
    pub fn builder() -> crate::output::send_ssh_public_key_output::Builder {
        crate::output::send_ssh_public_key_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendSerialConsoleSshPublicKeyOutput {
    /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    pub request_id: std::option::Option<std::string::String>,
    /// <p>Is true if the request succeeds and an error otherwise.</p>
    pub success: bool,
}
impl SendSerialConsoleSshPublicKeyOutput {
    /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    pub fn request_id(&self) -> std::option::Option<&str> {
        self.request_id.as_deref()
    }
    /// <p>Is true if the request succeeds and an error otherwise.</p>
    pub fn success(&self) -> bool {
        self.success
    }
}
impl std::fmt::Debug for SendSerialConsoleSshPublicKeyOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendSerialConsoleSshPublicKeyOutput");
        formatter.field("request_id", &self.request_id);
        formatter.field("success", &self.success);
        formatter.finish()
    }
}
/// See [`SendSerialConsoleSshPublicKeyOutput`](crate::output::SendSerialConsoleSshPublicKeyOutput).
pub mod send_serial_console_ssh_public_key_output {

    /// A builder for [`SendSerialConsoleSshPublicKeyOutput`](crate::output::SendSerialConsoleSshPublicKeyOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) request_id: std::option::Option<std::string::String>,
        pub(crate) success: std::option::Option<bool>,
    }
    impl Builder {
        /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
        pub fn request_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.request_id = Some(input.into());
            self
        }
        /// <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
        pub fn set_request_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.request_id = input;
            self
        }
        /// <p>Is true if the request succeeds and an error otherwise.</p>
        pub fn success(mut self, input: bool) -> Self {
            self.success = Some(input);
            self
        }
        /// <p>Is true if the request succeeds and an error otherwise.</p>
        pub fn set_success(mut self, input: std::option::Option<bool>) -> Self {
            self.success = input;
            self
        }
        /// Consumes the builder and constructs a [`SendSerialConsoleSshPublicKeyOutput`](crate::output::SendSerialConsoleSshPublicKeyOutput).
        pub fn build(self) -> crate::output::SendSerialConsoleSshPublicKeyOutput {
            crate::output::SendSerialConsoleSshPublicKeyOutput {
                request_id: self.request_id,
                success: self.success.unwrap_or_default(),
            }
        }
    }
}
impl SendSerialConsoleSshPublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`SendSerialConsoleSshPublicKeyOutput`](crate::output::SendSerialConsoleSshPublicKeyOutput).
    pub fn builder() -> crate::output::send_serial_console_ssh_public_key_output::Builder {
        crate::output::send_serial_console_ssh_public_key_output::Builder::default()
    }
}
