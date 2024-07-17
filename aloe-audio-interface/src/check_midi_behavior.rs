crate::ix!();

pub trait AcceptsMidi {
    
    fn accepts_midi(&self) -> bool;
}
    
pub trait ProducesMidi {

    fn produces_midi(&self) -> bool;
}
