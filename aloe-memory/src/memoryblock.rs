crate::ix!();

pub const base64EncodingTable: &'static str = ".ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+";

pub const base64DecodingTable: &[u8] = &[
    63, 0, 0, 0, 0, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 0, 0, 0, 0, 0, 0, 0,
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    0, 0, 0, 0, 0, 0, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52
];

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_MemoryBlock.h]

/**
  | A class to hold a resizable block of raw
  | data.
  | 
  | @tags{Core}
  |
  */
#[leak_detector]
pub struct MemoryBlock {
    data: <Self as HasHeapBlockType>::HeapBlockType,
    size: usize, // default = 0
}

pub trait HasHeapBlockType {
    type HeapBlockType;
}

impl HasHeapBlockType for MemoryBlock {
    type HeapBlockType = HeapBlock<u8,true>;
}

impl Default for MemoryBlock {
    
    /**
      | Create an uninitialised block with
      | 0 size.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl<Type> IndexMut<Type> for MemoryBlock {
    
    /**
      | Returns a byte from the memory block.
      | 
      | This returns a reference, so you can
      | also use it to set a byte.
      |
      */
    #[inline] fn index_mut(&mut self, offset: Type) -> &mut Self::Output {
        todo!();
        /*
            return data [offset];
        */
    }
}

impl<Type> Index<Type> for MemoryBlock {
    type Output = u8;
    
    /**
      | Returns a byte from the memory block.
      |
      */
    #[inline] fn index(&self, offset: Type) -> &Self::Output {
        todo!();
        /*
            return data [offset];
        */
    }
}

impl PartialEq<MemoryBlock> for MemoryBlock {
    
    /**
      | Compares two memory blocks.
      | 
      | -----------
      | @return
      | 
      | true only if the two blocks are the same
      | size and have identical contents.
      |
      */
    #[inline] fn eq(&self, other: &MemoryBlock) -> bool {
        todo!();
        /*
            return matches (other.data, other.size);
        */
    }
}

impl Eq for MemoryBlock {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/memory/aloe_MemoryBlock.cpp]
impl MemoryBlock {

    /**
      | Returns a void pointer to the data.
      | 
      | -----------
      | @note
      | 
      | the pointer returned will probably
      | become invalid when the block is resized.
      |
      */
    pub fn get_data_mut(&mut self)  {
        
        todo!();
        /*
            return data;
        */
    }

    /**
      | Returns a void pointer to the data.
      | 
      | -----------
      | @note
      | 
      | the pointer returned will probably
      | become invalid when the block is resized.
      |
      */
    pub fn get_data(&self)  {
        
        todo!();
        /*
            return data;
        */
    }

