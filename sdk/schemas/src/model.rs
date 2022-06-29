// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum Type {
    #[allow(missing_docs)] // documentation missing in model
    JsonSchemaDraft4,
    #[allow(missing_docs)] // documentation missing in model
    OpenApi3,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "JSONSchemaDraft4" => Type::JsonSchemaDraft4,
            "OpenApi3" => Type::OpenApi3,
            other => Type::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for Type {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Type::from(s))
    }
}
impl Type {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            Type::JsonSchemaDraft4 => "JSONSchemaDraft4",
            Type::OpenApi3 => "OpenApi3",
            Type::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["JSONSchemaDraft4", "OpenApi3"]
    }
}
impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DiscovererState {
    #[allow(missing_docs)] // documentation missing in model
    Started,
    #[allow(missing_docs)] // documentation missing in model
    Stopped,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for DiscovererState {
    fn from(s: &str) -> Self {
        match s {
            "STARTED" => DiscovererState::Started,
            "STOPPED" => DiscovererState::Stopped,
            other => DiscovererState::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for DiscovererState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DiscovererState::from(s))
    }
}
impl DiscovererState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DiscovererState::Started => "STARTED",
            DiscovererState::Stopped => "STOPPED",
            DiscovererState::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["STARTED", "STOPPED"]
    }
}
impl AsRef<str> for DiscovererState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SearchSchemaSummary {
    /// <p>The name of the registry.</p>
    pub registry_name: std::option::Option<std::string::String>,
    /// <p>The ARN of the schema.</p>
    pub schema_arn: std::option::Option<std::string::String>,
    /// <p>The name of the schema.</p>
    pub schema_name: std::option::Option<std::string::String>,
    /// <p>An array of schema version summaries.</p>
    pub schema_versions:
        std::option::Option<std::vec::Vec<crate::model::SearchSchemaVersionSummary>>,
}
impl SearchSchemaSummary {
    /// <p>The name of the registry.</p>
    pub fn registry_name(&self) -> std::option::Option<&str> {
        self.registry_name.as_deref()
    }
    /// <p>The ARN of the schema.</p>
    pub fn schema_arn(&self) -> std::option::Option<&str> {
        self.schema_arn.as_deref()
    }
    /// <p>The name of the schema.</p>
    pub fn schema_name(&self) -> std::option::Option<&str> {
        self.schema_name.as_deref()
    }
    /// <p>An array of schema version summaries.</p>
    pub fn schema_versions(
        &self,
    ) -> std::option::Option<&[crate::model::SearchSchemaVersionSummary]> {
        self.schema_versions.as_deref()
    }
}
impl std::fmt::Debug for SearchSchemaSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SearchSchemaSummary");
        formatter.field("registry_name", &self.registry_name);
        formatter.field("schema_arn", &self.schema_arn);
        formatter.field("schema_name", &self.schema_name);
        formatter.field("schema_versions", &self.schema_versions);
        formatter.finish()
    }
}
/// See [`SearchSchemaSummary`](crate::model::SearchSchemaSummary).
pub mod search_schema_summary {

