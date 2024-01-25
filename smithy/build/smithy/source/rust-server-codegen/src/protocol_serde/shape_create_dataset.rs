// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub async fn de_create_dataset_http_request<B>(
    #[allow(unused_variables)] request: ::http::Request<B>,
) -> std::result::Result<
    crate::input::CreateDatasetInput,
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
        let mut input = crate::input::create_dataset_input::Builder::default();
        #[allow(unused_variables)]
        let ::aws_smithy_runtime_api::http::RequestParts {
            uri, headers, body, ..
        } = ::aws_smithy_runtime_api::http::Request::try_from(request)?.into_parts();
        let bytes = ::hyper::body::to_bytes(body).await?;
        if !bytes.is_empty() {
            ::aws_smithy_http_server::protocol::content_type_header_classifier_smithy(
                &headers,
                Some("application/json"),
            )?;
            input = crate::protocol_serde::shape_create_dataset::de_create_dataset(
                bytes.as_ref(),
                input,
            )?;
        }
        input.build()?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn ser_create_dataset_http_response(
    #[allow(unused_variables)] output: crate::output::CreateDatasetOutput,
) -> std::result::Result<
    ::aws_smithy_http_server::response::Response,
    ::aws_smithy_http_server::protocol::rest_json_1::rejection::ResponseRejection,
> {
    Ok({
        #[allow(unused_mut)]
        let mut builder = ::http::Response::builder();
        builder = crate::protocol_serde::shape_create_dataset::ser_create_dataset_headers(
            &output, builder,
        )?;
        builder = ::aws_smithy_http::header::set_response_header_if_absent(
            builder,
            ::http::header::CONTENT_TYPE,
            "application/json",
        );
        let http_status: u16 = 201;
        builder = builder.status(http_status);
        let payload =
            crate::protocol_serde::shape_create_dataset_output::ser_create_dataset_output_output_output(&output)?
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
pub fn ser_create_dataset_http_error(
    error: &crate::error::CreateDatasetError,
) -> std::result::Result<
    ::aws_smithy_http_server::response::Response,
    ::aws_smithy_http_server::protocol::rest_json_1::rejection::ResponseRejection,
> {
    Ok({
        match error {
            crate::error::CreateDatasetError::ValidationException(output) => {
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
            crate::error::CreateDatasetError::ThrottlingError(output) => {
                let payload =
                    crate::protocol_serde::shape_throttling_error::ser_throttling_error_error(
                        output,
                    )?;
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
                    "ThrottlingError",
                );
                let content_length = payload.len();
                builder = ::aws_smithy_http::header::set_response_header_if_absent(
                    builder,
                    ::http::header::CONTENT_LENGTH,
                    content_length,
                );
                builder
                    .status(429)
                    .body(::aws_smithy_http_server::body::to_boxed(payload))?
            }
            crate::error::CreateDatasetError::ServerError(output) => {
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

pub(crate) fn de_create_dataset(
    value: &[u8],
    mut builder: crate::input::create_dataset_input::Builder,
) -> Result<
    crate::input::create_dataset_input::Builder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "name" => {
                        if let Some(v) =
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?
                        {
                            builder = builder.set_name(v);
                        }
                    }
                    "sql" => {
                        if let Some(v) =
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?
                        {
                            builder = builder.set_sql(v);
                        }
                    }
                    _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}

pub fn ser_create_dataset_headers(
    input: &crate::output::CreateDatasetOutput,
    mut builder: ::http::response::Builder,
) -> std::result::Result<::http::response::Builder, ::aws_smithy_types::error::operation::BuildError>
{
    {
        let formatted_1 = &input.name.as_str();
        if !formatted_1.is_empty() {
            let header_value = formatted_1;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "name",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-dataset-name", header_value);
        }
    }
    Ok(builder)
}
