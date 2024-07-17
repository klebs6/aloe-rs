crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2BroadPhase.h]

pub struct b2Pair
{
    proxy_ida: i32,
    proxy_idb: i32,
    next:      i32,
}

pub const B2_BROAD_PHASE_E_NULLPROXY: isize = -1;

/**
  | The broad-phase is used for computing pairs
  | and performing volume queries and ray casts.
  | This broad-phase does not persist
  | pairs. Instead, this reports potentially new
  | pairs.  It is up to the client to consume the
  | new pairs and to track subsequent overlap.
  */
pub struct b2BroadPhase {
    tree:           b2DynamicTree,
    proxy_count:    i32,
    move_buffer:    *mut i32,
    move_capacity:  i32,
    move_count:     i32,
    pair_buffer:    *mut b2Pair,
    pair_capacity:  i32,
    pair_count:     i32,
    query_proxy_id: i32,
}

/**
  | This is used to sort pairs.
  |
  */
#[inline] pub fn b2pair_less_than(
    pair1: &b2Pair,
    pair2: &b2Pair

) -> bool {
    
    todo!();
    /*
        if (pair1.proxyIdA < pair2.proxyIdA)
        {
            return true;
        }

        if (pair1.proxyIdA == pair2.proxyIdA)
        {
            return pair1.proxyIdB < pair2.proxyIdB;
        }

        return false;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2BroadPhase.cpp]
impl Drop for b2BroadPhase {
    fn drop(&mut self) {
        todo!();
        /* 
        b2Free(m_moveBuffer);
        b2Free(m_pairBuffer);
 */
    }
}

impl b2BroadPhase {

    /**
      | Get user data from a proxy. Returns NULL
      | if the id is invalid.
      |
      */
    #[inline] pub fn get_user_data(&self, proxy_id: i32)  {
        
