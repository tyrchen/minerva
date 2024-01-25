// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub struct QueryDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub data: ::aws_smithy_types::Blob,
}
impl QueryDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn data(&self) -> &::aws_smithy_types::Blob {
        &self.data
    }
}
impl QueryDatasetOutput {
    /// Creates a new builder-style object to manufacture [`QueryDatasetOutput`](crate::output::QueryDatasetOutput).
    pub fn builder() -> crate::output::query_dataset_output::Builder {
        crate::output::query_dataset_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub struct GetDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub name: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub fields: ::std::vec::Vec<crate::model::DatasetField>,
}
impl GetDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn fields(&self) -> &[crate::model::DatasetField] {
        use std::ops::Deref;
        self.fields.deref()
    }
}
impl GetDatasetOutput {
    /// Creates a new builder-style object to manufacture [`GetDatasetOutput`](crate::output::GetDatasetOutput).
    pub fn builder() -> crate::output::get_dataset_output::Builder {
        crate::output::get_dataset_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub struct CreateDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub name: ::std::string::String,
}
impl CreateDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
}
impl CreateDatasetOutput {
    /// Creates a new builder-style object to manufacture [`CreateDatasetOutput`](crate::output::CreateDatasetOutput).
    pub fn builder() -> crate::output::create_dataset_output::Builder {
        crate::output::create_dataset_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub struct ListDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub items: ::std::vec::Vec<crate::model::DatasetInfo>,
    #[allow(missing_docs)] // documentation missing in model
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListDatasetOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn items(&self) -> &[crate::model::DatasetInfo] {
        use std::ops::Deref;
        self.items.deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListDatasetOutput {
    /// Creates a new builder-style object to manufacture [`ListDatasetOutput`](crate::output::ListDatasetOutput).
    pub fn builder() -> crate::output::list_dataset_output::Builder {
        crate::output::list_dataset_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub struct SigninOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub token: ::std::string::String,
}
impl SigninOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn token(&self) -> &str {
        use std::ops::Deref;
        self.token.deref()
    }
}
impl SigninOutput {
    /// Creates a new builder-style object to manufacture [`SigninOutput`](crate::output::SigninOutput).
    pub fn builder() -> crate::output::signin_output::Builder {
        crate::output::signin_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub struct HealthCheckOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::string::String,
}
impl HealthCheckOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(&self) -> &str {
        use std::ops::Deref;
        self.message.deref()
    }
}
impl HealthCheckOutput {
    /// Creates a new builder-style object to manufacture [`HealthCheckOutput`](crate::output::HealthCheckOutput).
    pub fn builder() -> crate::output::health_check_output::Builder {
        crate::output::health_check_output::Builder::default()
    }
}
/// See [`QueryDatasetOutput`](crate::output::QueryDatasetOutput).
///
pub mod query_dataset_output {

    #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
    /// Holds one variant for each of the ways the builder can fail.
    #[non_exhaustive]
    #[allow(clippy::enum_variant_names)]
    pub enum ConstraintViolation {
        /// `data` was not provided but it is required when building `QueryDatasetOutput`.
        MissingData,
    }
    impl ::std::fmt::Display for ConstraintViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConstraintViolation::MissingData => write!(
                    f,
                    "`data` was not provided but it is required when building `QueryDatasetOutput`"
                ),
            }
        }
    }
    impl ::std::error::Error for ConstraintViolation {}
    impl ::std::convert::TryFrom<Builder> for crate::output::QueryDatasetOutput {
        type Error = ConstraintViolation;

        fn try_from(builder: Builder) -> Result<Self, Self::Error> {
            builder.build()
        }
    }
    /// A builder for [`QueryDatasetOutput`](crate::output::QueryDatasetOutput).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) data: ::std::option::Option<::aws_smithy_types::Blob>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn data(mut self, input: ::aws_smithy_types::Blob) -> Self {
            self.data = Some(input);
            self
        }
        /// Consumes the builder and constructs a [`QueryDatasetOutput`](crate::output::QueryDatasetOutput).
        ///
        /// The builder fails to construct a [`QueryDatasetOutput`](crate::output::QueryDatasetOutput) if a [`ConstraintViolation`] occurs.
        ///
        pub fn build(self) -> Result<crate::output::QueryDatasetOutput, ConstraintViolation> {
            self.build_enforcing_all_constraints()
        }
        fn build_enforcing_all_constraints(
            self,
        ) -> Result<crate::output::QueryDatasetOutput, ConstraintViolation> {
            Ok(crate::output::QueryDatasetOutput {
                data: self.data.ok_or(ConstraintViolation::MissingData)?,
            })
        }
    }
}
/// See [`GetDatasetOutput`](crate::output::GetDatasetOutput).
///
pub mod get_dataset_output {

