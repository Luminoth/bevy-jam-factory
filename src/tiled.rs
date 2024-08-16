pub fn require_object_string_property<'a>(
    object: &'a tiled::Object,
    property: impl AsRef<str>,
) -> anyhow::Result<&'a String> {
    let Some(property_value) = object.properties.get(property.as_ref()) else {
        anyhow::bail!(
            "Object {} missing property '{}'",
            object.id(),
            property.as_ref(),
        );
    };

    let tiled::PropertyValue::StringValue(value) = property_value else {
        anyhow::bail!(
            "Object {} has invalid property '{}' {:?}",
            object.id(),
            property.as_ref(),
            property_value,
        );
    };

    Ok(value)
}

pub fn require_object_int_property(
    object: &tiled::Object,
    property: impl AsRef<str>,
) -> anyhow::Result<i32> {
    let Some(property_value) = object.properties.get(property.as_ref()) else {
        anyhow::bail!(
            "Object {} missing property '{}'",
            object.id(),
            property.as_ref(),
        );
    };

    let tiled::PropertyValue::IntValue(value) = property_value else {
        anyhow::bail!(
            "Object {} has invalid property '{}' {:?}",
            object.id(),
            property.as_ref(),
            property_value,
        );
    };

    Ok(*value)
}
