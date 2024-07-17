crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/ContinuousTest.h]

pub struct ContinuousTest {
    base: Test,
    body:             *mut b2Body,
    angular_velocity: f32,
}

impl Default for ContinuousTest {
    
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

    #if 1
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 20.0f);
                //bd.angle = 0.1f;

                b2PolygonShape shape;
                shape.SetAsBox(2.0f, 0.1f);

                m_body = m_world->CreateBody(&bd);
                m_body->CreateFixture(&shape, 1.0f);

                m_angularVelocity = RandomFloat(-50.0f, 50.0f);
                //m_angularVelocity = 46.661274f;
                m_body->SetLinearVelocity(b2Vec2(0.0f, -100.0f));
                m_body->SetAngularVelocity(m_angularVelocity);
            }
    #else
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 2.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2CircleShape shape;
                shape.m_p.SetZero();
                shape.m_radius = 0.5f;
                body->CreateFixture(&shape, 1.0f);

                bd.bullet = true;
                bd.position.Set(0.0f, 10.0f);
                body = m_world->CreateBody(&bd);
                body->CreateFixture(&shape, 1.0f);
                body->SetLinearVelocity(b2Vec2(0.0f, -100.0f));
            }
    #endi
        */
    }
}

impl ContinuousTest {

    pub fn launch(&mut self)  {
        
        todo!();
        /*
            m_body->SetTransform(b2Vec2(0.0f, 20.0f), 0.0f);
            m_angularVelocity = RandomFloat(-50.0f, 50.0f);
            m_body->SetLinearVelocity(b2Vec2(0.0f, -100.0f));
            m_body->SetAngularVelocity(m_angularVelocity);
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            if (m_stepCount == 12)
            {
                m_stepCount += 0;
            }

            Test::Step(settings);

            extern int32 b2_gjkCalls, b2_gjkIters, b2_gjkMaxIters;

            if (b2_gjkCalls > 0)
            {
                m_debugDraw.DrawString(5, m_textLine, "gjk calls = %d, ave gjk iters = %3.1f, max gjk iters = %d",
                    b2_gjkCalls, b2_gjkIters / float32(b2_gjkCalls), b2_gjkMaxIters);
                m_textLine += 15;
            }

            extern int32 b2_toiCalls, b2_toiIters;
            extern int32 b2_toiRootIters, b2_toiMaxRootIters;

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
                //Launch();
            }
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new ContinuousTest;
        */
    }
}
