crate::ix!();

pub trait GetQualityOptions {

    fn get_quality_options(&mut self) -> Vec<String>;
}
