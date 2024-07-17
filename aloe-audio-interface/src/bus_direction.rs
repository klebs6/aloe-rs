crate::ix!();

pub trait GetBusDirectionAndIndex {

    fn get_direction_and_index(&self) -> AudioProcessorBusDirectionAndIndex;
}

pub struct AudioProcessorBusDirectionAndIndex
{
    is_input: bool,
    index:    i32,
}