    /**
      | Returns an iterator for the data.
      |
      */
    pub fn begin_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return data;
        */
    }

    /**
      | Returns an iterator for the data.
      |
      */
    pub fn begin(&self) -> *const u8 {
        
        todo!();
        /*
            return data;
        */
    }

    /**
      | Returns an end-iterator for the data.
      |
      */
    pub fn end_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return begin() + getSize();
        */
    }

    /**
      | Returns an end-iterator for the data.
      |
      */
    pub fn end(&self) -> *const u8 {
        
        todo!();
        /*
            return begin() + getSize();
        */
    }

    /**
      | Returns true if the memory block has
      | zero size.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return getSize() == 0;
        */
    }

    /**
      | Returns the block's current allocated
      | size, in bytes.
      |
      */
    pub fn get_size(&self) -> usize {
        
        todo!();
        /*
            return size;
        */
    }

    /**
      | Creates a memory block with a given initial
      | size.
      | 
      | -----------
      | @param initialSize
      | 
      | the size of block to create
      | ----------
      | @param initialiseToZero
      | 
      | whether to clear the memory or just leave
      | it uninitialised
      |
      */
    pub fn new_from_initial_size(
        initial_size:       usize,
        initialise_to_zero: Option<bool>) -> Self {
    
        let initialise_to_zero: bool =
            initialise_to_zero.unwrap_or(false);

        todo!();
        /*


            if (initialSize > 0)
        {
            size = initialSize;
            data.allocate (initialSize, initialiseToZero);
        }
        else
        {
            size = 0;
        }
        */
    }
    
    /**
      | Creates a copy of another memory block.
      |
      */
    pub fn new_from_ref(other: &MemoryBlock) -> Self {
    
        todo!();
        /*
        : size(other.size),

            if (size > 0)
        {
            jassert (other.data != nullptr);
            data.malloc (size);
            memcpy (data, other.data, size);
        }
        */
    }
    
    /**
      | Creates a memory block using a copy of
      | a block of data.
      | 
      | -----------
      | @param dataToInitialiseFrom
      | 
      | some data to copy into this block
      | ----------
      | @param sizeInBytes
      | 
      | how much space to use
      |
      */
    pub fn new_from_raw(
        data_to_initialise_from: *const c_void,
        size_in_bytes:           usize) -> Self {
    
        todo!();
        /*
        : size(sizeInBytes),

            jassert (((ssize_t) sizeInBytes) >= 0);

        if (size > 0)
        {
            jassert (dataToInitialiseFrom != nullptr); // non-zero size, but a zero pointer passed-in?

            data.malloc (size);

            if (dataToInitialiseFrom != nullptr)
                memcpy (data, dataToInitialiseFrom, size);
        }
        */
    }
    
    /**
      | Copies another memory block onto this
      | one.
      | 
      | This block will be resized and copied
      | to exactly match the other one.
      |
      */
    pub fn assign_from_ref(&mut self, other: &MemoryBlock) -> &mut MemoryBlock {
        
        todo!();
        /*
            if (this != &other)
        {
            setSize (other.size, false);
            memcpy (data, other.data, size);
        }

        return *this;
        */
    }
    
    pub fn new_from_other(other: MemoryBlock) -> Self {
    
        todo!();
        /*
        : data(std::move (other.data)),
        : size(other.size),

        
        */
    }
    
    pub fn assign_from(&mut self, other: MemoryBlock) -> &mut MemoryBlock {
        
        todo!();
        /*
            data = std::move (other.data);
        size = other.size;
        return *this;
        */
    }
    
    /**
      | Returns true if the data in this MemoryBlock
      | matches the raw bytes passed-in.
      |
      */
    pub fn matches(&self, 
        data_to_compare: *const c_void,
        data_size:       usize) -> bool {
        
        todo!();
        /*
            return size == dataSize
                && memcmp (data, dataToCompare, size) == 0;
        */
    }

    /**
       this will resize the block to this size
      */
    /**
      | Resizes the memory block.
      | 
      | Any data that is present in both the old
      | and new sizes will be retained. When
      | enlarging the block, the new space that
      | is allocated at the end can either be
      | cleared, or left uninitialised.
      | 
      | -----------
      | @param newSize
      | 
      | the new desired size for the block @param
      | initialiseNewSpaceToZero if the block
      | gets enlarged, this determines whether
      | to clear the new section or just leave
      | it uninitialised @see ensureSize
      |
      */
    pub fn set_size(
        &mut self, 
        new_size:           usize,
        initialise_to_zero: Option<bool>)  {

        let initialise_to_zero: bool = initialise_to_zero.unwrap_or(false);
        
        todo!();
        /*
            if (size != newSize)
        {
            if (newSize <= 0)
            {
                reset();
            }
            else
            {
                if (data != nullptr)
                {
                    data.realloc (newSize);

                    if (initialiseToZero && (newSize > size))
                        zeromem (data + size, newSize - size);
                }
                else
                {
                    data.allocate (newSize, initialiseToZero);
                }

                size = newSize;
            }
        }
        */
    }
    
    /**
      | Frees all the blocks data, setting its
      | size to 0.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            data.free();
        size = 0;
        */
    }
    
    /**
      | Increases the block's size only if it's
      | smaller than a given size.
      | 
      | -----------
      | @param minimumSize
      | 
      | if the block is already bigger than this
      | size, no action will be taken; otherwise
      | it will be increased to this size @param
      | initialiseNewSpaceToZero if the block
      | gets enlarged, this determines whether
      | to clear the new section or just leave
      | it uninitialised @see setSize
      |
      */
    pub fn ensure_size(
        &mut self, 
        minimum_size:       usize,
        initialise_to_zero: Option<bool>)  {

        let initialise_to_zero: bool = initialise_to_zero.unwrap_or(false);
        
        todo!();
        /*
            if (size < minimumSize)
            setSize (minimumSize, initialiseToZero);
        */
    }
    
    /**
      | Exchanges the contents of this and another
      | memory block.
      | 
      | No actual copying is required for this,
      | so it's very fast.
      |
      */
    pub fn swap_with(&mut self, other: &mut MemoryBlock)  {
        
        todo!();
        /*
            std::swap (size, other.size);
        data.swapWith (other.data);
        */
    }
    
    /**
      | Fills the entire memory block with a
      | repeated byte value.
      | 
      | This is handy for clearing a block of
      | memory to zero.
      |
      */
    pub fn fill_with(&mut self, value: u8)  {
        
        todo!();
        /*
            memset (data, (int) value, size);
        */
    }
    
    /**
      | Adds another block of data to the end
      | of this one.
      | 
      | The data pointer must not be null. This
      | block's size will be increased accordingly.
      |
      */
    pub fn append(&mut self, 
        src_data:  *const c_void,
        num_bytes: usize)  {
        
        todo!();
        /*
            if (numBytes > 0)
        {
            jassert (srcData != nullptr); // this must not be null!
            auto oldSize = size;
            setSize (size + numBytes);
            memcpy (data + oldSize, srcData, numBytes);
        }
        */
    }
    
    /**
      | Resizes this block to the given size
      | and fills its contents from the supplied
      | buffer.
      | 
      | The data pointer must not be null.
      |
      */
    pub fn replace_all(&mut self, 
        src_data:  *const c_void,
        num_bytes: usize)  {
        
        todo!();
        /*
            if (numBytes <= 0)
        {
            reset();
            return;
        }

        jassert (srcData != nullptr); // this must not be null!
        setSize (numBytes);
        memcpy (data, srcData, numBytes);
        */
    }
    
    /**
      | Inserts some data into the block.
      | 
      | The dataToInsert pointer must not be
      | null. This block's size will be increased
      | accordingly.
      | 
      | If the insert position lies outside
      | the valid range of the block, it will
      | be clipped to within the range before
      | being used.
      |
      */
    pub fn insert(&mut self, 
        src_data:        *const c_void,
        num_bytes:       usize,
        insert_position: usize)  {
        
        todo!();
        /*
            if (numBytes > 0)
        {
            jassert (srcData != nullptr); // this must not be null!
            insertPosition = jmin (size, insertPosition);
            auto trailingDataSize = size - insertPosition;
            setSize (size + numBytes, false);

            if (trailingDataSize > 0)
                memmove (data + insertPosition + numBytes,
                         data + insertPosition,
                         trailingDataSize);

            memcpy (data + insertPosition, srcData, numBytes);
        }
        */
    }
    
    /**
      | Chops out a section of the block.
      | 
      | This will remove a section of the memory
      | block and close the gap around it, shifting
      | any subsequent data downwards and reducing
      | the size of the block.
      | 
      | If the range specified goes beyond the
      | size of the block, it will be clipped.
      |
      */
    pub fn remove_section(&mut self, 
        start_byte:          usize,
        num_bytes_to_remove: usize)  {
        
        todo!();
        /*
            if (startByte + numBytesToRemove >= size)
        {
            setSize (startByte);
        }
        else if (numBytesToRemove > 0)
        {
            memmove (data + startByte,
                     data + startByte + numBytesToRemove,
                     size - (startByte + numBytesToRemove));

            setSize (size - numBytesToRemove);
        }
        */
    }
    
    /**
      | Copies data into this MemoryBlock from
      | a memory address.
      | 
      | -----------
      | @param srcData
      | 
      | the memory location of the data to copy
      | into this block
      | ----------
      | @param destinationOffset
      | 
      | the offset in this block at which the
      | data being copied should begin
      | ----------
      | @param numBytes
      | 
      | how much to copy in (if this goes beyond
      | the size of the memory block, it will
      | be clipped so not to do anything nasty)
      |
      */
    pub fn copy_from(&mut self, 
        src:    *const c_void,
        offset: i32,
        num:    usize)  {
        
        todo!();
        /*
            auto* d = static_cast<const char*> (src);

        if (offset < 0)
        {
            d -= offset;
            num += (size_t) -offset;
            offset = 0;
        }

        if ((size_t) offset + num > size)
            num = size - (size_t) offset;

        if (num > 0)
            memcpy (data + offset, d, num);
        */
    }

    /**
      | Copies data from this MemoryBlock to
      | a memory address.
      | 
      | -----------
      | @param destData
      | 
      | the memory location to write to
      | ----------
      | @param sourceOffset
      | 
      | the offset within this block from which
      | the copied data will be read
      | ----------
      | @param numBytes
      | 
      | how much to copy (if this extends beyond
      | the limits of the memory block, zeros
      | will be used for that portion of the data)
      |
      */
    pub fn copy_to(&self, 
        dst:    *mut c_void,
        offset: i32,
        num:    usize)  {
        
        todo!();
        /*
            auto* d = static_cast<char*> (dst);

        if (offset < 0)
        {
            zeromem (d, (size_t) -offset);
            d -= offset;
            num -= (size_t) -offset;
            offset = 0;
        }

        if ((size_t) offset + num > size)
        {
            auto newNum = (size_t) size - (size_t) offset;
            zeromem (d + newNum, num - newNum);
            num = newNum;
        }

        if (num > 0)
            memcpy (d, data + offset, num);
        */
    }
    
    /**
      | Attempts to parse the contents of the
      | block as a zero-terminated UTF8 string.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return String::fromUTF8 (data, (int) size);
        */
    }
    
    /**
      | Reads a number of bits from the memory
      | block, treating it as one long binary
      | sequence
      |
      */
    pub fn get_bit_range(&self, 
        bit_range_start: usize,
        num_bits:        usize) -> i32 {
        
        todo!();
        /*
            int res = 0;

        auto byte = bitRangeStart >> 3;
        auto offsetInByte = bitRangeStart & 7;
        size_t bitsSoFar = 0;

        while (numBits > 0 && (size_t) byte < size)
        {
            auto bitsThisTime = jmin (numBits, 8 - offsetInByte);
            const int mask = (0xff >> (8 - bitsThisTime)) << offsetInByte;

            res |= (((data[byte] & mask) >> offsetInByte) << bitsSoFar);

            bitsSoFar += bitsThisTime;
            numBits -= bitsThisTime;
            ++byte;
            offsetInByte = 0;
        }

        return res;
        */
    }
    
    /**
      | Sets a number of bits in the memory block,
      | treating it as a long binary sequence.
      |
      */
    pub fn set_bit_range(&mut self, 
        bit_range_start: usize,
        num_bits:        usize,
        bits_to_set:     i32)  {
        
        todo!();
        /*
            auto byte = bitRangeStart >> 3;
        auto offsetInByte = bitRangeStart & 7;
        uint32 mask = ~((((uint32) 0xffffffff) << (32 - numBits)) >> (32 - numBits));

        while (numBits > 0 && (size_t) byte < size)
        {
            auto bitsThisTime = jmin (numBits, 8 - offsetInByte);

            const uint32 tempMask = (mask << offsetInByte) | ~((((uint32) 0xffffffff) >> offsetInByte) << offsetInByte);
            const uint32 tempBits = (uint32) bitsToSet << offsetInByte;

            data[byte] = (char) (((uint32) data[byte] & tempMask) | tempBits);

            ++byte;
            numBits -= bitsThisTime;
            bitsToSet >>= bitsThisTime;
            mask >>= bitsThisTime;
            offsetInByte = 0;
        }
        */
    }
    
    /**
      | Parses a string of hexadecimal numbers
      | and writes this data into the memory
      | block.
      | 
      | The block will be resized to the number
      | of valid bytes read from the string.
      | 
      | Non-hex characters in the string will
      | be ignored.
      | 
      | @see String::toHexString()
      |
      */
    pub fn load_from_hex_string(&mut self, hex: &str)  {
        
        todo!();
        /*
            ensureSize ((size_t) hex.length() >> 1);
        char* dest = data;
        auto t = hex.text;

        for (;;)
        {
            aloe_wchar byte = 0;

            for (int loop = 2; --loop >= 0;)
            {
                byte <<= 4;

                for (;;)
                {
                    auto c = t.getAndAdvance();

                    if (c >= '0' && c <= '9')    { byte |= c - '0';        break; }
                    if (c >= 'a' && c <= 'z')    { byte |= c - ('a' - 10); break; }
                    if (c >= 'A' && c <= 'Z')    { byte |= c - ('A' - 10); break; }

                    if (c == 0)
                    {
                        setSize (static_cast<size_t> (dest - data));
                        return;
                    }
                }
            }

            *dest++ = (char) byte;
        }
        */
    }
    
    /**
      | Returns a string of characters in a Aloe-specific
      | text encoding that represents the binary
      | contents of this block.
      | 
      | This uses a Aloe-specific (i.e. not
      | standard!) 64-bit encoding system
      | to convert binary data into a string
      | of ASCII characters for purposes like
      | storage in XML.
      | 
      | -----------
      | @note
      | 
      | this proprietary format is mainly kept
      | here for backwards-compatibility,
      | and you may prefer to use the Base64::toBase64()
      | method if you want to use the standard
      | base-64 encoding.
      | 
      | @see fromBase64Encoding, Base64::toBase64,
      | Base64::convertToBase64
      |
      */
    pub fn to_base_64encoding(&self) -> String {
        
        todo!();
        /*
            auto numChars = ((size << 3) + 5) / 6;

        String destString ((unsigned int) size); // store the length, followed by a '.', and then the data.
        auto initialLen = destString.length();
        destString.preallocateBytes ((size_t) initialLen * sizeof (CharPointerType::CharType) + 2 + numChars);

        auto d = destString.getCharPointer();
        d += initialLen;
        d.write ('.');

        for (size_t i = 0; i < numChars; ++i)
            d.write ((aloe_wchar) (uint8) base64EncodingTable[getBitRange (i * 6, 6)]);

        d.writeNull();
        return destString;
        */
    }
    
    /**
      | Takes a string created by MemoryBlock::toBase64Encoding()
      | and extracts the original data.
      | 
      | The string passed in must have been created
      | by to64BitEncoding(), and this block
      | will be resized to recreate the original
      | data block.
      | 
      | -----------
      | @note
      | 
      | these methods use a Aloe-specific (i.e.
      | not standard!) 64-bit encoding system.
      | 
      | You may prefer to use the Base64::convertFromBase64()
      | method if you want to use the standard
      | base-64 encoding.
      | 
      | @see toBase64Encoding, Base64::convertFromBase64
      |
      */
    pub fn from_base_64encoding(&mut self, s: &str) -> bool {
        
        todo!();
        /*
            auto dot = CharacterFunctions::find (s.text, (aloe_wchar) '.');

        if (dot.isEmpty())
            return false;

        auto numBytesNeeded = String (s.text, dot).getIntValue();

        setSize ((size_t) numBytesNeeded, true);

        auto srcChars = dot + 1;
        int pos = 0;

        for (;;)
        {
            auto c = (int) srcChars.getAndAdvance();

            if (c == 0)
                return true;

            c -= 43;

            if (isPositiveAndBelow (c, numElementsInArray (base64DecodingTable)))
            {
                setBitRange ((size_t) pos, 6, base64DecodingTable[c]);
                pos += 6;
            }
        }
        */
    }
}
