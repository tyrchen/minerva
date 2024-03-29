// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub async fn de_health_check_http_request<B>(
    #[allow(unused_variables)] request: ::http::Request<B>,
) -> std::result::Result<
    crate::input::HealthCheckInput,
    ::aws_smithy_http_server::protocol::rest_json_1::rejection::RequestRejection,
>
where
    B: ::aws_smithy_http_server::body::HttpBody + Send,
    B::Data: Send,
    ::aws_smithy_http_server::protocol::rest_json_1::rejection::RequestRejection:
        From<<B as ::aws_smithy_http_server::body::HttpBody>::Error>,
{
    Ok({
        #[allow(unused_mut)]
        let mut input = crate::input::health_check_input::Builder::default();
        #[allow(unused_variables)]
        let ::aws_smithy_runtime_api::http::RequestParts {
            uri, headers, body, ..
        } = ::aws_smithy_runtime_api::http::Request::try_from(request)?.into_parts();
        if let Some(value) =
            crate::protocol_serde::shape_health_check_input::de_message_header(&headers)?
        {
            input = input.set_message(value);
        }
        input.build()?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn ser_health_check_http_response(
    #[allow(unused_variables)] output: crate::output::HealthCheckOutput,
) -> std::result::Result<
    ::aws_smithy_http_server::response::Response,
    ::aws_smithy_http_server::protocol::rest_json_1::rejection::ResponseRejection,
> {
    Ok({
        #[allow(unused_mut)]
        let mut builder = ::http::Response::builder();
        builder = ::aws_smithy_http::header::set_response_header_if_absent(
            builder,
            ::http::header::CONTENT_TYPE,
            "application/json",
        );
        let http_status: u16 = 200;
        builder = builder.status(http_status);
        let payload =
            crate::protocol_serde::shape_health_check_output::ser_health_check_output_output_output(&output)?
        ;
        let content_length = payload.len();
        builder = ::aws_smithy_http::header::set_response_header_if_absent(
            builder,
            ::http::header::CONTENT_LENGTH,
            content_length,
        );
        let body = ::aws_smithy_http_server::body::to_boxed(payload);
        builder.body(body)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn ser_health_check_http_error(
    error: &crate::error::HealthCheckError,
) -> std::result::Result<
    ::aws_smithy_http_server::response::Response,
    ::aws_smithy_http_server::protocol::rest_json_1::rejection::ResponseRejection,
> {
    Ok({
        match error {
            crate::error::HealthCheckError::ValidationException(output) => {
                let payload = crate::protocol_serde::shape_validation_exception::ser_validation_exception_error(output)?;
                #[allow(unused_mut)]
                let mut builder = ::http::Response::builder();
                builder = ::aws_smithy_http::header::set_response_header_if_absent(
                    builder,
                    ::http::header::CONTENT_TYPE,
                    "application/json",
                );
                builder = ::aws_smithy_http::header::set_response_header_if_absent(
                    builder,
                    http::header::HeaderName::from_static("x-amzn-errortype"),
                    "ValidationException",
                );
                let content_length = payload.len();
                builder = ::aws_smithy_http::header::set_response_header_if_absent(
                    builder,
                    ::http::header::CONTENT_LENGTH,
                    content_length,
                );
                builder
                    .status(400)
                    .body(::aws_smithy_http_server::body::to_boxed(payload))?
            }
            crate::error::HealthCheckError::ServerError(output) => {
                let payload =
                    crate::protocol_serde::shape_server_error::ser_server_error_error(output)?;
                #[allow(unused_mut)]
                let mut builder = ::http::Response::builder();
                builder = ::aws_smithy_http::header::set_response_header_if_absent(
                    builder,
                    ::http::header::CONTENT_TYPE,
                    "application/json",
                );
                builder = ::aws_smithy_http::header::set_response_header_if_absent(
                    builder,
                    http::header::HeaderName::from_static("x-amzn-errortype"),
                    "ServerError",
                );
                let content_length = payload.len();
                builder = ::aws_smithy_http::header::set_response_header_if_absent(
                    builder,
                    ::http::header::CONTENT_LENGTH,
                    content_length,
                );
                builder
                    .status(500)
                    .body(::aws_smithy_http_server::body::to_boxed(payload))?
            }
        }
    })
}
