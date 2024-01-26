// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
pub struct SampleDataset;

impl ::aws_smithy_http_server::operation::OperationShape for SampleDataset {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.minerva#SampleDataset",
            "com.minerva",
            "SampleDataset",
        );

    type Input = crate::input::SampleDatasetInput;
    type Output = crate::output::SampleDatasetOutput;
    type Error = crate::error::SampleDatasetError;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for SampleDataset {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}

#[allow(missing_docs)] // documentation missing in model
pub struct QueryDataset;

impl ::aws_smithy_http_server::operation::OperationShape for QueryDataset {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.minerva#QueryDataset",
            "com.minerva",
            "QueryDataset",
        );

    type Input = crate::input::QueryDatasetInput;
    type Output = crate::output::QueryDatasetOutput;
    type Error = crate::error::QueryDatasetError;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for QueryDataset {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}

#[allow(missing_docs)] // documentation missing in model
pub struct GetDataset;

impl ::aws_smithy_http_server::operation::OperationShape for GetDataset {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.minerva#GetDataset",
            "com.minerva",
            "GetDataset",
        );

    type Input = crate::input::GetDatasetInput;
    type Output = crate::output::GetDatasetOutput;
    type Error = crate::error::GetDatasetError;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for GetDataset {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}

#[allow(missing_docs)] // documentation missing in model
pub struct CreateDataset;

impl ::aws_smithy_http_server::operation::OperationShape for CreateDataset {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.minerva#CreateDataset",
            "com.minerva",
            "CreateDataset",
        );

    type Input = crate::input::CreateDatasetInput;
    type Output = crate::output::CreateDatasetOutput;
    type Error = crate::error::CreateDatasetError;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for CreateDataset {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}

#[allow(missing_docs)] // documentation missing in model
pub struct ListDataset;

impl ::aws_smithy_http_server::operation::OperationShape for ListDataset {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.minerva#ListDataset",
            "com.minerva",
            "ListDataset",
        );

    type Input = crate::input::ListDatasetInput;
    type Output = crate::output::ListDatasetOutput;
    type Error = crate::error::ListDatasetError;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for ListDataset {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}

/// Signin to get a token.
pub struct Signin;

impl ::aws_smithy_http_server::operation::OperationShape for Signin {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.minerva#Signin",
            "com.minerva",
            "Signin",
        );

    type Input = crate::input::SigninInput;
    type Output = crate::output::SigninOutput;
    type Error = crate::error::SigninError;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for Signin {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}

#[allow(missing_docs)] // documentation missing in model
pub struct HealthCheck;

impl ::aws_smithy_http_server::operation::OperationShape for HealthCheck {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.minerva#HealthCheck",
            "com.minerva",
            "HealthCheck",
        );

    type Input = crate::input::HealthCheckInput;
    type Output = crate::output::HealthCheckOutput;
    type Error = crate::error::HealthCheckError;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for HealthCheck {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}