    /// A builder for [`SearchSchemaSummary`](crate::model::SearchSchemaSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) registry_name: std::option::Option<std::string::String>,
        pub(crate) schema_arn: std::option::Option<std::string::String>,
        pub(crate) schema_name: std::option::Option<std::string::String>,
        pub(crate) schema_versions:
            std::option::Option<std::vec::Vec<crate::model::SearchSchemaVersionSummary>>,
    }
    impl Builder {
        /// <p>The name of the registry.</p>
        pub fn registry_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.registry_name = Some(input.into());
            self
        }
        /// <p>The name of the registry.</p>
        pub fn set_registry_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.registry_name = input;
            self
        }
        /// <p>The ARN of the schema.</p>
        pub fn schema_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the schema.</p>
        pub fn set_schema_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.schema_arn = input;
            self
        }
        /// <p>The name of the schema.</p>
        pub fn schema_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_name = Some(input.into());
            self
        }
        /// <p>The name of the schema.</p>
        pub fn set_schema_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.schema_name = input;
            self
        }
        /// Appends an item to `schema_versions`.
        ///
        /// To override the contents of this collection use [`set_schema_versions`](Self::set_schema_versions).
        ///
        /// <p>An array of schema version summaries.</p>
        pub fn schema_versions(mut self, input: crate::model::SearchSchemaVersionSummary) -> Self {
            let mut v = self.schema_versions.unwrap_or_default();
            v.push(input);
            self.schema_versions = Some(v);
            self
        }
        /// <p>An array of schema version summaries.</p>
        pub fn set_schema_versions(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SearchSchemaVersionSummary>>,
        ) -> Self {
            self.schema_versions = input;
            self
        }
        /// Consumes the builder and constructs a [`SearchSchemaSummary`](crate::model::SearchSchemaSummary).
        pub fn build(self) -> crate::model::SearchSchemaSummary {
            crate::model::SearchSchemaSummary {
                registry_name: self.registry_name,
                schema_arn: self.schema_arn,
                schema_name: self.schema_name,
                schema_versions: self.schema_versions,
            }
        }
    }
}
impl SearchSchemaSummary {
    /// Creates a new builder-style object to manufacture [`SearchSchemaSummary`](crate::model::SearchSchemaSummary).
    pub fn builder() -> crate::model::search_schema_summary::Builder {
        crate::model::search_schema_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SearchSchemaVersionSummary {
    /// <p>The date the schema version was created.</p>
    pub created_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The version number of the schema</p>
    pub schema_version: std::option::Option<std::string::String>,
    /// <p>The type of schema.</p>
    pub r#type: std::option::Option<crate::model::Type>,
}
impl SearchSchemaVersionSummary {
    /// <p>The date the schema version was created.</p>
    pub fn created_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The version number of the schema</p>
    pub fn schema_version(&self) -> std::option::Option<&str> {
        self.schema_version.as_deref()
    }
    /// <p>The type of schema.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::model::Type> {
        self.r#type.as_ref()
    }
}
impl std::fmt::Debug for SearchSchemaVersionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SearchSchemaVersionSummary");
        formatter.field("created_date", &self.created_date);
        formatter.field("schema_version", &self.schema_version);
        formatter.field("r#type", &self.r#type);
        formatter.finish()
    }
}
/// See [`SearchSchemaVersionSummary`](crate::model::SearchSchemaVersionSummary).
pub mod search_schema_version_summary {

    /// A builder for [`SearchSchemaVersionSummary`](crate::model::SearchSchemaVersionSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) created_date: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) schema_version: std::option::Option<std::string::String>,
        pub(crate) r#type: std::option::Option<crate::model::Type>,
    }
    impl Builder {
        /// <p>The date the schema version was created.</p>
        pub fn created_date(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_date = Some(input);
            self
        }
        /// <p>The date the schema version was created.</p>
        pub fn set_created_date(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.created_date = input;
            self
        }
        /// <p>The version number of the schema</p>
        pub fn schema_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_version = Some(input.into());
            self
        }
        /// <p>The version number of the schema</p>
        pub fn set_schema_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.schema_version = input;
            self
        }
        /// <p>The type of schema.</p>
        pub fn r#type(mut self, input: crate::model::Type) -> Self {
            self.r#type = Some(input);
            self
        }
        /// <p>The type of schema.</p>
        pub fn set_type(mut self, input: std::option::Option<crate::model::Type>) -> Self {
            self.r#type = input;
            self
        }
        /// Consumes the builder and constructs a [`SearchSchemaVersionSummary`](crate::model::SearchSchemaVersionSummary).
        pub fn build(self) -> crate::model::SearchSchemaVersionSummary {
            crate::model::SearchSchemaVersionSummary {
                created_date: self.created_date,
                schema_version: self.schema_version,
                r#type: self.r#type,
            }
        }
    }
}
impl SearchSchemaVersionSummary {
    /// Creates a new builder-style object to manufacture [`SearchSchemaVersionSummary`](crate::model::SearchSchemaVersionSummary).
    pub fn builder() -> crate::model::search_schema_version_summary::Builder {
        crate::model::search_schema_version_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum CodeGenerationStatus {
    #[allow(missing_docs)] // documentation missing in model
    CreateComplete,
    #[allow(missing_docs)] // documentation missing in model
    CreateFailed,
    #[allow(missing_docs)] // documentation missing in model
    CreateInProgress,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for CodeGenerationStatus {
    fn from(s: &str) -> Self {
        match s {
            "CREATE_COMPLETE" => CodeGenerationStatus::CreateComplete,
            "CREATE_FAILED" => CodeGenerationStatus::CreateFailed,
            "CREATE_IN_PROGRESS" => CodeGenerationStatus::CreateInProgress,
            other => CodeGenerationStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for CodeGenerationStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(CodeGenerationStatus::from(s))
    }
}
impl CodeGenerationStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            CodeGenerationStatus::CreateComplete => "CREATE_COMPLETE",
            CodeGenerationStatus::CreateFailed => "CREATE_FAILED",
            CodeGenerationStatus::CreateInProgress => "CREATE_IN_PROGRESS",
            CodeGenerationStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CREATE_COMPLETE", "CREATE_FAILED", "CREATE_IN_PROGRESS"]
    }
}
impl AsRef<str> for CodeGenerationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SchemaVersionSummary {
    /// <p>The ARN of the schema version.</p>
    pub schema_arn: std::option::Option<std::string::String>,
    /// <p>The name of the schema.</p>
    pub schema_name: std::option::Option<std::string::String>,
    /// <p>The version number of the schema.</p>
    pub schema_version: std::option::Option<std::string::String>,
    /// <p>The type of schema.</p>
    pub r#type: std::option::Option<crate::model::Type>,
}
impl SchemaVersionSummary {
    /// <p>The ARN of the schema version.</p>
    pub fn schema_arn(&self) -> std::option::Option<&str> {
        self.schema_arn.as_deref()
    }
    /// <p>The name of the schema.</p>
    pub fn schema_name(&self) -> std::option::Option<&str> {
        self.schema_name.as_deref()
    }
    /// <p>The version number of the schema.</p>
    pub fn schema_version(&self) -> std::option::Option<&str> {
        self.schema_version.as_deref()
    }
    /// <p>The type of schema.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::model::Type> {
        self.r#type.as_ref()
    }
}
impl std::fmt::Debug for SchemaVersionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SchemaVersionSummary");
        formatter.field("schema_arn", &self.schema_arn);
        formatter.field("schema_name", &self.schema_name);
        formatter.field("schema_version", &self.schema_version);
        formatter.field("r#type", &self.r#type);
        formatter.finish()
    }
}
/// See [`SchemaVersionSummary`](crate::model::SchemaVersionSummary).
pub mod schema_version_summary {