    #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
    /// Holds one variant for each of the ways the builder can fail.
    #[non_exhaustive]
    #[allow(clippy::enum_variant_names)]
    pub enum ConstraintViolation {
        /// `name` was not provided but it is required when building `GetDatasetOutput`.
        MissingName,
        /// `fields` was not provided but it is required when building `GetDatasetOutput`.
        MissingFields,
    }
    impl ::std::fmt::Display for ConstraintViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConstraintViolation::MissingName => write!(
                    f,
                    "`name` was not provided but it is required when building `GetDatasetOutput`"
                ),
                ConstraintViolation::MissingFields => write!(
                    f,
                    "`fields` was not provided but it is required when building `GetDatasetOutput`"
                ),
            }
        }
    }
    impl ::std::error::Error for ConstraintViolation {}
    impl ::std::convert::TryFrom<Builder> for crate::output::GetDatasetOutput {
        type Error = ConstraintViolation;

        fn try_from(builder: Builder) -> Result<Self, Self::Error> {
            builder.build()
        }
    }
    /// A builder for [`GetDatasetOutput`](crate::output::GetDatasetOutput).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: ::std::option::Option<::std::string::String>,
        pub(crate) fields: ::std::option::Option<::std::vec::Vec<crate::model::DatasetField>>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn name(mut self, input: ::std::string::String) -> Self {
            self.name = Some(input);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn fields(mut self, input: ::std::vec::Vec<crate::model::DatasetField>) -> Self {
            self.fields = Some(input);
            self
        }
        /// Consumes the builder and constructs a [`GetDatasetOutput`](crate::output::GetDatasetOutput).
        ///
        /// The builder fails to construct a [`GetDatasetOutput`](crate::output::GetDatasetOutput) if a [`ConstraintViolation`] occurs.
        ///
        /// If the builder fails, it will return the _first_ encountered [`ConstraintViolation`].
        pub fn build(self) -> Result<crate::output::GetDatasetOutput, ConstraintViolation> {
            self.build_enforcing_all_constraints()
        }
        fn build_enforcing_all_constraints(
            self,
        ) -> Result<crate::output::GetDatasetOutput, ConstraintViolation> {
            Ok(crate::output::GetDatasetOutput {
                name: self.name.ok_or(ConstraintViolation::MissingName)?,
                fields: self.fields.ok_or(ConstraintViolation::MissingFields)?,
            })
        }
    }
}
/// See [`CreateDatasetOutput`](crate::output::CreateDatasetOutput).
///
pub mod create_dataset_output {

    #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
    /// Holds one variant for each of the ways the builder can fail.
    #[non_exhaustive]
    #[allow(clippy::enum_variant_names)]
    pub enum ConstraintViolation {
        /// `name` was not provided but it is required when building `CreateDatasetOutput`.
        MissingName,
    }
    impl ::std::fmt::Display for ConstraintViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConstraintViolation::MissingName => write!(f, "`name` was not provided but it is required when building `CreateDatasetOutput`"),
            }
        }
    }
    impl ::std::error::Error for ConstraintViolation {}
    impl ::std::convert::TryFrom<Builder> for crate::output::CreateDatasetOutput {
        type Error = ConstraintViolation;

        fn try_from(builder: Builder) -> Result<Self, Self::Error> {
            builder.build()
        }
    }
    /// A builder for [`CreateDatasetOutput`](crate::output::CreateDatasetOutput).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn name(mut self, input: ::std::string::String) -> Self {
            self.name = Some(input);
            self
        }
        /// Consumes the builder and constructs a [`CreateDatasetOutput`](crate::output::CreateDatasetOutput).
        ///
        /// The builder fails to construct a [`CreateDatasetOutput`](crate::output::CreateDatasetOutput) if a [`ConstraintViolation`] occurs.
        ///
        pub fn build(self) -> Result<crate::output::CreateDatasetOutput, ConstraintViolation> {
            self.build_enforcing_all_constraints()
        }
        fn build_enforcing_all_constraints(
            self,
        ) -> Result<crate::output::CreateDatasetOutput, ConstraintViolation> {
            Ok(crate::output::CreateDatasetOutput {
                name: self.name.ok_or(ConstraintViolation::MissingName)?,
            })
        }
    }
}
/// See [`ListDatasetOutput`](crate::output::ListDatasetOutput).
///
pub mod list_dataset_output {

