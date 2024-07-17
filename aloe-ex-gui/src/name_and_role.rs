crate::ix!();

pub struct NameAndRole
{
    name: *const u8,
    role: AccessibilityRole,
}

lazy_static!{
    /*
    constexpr NameAndRole accessibilityRoles[]
    {
        { "Ignored",       AccessibilityRole::ignored },
        { "Unspecified",   AccessibilityRole::unspecified },
        { "Button",        AccessibilityRole::button },
        { "ComboBox",      AccessibilityRole::comboBox },
        { "Slider",        AccessibilityRole::slider },
        { "Static Text",   AccessibilityRole::staticText },
        { "Editable Text", AccessibilityRole::editableText },
        { "Image",         AccessibilityRole::image },
        { "Group",         AccessibilityRole::group },
        { "Window",        AccessibilityRole::window }
    };
    */
}
