// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `CreateHomeRegionControl` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct CreateHomeRegionControlError {
    /// Kind of error that occurred.
    pub kind: CreateHomeRegionControlErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `CreateHomeRegionControl` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum CreateHomeRegionControlErrorKind {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Exception raised to indicate that authorization of an action was successful, when the <code>DryRun</code> flag is set to true.</p>
    DryRunOperation(crate::error::DryRunOperation),
    /// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for CreateHomeRegionControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            CreateHomeRegionControlErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            CreateHomeRegionControlErrorKind::DryRunOperation(_inner) => _inner.fmt(f),
            CreateHomeRegionControlErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            CreateHomeRegionControlErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            CreateHomeRegionControlErrorKind::ServiceUnavailableException(_inner) => _inner.fmt(f),
            CreateHomeRegionControlErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            CreateHomeRegionControlErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreateHomeRegionControlError {
    fn code(&self) -> Option<&str> {
        CreateHomeRegionControlError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreateHomeRegionControlError {
    /// Creates a new `CreateHomeRegionControlError`.
    pub fn new(kind: CreateHomeRegionControlErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `CreateHomeRegionControlError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: CreateHomeRegionControlErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `CreateHomeRegionControlError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: CreateHomeRegionControlErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `CreateHomeRegionControlErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            CreateHomeRegionControlErrorKind::AccessDeniedException(_)
        )
    }
    /// Returns `true` if the error kind is `CreateHomeRegionControlErrorKind::DryRunOperation`.
    pub fn is_dry_run_operation(&self) -> bool {
        matches!(
            &self.kind,
            CreateHomeRegionControlErrorKind::DryRunOperation(_)
        )
    }
    /// Returns `true` if the error kind is `CreateHomeRegionControlErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(
            &self.kind,
            CreateHomeRegionControlErrorKind::InternalServerError(_)
        )
    }
    /// Returns `true` if the error kind is `CreateHomeRegionControlErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            CreateHomeRegionControlErrorKind::InvalidInputException(_)
        )
    }
    /// Returns `true` if the error kind is `CreateHomeRegionControlErrorKind::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            CreateHomeRegionControlErrorKind::ServiceUnavailableException(_)
        )
    }
    /// Returns `true` if the error kind is `CreateHomeRegionControlErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            CreateHomeRegionControlErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for CreateHomeRegionControlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            CreateHomeRegionControlErrorKind::AccessDeniedException(_inner) => Some(_inner),
            CreateHomeRegionControlErrorKind::DryRunOperation(_inner) => Some(_inner),
            CreateHomeRegionControlErrorKind::InternalServerError(_inner) => Some(_inner),
            CreateHomeRegionControlErrorKind::InvalidInputException(_inner) => Some(_inner),
            CreateHomeRegionControlErrorKind::ServiceUnavailableException(_inner) => Some(_inner),
            CreateHomeRegionControlErrorKind::ThrottlingException(_inner) => Some(_inner),
            CreateHomeRegionControlErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `DescribeHomeRegionControls` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeHomeRegionControlsError {
    /// Kind of error that occurred.
    pub kind: DescribeHomeRegionControlsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DescribeHomeRegionControls` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeHomeRegionControlsErrorKind {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeHomeRegionControlsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeHomeRegionControlsErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            DescribeHomeRegionControlsErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            DescribeHomeRegionControlsErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            DescribeHomeRegionControlsErrorKind::ServiceUnavailableException(_inner) => {
                _inner.fmt(f)
            }
            DescribeHomeRegionControlsErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            DescribeHomeRegionControlsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeHomeRegionControlsError {
    fn code(&self) -> Option<&str> {
        DescribeHomeRegionControlsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeHomeRegionControlsError {
    /// Creates a new `DescribeHomeRegionControlsError`.
    pub fn new(kind: DescribeHomeRegionControlsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeHomeRegionControlsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeHomeRegionControlsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeHomeRegionControlsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeHomeRegionControlsErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DescribeHomeRegionControlsErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHomeRegionControlsErrorKind::AccessDeniedException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeHomeRegionControlsErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHomeRegionControlsErrorKind::InternalServerError(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeHomeRegionControlsErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHomeRegionControlsErrorKind::InvalidInputException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeHomeRegionControlsErrorKind::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHomeRegionControlsErrorKind::ServiceUnavailableException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeHomeRegionControlsErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeHomeRegionControlsErrorKind::ThrottlingException(_)
        )
    }
}
impl std::error::Error for DescribeHomeRegionControlsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeHomeRegionControlsErrorKind::AccessDeniedException(_inner) => Some(_inner),
            DescribeHomeRegionControlsErrorKind::InternalServerError(_inner) => Some(_inner),
            DescribeHomeRegionControlsErrorKind::InvalidInputException(_inner) => Some(_inner),
            DescribeHomeRegionControlsErrorKind::ServiceUnavailableException(_inner) => {
                Some(_inner)
            }
            DescribeHomeRegionControlsErrorKind::ThrottlingException(_inner) => Some(_inner),
            DescribeHomeRegionControlsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `GetHomeRegion` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetHomeRegionError {
    /// Kind of error that occurred.
    pub kind: GetHomeRegionErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `GetHomeRegion` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetHomeRegionErrorKind {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetHomeRegionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetHomeRegionErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            GetHomeRegionErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            GetHomeRegionErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            GetHomeRegionErrorKind::ServiceUnavailableException(_inner) => _inner.fmt(f),
            GetHomeRegionErrorKind::ThrottlingException(_inner) => _inner.fmt(f),
            GetHomeRegionErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetHomeRegionError {
    fn code(&self) -> Option<&str> {
        GetHomeRegionError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetHomeRegionError {
    /// Creates a new `GetHomeRegionError`.
    pub fn new(kind: GetHomeRegionErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetHomeRegionError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetHomeRegionErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `GetHomeRegionError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetHomeRegionErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetHomeRegionErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(&self.kind, GetHomeRegionErrorKind::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `GetHomeRegionErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, GetHomeRegionErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `GetHomeRegionErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(&self.kind, GetHomeRegionErrorKind::InvalidInputException(_))
    }
    /// Returns `true` if the error kind is `GetHomeRegionErrorKind::ServiceUnavailableException`.
    pub fn is_service_unavailable_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetHomeRegionErrorKind::ServiceUnavailableException(_)
        )
    }
    /// Returns `true` if the error kind is `GetHomeRegionErrorKind::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(&self.kind, GetHomeRegionErrorKind::ThrottlingException(_))
    }
}
impl std::error::Error for GetHomeRegionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetHomeRegionErrorKind::AccessDeniedException(_inner) => Some(_inner),
            GetHomeRegionErrorKind::InternalServerError(_inner) => Some(_inner),
            GetHomeRegionErrorKind::InvalidInputException(_inner) => Some(_inner),
            GetHomeRegionErrorKind::ServiceUnavailableException(_inner) => Some(_inner),
            GetHomeRegionErrorKind::ThrottlingException(_inner) => Some(_inner),
            GetHomeRegionErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The request was denied due to request throttling.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ThrottlingException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
    /// <p>The number of seconds the caller should wait before retrying.</p>
    pub retry_after_seconds: i32,
}
impl ThrottlingException {
    /// <p>The number of seconds the caller should wait before retrying.</p>
    pub fn retry_after_seconds(&self) -> i32 {
        self.retry_after_seconds
    }
}
impl std::fmt::Debug for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ThrottlingException");
        formatter.field("message", &self.message);
        formatter.field("retry_after_seconds", &self.retry_after_seconds);
        formatter.finish()
    }
}
impl ThrottlingException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ThrottlingException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThrottlingException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ThrottlingException {}
/// See [`ThrottlingException`](crate::error::ThrottlingException).
pub mod throttling_exception {

