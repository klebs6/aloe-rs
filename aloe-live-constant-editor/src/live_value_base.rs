crate::ix!();

pub trait LiveValueBaseInterface {

    fn create_property_component(&mut self, _0: &mut CodeDocument) -> *mut LivePropertyEditorBase;

    fn get_string_value(&self, prefer_hex: bool) -> String;

    fn get_code_value(&self, prefer_hex: bool) -> String;

    fn set_string_value(&mut self, _0: &String);

    fn get_original_string_value(&self, prefer_hex: bool) -> String;

    fn is_string(&self) -> bool;
}

#[no_copy]
pub struct LiveValueBase {
    name:        String,
    source_file: String,
    source_line: i32,
}

impl LiveValueBase {

    pub fn new(
        file: *const u8,
        line: i32) -> Self {
    
        todo!();
        /*
        : source_file(file),
        : source_line(line),

            name = File (sourceFile).getFileName() + " : " + String (sourceLine);
        */
    }
}