    /// A builder for [`SchemaVersionSummary`](crate::model::SchemaVersionSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) schema_arn: std::option::Option<std::string::String>,
        pub(crate) schema_name: std::option::Option<std::string::String>,
        pub(crate) schema_version: std::option::Option<std::string::String>,
        pub(crate) r#type: std::option::Option<crate::model::Type>,
    }
    impl Builder {
        /// <p>The ARN of the schema version.</p>
        pub fn schema_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the schema version.</p>
        pub fn set_schema_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.schema_arn = input;
            self
        }
        /// <p>The name of the schema.</p>
        pub fn schema_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_name = Some(input.into());
            self
        }
        /// <p>The name of the schema.</p>
        pub fn set_schema_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.schema_name = input;
            self
        }
        /// <p>The version number of the schema.</p>
        pub fn schema_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_version = Some(input.into());
            self
        }
        /// <p>The version number of the schema.</p>
        pub fn set_schema_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.schema_version = input;
            self
        }
        /// <p>The type of schema.</p>
        pub fn r#type(mut self, input: crate::model::Type) -> Self {
            self.r#type = Some(input);
            self
        }
        /// <p>The type of schema.</p>
        pub fn set_type(mut self, input: std::option::Option<crate::model::Type>) -> Self {
            self.r#type = input;
            self
        }
        /// Consumes the builder and constructs a [`SchemaVersionSummary`](crate::model::SchemaVersionSummary).
        pub fn build(self) -> crate::model::SchemaVersionSummary {
            crate::model::SchemaVersionSummary {
                schema_arn: self.schema_arn,
                schema_name: self.schema_name,
                schema_version: self.schema_version,
                r#type: self.r#type,
            }
        }
    }
}
impl SchemaVersionSummary {
    /// Creates a new builder-style object to manufacture [`SchemaVersionSummary`](crate::model::SchemaVersionSummary).
    pub fn builder() -> crate::model::schema_version_summary::Builder {
        crate::model::schema_version_summary::Builder::default()
    }
}

/// <p>A summary of schema details.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SchemaSummary {
    /// <p>The date and time that schema was modified.</p>
    pub last_modified: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The ARN of the schema.</p>
    pub schema_arn: std::option::Option<std::string::String>,
    /// <p>The name of the schema.</p>
    pub schema_name: std::option::Option<std::string::String>,
    /// <p>Tags associated with the schema.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The number of versions available for the schema.</p>
    pub version_count: i64,
}
impl SchemaSummary {
    /// <p>The date and time that schema was modified.</p>
    pub fn last_modified(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
    /// <p>The ARN of the schema.</p>
    pub fn schema_arn(&self) -> std::option::Option<&str> {
        self.schema_arn.as_deref()
    }
    /// <p>The name of the schema.</p>
    pub fn schema_name(&self) -> std::option::Option<&str> {
        self.schema_name.as_deref()
    }
    /// <p>Tags associated with the schema.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
    /// <p>The number of versions available for the schema.</p>
    pub fn version_count(&self) -> i64 {
        self.version_count
    }
}
impl std::fmt::Debug for SchemaSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SchemaSummary");
        formatter.field("last_modified", &self.last_modified);
        formatter.field("schema_arn", &self.schema_arn);
        formatter.field("schema_name", &self.schema_name);
        formatter.field("tags", &self.tags);
        formatter.field("version_count", &self.version_count);
        formatter.finish()
    }
}
/// See [`SchemaSummary`](crate::model::SchemaSummary).
pub mod schema_summary {

