#[allow(dead_code)]
pub fn require_object_string_property<'a>(
    object: &'a tiled::Object,
    property: impl AsRef<str>,
) -> &'a String {
    let property_value = object.properties.get(property.as_ref()).unwrap_or_else(|| {
        panic!(
            "Object {} missing property '{}'",
            object.id(),
            property.as_ref(),
        )
    });

    let tiled::PropertyValue::StringValue(value) = property_value else {
        panic!(
            "Object {} has invalid property '{}' {:?}",
            object.id(),
            property.as_ref(),
            property_value,
        );
    };

    value
}