    #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
    /// Holds one variant for each of the ways the builder can fail.
    #[non_exhaustive]
    #[allow(clippy::enum_variant_names)]
    pub enum ConstraintViolation {
        /// `items` was not provided but it is required when building `ListDatasetOutput`.
        MissingItems,
    }
    impl ::std::fmt::Display for ConstraintViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConstraintViolation::MissingItems => write!(
                    f,
                    "`items` was not provided but it is required when building `ListDatasetOutput`"
                ),
            }
        }
    }
    impl ::std::error::Error for ConstraintViolation {}
    impl ::std::convert::TryFrom<Builder> for crate::output::ListDatasetOutput {
        type Error = ConstraintViolation;

        fn try_from(builder: Builder) -> Result<Self, Self::Error> {
            builder.build()
        }
    }
    /// A builder for [`ListDatasetOutput`](crate::output::ListDatasetOutput).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) items: ::std::option::Option<::std::vec::Vec<crate::model::DatasetInfo>>,
        pub(crate) next_token: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn items(mut self, input: ::std::vec::Vec<crate::model::DatasetInfo>) -> Self {
            self.items = Some(input);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListDatasetOutput`](crate::output::ListDatasetOutput).
        ///
        /// The builder fails to construct a [`ListDatasetOutput`](crate::output::ListDatasetOutput) if a [`ConstraintViolation`] occurs.
        ///
        pub fn build(self) -> Result<crate::output::ListDatasetOutput, ConstraintViolation> {
            self.build_enforcing_all_constraints()
        }
        fn build_enforcing_all_constraints(
            self,
        ) -> Result<crate::output::ListDatasetOutput, ConstraintViolation> {
            Ok(crate::output::ListDatasetOutput {
                items: self.items.ok_or(ConstraintViolation::MissingItems)?,
                next_token: self.next_token,
            })
        }
    }
}
/// See [`SigninOutput`](crate::output::SigninOutput).
///
pub mod signin_output {

    #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
    /// Holds one variant for each of the ways the builder can fail.
    #[non_exhaustive]
    #[allow(clippy::enum_variant_names)]
    pub enum ConstraintViolation {
        /// `token` was not provided but it is required when building `SigninOutput`.
        MissingToken,
    }
    impl ::std::fmt::Display for ConstraintViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConstraintViolation::MissingToken => write!(
                    f,
                    "`token` was not provided but it is required when building `SigninOutput`"
                ),
            }
        }
    }
    impl ::std::error::Error for ConstraintViolation {}
    impl ::std::convert::TryFrom<Builder> for crate::output::SigninOutput {
        type Error = ConstraintViolation;

        fn try_from(builder: Builder) -> Result<Self, Self::Error> {
            builder.build()
        }
    }
    /// A builder for [`SigninOutput`](crate::output::SigninOutput).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) token: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn token(mut self, input: ::std::string::String) -> Self {
            self.token = Some(input);
            self
        }
        /// Consumes the builder and constructs a [`SigninOutput`](crate::output::SigninOutput).
        ///
        /// The builder fails to construct a [`SigninOutput`](crate::output::SigninOutput) if a [`ConstraintViolation`] occurs.
        ///
        pub fn build(self) -> Result<crate::output::SigninOutput, ConstraintViolation> {
            self.build_enforcing_all_constraints()
        }
        fn build_enforcing_all_constraints(
            self,
        ) -> Result<crate::output::SigninOutput, ConstraintViolation> {
            Ok(crate::output::SigninOutput {
                token: self.token.ok_or(ConstraintViolation::MissingToken)?,
            })
        }
    }
}
/// See [`HealthCheckOutput`](crate::output::HealthCheckOutput).
///
pub mod health_check_output {

    #[derive(::std::cmp::PartialEq, ::std::fmt::Debug)]
    /// Holds one variant for each of the ways the builder can fail.
    #[non_exhaustive]
    #[allow(clippy::enum_variant_names)]
    pub enum ConstraintViolation {
        /// `message` was not provided but it is required when building `HealthCheckOutput`.
        MissingMessage,
    }
    impl ::std::fmt::Display for ConstraintViolation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConstraintViolation::MissingMessage => write!(f, "`message` was not provided but it is required when building `HealthCheckOutput`"),
            }
        }
    }
    impl ::std::error::Error for ConstraintViolation {}
    impl ::std::convert::TryFrom<Builder> for crate::output::HealthCheckOutput {
        type Error = ConstraintViolation;

        fn try_from(builder: Builder) -> Result<Self, Self::Error> {
            builder.build()
        }
    }
    /// A builder for [`HealthCheckOutput`](crate::output::HealthCheckOutput).
    #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: ::std::string::String) -> Self {
            self.message = Some(input);
            self
        }
        /// Consumes the builder and constructs a [`HealthCheckOutput`](crate::output::HealthCheckOutput).
        ///
        /// The builder fails to construct a [`HealthCheckOutput`](crate::output::HealthCheckOutput) if a [`ConstraintViolation`] occurs.
        ///
        pub fn build(self) -> Result<crate::output::HealthCheckOutput, ConstraintViolation> {
            self.build_enforcing_all_constraints()
        }
        fn build_enforcing_all_constraints(
            self,
        ) -> Result<crate::output::HealthCheckOutput, ConstraintViolation> {
            Ok(crate::output::HealthCheckOutput {
                message: self.message.ok_or(ConstraintViolation::MissingMessage)?,
            })
        }
    }
}