crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/BulletTest.h]

pub struct BulletTest {
    base:   Test,
    body:   *mut b2Body,
    bullet: *mut b2Body,
    x:      f32,
}

impl Default for BulletTest {
    
    fn default() -> Self {
        todo!();
        /*


            {
                b2BodyDef bd;
                bd.position.Set(0.0f, 0.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2EdgeShape edge;

                edge.Set(b2Vec2(-10.0f, 0.0f), b2Vec2(10.0f, 0.0f));
                body->CreateFixture(&edge, 0.0f);

                b2PolygonShape shape;
                shape.SetAsBox(0.2f, 1.0f, b2Vec2(0.5f, 1.0f), 0.0f);
                body->CreateFixture(&shape, 0.0f);
            }

            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 4.0f);

                b2PolygonShape box;
                box.SetAsBox(2.0f, 0.1f);

                m_body = m_world->CreateBody(&bd);
                m_body->CreateFixture(&box, 1.0f);

                box.SetAsBox(0.25f, 0.25f);

                //m_x = RandomFloat(-1.0f, 1.0f);
                m_x = 0.20352793f;
                bd.position.Set(m_x, 10.0f);
                bd.bullet = true;

                m_bullet = m_world->CreateBody(&bd);
                m_bullet->CreateFixture(&box, 100.0f);

                m_bullet->SetLinearVelocity(b2Vec2(0.0f, -50.0f));
            
        */
    }
}

impl BulletTest {

    pub fn launch(&mut self)  {
        
        todo!();
        /*
            m_body->SetTransform(b2Vec2(0.0f, 4.0f), 0.0f);
            m_body->SetLinearVelocity(b2Vec2_zero);
            m_body->SetAngularVelocity(0.0f);

            m_x = RandomFloat(-1.0f, 1.0f);
            m_bullet->SetTransform(b2Vec2(m_x, 10.0f), 0.0f);
            m_bullet->SetLinearVelocity(b2Vec2(0.0f, -50.0f));
            m_bullet->SetAngularVelocity(0.0f);

            extern int32 b2_gjkCalls, b2_gjkIters, b2_gjkMaxIters;
            extern int32 b2_toiCalls, b2_toiIters, b2_toiMaxIters;
            extern int32 b2_toiRootIters, b2_toiMaxRootIters;

            b2_gjkCalls = 0;
            b2_gjkIters = 0;
            b2_gjkMaxIters = 0;

            b2_toiCalls = 0;
            b2_toiIters = 0;
            b2_toiMaxIters = 0;
            b2_toiRootIters = 0;
            b2_toiMaxRootIters = 0;
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            extern int32 b2_gjkCalls, b2_gjkIters, b2_gjkMaxIters;
            extern int32 b2_toiCalls, b2_toiIters;
            extern int32 b2_toiRootIters, b2_toiMaxRootIters;

            if (b2_gjkCalls > 0)
            {
                m_debugDraw.DrawString(5, m_textLine, "gjk calls = %d, ave gjk iters = %3.1f, max gjk iters = %d",
                    b2_gjkCalls, b2_gjkIters / float32(b2_gjkCalls), b2_gjkMaxIters);
                m_textLine += 15;
            }

            if (b2_toiCalls > 0)
            {
                m_debugDraw.DrawString(5, m_textLine, "toi calls = %d, ave toi iters = %3.1f, max toi iters = %d",
                    b2_toiCalls, b2_toiIters / float32(b2_toiCalls), b2_toiMaxRootIters);
                m_textLine += 15;

                m_debugDraw.DrawString(5, m_textLine, "ave toi root iters = %3.1f, max toi root iters = %d",
                    b2_toiRootIters / float32(b2_toiCalls), b2_toiMaxRootIters);
                m_textLine += 15;
            }

            if (m_stepCount % 60 == 0)
            {
                Launch();
            }
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new BulletTest;
        */
    }
}