    /// A builder for [`SchemaSummary`](crate::model::SchemaSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) last_modified: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) schema_arn: std::option::Option<std::string::String>,
        pub(crate) schema_name: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) version_count: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The date and time that schema was modified.</p>
        pub fn last_modified(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.last_modified = Some(input);
            self
        }
        /// <p>The date and time that schema was modified.</p>
        pub fn set_last_modified(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.last_modified = input;
            self
        }
        /// <p>The ARN of the schema.</p>
        pub fn schema_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the schema.</p>
        pub fn set_schema_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.schema_arn = input;
            self
        }
        /// <p>The name of the schema.</p>
        pub fn schema_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.schema_name = Some(input.into());
            self
        }
        /// <p>The name of the schema.</p>
        pub fn set_schema_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.schema_name = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Tags associated with the schema.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>Tags associated with the schema.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// <p>The number of versions available for the schema.</p>
        pub fn version_count(mut self, input: i64) -> Self {
            self.version_count = Some(input);
            self
        }
        /// <p>The number of versions available for the schema.</p>
        pub fn set_version_count(mut self, input: std::option::Option<i64>) -> Self {
            self.version_count = input;
            self
        }
        /// Consumes the builder and constructs a [`SchemaSummary`](crate::model::SchemaSummary).
        pub fn build(self) -> crate::model::SchemaSummary {
            crate::model::SchemaSummary {
                last_modified: self.last_modified,
                schema_arn: self.schema_arn,
                schema_name: self.schema_name,
                tags: self.tags,
                version_count: self.version_count.unwrap_or_default(),
            }
        }
    }
}
impl SchemaSummary {
    /// Creates a new builder-style object to manufacture [`SchemaSummary`](crate::model::SchemaSummary).
    pub fn builder() -> crate::model::schema_summary::Builder {
        crate::model::schema_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RegistrySummary {
    /// <p>The ARN of the registry.</p>
    pub registry_arn: std::option::Option<std::string::String>,
    /// <p>The name of the registry.</p>
    pub registry_name: std::option::Option<std::string::String>,
    /// <p>Tags associated with the registry.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl RegistrySummary {
    /// <p>The ARN of the registry.</p>
    pub fn registry_arn(&self) -> std::option::Option<&str> {
        self.registry_arn.as_deref()
    }
    /// <p>The name of the registry.</p>
    pub fn registry_name(&self) -> std::option::Option<&str> {
        self.registry_name.as_deref()
    }
    /// <p>Tags associated with the registry.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl std::fmt::Debug for RegistrySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RegistrySummary");
        formatter.field("registry_arn", &self.registry_arn);
        formatter.field("registry_name", &self.registry_name);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`RegistrySummary`](crate::model::RegistrySummary).
pub mod registry_summary {

    /// A builder for [`RegistrySummary`](crate::model::RegistrySummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) registry_arn: std::option::Option<std::string::String>,
        pub(crate) registry_name: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The ARN of the registry.</p>
        pub fn registry_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.registry_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the registry.</p>
        pub fn set_registry_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.registry_arn = input;
            self
        }
        /// <p>The name of the registry.</p>
        pub fn registry_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.registry_name = Some(input.into());
            self
        }
        /// <p>The name of the registry.</p>
        pub fn set_registry_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.registry_name = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Tags associated with the registry.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>Tags associated with the registry.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`RegistrySummary`](crate::model::RegistrySummary).
        pub fn build(self) -> crate::model::RegistrySummary {
            crate::model::RegistrySummary {
                registry_arn: self.registry_arn,
                registry_name: self.registry_name,
                tags: self.tags,
            }
        }
    }
}
impl RegistrySummary {
    /// Creates a new builder-style object to manufacture [`RegistrySummary`](crate::model::RegistrySummary).
    pub fn builder() -> crate::model::registry_summary::Builder {
        crate::model::registry_summary::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DiscovererSummary {
    /// <p>The ARN of the discoverer.</p>
    pub discoverer_arn: std::option::Option<std::string::String>,
    /// <p>The ID of the discoverer.</p>
    pub discoverer_id: std::option::Option<std::string::String>,
    /// <p>The ARN of the event bus.</p>
    pub source_arn: std::option::Option<std::string::String>,
    /// <p>The state of the discoverer.</p>
    pub state: std::option::Option<crate::model::DiscovererState>,
    /// <p>The Status if the discoverer will discover schemas from events sent from another account.</p>
    pub cross_account: bool,
    /// <p>Tags associated with the resource.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl DiscovererSummary {
    /// <p>The ARN of the discoverer.</p>
    pub fn discoverer_arn(&self) -> std::option::Option<&str> {
        self.discoverer_arn.as_deref()
    }
    /// <p>The ID of the discoverer.</p>
    pub fn discoverer_id(&self) -> std::option::Option<&str> {
        self.discoverer_id.as_deref()
    }
    /// <p>The ARN of the event bus.</p>
    pub fn source_arn(&self) -> std::option::Option<&str> {
        self.source_arn.as_deref()
    }
    /// <p>The state of the discoverer.</p>
    pub fn state(&self) -> std::option::Option<&crate::model::DiscovererState> {
        self.state.as_ref()
    }
    /// <p>The Status if the discoverer will discover schemas from events sent from another account.</p>
    pub fn cross_account(&self) -> bool {
        self.cross_account
    }
    /// <p>Tags associated with the resource.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl std::fmt::Debug for DiscovererSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DiscovererSummary");
        formatter.field("discoverer_arn", &self.discoverer_arn);
        formatter.field("discoverer_id", &self.discoverer_id);
        formatter.field("source_arn", &self.source_arn);
        formatter.field("state", &self.state);
        formatter.field("cross_account", &self.cross_account);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`DiscovererSummary`](crate::model::DiscovererSummary).
pub mod discoverer_summary {

    /// A builder for [`DiscovererSummary`](crate::model::DiscovererSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) discoverer_arn: std::option::Option<std::string::String>,
        pub(crate) discoverer_id: std::option::Option<std::string::String>,
        pub(crate) source_arn: std::option::Option<std::string::String>,
        pub(crate) state: std::option::Option<crate::model::DiscovererState>,
        pub(crate) cross_account: std::option::Option<bool>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>The ARN of the discoverer.</p>
        pub fn discoverer_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.discoverer_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the discoverer.</p>
        pub fn set_discoverer_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.discoverer_arn = input;
            self
        }
        /// <p>The ID of the discoverer.</p>
        pub fn discoverer_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.discoverer_id = Some(input.into());
            self
        }
        /// <p>The ID of the discoverer.</p>
        pub fn set_discoverer_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.discoverer_id = input;
            self
        }
        /// <p>The ARN of the event bus.</p>
        pub fn source_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.source_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the event bus.</p>
        pub fn set_source_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.source_arn = input;
            self
        }
        /// <p>The state of the discoverer.</p>
        pub fn state(mut self, input: crate::model::DiscovererState) -> Self {
            self.state = Some(input);
            self
        }
        /// <p>The state of the discoverer.</p>
        pub fn set_state(
            mut self,
            input: std::option::Option<crate::model::DiscovererState>,
        ) -> Self {
            self.state = input;
            self
        }
        /// <p>The Status if the discoverer will discover schemas from events sent from another account.</p>
        pub fn cross_account(mut self, input: bool) -> Self {
            self.cross_account = Some(input);
            self
        }
        /// <p>The Status if the discoverer will discover schemas from events sent from another account.</p>
        pub fn set_cross_account(mut self, input: std::option::Option<bool>) -> Self {
            self.cross_account = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Tags associated with the resource.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>Tags associated with the resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`DiscovererSummary`](crate::model::DiscovererSummary).
        pub fn build(self) -> crate::model::DiscovererSummary {
            crate::model::DiscovererSummary {
                discoverer_arn: self.discoverer_arn,
                discoverer_id: self.discoverer_id,
                source_arn: self.source_arn,
                state: self.state,
                cross_account: self.cross_account.unwrap_or_default(),
                tags: self.tags,
            }
        }
    }
}
impl DiscovererSummary {
    /// Creates a new builder-style object to manufacture [`DiscovererSummary`](crate::model::DiscovererSummary).
    pub fn builder() -> crate::model::discoverer_summary::Builder {
        crate::model::discoverer_summary::Builder::default()
    }
}
