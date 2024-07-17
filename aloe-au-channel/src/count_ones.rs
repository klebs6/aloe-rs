crate::ix!();

/**
  | counting the one bits in a word
  |
  */
#[inline] pub fn count_ones(x: u32) -> u32 {
    
    todo!();
        /*
            // secret magic algorithm for counting bits in a word.
        UInt32 t;
        x = x - ((x >> 1) & 0x55555555);
        t = ((x >> 2) & 0x33333333);
        x = (x & 0x33333333) + t;
        x = (x + (x >> 4)) & 0x0F0F0F0F;
        x = x + (x << 8);
        x = x + (x << 16);
        return x >> 24;
        */
}
