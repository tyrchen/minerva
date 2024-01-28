// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_dataset;

pub(crate) mod shape_get_dataset;

pub(crate) mod shape_health_check;

pub(crate) mod shape_list_dataset;

pub(crate) mod shape_query_dataset;

pub(crate) mod shape_sample_dataset;

pub(crate) mod shape_signin;

pub(crate) mod shape_clickhouse_query_error;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_create_dataset_output;

pub(crate) mod shape_forbidden_error;

pub(crate) mod shape_get_dataset_input;

pub(crate) mod shape_get_dataset_output;

pub(crate) mod shape_health_check_input;

pub(crate) mod shape_health_check_output;

pub(crate) mod shape_list_dataset_input;

pub(crate) mod shape_list_dataset_output;

pub(crate) mod shape_not_found_error;

pub(crate) mod shape_query_dataset_input;

pub(crate) mod shape_query_dataset_output;

pub(crate) mod shape_sample_dataset_input;

pub(crate) mod shape_sample_dataset_output;

pub(crate) mod shape_server_error;

pub(crate) mod shape_signin_output;

pub(crate) mod shape_throttling_error;

pub(crate) mod shape_unauthorized_error;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_dataset_field;

pub(crate) mod shape_dataset_info;

pub(crate) mod shape_validation_exception_field;
