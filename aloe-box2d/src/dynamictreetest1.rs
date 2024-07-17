crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/DynamicTreeTest.h]
pub const DYNAMIC_TREE_TEST_ACTOR_COUNT: usize = 128;

pub struct DynamicTreeTestActor
{
    aabb:     b2AABB,
    fraction: f32,
    overlap:  bool,
    proxy_id: i32,
}

pub struct DynamicTreeTest {
    base:            Test,
    world_extent:    f32,
    proxy_extent:    f32,
    tree:            b2DynamicTree,
    queryaabb:       b2AABB,
    ray_cast_input:  b2RayCastInput,
    ray_cast_output: b2RayCastOutput,
    ray_actor:       *mut DynamicTreeTestActor,
    actors:          [DynamicTreeTestActor; DYNAMIC_TREE_TEST_ACTOR_COUNT],
    step_count:      i32,
    automated:       bool,
}

impl Default for DynamicTreeTest {
    
    fn default() -> Self {
        todo!();
        /*


            m_worldExtent = 15.0f;
            m_proxyExtent = 0.5f;

            srand(888);

            for (int32 i = 0; i < e_actorCount; ++i)
            {
                DynamicTreeTestActor* actor = m_actors + i;
                GetRandomAABB(&actor->aabb);
                actor->proxyId = m_tree.CreateProxy(actor->aabb, actor);
            }

            m_stepCount = 0;

            float32 h = m_worldExtent;
            m_queryAABB.lowerBound.Set(-3.0f, -4.0f + h);
            m_queryAABB.upperBound.Set(5.0f, 6.0f + h);

            m_rayCastInput.p1.Set(-5.0, 5.0f + h);
            m_rayCastInput.p2.Set(7.0f, -4.0f + h);
            //m_rayCastInput.p1.Set(0.0f, 2.0f + h);
            //m_rayCastInput.p2.Set(0.0f, -2.0f + h);
            m_rayCastInput.maxFraction = 1.0f;

            m_automated = false
        */
    }
}

impl DynamicTreeTest {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new DynamicTreeTest;
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            B2_NOT_USED(settings);

            m_rayActor = NULL;
            for (int32 i = 0; i < e_actorCount; ++i)
            {
                m_actors[i].fraction = 1.0f;
                m_actors[i].overlap = false;
            }

            if (m_automated == true)
            {
                int32 actionCount = b2Max(1, e_actorCount >> 2);

                for (int32 i = 0; i < actionCount; ++i)
                {
                    Action();
                }
            }

            Query();
            RayCast();

            for (int32 i = 0; i < e_actorCount; ++i)
            {
                DynamicTreeTestActor* actor = m_actors + i;
                if (actor->proxyId == b2_nullNode)
                    continue;

                b2Color c(0.9f, 0.9f, 0.9f);
                if (actor == m_rayActor && actor->overlap)
                {
                    c.Set(0.9f, 0.6f, 0.6f);
                }
                else if (actor == m_rayActor)
                {
                    c.Set(0.6f, 0.9f, 0.6f);
                }
                else if (actor->overlap)
                {
                    c.Set(0.6f, 0.6f, 0.9f);
                }

                m_debugDraw.DrawAABB(&actor->aabb, c);
            }

            b2Color c(0.7f, 0.7f, 0.7f);
            m_debugDraw.DrawAABB(&m_queryAABB, c);

            m_debugDraw.DrawSegment(m_rayCastInput.p1, m_rayCastInput.p2, c);

            b2Color c1(0.2f, 0.9f, 0.2f);
            b2Color c2(0.9f, 0.2f, 0.2f);
            m_debugDraw.DrawPoint(m_rayCastInput.p1, 6.0f, c1);
            m_debugDraw.DrawPoint(m_rayCastInput.p2, 6.0f, c2);

            if (m_rayActor)
            {
                b2Color cr(0.2f, 0.2f, 0.9f);
                b2Vec2 p = m_rayCastInput.p1 + m_rayActor->fraction * (m_rayCastInput.p2 - m_rayCastInput.p1);
                m_debugDraw.DrawPoint(p, 6.0f, cr);
            }

            {
                int32 height = m_tree.GetHeight();
                m_debugDraw.DrawString(5, m_textLine, "dynamic tree height = %d", height);
                m_textLine += 15;
            }