        todo!();
        /*
            return m_tree.GetUserData(proxyId);
        */
    }
    
    /**
      | Test overlap of fat AABBs.
      |
      */
    #[inline] pub fn test_overlap(&self, 
        proxy_ida: i32,
        proxy_idb: i32) -> bool {
        
        todo!();
        /*
            const b2AABB& aabbA = m_tree.GetFatAABB(proxyIdA);
        const b2AABB& aabbB = m_tree.GetFatAABB(proxyIdB);
        return b2TestOverlap(aabbA, aabbB);
        */
    }
    
    /**
      | Get the fat AABB for a proxy.
      |
      */
    #[inline] pub fn get_fataabb(&self, proxy_id: i32) -> &b2AABB {
        
        todo!();
        /*
            return m_tree.GetFatAABB(proxyId);
        */
    }
    
    /**
      | Get the number of proxies.
      |
      */
    #[inline] pub fn get_proxy_count(&self) -> i32 {
        
        todo!();
        /*
            return m_proxyCount;
        */
    }
    
    /**
      | Get the height of the embedded tree.
      |
      */
    #[inline] pub fn get_tree_height(&self) -> i32 {
        
        todo!();
        /*
            return m_tree.GetHeight();
        */
    }
    
    /**
       Get the balance of the embedded tree.
      */
    #[inline] pub fn get_tree_balance(&self) -> i32 {
        
        todo!();
        /*
            return m_tree.GetMaxBalance();
        */
    }
    
    /**
       Get the quality metric of the embedded
       tree.
      */
    #[inline] pub fn get_tree_quality(&self) -> f32 {
        
        todo!();
        /*
            return m_tree.GetAreaRatio();
        */
    }
    
    /**
      | Update the pairs. This results in pair
      | callbacks. This can only add pairs.
      |
      */
    pub fn update_pairs<T>(&mut self, callback: *mut T)  {
    
        todo!();
        /*
            // Reset pair buffer
        m_pairCount = 0;

        // Perform tree queries for all moving proxies.
        for (int32 i = 0; i < m_moveCount; ++i)
        {
            m_queryProxyId = m_moveBuffer[i];
            if (m_queryProxyId == B2_BROAD_PHASE_E_NULLPROXY)
            {
                continue;
            }

            // We have to query the tree with the fat AABB so that
            // we don't fail to create a pair that may touch later.
            const b2AABB& fatAABB = m_tree.GetFatAABB(m_queryProxyId);

            // Query tree, create pairs and add them pair buffer.
            m_tree.Query(this, fatAABB);
        }

        // Reset move buffer
        m_moveCount = 0;

        // Sort the pair buffer to expose duplicates.
        std::sort(m_pairBuffer, m_pairBuffer + m_pairCount, b2PairLessThan);

        // Send the pairs back to the client.
        int32 i = 0;
        while (i < m_pairCount)
        {
            b2Pair* primaryPair = m_pairBuffer + i;
            void* userDataA = m_tree.GetUserData(primaryPair->proxyIdA);
            void* userDataB = m_tree.GetUserData(primaryPair->proxyIdB);

            callback->AddPair(userDataA, userDataB);
            ++i;

            // Skip any duplicate pairs.
            while (i < m_pairCount)
            {
                b2Pair* pair = m_pairBuffer + i;
                if (pair->proxyIdA != primaryPair->proxyIdA || pair->proxyIdB != primaryPair->proxyIdB)
                {
                    break;
                }
                ++i;
            }
        }

        // Try to keep the tree balanced.
        //m_tree.Rebalance(4);
        */
    }
    
    /**
      | Query an AABB for overlapping proxies.
      | The callback class is called for each
      | proxy that overlaps the supplied AABB.
      |
      */
    #[inline] pub fn query<T>(&self, 
        callback: *mut T,
        aabb:     &b2AABB)  {
    
        todo!();
        /*
            m_tree.Query(callback, aabb);
        */
    }
    
    /**
      | Ray-cast against the proxies in the
      | tree. This relies on the callback to
      | perform a exact ray-cast in the case
      | were the proxy contains a shape. The
      | callback also performs the any collision
      | filtering. This has performance roughly
      | equal to k * log(n), where k is the number
      | of collisions and n is the number of proxies
      | in the tree.
      | 
      | -----------
      | @param input
      | 
      | the ray-cast input data. The ray extends
      | from p1 to p1 + maxFraction * (p2 - p1).
      | @param callback a callback class that
      | is called for each proxy that is hit by
      | the ray.
      |
      */
    #[inline] pub fn ray_cast<T>(&self, 
        callback: *mut T,
        input:    &b2RayCastInput)  {
    
        todo!();
        /*
            m_tree.RayCast(callback, input);
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*


            m_proxyCount = 0;

        m_pairCapacity = 16;
        m_pairCount = 0;
        m_pairBuffer = (b2Pair*)b2Alloc(m_pairCapacity * sizeof(b2Pair));

        m_moveCapacity = 16;
        m_moveCount = 0;
        m_moveBuffer = (int32*)b2Alloc(m_moveCapacity * sizeof(int32));
        */
    }
    
    /**
      | Create a proxy with an initial AABB.
      | Pairs are not reported until UpdatePairs
      | is called.
      |
      */
    pub fn create_proxy(&mut self, 
        aabb:      &b2AABB,
        user_data: *mut c_void) -> i32 {
        
        todo!();
        /*
            int32 proxyId = m_tree.CreateProxy(aabb, userData);
        ++m_proxyCount;
        BufferMove(proxyId);
        return proxyId;
        */
    }
    
    /**
      | Destroy a proxy. It is up to the client
      | to remove any pairs.
      |
      */
    pub fn destroy_proxy(&mut self, proxy_id: i32)  {
        
        todo!();
        /*
            UnBufferMove(proxyId);
        --m_proxyCount;
        m_tree.DestroyProxy(proxyId);
        */
    }
    
    /**
      | Call MoveProxy as many times as you like,
      | then when you are done call UpdatePairs to
      | finalized the proxy pairs (for your time
      | step).
      */
    pub fn move_proxy(&mut self, 
        proxy_id:     i32,
        aabb:         &b2AABB,
        displacement: &b2Vec2)  {
        
        todo!();
        /*
            bool buffer = m_tree.MoveProxy(proxyId, aabb, displacement);
        if (buffer)
        {
            BufferMove(proxyId);
        }
        */
    }
    
    /**
      | Call to trigger a re-processing of it's
      | pairs on the next call to UpdatePairs.
      |
      */
    pub fn touch_proxy(&mut self, proxy_id: i32)  {
        
        todo!();
        /*
            BufferMove(proxyId);
        */
    }
    
    pub fn buffer_move(&mut self, proxy_id: i32)  {
        
        todo!();
        /*
            if (m_moveCount == m_moveCapacity)
        {
            int32* oldBuffer = m_moveBuffer;
            m_moveCapacity *= 2;
            m_moveBuffer = (int32*)b2Alloc(m_moveCapacity * sizeof(int32));
            memcpy(m_moveBuffer, oldBuffer, m_moveCount * sizeof(int32));
            b2Free(oldBuffer);
        }

        m_moveBuffer[m_moveCount] = proxyId;
        ++m_moveCount;
        */
    }
    
    pub fn un_buffer_move(&mut self, proxy_id: i32)  {
        
        todo!();
        /*
            for (int32 i = 0; i < m_moveCount; ++i)
        {
            if (m_moveBuffer[i] == proxyId)
            {
                m_moveBuffer[i] = B2_BROAD_PHASE_E_NULLPROXY;
                return;
            }
        }
        */
    }

    /**
       This is called from b2DynamicTree::Query
       when we are gathering pairs.
      */
    pub fn query_callback(&mut self, proxy_id: i32) -> bool {
        
        todo!();
        /*
            // A proxy cannot form a pair with itself.
        if (proxyId == m_queryProxyId)
        {
            return true;
        }

        // Grow the pair buffer as needed.
        if (m_pairCount == m_pairCapacity)
        {
            b2Pair* oldBuffer = m_pairBuffer;
            m_pairCapacity *= 2;
            m_pairBuffer = (b2Pair*)b2Alloc(m_pairCapacity * sizeof(b2Pair));
            memcpy(m_pairBuffer, oldBuffer, m_pairCount * sizeof(b2Pair));
            b2Free(oldBuffer);
        }

        m_pairBuffer[m_pairCount].proxyIdA = b2Min(proxyId, m_queryProxyId);
        m_pairBuffer[m_pairCount].proxyIdB = b2Max(proxyId, m_queryProxyId);
        ++m_pairCount;

        return true;
        */
    }
}
