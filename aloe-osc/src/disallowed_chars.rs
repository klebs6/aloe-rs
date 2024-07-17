crate::ix!();

pub trait GetDisallowedChars {

    fn get_disallowed_chars() -> &'static str;
}

impl GetDisallowedChars for OSCAddress {

    fn get_disallowed_chars() -> &'static str { " #*,?/[]{}" }
}

impl GetDisallowedChars for OSCAddressPattern {

    fn get_disallowed_chars() -> &'static str { " #/" }
}
