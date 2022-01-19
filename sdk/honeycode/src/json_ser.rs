// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_create_table_rows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchCreateTableRowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("clientRequestToken").string(var_1);
    }
    if let Some(var_2) = &input.rows_to_create {
        let mut array_3 = object.key("rowsToCreate").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_create_row_data(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_delete_table_rows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteTableRowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.client_request_token {
        object.key("clientRequestToken").string(var_6);
    }
    if let Some(var_7) = &input.row_ids {
        let mut array_8 = object.key("rowIds").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9);
            }
        }
        array_8.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_update_table_rows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchUpdateTableRowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.client_request_token {
        object.key("clientRequestToken").string(var_10);
    }
    if let Some(var_11) = &input.rows_to_update {
        let mut array_12 = object.key("rowsToUpdate").start_array();
        for item_13 in var_11 {
            {
                let mut object_14 = array_12.value().start_object();
                crate::json_ser::serialize_structure_crate_model_update_row_data(
                    &mut object_14,
                    item_13,
                )?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_upsert_table_rows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchUpsertTableRowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_15) = &input.client_request_token {
        object.key("clientRequestToken").string(var_15);
    }
    if let Some(var_16) = &input.rows_to_upsert {
        let mut array_17 = object.key("rowsToUpsert").start_array();
        for item_18 in var_16 {
            {
                let mut object_19 = array_17.value().start_object();
                crate::json_ser::serialize_structure_crate_model_upsert_row_data(
                    &mut object_19,
                    item_18,
                )?;
                object_19.finish();
            }
        }
        array_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_screen_data_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetScreenDataInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.app_id {
        object.key("appId").string(var_20);
    }
    if let Some(var_21) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_21).into()),
        );
    }
    if let Some(var_22) = &input.next_token {
        object.key("nextToken").string(var_22);
    }
    if let Some(var_23) = &input.screen_id {
        object.key("screenId").string(var_23);
    }
    if let Some(var_24) = &input.variables {
        let mut object_25 = object.key("variables").start_object();
        for (key_26, value_27) in var_24 {
            {
                let mut object_28 = object_25.key(key_26).start_object();
                crate::json_ser::serialize_structure_crate_model_variable_value(
                    &mut object_28,
                    value_27,
                )?;
                object_28.finish();
            }
        }
        object_25.finish();
    }
    if let Some(var_29) = &input.workbook_id {
        object.key("workbookId").string(var_29);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_invoke_screen_automation_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::InvokeScreenAutomationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.client_request_token {
        object.key("clientRequestToken").string(var_30);
    }
    if let Some(var_31) = &input.row_id {
        object.key("rowId").string(var_31);
    }
    if let Some(var_32) = &input.variables {
        let mut object_33 = object.key("variables").start_object();
        for (key_34, value_35) in var_32 {
            {
                let mut object_36 = object_33.key(key_34).start_object();
                crate::json_ser::serialize_structure_crate_model_variable_value(
                    &mut object_36,
                    value_35,
                )?;
                object_36.finish();
            }
        }
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_table_rows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTableRowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    if let Some(var_38) = &input.next_token {
        object.key("nextToken").string(var_38);
    }
    if let Some(var_39) = &input.row_ids {
        let mut array_40 = object.key("rowIds").start_array();
        for item_41 in var_39 {
            {
                array_40.value().string(item_41);
            }
        }
        array_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_query_table_rows_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::QueryTableRowsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.filter_formula {
        let mut object_43 = object.key("filterFormula").start_object();
        crate::json_ser::serialize_structure_crate_model_filter(&mut object_43, var_42)?;
        object_43.finish();
    }
    if let Some(var_44) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_44).into()),
        );
    }
    if let Some(var_45) = &input.next_token {
        object.key("nextToken").string(var_45);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_table_data_import_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartTableDataImportJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.client_request_token {
        object.key("clientRequestToken").string(var_46);
    }
    if let Some(var_47) = &input.data_format {
        object.key("dataFormat").string(var_47.as_str());
    }
    if let Some(var_48) = &input.data_source {
        let mut object_49 = object.key("dataSource").start_object();
        crate::json_ser::serialize_structure_crate_model_import_data_source(
            &mut object_49,
            var_48,
        )?;
        object_49.finish();
    }
    if let Some(var_50) = &input.import_options {
        let mut object_51 = object.key("importOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_import_options(&mut object_51, var_50)?;
        object_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.tags {
        let mut object_53 = object.key("tags").start_object();
        for (key_54, value_55) in var_52 {
            {
                object_53.key(key_54).string(value_55);
            }
        }
        object_53.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_create_row_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateRowData,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.batch_item_id {
        object.key("batchItemId").string(var_56);
    }
    if let Some(var_57) = &input.cells_to_create {
        let mut object_58 = object.key("cellsToCreate").start_object();
        for (key_59, value_60) in var_57 {
            {
                let mut object_61 = object_58.key(key_59).start_object();
                crate::json_ser::serialize_structure_crate_model_cell_input(
                    &mut object_61,
                    value_60,
                )?;
                object_61.finish();
            }
        }
        object_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_row_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateRowData,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.row_id {
        object.key("rowId").string(var_62);
    }
    if let Some(var_63) = &input.cells_to_update {
        let mut object_64 = object.key("cellsToUpdate").start_object();
        for (key_65, value_66) in var_63 {
            {
                let mut object_67 = object_64.key(key_65).start_object();
                crate::json_ser::serialize_structure_crate_model_cell_input(
                    &mut object_67,
                    value_66,
                )?;
                object_67.finish();
            }
        }
        object_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_upsert_row_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpsertRowData,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.batch_item_id {
        object.key("batchItemId").string(var_68);
    }
    if let Some(var_69) = &input.filter {
        let mut object_70 = object.key("filter").start_object();
        crate::json_ser::serialize_structure_crate_model_filter(&mut object_70, var_69)?;
        object_70.finish();
    }
    if let Some(var_71) = &input.cells_to_update {
        let mut object_72 = object.key("cellsToUpdate").start_object();
        for (key_73, value_74) in var_71 {
            {
                let mut object_75 = object_72.key(key_73).start_object();
                crate::json_ser::serialize_structure_crate_model_cell_input(
                    &mut object_75,
                    value_74,
                )?;
                object_75.finish();
            }
        }
        object_72.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_variable_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VariableValue,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.raw_value {
        object.key("rawValue").string(var_76);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_77) = &input.formula {
        object.key("formula").string(var_77);
    }
    if let Some(var_78) = &input.context_row_id {
        object.key("contextRowId").string(var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_import_data_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImportDataSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.data_source_config {
        let mut object_80 = object.key("dataSourceConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_import_data_source_config(
            &mut object_80,
            var_79,
        )?;
        object_80.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_import_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImportOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.destination_options {
        let mut object_82 = object.key("destinationOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_options(
            &mut object_82,
            var_81,
        )?;
        object_82.finish();
    }
    if let Some(var_83) = &input.delimited_text_options {
        let mut object_84 = object.key("delimitedTextOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_delimited_text_import_options(
            &mut object_84,
            var_83,
        )?;
        object_84.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cell_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CellInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.fact {
        object.key("fact").string(var_85);
    }
    if let Some(var_86) = &input.facts {
        let mut array_87 = object.key("facts").start_array();
        for item_88 in var_86 {
            {
                array_87.value().string(item_88);
            }
        }
        array_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_import_data_source_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImportDataSourceConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.data_source_url {
        object.key("dataSourceUrl").string(var_89);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_destination_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DestinationOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.column_map {
        let mut object_91 = object.key("columnMap").start_object();
        for (key_92, value_93) in var_90 {
            {
                let mut object_94 = object_91.key(key_92).start_object();
                crate::json_ser::serialize_structure_crate_model_source_data_column_properties(
                    &mut object_94,
                    value_93,
                )?;
                object_94.finish();
            }
        }
        object_91.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_delimited_text_import_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DelimitedTextImportOptions,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.delimiter {
        object.key("delimiter").string(var_95);
    }
    if input.has_header_row {
        object.key("hasHeaderRow").boolean(input.has_header_row);
    }
    if input.ignore_empty_rows {
        object
            .key("ignoreEmptyRows")
            .boolean(input.ignore_empty_rows);
    }
    if let Some(var_96) = &input.data_character_encoding {
        object.key("dataCharacterEncoding").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source_data_column_properties(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceDataColumnProperties,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.column_index != 0 {
        object.key("columnIndex").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.column_index).into()),
        );
    }
    Ok(())
}
