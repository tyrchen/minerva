// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dataset_field(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DatasetField,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    {
        object.key("type").string(input.r#type.as_str());
    }
    {
        object.key("nullable").boolean(input.nullable);
    }
    Ok(())
}
