// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Citation associated with the agent response
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Citation {
    /// Generate response part
    pub generated_response_part: ::std::option::Option<crate::types::GeneratedResponsePart>,
    /// list of retrieved references
    pub retrieved_references: ::std::option::Option<::std::vec::Vec<crate::types::RetrievedReference>>,
}
impl Citation {
    /// Generate response part
    pub fn generated_response_part(&self) -> ::std::option::Option<&crate::types::GeneratedResponsePart> {
        self.generated_response_part.as_ref()
    }
    /// list of retrieved references
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.retrieved_references.is_none()`.
    pub fn retrieved_references(&self) -> &[crate::types::RetrievedReference] {
        self.retrieved_references.as_deref().unwrap_or_default()
    }
}
impl Citation {
    /// Creates a new builder-style object to manufacture [`Citation`](crate::types::Citation).
    pub fn builder() -> crate::types::builders::CitationBuilder {
        crate::types::builders::CitationBuilder::default()
    }
}

/// A builder for [`Citation`](crate::types::Citation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CitationBuilder {
    pub(crate) generated_response_part: ::std::option::Option<crate::types::GeneratedResponsePart>,
    pub(crate) retrieved_references: ::std::option::Option<::std::vec::Vec<crate::types::RetrievedReference>>,
}
impl CitationBuilder {
    /// Generate response part
    pub fn generated_response_part(mut self, input: crate::types::GeneratedResponsePart) -> Self {
        self.generated_response_part = ::std::option::Option::Some(input);
        self
    }
    /// Generate response part
    pub fn set_generated_response_part(mut self, input: ::std::option::Option<crate::types::GeneratedResponsePart>) -> Self {
        self.generated_response_part = input;
        self
    }
    /// Generate response part
    pub fn get_generated_response_part(&self) -> &::std::option::Option<crate::types::GeneratedResponsePart> {
        &self.generated_response_part
    }
    /// Appends an item to `retrieved_references`.
    ///
    /// To override the contents of this collection use [`set_retrieved_references`](Self::set_retrieved_references).
    ///
    /// list of retrieved references
    pub fn retrieved_references(mut self, input: crate::types::RetrievedReference) -> Self {
        let mut v = self.retrieved_references.unwrap_or_default();
        v.push(input);
        self.retrieved_references = ::std::option::Option::Some(v);
        self
    }
    /// list of retrieved references
    pub fn set_retrieved_references(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RetrievedReference>>) -> Self {
        self.retrieved_references = input;
        self
    }
    /// list of retrieved references
    pub fn get_retrieved_references(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RetrievedReference>> {
        &self.retrieved_references
    }
    /// Consumes the builder and constructs a [`Citation`](crate::types::Citation).
    pub fn build(self) -> crate::types::Citation {
        crate::types::Citation {
            generated_response_part: self.generated_response_part,
            retrieved_references: self.retrieved_references,
        }
    }
}
