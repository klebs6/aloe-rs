crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2DynamicTree.h]

pub const b2_nullNode: isize = -1;

pub union B2TreeNodeU
{
    parent: i32,
    next:   i32,
}

/**
  | A node in the dynamic tree. The client
  | does not interact with this directly.
  |
  */
pub struct b2TreeNode {

    /**
      | Enlarged AABB
      |
      */
    aabb:      b2AABB,

    user_data: *mut c_void,
    u:         B2TreeNodeU,
    child1:    i32,
    child2:    i32,

    /**
      | leaf = 0, free node = -1
      |
      */
    height:    i32,
}

impl b2TreeNode {

    pub fn is_leaf(&self) -> bool {
        
        todo!();
        /*
            return child1 == b2_nullNode;
        */
    }
}

/**
  | A dynamic AABB tree broad-phase, inspired by
  | Nathanael Presson's btDbvt.
  |
  | A dynamic tree arranges data in a binary tree
  | to accelerate queries such as volume queries
  | and ray casts. Leafs are proxies with an
  | AABB. In the tree we expand the proxy AABB by
  | b2_fatAABBFactor so that the proxy AABB is
  | bigger than the client object. This allows the
  | client object to move by small amounts without
  | triggering a tree update.
  |
  | Nodes are pooled and relocatable, so we use
  | node indices rather than pointers.
  */
pub struct b2DynamicTree {

    root:            i32,
    nodes:           *mut b2TreeNode,
    node_count:      i32,
    node_capacity:   i32,
    free_list:       i32,

    /**
      | This is used to incrementally traverse
      | the tree for re-balancing.
      |
      */
    path:            u32,

    insertion_count: i32,
}

impl Default for b2DynamicTree {
    
    /**
      | Constructing the tree initializes
      | the node pool.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl Drop for b2DynamicTree {

    /**
      | Destroy the tree, freeing the node pool.
      |
      */
    fn drop(&mut self) {
        todo!();
        /* 
        // This frees the entire tree in one shot.
        b2Free(m_nodes);
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Collision/b2DynamicTree.cpp]
impl b2DynamicTree {

    /**
      | Get proxy user data.
      |
      | @return the proxy user data or 0 if the id
      | is invalid.
      */
    #[inline] pub fn get_user_data(&self, proxy_id: i32)  {
        
        todo!();
        /*
            b2Assert(0 <= proxyId && proxyId < m_nodeCapacity);
        return m_nodes[proxyId].userData;
        */
    }
    
    /**
      | Get the fat AABB for a proxy.
      |
      */
    #[inline] pub fn get_fataabb(&self, proxy_id: i32) -> &b2AABB {
        
