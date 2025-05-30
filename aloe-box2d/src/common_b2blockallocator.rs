crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2BlockAllocator.h]

pub const b2_chunkSize:           usize = 16 * 1024;
pub const b2_maxBlockSize:        usize = 640;
pub const b2_blockSizes:          usize = 14;
pub const b2_chunkArrayIncrement: usize = 128;

pub struct b2Chunk
{
    block_size: i32,
    blocks:     *mut b2Block,
}

pub struct b2Block
{
    next: *mut b2Block,
}

/**
  | This is a small object allocator used for
  | allocating small objects that persist for more
  | than one time step.  See:
  | http://www.codeproject.com/useritems/Small_Block_Allocator.asp
  */
pub struct b2BlockAllocator {
    chunks:      *mut b2Chunk,
    chunk_count: i32,
    chunk_space: i32,
    free_lists:  *mut [b2Block; b2_blockSizes],
}

impl Drop for b2BlockAllocator {
    fn drop(&mut self) {
        todo!();
        /* 
        for (int32 i = 0; i < m_chunkCount; ++i)
        {
            b2Free(m_chunks[i].blocks);
        }

        b2Free(m_chunks);
 */
    }
}

pub mod b2_block_allocator {
    use super::*;

    lazy_static!{
        /*
        static int32 s_blockSizes[b2_blockSizes];
        static uint8 s_blockSizeLookup[b2_maxBlockSize + 1];
        static bool s_blockSizeLookupInitialized;
        int32 b2BlockAllocator::s_blockSizes[b2_blockSizes] =
        {
            16,     // 0
            32,     // 1
            64,     // 2
            96,     // 3
            128,    // 4
            160,    // 5
            192,    // 6
            224,    // 7
            256,    // 8
            320,    // 9
            384,    // 10
            448,    // 11
            512,    // 12
            640,    // 13
        };
        uint8 b2BlockAllocator::s_blockSizeLookup[b2_maxBlockSize + 1];
        bool b2BlockAllocator::s_blockSizeLookupInitialized;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2BlockAllocator.cpp]
impl Default for b2BlockAllocator {

    fn default() -> Self {
    
        todo!();
        /*


            b2Assert(b2_blockSizes < UCHAR_MAX);

        m_chunkSpace = b2_chunkArrayIncrement;
        m_chunkCount = 0;
        m_chunks = (b2Chunk*)b2Alloc(m_chunkSpace * sizeof(b2Chunk));

        memset(m_chunks, 0, m_chunkSpace * sizeof(b2Chunk));
        memset(m_freeLists, 0, sizeof(m_freeLists));

        if (s_blockSizeLookupInitialized == false)
        {
            int32 j = 0;
            for (int32 i = 1; i <= b2_maxBlockSize; ++i)
            {
                b2Assert(j < b2_blockSizes);
                if (i <= s_blockSizes[j])
                {
                    s_blockSizeLookup[i] = (uint8)j;
                }
                else
                {
                    ++j;
                    s_blockSizeLookup[i] = (uint8)j;
                }
            }

            s_blockSizeLookupInitialized = true;
        }
        */
    }
}

impl b2BlockAllocator {
    
    /**
      | Allocate memory. This will use b2Alloc
      | if the size is larger than b2_maxBlockSize.
      |
      */
    pub fn allocate(&mut self, size: i32)  {
        
        todo!();
        /*
            if (size == 0)
            return NULL;

        b2Assert(0 < size);

        if (size > b2_maxBlockSize)
        {
            return b2Alloc(size);
        }

        int32 index = s_blockSizeLookup[size];
        b2Assert(0 <= index && index < b2_blockSizes);

        if (m_freeLists[index])
        {
            b2Block* block = m_freeLists[index];
            m_freeLists[index] = block->next;
            return block;
        }
        else
        {
            if (m_chunkCount == m_chunkSpace)
            {
                b2Chunk* oldChunks = m_chunks;
                m_chunkSpace += b2_chunkArrayIncrement;
                m_chunks = (b2Chunk*)b2Alloc(m_chunkSpace * sizeof(b2Chunk));
                memcpy(m_chunks, oldChunks, m_chunkCount * sizeof(b2Chunk));
                memset(m_chunks + m_chunkCount, 0, b2_chunkArrayIncrement * sizeof(b2Chunk));
                b2Free(oldChunks);
            }

            b2Chunk* chunk = m_chunks + m_chunkCount;
            chunk->blocks = (b2Block*)b2Alloc(b2_chunkSize);
    #if defined(_DEBUG)
            memset(chunk->blocks, 0xcd, b2_chunkSize);
    #endif
            int32 blockSize = s_blockSizes[index];
            chunk->blockSize = blockSize;
            int32 blockCount = b2_chunkSize / blockSize;
            b2Assert(blockCount * blockSize <= b2_chunkSize);
            for (int32 i = 0; i < blockCount - 1; ++i)
            {
                b2Block* block = (b2Block*)((int8*)chunk->blocks + blockSize * i);
                b2Block* next = (b2Block*)((int8*)chunk->blocks + blockSize * (i + 1));
                block->next = next;
            }
            b2Block* last = (b2Block*)((int8*)chunk->blocks + blockSize * (blockCount - 1));
            last->next = NULL;

            m_freeLists[index] = chunk->blocks->next;
            ++m_chunkCount;

            return chunk->blocks;
        }
        */
    }
    
    /**
      | Free memory. This will use b2Free if
      | the size is larger than b2_maxBlockSize.
      |
      */
    pub fn free(
        &mut self, 
        p:    *mut c_void,
        size: i32

    ) {
        
        todo!();
        /*
            if (size == 0)
        {
            return;
        }

        b2Assert(0 < size);

        if (size > b2_maxBlockSize)
        {
            b2Free(p);
            return;
        }

        int32 index = s_blockSizeLookup[size];
        b2Assert(0 <= index && index < b2_blockSizes);

    #ifdef _DEBUG
        // Verify the memory address and size is valid.
        int32 blockSize = s_blockSizes[index];
        bool found = false;
        for (int32 i = 0; i < m_chunkCount; ++i)
        {
            b2Chunk* chunk = m_chunks + i;
            if (chunk->blockSize != blockSize)
            {
                b2Assert(   (int8*)p + blockSize <= (int8*)chunk->blocks ||
                            (int8*)chunk->blocks + b2_chunkSize <= (int8*)p);
            }
            else
            {
                if ((int8*)chunk->blocks <= (int8*)p && (int8*)p + blockSize <= (int8*)chunk->blocks + b2_chunkSize)
                {
                    found = true;
                }
            }
        }

        b2Assert(found);

        memset(p, 0xfd, blockSize);
    #endif

        b2Block* block = (b2Block*)p;
        block->next = m_freeLists[index];
        m_freeLists[index] = block;
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            for (int32 i = 0; i < m_chunkCount; ++i)
        {
            b2Free(m_chunks[i].blocks);
        }

        m_chunkCount = 0;
        memset(m_chunks, 0, m_chunkSpace * sizeof(b2Chunk));

        memset(m_freeLists, 0, sizeof(m_freeLists));
        */
    }
}
