// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_server_error_error(
    value: &crate::error::ServerError,
) -> Result<String, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_server_error::ser_server_error(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn ser_server_error(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::error::ServerError,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("code").string(input.code.as_str());
    }
    {
        object.key("message").string(input.message.as_str());
    }
    Ok(())
}