// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Structure representing a single entry of additional contextual content
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AdditionalContentEntry {
    /// The name/identifier for this context entry
    pub name: ::std::string::String,
    /// A description of what this context entry represents
    pub description: ::std::string::String,
    /// The actual contextual content
    pub inner_context: ::std::option::Option<::std::string::String>,
}
impl AdditionalContentEntry {
    /// The name/identifier for this context entry
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }

    /// A description of what this context entry represents
    pub fn description(&self) -> &str {
        use std::ops::Deref;
        self.description.deref()
    }

    /// The actual contextual content
    pub fn inner_context(&self) -> ::std::option::Option<&str> {
        self.inner_context.as_deref()
    }
}
impl ::std::fmt::Debug for AdditionalContentEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdditionalContentEntry");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("inner_context", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl AdditionalContentEntry {
    /// Creates a new builder-style object to manufacture
    /// [`AdditionalContentEntry`](crate::types::AdditionalContentEntry).
    pub fn builder() -> crate::types::builders::AdditionalContentEntryBuilder {
        crate::types::builders::AdditionalContentEntryBuilder::default()
    }
}

/// A builder for [`AdditionalContentEntry`](crate::types::AdditionalContentEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct AdditionalContentEntryBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) inner_context: ::std::option::Option<::std::string::String>,
}
impl AdditionalContentEntryBuilder {
    /// The name/identifier for this context entry
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }

    /// The name/identifier for this context entry
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }

    /// The name/identifier for this context entry
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }

    /// A description of what this context entry represents
    /// This field is required.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }

    /// A description of what this context entry represents
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }

    /// A description of what this context entry represents
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }

    /// The actual contextual content
    pub fn inner_context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner_context = ::std::option::Option::Some(input.into());
        self
    }

    /// The actual contextual content
    pub fn set_inner_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner_context = input;
        self
    }

    /// The actual contextual content
    pub fn get_inner_context(&self) -> &::std::option::Option<::std::string::String> {
        &self.inner_context
    }

    /// Consumes the builder and constructs a
    /// [`AdditionalContentEntry`](crate::types::AdditionalContentEntry). This method will fail
    /// if any of the following fields are not set:
    /// - [`name`](crate::types::builders::AdditionalContentEntryBuilder::name)
    /// - [`description`](crate::types::builders::AdditionalContentEntryBuilder::description)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::AdditionalContentEntry, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::types::AdditionalContentEntry {
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building AdditionalContentEntry",
                )
            })?,
            description: self.description.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "description",
                    "description was not specified but it is required when building AdditionalContentEntry",
                )
            })?,
            inner_context: self.inner_context,
        })
    }
}
impl ::std::fmt::Debug for AdditionalContentEntryBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdditionalContentEntryBuilder");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("inner_context", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