            ++m_stepCount;
        */
    }
    
    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'a':
                m_automated = !m_automated;
                break;

            case 'c':
                CreateProxy();
                break;

            case 'd':
                DestroyProxy();
                break;

            case 'm':
                MoveProxy();
                break;
            }
        */
    }
    
    pub fn query_callback(&mut self, proxy_id: i32) -> bool {
        
        todo!();
        /*
            DynamicTreeTestActor* actor = (DynamicTreeTestActor*)m_tree.GetUserData(proxyId);
            actor->overlap = b2TestOverlap(m_queryAABB, actor->aabb);
            return true;
        */
    }
    
    pub fn ray_cast_callback(&mut self, 
        input:    &b2RayCastInput,
        proxy_id: i32) -> f32 {
        
        todo!();
        /*
            DynamicTreeTestActor* actor = (DynamicTreeTestActor*)m_tree.GetUserData(proxyId);

            b2RayCastOutput output;
            bool hit = actor->aabb.RayCast(&output, input);

            if (hit)
            {
                m_rayCastOutput = output;
                m_rayActor = actor;
                m_rayActor->fraction = output.fraction;
                return output.fraction;
            }

            return input.maxFraction;
        */
    }
    
    pub fn get_randomaabb(&mut self, aabb: *mut b2AABB)  {
        
        todo!();
        /*
            b2Vec2 w; w.Set(2.0f * m_proxyExtent, 2.0f * m_proxyExtent);
            //aabb->lowerBound.x = -m_proxyExtent;
            //aabb->lowerBound.y = -m_proxyExtent + m_worldExtent;
            aabb->lowerBound.x = RandomFloat(-m_worldExtent, m_worldExtent);
            aabb->lowerBound.y = RandomFloat(0.0f, 2.0f * m_worldExtent);
            aabb->upperBound = aabb->lowerBound + w;
        */
    }
    
    pub fn moveaabb(&mut self, aabb: *mut b2AABB)  {
        
        todo!();
        /*
            b2Vec2 d;
            d.x = RandomFloat(-0.5f, 0.5f);
            d.y = RandomFloat(-0.5f, 0.5f);
            //d.x = 2.0f;
            //d.y = 0.0f;
            aabb->lowerBound += d;
            aabb->upperBound += d;

            b2Vec2 c0 = 0.5f * (aabb->lowerBound + aabb->upperBound);
            b2Vec2 min; min.Set(-m_worldExtent, 0.0f);
            b2Vec2 max; max.Set(m_worldExtent, 2.0f * m_worldExtent);
            b2Vec2 c = b2Clamp(c0, min, max);

            aabb->lowerBound += c - c0;
            aabb->upperBound += c - c0;
        */
    }
    
    pub fn create_proxy(&mut self)  {
        
        todo!();
        /*
            for (int32 i = 0; i < e_actorCount; ++i)
            {
                int32 j = rand() % e_actorCount;
                DynamicTreeTestActor* actor = m_actors + j;
                if (actor->proxyId == b2_nullNode)
                {
                    GetRandomAABB(&actor->aabb);
                    actor->proxyId = m_tree.CreateProxy(actor->aabb, actor);
                    return;
                }
            }
        */
    }
    
    pub fn destroy_proxy(&mut self)  {
        
        todo!();
        /*
            for (int32 i = 0; i < e_actorCount; ++i)
            {
                int32 j = rand() % e_actorCount;
                DynamicTreeTestActor* actor = m_actors + j;
                if (actor->proxyId != b2_nullNode)
                {
                    m_tree.DestroyProxy(actor->proxyId);
                    actor->proxyId = b2_nullNode;
                    return;
                }
            }
        */
    }
    
    pub fn move_proxy(&mut self)  {
        
        todo!();
        /*
            for (int32 i = 0; i < e_actorCount; ++i)
            {
                int32 j = rand() % e_actorCount;
                DynamicTreeTestActor* actor = m_actors + j;
                if (actor->proxyId == b2_nullNode)
                {
                    continue;
                }

                b2AABB aabb0 = actor->aabb;
                MoveAABB(&actor->aabb);
                b2Vec2 displacement = actor->aabb.GetCenter() - aabb0.GetCenter();
                m_tree.MoveProxy(actor->proxyId, actor->aabb, displacement);
                return;
            }
        */
    }
    
    pub fn action(&mut self)  {
        
        todo!();
        /*
            int32 choice = rand() % 20;

            switch (choice)
            {
            case 0:
                CreateProxy();
                break;

            case 1:
                DestroyProxy();
                break;

            default:
                MoveProxy();
            }
        */
    }
    
    pub fn query(&mut self)  {
        
        todo!();
        /*
            m_tree.Query(this, m_queryAABB);

            for (int32 i = 0; i < e_actorCount; ++i)
            {
                if (m_actors[i].proxyId == b2_nullNode)
                {
                    continue;
                }

                bool overlap = b2TestOverlap(m_queryAABB, m_actors[i].aabb);
                B2_NOT_USED(overlap);
                b2Assert(overlap == m_actors[i].overlap);
            }
        */
    }
    
    pub fn ray_cast(&mut self)  {
        
        todo!();
        /*
            m_rayActor = NULL;

            b2RayCastInput input = m_rayCastInput;

            // Ray cast against the dynamic tree.
            m_tree.RayCast(this, input);

            // Brute force ray cast.
            DynamicTreeTestActor* bruteActor = NULL;
            b2RayCastOutput bruteOutput;
            for (int32 i = 0; i < e_actorCount; ++i)
            {
                if (m_actors[i].proxyId == b2_nullNode)
                {
                    continue;
                }

                b2RayCastOutput output;
                bool hit = m_actors[i].aabb.RayCast(&output, input);
                if (hit)
                {
                    bruteActor = m_actors + i;
                    bruteOutput = output;
                    input.maxFraction = output.fraction;
                }
            }

            if (bruteActor != NULL)
            {
                b2Assert(bruteOutput.fraction == m_rayCastOutput.fraction);
            }
        */
    }
}