    /// A builder for [`ThrottlingException`](crate::error::ThrottlingException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
        pub(crate) retry_after_seconds: std::option::Option<i32>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// <p>The number of seconds the caller should wait before retrying.</p>
        pub fn retry_after_seconds(mut self, input: i32) -> Self {
            self.retry_after_seconds = Some(input);
            self
        }
        /// <p>The number of seconds the caller should wait before retrying.</p>
        pub fn set_retry_after_seconds(mut self, input: std::option::Option<i32>) -> Self {
            self.retry_after_seconds = input;
            self
        }
        /// Consumes the builder and constructs a [`ThrottlingException`](crate::error::ThrottlingException).
        pub fn build(self) -> crate::error::ThrottlingException {
            crate::error::ThrottlingException {
                message: self.message,
                retry_after_seconds: self.retry_after_seconds.unwrap_or_default(),
            }
        }
    }
}
impl ThrottlingException {
    /// Creates a new builder-style object to manufacture [`ThrottlingException`](crate::error::ThrottlingException).
    pub fn builder() -> crate::error::throttling_exception::Builder {
        crate::error::throttling_exception::Builder::default()
    }
}

/// <p>Exception raised when a request fails due to temporary unavailability of the service.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ServiceUnavailableException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ServiceUnavailableException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ServiceUnavailableException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ServiceUnavailableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUnavailableException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ServiceUnavailableException {}
/// See [`ServiceUnavailableException`](crate::error::ServiceUnavailableException).
pub mod service_unavailable_exception {

