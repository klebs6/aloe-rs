crate::ix!();

/**
  | A fractional ratio integer
  |
  */
pub struct GridFr {
    fraction: u64,
}

impl From<i32> for GridFr {

    fn from(f: i32) -> Self {
    
        todo!();
        /*


            : fraction (static_cast<unsigned long long> (f))
        */
    }
}
    
impl From<u64> for GridFr {

    fn from(p: u64) -> Self {
    
        todo!();
        /*


            : fraction (p)
        */
    }
}