        todo!();
        /*
            b2Assert(0 <= proxyId && proxyId < m_nodeCapacity);
        return m_nodes[proxyId].aabb;
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
            b2GrowableStack<int32, 256> stack;
        stack.Push(m_root);

        while (stack.GetCount() > 0)
        {
            int32 nodeId = stack.Pop();
            if (nodeId == b2_nullNode)
            {
                continue;
            }

            const b2TreeNode* node = m_nodes + nodeId;

            if (b2TestOverlap(node->aabb, aabb))
            {
                if (node->IsLeaf())
                {
                    bool proceed = callback->QueryCallback(nodeId);
                    if (proceed == false)
                    {
                        return;
                    }
                }
                else
                {
                    stack.Push(node->child1);
                    stack.Push(node->child2);
                }
            }
        }
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
      | ----------
      | @param callback
      | 
      | a callback class that is called for each
      | proxy that is hit by the ray.
      |
      */
    #[inline] pub fn ray_cast<T>(&self, 
        callback: *mut T,
        input:    &b2RayCastInput)  {
    
        todo!();
        /*
            b2Vec2 p1 = input.p1;
        b2Vec2 p2 = input.p2;
        b2Vec2 r = p2 - p1;
        b2Assert(r.LengthSquared() > 0.0f);
        r.Normalize();

        // v is perpendicular to the segment.
        b2Vec2 v = b2Cross(1.0f, r);
        b2Vec2 abs_v = b2Abs(v);

        // Separating axis for segment (Gino, p80).
        // |dot(v, p1 - c)| > dot(|v|, h)

        float32 maxFraction = input.maxFraction;

        // Build a bounding box for the segment.
        b2AABB segmentAABB;
        {
            b2Vec2 t = p1 + maxFraction * (p2 - p1);
            segmentAABB.lowerBound = b2Min(p1, t);
            segmentAABB.upperBound = b2Max(p1, t);
        }

        b2GrowableStack<int32, 256> stack;
        stack.Push(m_root);

        while (stack.GetCount() > 0)
        {
            int32 nodeId = stack.Pop();
            if (nodeId == b2_nullNode)
            {
                continue;
            }

            const b2TreeNode* node = m_nodes + nodeId;

            if (b2TestOverlap(node->aabb, segmentAABB) == false)
            {
                continue;
            }

            // Separating axis for segment (Gino, p80).
            // |dot(v, p1 - c)| > dot(|v|, h)
            b2Vec2 c = node->aabb.GetCenter();
            b2Vec2 h = node->aabb.GetExtents();
            float32 separation = b2Abs(b2Dot(v, p1 - c)) - b2Dot(abs_v, h);
            if (separation > 0.0f)
            {
                continue;
            }

            if (node->IsLeaf())
            {
                b2RayCastInput subInput;
                subInput.p1 = input.p1;
                subInput.p2 = input.p2;
                subInput.maxFraction = maxFraction;

                float32 value = callback->RayCastCallback(subInput, nodeId);

                if (value == 0.0f)
                {
                    // The client has terminated the ray cast.
                    return;
                }

                if (value > 0.0f)
                {
                    // Update segment bounding box.
                    maxFraction = value;
                    b2Vec2 t = p1 + maxFraction * (p2 - p1);
                    segmentAABB.lowerBound = b2Min(p1, t);
                    segmentAABB.upperBound = b2Max(p1, t);
                }
            }
            else
            {
                stack.Push(node->child1);
                stack.Push(node->child2);
            }
        }
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*


            m_root = b2_nullNode;

        m_nodeCapacity = 16;
        m_nodeCount = 0;
        m_nodes = (b2TreeNode*)b2Alloc(m_nodeCapacity * sizeof(b2TreeNode));
        memset((void*)m_nodes, 0, m_nodeCapacity * sizeof(b2TreeNode));

        // Build a linked list for the free list.
        for (int32 i = 0; i < m_nodeCapacity - 1; ++i)
        {
            m_nodes[i].next = i + 1;
            m_nodes[i].height = -1;
        }
        m_nodes[m_nodeCapacity-1].next = b2_nullNode;
        m_nodes[m_nodeCapacity-1].height = -1;
        m_freeList = 0;

        m_path = 0;

        m_insertionCount = 0;
        */
    }

    /**
       Allocate a node from the pool. Grow the
       pool if necessary.
      */
    pub fn allocate_node(&mut self) -> i32 {
        
        todo!();
        /*
            // Expand the node pool as needed.
        if (m_freeList == b2_nullNode)
        {
            b2Assert(m_nodeCount == m_nodeCapacity);

            // The free list is empty. Rebuild a bigger pool.
            b2TreeNode* oldNodes = m_nodes;
            m_nodeCapacity *= 2;
            m_nodes = (b2TreeNode*)b2Alloc(m_nodeCapacity * sizeof(b2TreeNode));
            memcpy(m_nodes, oldNodes, m_nodeCount * sizeof(b2TreeNode));
            b2Free(oldNodes);

            // Build a linked list for the free list. The parent
            // pointer becomes the "next" pointer.
            for (int32 i = m_nodeCount; i < m_nodeCapacity - 1; ++i)
            {
                m_nodes[i].next = i + 1;
                m_nodes[i].height = -1;
            }
            m_nodes[m_nodeCapacity-1].next = b2_nullNode;
            m_nodes[m_nodeCapacity-1].height = -1;
            m_freeList = m_nodeCount;
        }

        // Peel a node off the free list.
        int32 nodeId = m_freeList;
        m_freeList = m_nodes[nodeId].next;
        m_nodes[nodeId].parent = b2_nullNode;
        m_nodes[nodeId].child1 = b2_nullNode;
        m_nodes[nodeId].child2 = b2_nullNode;
        m_nodes[nodeId].height = 0;
        m_nodes[nodeId].userData = NULL;
        ++m_nodeCount;
        return nodeId;
        */
    }

    /**
      | Return a node to the pool.
      |
      */
    pub fn free_node(&mut self, node_id: i32)  {
        
        todo!();
        /*
            b2Assert(0 <= nodeId && nodeId < m_nodeCapacity);
        b2Assert(0 < m_nodeCount);
        m_nodes[nodeId].next = m_freeList;
        m_nodes[nodeId].height = -1;
        m_freeList = nodeId;
        --m_nodeCount;
        */
    }

    /**
      | Create a proxy in the tree as a leaf
      | node. We return the index of the node
      | instead of a pointer so that we can grow
      | the node pool.
      |
      | Create a proxy. Provide a tight fitting
      | AABB and a userData pointer.
      |
      */
    pub fn create_proxy(&mut self, 
        aabb:      &b2AABB,
        user_data: *mut c_void) -> i32 {
        
        todo!();
        /*
            int32 proxyId = AllocateNode();

        // Fatten the aabb.
        b2Vec2 r(b2_aabbExtension, b2_aabbExtension);
        m_nodes[proxyId].aabb.lowerBound = aabb.lowerBound - r;
        m_nodes[proxyId].aabb.upperBound = aabb.upperBound + r;
        m_nodes[proxyId].userData = userData;
        m_nodes[proxyId].height = 0;

        InsertLeaf(proxyId);

        return proxyId;
        */
    }
    
    /**
      | Destroy a proxy. This asserts if the
      | id is invalid.
      |
      */
    pub fn destroy_proxy(&mut self, proxy_id: i32)  {
        
        todo!();
        /*
            b2Assert(0 <= proxyId && proxyId < m_nodeCapacity);
        b2Assert(m_nodes[proxyId].IsLeaf());

        RemoveLeaf(proxyId);
        FreeNode(proxyId);
        */
    }
    
    /**
      | Move a proxy with a swepted AABB. If the
      | proxy has moved outside of its fattened
      | AABB, then the proxy is removed from the
      | tree and re-inserted. Otherwise the
      | function returns immediately.
      |
      | @return true if the proxy was re-inserted.
      */
    pub fn move_proxy(&mut self, 
        proxy_id:     i32,
        aabb:         &b2AABB,
        displacement: &b2Vec2) -> bool {
        
        todo!();
        /*
            b2Assert(0 <= proxyId && proxyId < m_nodeCapacity);

        b2Assert(m_nodes[proxyId].IsLeaf());

        if (m_nodes[proxyId].aabb.Contains(aabb))
        {
            return false;
        }

        RemoveLeaf(proxyId);

        // Extend AABB.
        b2AABB b = aabb;
        b2Vec2 r(b2_aabbExtension, b2_aabbExtension);
        b.lowerBound = b.lowerBound - r;
        b.upperBound = b.upperBound + r;

        // Predict AABB displacement.
        b2Vec2 d = b2_aabbMultiplier * displacement;

        if (d.x < 0.0f)
        {
            b.lowerBound.x += d.x;
        }
        else
        {
            b.upperBound.x += d.x;
        }

        if (d.y < 0.0f)
        {
            b.lowerBound.y += d.y;
        }
        else
        {
            b.upperBound.y += d.y;
        }

        m_nodes[proxyId].aabb = b;

        InsertLeaf(proxyId);
        return true;
        */
    }
    
    pub fn insert_leaf(&mut self, leaf: i32)  {
        
        todo!();
        /*
            ++m_insertionCount;

        if (m_root == b2_nullNode)
        {
            m_root = leaf;
            m_nodes[m_root].parent = b2_nullNode;
            return;
        }

        // Find the best sibling for this node
        b2AABB leafAABB = m_nodes[leaf].aabb;
        int32 index = m_root;
        while (m_nodes[index].IsLeaf() == false)
        {
            int32 child1 = m_nodes[index].child1;
            int32 child2 = m_nodes[index].child2;

            float32 area = m_nodes[index].aabb.GetPerimeter();

            b2AABB combinedAABB;
            combinedAABB.Combine(m_nodes[index].aabb, leafAABB);
            float32 combinedArea = combinedAABB.GetPerimeter();

            // Cost of creating a new parent for this node and the new leaf
            float32 cost = 2.0f * combinedArea;

            // Minimum cost of pushing the leaf further down the tree
            float32 inheritanceCost = 2.0f * (combinedArea - area);

            // Cost of descending into child1
            float32 cost1;
            if (m_nodes[child1].IsLeaf())
            {
                b2AABB aabb;
                aabb.Combine(leafAABB, m_nodes[child1].aabb);
                cost1 = aabb.GetPerimeter() + inheritanceCost;
            }
            else
            {
                b2AABB aabb;
                aabb.Combine(leafAABB, m_nodes[child1].aabb);
                float32 oldArea = m_nodes[child1].aabb.GetPerimeter();
                float32 newArea = aabb.GetPerimeter();
                cost1 = (newArea - oldArea) + inheritanceCost;
            }

            // Cost of descending into child2
            float32 cost2;
            if (m_nodes[child2].IsLeaf())
            {
                b2AABB aabb;
                aabb.Combine(leafAABB, m_nodes[child2].aabb);
                cost2 = aabb.GetPerimeter() + inheritanceCost;
            }
            else
            {
                b2AABB aabb;
                aabb.Combine(leafAABB, m_nodes[child2].aabb);
                float32 oldArea = m_nodes[child2].aabb.GetPerimeter();
                float32 newArea = aabb.GetPerimeter();
                cost2 = newArea - oldArea + inheritanceCost;
            }

            // Descend according to the minimum cost.
            if (cost < cost1 && cost < cost2)
            {
                break;
            }

            // Descend
            if (cost1 < cost2)
            {
                index = child1;
            }
            else
            {
                index = child2;
            }
        }

        int32 sibling = index;

        // Create a new parent.
        int32 oldParent = m_nodes[sibling].parent;
        int32 newParent = AllocateNode();
        m_nodes[newParent].parent = oldParent;
        m_nodes[newParent].userData = NULL;
        m_nodes[newParent].aabb.Combine(leafAABB, m_nodes[sibling].aabb);
        m_nodes[newParent].height = m_nodes[sibling].height + 1;

        if (oldParent != b2_nullNode)
        {
            // The sibling was not the root.
            if (m_nodes[oldParent].child1 == sibling)
            {
                m_nodes[oldParent].child1 = newParent;
            }
            else
            {
                m_nodes[oldParent].child2 = newParent;
            }

            m_nodes[newParent].child1 = sibling;
            m_nodes[newParent].child2 = leaf;
            m_nodes[sibling].parent = newParent;
            m_nodes[leaf].parent = newParent;
        }
        else
        {
            // The sibling was the root.
            m_nodes[newParent].child1 = sibling;
            m_nodes[newParent].child2 = leaf;
            m_nodes[sibling].parent = newParent;
            m_nodes[leaf].parent = newParent;
            m_root = newParent;
        }

        // Walk back up the tree fixing heights and AABBs
        index = m_nodes[leaf].parent;
        while (index != b2_nullNode)
        {
            index = Balance(index);

            int32 child1 = m_nodes[index].child1;
            int32 child2 = m_nodes[index].child2;

            b2Assert(child1 != b2_nullNode);
            b2Assert(child2 != b2_nullNode);

            m_nodes[index].height = 1 + b2Max(m_nodes[child1].height, m_nodes[child2].height);
            m_nodes[index].aabb.Combine(m_nodes[child1].aabb, m_nodes[child2].aabb);

            index = m_nodes[index].parent;
        }

        //Validate();
        */
    }
    
    pub fn remove_leaf(&mut self, leaf: i32)  {
        
        todo!();
        /*
            if (leaf == m_root)
        {
            m_root = b2_nullNode;
            return;
        }

        int32 parent = m_nodes[leaf].parent;
        int32 grandParent = m_nodes[parent].parent;
        int32 sibling;
        if (m_nodes[parent].child1 == leaf)
        {
            sibling = m_nodes[parent].child2;
        }
        else
        {
            sibling = m_nodes[parent].child1;
        }

        if (grandParent != b2_nullNode)
        {
            // Destroy parent and connect sibling to grandParent.
            if (m_nodes[grandParent].child1 == parent)
            {
                m_nodes[grandParent].child1 = sibling;
            }
            else
            {
                m_nodes[grandParent].child2 = sibling;
            }
            m_nodes[sibling].parent = grandParent;
            FreeNode(parent);

            // Adjust ancestor bounds.
            int32 index = grandParent;
            while (index != b2_nullNode)
            {
                index = Balance(index);

                int32 child1 = m_nodes[index].child1;
                int32 child2 = m_nodes[index].child2;

                m_nodes[index].aabb.Combine(m_nodes[child1].aabb, m_nodes[child2].aabb);
                m_nodes[index].height = 1 + b2Max(m_nodes[child1].height, m_nodes[child2].height);

                index = m_nodes[index].parent;
            }
        }
        else
        {
            m_root = sibling;
            m_nodes[sibling].parent = b2_nullNode;
            FreeNode(parent);
        }

        //Validate();
        */
    }

    /**
      | Perform a left or right rotation if node
      | A is imbalanced.
      |
      | Returns the new root index.
      */
    pub fn balance(&mut self, ia: i32) -> i32 {
        
        todo!();
        /*
            b2Assert(iA != b2_nullNode);

        b2TreeNode* A = m_nodes + iA;
        if (A->IsLeaf() || A->height < 2)
        {
            return iA;
        }

        int32 iB = A->child1;
        int32 iC = A->child2;
        b2Assert(0 <= iB && iB < m_nodeCapacity);
        b2Assert(0 <= iC && iC < m_nodeCapacity);

        b2TreeNode* B = m_nodes + iB;
        b2TreeNode* C = m_nodes + iC;

        int32 balance = C->height - B->height;

        // Rotate C up
        if (balance > 1)
        {
            int32 iF = C->child1;
            int32 iG = C->child2;
            b2TreeNode* F = m_nodes + iF;
            b2TreeNode* G = m_nodes + iG;
            b2Assert(0 <= iF && iF < m_nodeCapacity);
            b2Assert(0 <= iG && iG < m_nodeCapacity);

            // Swap A and C
            C->child1 = iA;
            C->parent = A->parent;
            A->parent = iC;

            // A's old parent should point to C
            if (C->parent != b2_nullNode)
            {
                if (m_nodes[C->parent].child1 == iA)
                {
                    m_nodes[C->parent].child1 = iC;
                }
                else
                {
                    b2Assert(m_nodes[C->parent].child2 == iA);
                    m_nodes[C->parent].child2 = iC;
                }
            }
            else
            {
                m_root = iC;
            }

            // Rotate
            if (F->height > G->height)
            {
                C->child2 = iF;
                A->child2 = iG;
                G->parent = iA;
                A->aabb.Combine(B->aabb, G->aabb);
                C->aabb.Combine(A->aabb, F->aabb);

                A->height = 1 + b2Max(B->height, G->height);
                C->height = 1 + b2Max(A->height, F->height);
            }
            else
            {
                C->child2 = iG;
                A->child2 = iF;
                F->parent = iA;
                A->aabb.Combine(B->aabb, F->aabb);
                C->aabb.Combine(A->aabb, G->aabb);

                A->height = 1 + b2Max(B->height, F->height);
                C->height = 1 + b2Max(A->height, G->height);
            }

            return iC;
        }

        // Rotate B up
        if (balance < -1)
        {
            int32 iD = B->child1;
            int32 iE = B->child2;
            b2TreeNode* D = m_nodes + iD;
            b2TreeNode* E = m_nodes + iE;
            b2Assert(0 <= iD && iD < m_nodeCapacity);
            b2Assert(0 <= iE && iE < m_nodeCapacity);

            // Swap A and B
            B->child1 = iA;
            B->parent = A->parent;
            A->parent = iB;

            // A's old parent should point to B
            if (B->parent != b2_nullNode)
            {
                if (m_nodes[B->parent].child1 == iA)
                {
                    m_nodes[B->parent].child1 = iB;
                }
                else
                {
                    b2Assert(m_nodes[B->parent].child2 == iA);
                    m_nodes[B->parent].child2 = iB;
                }
            }
            else
            {
                m_root = iB;
            }

            // Rotate
            if (D->height > E->height)
            {
                B->child2 = iD;
                A->child1 = iE;
                E->parent = iA;
                A->aabb.Combine(C->aabb, E->aabb);
                B->aabb.Combine(A->aabb, D->aabb);

                A->height = 1 + b2Max(C->height, E->height);
                B->height = 1 + b2Max(A->height, D->height);
            }
            else
            {
                B->child2 = iE;
                A->child1 = iD;
                D->parent = iA;
                A->aabb.Combine(C->aabb, D->aabb);
                B->aabb.Combine(A->aabb, E->aabb);

                A->height = 1 + b2Max(C->height, D->height);
                B->height = 1 + b2Max(A->height, E->height);
            }

            return iB;
        }

        return iA;
        */
    }
    
    /**
      | Compute the height of the binary tree
      | in O(N) time. Should not be called often.
      |
      */
    pub fn get_height(&self) -> i32 {
        
        todo!();
        /*
            if (m_root == b2_nullNode)
        {
            return 0;
        }

        return m_nodes[m_root].height;
        */
    }
    
    /**
      | Get the ratio of the sum of the node areas
      | to the root area.
      |
      */
    pub fn get_area_ratio(&self) -> f32 {
        
        todo!();
        /*
            if (m_root == b2_nullNode)
        {
            return 0.0f;
        }

        const b2TreeNode* root = m_nodes + m_root;
        float32 rootArea = root->aabb.GetPerimeter();

        float32 totalArea = 0.0f;
        for (int32 i = 0; i < m_nodeCapacity; ++i)
        {
            const b2TreeNode* node = m_nodes + i;
            if (node->height < 0)
            {
                // Free node in pool
                continue;
            }

            totalArea += node->aabb.GetPerimeter();
        }

        return totalArea / rootArea;
        */
    }

    /**
      | Compute the height of a sub-tree.
      |
      */
    pub fn compute_height_with_nodeid(&self, node_id: i32) -> i32 {
        
        todo!();
        /*
            b2Assert(0 <= nodeId && nodeId < m_nodeCapacity);
        b2TreeNode* node = m_nodes + nodeId;

        if (node->IsLeaf())
        {
            return 0;
        }

        int32 height1 = ComputeHeight(node->child1);
        int32 height2 = ComputeHeight(node->child2);
        return 1 + b2Max(height1, height2);
        */
    }
    
    pub fn compute_height(&self) -> i32 {
        
        todo!();
        /*
            int32 height = ComputeHeight(m_root);
        return height;
        */
    }
    
    pub fn validate_structure(&self, index: i32)  {
        
        todo!();
        /*
            if (index == b2_nullNode)
        {
            return;
        }

        if (index == m_root)
        {
            b2Assert(m_nodes[index].parent == b2_nullNode);
        }

        const b2TreeNode* node = m_nodes + index;

        int32 child1 = node->child1;
        int32 child2 = node->child2;

        if (node->IsLeaf())
        {
            b2Assert(child1 == b2_nullNode);
            b2Assert(child2 == b2_nullNode);
            b2Assert(node->height == 0);
            return;
        }

        b2Assert(0 <= child1 && child1 < m_nodeCapacity);
        b2Assert(0 <= child2 && child2 < m_nodeCapacity);

        b2Assert(m_nodes[child1].parent == index);
        b2Assert(m_nodes[child2].parent == index);

        ValidateStructure(child1);
        ValidateStructure(child2);
        */
    }
    
    pub fn validate_metrics(&self, index: i32)  {
        
        todo!();
        /*
            if (index == b2_nullNode)
        {
            return;
        }

        const b2TreeNode* node = m_nodes + index;

        int32 child1 = node->child1;
        int32 child2 = node->child2;

        if (node->IsLeaf())
        {
            b2Assert(child1 == b2_nullNode);
            b2Assert(child2 == b2_nullNode);
            b2Assert(node->height == 0);
            return;
        }

        b2Assert(0 <= child1 && child1 < m_nodeCapacity);
        b2Assert(0 <= child2 && child2 < m_nodeCapacity);

        int32 height1 = m_nodes[child1].height;
        int32 height2 = m_nodes[child2].height;
        int32 height;
        height = 1 + b2Max(height1, height2);
        b2Assert(node->height == height);
        ignoreUnused (height);

        b2AABB aabb;
        aabb.Combine(m_nodes[child1].aabb, m_nodes[child2].aabb);

        b2Assert(aabb.lowerBound == node->aabb.lowerBound);
        b2Assert(aabb.upperBound == node->aabb.upperBound);

        ValidateMetrics(child1);
        ValidateMetrics(child2);
        */
    }
    
    /**
      | Validate this tree. For testing.
      |
      */
    pub fn validate(&self)  {
        
        todo!();
        /*
            ValidateStructure(m_root);
        ValidateMetrics(m_root);

        int32 freeCount = 0;
        int32 freeIndex = m_freeList;
        while (freeIndex != b2_nullNode)
        {
            b2Assert(0 <= freeIndex && freeIndex < m_nodeCapacity);
            freeIndex = m_nodes[freeIndex].next;
            ++freeCount;
        }

        b2Assert(GetHeight() == ComputeHeight());

        b2Assert(m_nodeCount + freeCount == m_nodeCapacity);
        */
    }
    
    /**
      | Get the maximum balance of an node in
      | the tree. The balance is the difference
      | in height of the two children of a node.
      |
      */
    pub fn get_max_balance(&self) -> i32 {
        
        todo!();
        /*
            int32 maxBalance = 0;
        for (int32 i = 0; i < m_nodeCapacity; ++i)
        {
            const b2TreeNode* node = m_nodes + i;
            if (node->height <= 1)
            {
                continue;
            }

            b2Assert(node->IsLeaf() == false);

            int32 child1 = node->child1;
            int32 child2 = node->child2;
            int32 balance = b2Abs(m_nodes[child2].height - m_nodes[child1].height);
            maxBalance = b2Max(maxBalance, balance);
        }

        return maxBalance;
        */
    }
    
    /**
      | Build an optimal tree. Very expensive.
      | For testing.
      |
      */
    pub fn rebuild_bottom_up(&mut self)  {
        
        todo!();
        /*
            int32* nodes = (int32*)b2Alloc(m_nodeCount * sizeof(int32));
        int32 count = 0;

        // Build array of leaves. Free the rest.
        for (int32 i = 0; i < m_nodeCapacity; ++i)
        {
            if (m_nodes[i].height < 0)
            {
                // free node in pool
                continue;
            }

            if (m_nodes[i].IsLeaf())
            {
                m_nodes[i].parent = b2_nullNode;
                nodes[count] = i;
                ++count;
            }
            else
            {
                FreeNode(i);
            }
        }

        while (count > 1)
        {
            float32 minCost = b2_maxFloat;
            int32 iMin = -1, jMin = -1;
            for (int32 i = 0; i < count; ++i)
            {
                b2AABB aabbi = m_nodes[nodes[i]].aabb;

                for (int32 j = i + 1; j < count; ++j)
                {
                    b2AABB aabbj = m_nodes[nodes[j]].aabb;
                    b2AABB b;
                    b.Combine(aabbi, aabbj);
                    float32 cost = b.GetPerimeter();
                    if (cost < minCost)
                    {
                        iMin = i;
                        jMin = j;
                        minCost = cost;
                    }
                }
            }

            int32 index1 = nodes[iMin];
            int32 index2 = nodes[jMin];
            b2TreeNode* child1 = m_nodes + index1;
            b2TreeNode* child2 = m_nodes + index2;

            int32 parentIndex = AllocateNode();
            b2TreeNode* parent = m_nodes + parentIndex;
            parent->child1 = index1;
            parent->child2 = index2;
            parent->height = 1 + b2Max(child1->height, child2->height);
            parent->aabb.Combine(child1->aabb, child2->aabb);
            parent->parent = b2_nullNode;

            child1->parent = parentIndex;
            child2->parent = parentIndex;

            nodes[jMin] = nodes[count-1];
            nodes[iMin] = parentIndex;
            --count;
        }

        m_root = nodes[0];
        b2Free(nodes);

        Validate();
        */
    }
}