    /// A builder for [`ServiceUnavailableException`](crate::error::ServiceUnavailableException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ServiceUnavailableException`](crate::error::ServiceUnavailableException).
        pub fn build(self) -> crate::error::ServiceUnavailableException {
            crate::error::ServiceUnavailableException {
                message: self.message,
            }
        }
    }
}
impl ServiceUnavailableException {
    /// Creates a new builder-style object to manufacture [`ServiceUnavailableException`](crate::error::ServiceUnavailableException).
    pub fn builder() -> crate::error::service_unavailable_exception::Builder {
        crate::error::service_unavailable_exception::Builder::default()
    }
}

/// <p>Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidInputException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidInputException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidInputException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidInputException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidInputException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidInputException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidInputException {}
/// See [`InvalidInputException`](crate::error::InvalidInputException).
pub mod invalid_input_exception {

    /// A builder for [`InvalidInputException`](crate::error::InvalidInputException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidInputException`](crate::error::InvalidInputException).
        pub fn build(self) -> crate::error::InvalidInputException {
            crate::error::InvalidInputException {
                message: self.message,
            }
        }
    }
}
impl InvalidInputException {
    /// Creates a new builder-style object to manufacture [`InvalidInputException`](crate::error::InvalidInputException).
    pub fn builder() -> crate::error::invalid_input_exception::Builder {
        crate::error::invalid_input_exception::Builder::default()
    }
}

/// <p>Exception raised when an internal, configuration, or dependency error is encountered.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServerError {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerError {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerError")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerError {}
/// See [`InternalServerError`](crate::error::InternalServerError).
pub mod internal_server_error {

    /// A builder for [`InternalServerError`](crate::error::InternalServerError).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServerError`](crate::error::InternalServerError).
        pub fn build(self) -> crate::error::InternalServerError {
            crate::error::InternalServerError {
                message: self.message,
            }
        }
    }
}
impl InternalServerError {
    /// Creates a new builder-style object to manufacture [`InternalServerError`](crate::error::InternalServerError).
    pub fn builder() -> crate::error::internal_server_error::Builder {
        crate::error::internal_server_error::Builder::default()
    }
}

/// <p>You do not have sufficient access to perform this action.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AccessDeniedException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessDeniedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl AccessDeniedException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessDeniedException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for AccessDeniedException {}
/// See [`AccessDeniedException`](crate::error::AccessDeniedException).
pub mod access_denied_exception {

    /// A builder for [`AccessDeniedException`](crate::error::AccessDeniedException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`AccessDeniedException`](crate::error::AccessDeniedException).
        pub fn build(self) -> crate::error::AccessDeniedException {
            crate::error::AccessDeniedException {
                message: self.message,
            }
        }
    }
}
impl AccessDeniedException {
    /// Creates a new builder-style object to manufacture [`AccessDeniedException`](crate::error::AccessDeniedException).
    pub fn builder() -> crate::error::access_denied_exception::Builder {
        crate::error::access_denied_exception::Builder::default()
    }
}

/// <p>Exception raised to indicate that authorization of an action was successful, when the <code>DryRun</code> flag is set to true.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DryRunOperation {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DryRunOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DryRunOperation");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl DryRunOperation {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for DryRunOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DryRunOperation")?;
        if let Some(inner_6) = &self.message {
            write!(f, ": {}", inner_6)?;
        }
        Ok(())
    }
}
impl std::error::Error for DryRunOperation {}
/// See [`DryRunOperation`](crate::error::DryRunOperation).
pub mod dry_run_operation {

    /// A builder for [`DryRunOperation`](crate::error::DryRunOperation).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`DryRunOperation`](crate::error::DryRunOperation).
        pub fn build(self) -> crate::error::DryRunOperation {
            crate::error::DryRunOperation {
                message: self.message,
            }
        }
    }
}
impl DryRunOperation {
    /// Creates a new builder-style object to manufacture [`DryRunOperation`](crate::error::DryRunOperation).
    pub fn builder() -> crate::error::dry_run_operation::Builder {
        crate::error::dry_run_operation::Builder::default()
    }
}
