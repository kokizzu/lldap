use lldap_domain::{
    schema::{AttributeList, AttributeSchema, Schema},
    types::AttributeType,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct PublicSchema(Schema);

impl PublicSchema {
    pub fn get_schema(&self) -> &Schema {
        &self.0
    }
}

pub trait SchemaAttributeExtractor: std::marker::Send {
    fn get_attributes(schema: &PublicSchema) -> &AttributeList;
}

pub struct SchemaUserAttributeExtractor;

impl SchemaAttributeExtractor for SchemaUserAttributeExtractor {
    fn get_attributes(schema: &PublicSchema) -> &AttributeList {
        &schema.get_schema().user_attributes
    }
}

pub struct SchemaGroupAttributeExtractor;

impl SchemaAttributeExtractor for SchemaGroupAttributeExtractor {
    fn get_attributes(schema: &PublicSchema) -> &AttributeList {
        &schema.get_schema().group_attributes
    }
}

impl From<Schema> for PublicSchema {
    fn from(mut schema: Schema) -> Self {
        schema.user_attributes.attributes.extend_from_slice(&[
            AttributeSchema {
                name: "user_id".into(),
                attribute_type: AttributeType::String,
                is_list: false,
                is_visible: true,
                is_editable: false,
                is_hardcoded: true,
                is_readonly: true,
            },
            AttributeSchema {
                name: "creation_date".into(),
                attribute_type: AttributeType::DateTime,
                is_list: false,
                is_visible: true,
                is_editable: false,
                is_hardcoded: true,
                is_readonly: true,
            },
            AttributeSchema {
                name: "mail".into(),
                attribute_type: AttributeType::String,
                is_list: false,
                is_visible: true,
                is_editable: true,
                is_hardcoded: true,
                is_readonly: false,
            },
            AttributeSchema {
                name: "uuid".into(),
                attribute_type: AttributeType::String,
                is_list: false,
                is_visible: true,
                is_editable: false,
                is_hardcoded: true,
                is_readonly: true,
            },
            AttributeSchema {
                name: "display_name".into(),
                attribute_type: AttributeType::String,
                is_list: false,
                is_visible: true,
                is_editable: true,
                is_hardcoded: true,
                is_readonly: false,
            },
        ]);
        schema
            .user_attributes
            .attributes
            .sort_by(|a, b| a.name.cmp(&b.name));
        schema.group_attributes.attributes.extend_from_slice(&[
            AttributeSchema {
                name: "group_id".into(),
                attribute_type: AttributeType::Integer,
                is_list: false,
                is_visible: true,
                is_editable: false,
                is_hardcoded: true,
                is_readonly: true,
            },
            AttributeSchema {
                name: "creation_date".into(),
                attribute_type: AttributeType::DateTime,
                is_list: false,
                is_visible: true,
                is_editable: false,
                is_hardcoded: true,
                is_readonly: true,
            },
            AttributeSchema {
                name: "uuid".into(),
                attribute_type: AttributeType::String,
                is_list: false,
                is_visible: true,
                is_editable: false,
                is_hardcoded: true,
                is_readonly: true,
            },
            AttributeSchema {
                name: "display_name".into(),
                attribute_type: AttributeType::String,
                is_list: false,
                is_visible: true,
                is_editable: true,
                is_hardcoded: true,
                is_readonly: false,
            },
        ]);
        schema
            .group_attributes
            .attributes
            .sort_by(|a, b| a.name.cmp(&b.name));
        PublicSchema(schema)
    }
}
