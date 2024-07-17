crate::ix!();

#[leak_detector]
pub struct ThumbnailCacheEntry {
    hash:      i64,
    last_used: u32,
    data:      MemoryBlock,
}

impl ThumbnailCacheEntry {

    pub fn new_from_hash_code(hash_code: i64) -> Self {
    
        todo!();
        /*


            : hash (hashCode),
              lastUsed (Time::getMillisecondCounter())
        */
    }
    
    pub fn new<R: Read>(in_: &mut R) -> Self {
    
        todo!();
        /*


            : hash (in.readInt64()),
              lastUsed (0)

            const int64 len = in.readInt64();
            in.readIntoMemoryBlock (data, (ssize_t) len);
        */
    }
    
    pub fn write<W: Write>(&mut self, out: &mut W)  {
        
        todo!();
        /*
            out.writeInt64 (hash);
            out.writeInt64 ((int64) data.getSize());
            out << data;
        */
    }
}
