crate::ix!();

pub mod token_types {

    use super::*;

    macro_rules! aloe_declare_js_token {
        ($name:ident, $str:expr) => {
            /*
                    static const char* const name = str;
            */
        }
    }

    aloe_js_keywords!{
        ALOE_DECLARE_JS_TOKEN
    }

    aloe_js_operators!{
        ALOE_DECLARE_JS_TOKEN
    }

    aloe_declare_js_token!{
        eof,        "$eof"
    }

    aloe_declare_js_token!{
        literal,    "$literal"
    }

    aloe_declare_js_token!{
        identifier, "$identifier"
    }
}

