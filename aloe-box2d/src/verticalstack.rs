crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/VerticalStack.h]

pub const VERTICAL_STACK_E_COLUMNCOUNT: usize = 5;
pub const VERTICAL_STACK_E_ROWCOUNT:    usize = 16;

pub struct VerticalStack {
    base:    Test,
    bullet:  *mut b2Body,
    bodies:  *mut [b2Body; VERTICAL_STACK_E_ROWCOUNT * VERTICAL_STACK_E_COLUMNCOUNT],
    indices: [i32; VERTICAL_STACK_E_ROWCOUNT * VERTICAL_STACK_E_COLUMNCOUNT],
}

impl Default for VerticalStack {
    
    fn default() -> Self {
        todo!();
        /*
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);

                shape.Set(b2Vec2(20.0f, 0.0f), b2Vec2(20.0f, 20.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            float32 xs[5] = {0.0f, -10.0f, -5.0f, 5.0f, 10.0f};

            for (int32 j = 0; j < VERTICAL_STACK_E_COLUMNCOUNT; ++j)
            {
                b2PolygonShape shape;
                shape.SetAsBox(0.5f, 0.5f);

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.density = 1.0f;
                fd.friction = 0.3f;

                for (int i = 0; i < VERTICAL_STACK_E_ROWCOUNT; ++i)
                {
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;

                    int32 n = j * VERTICAL_STACK_E_ROWCOUNT + i;
                    b2Assert(n < VERTICAL_STACK_E_ROWCOUNT * VERTICAL_STACK_E_COLUMNCOUNT);
                    m_indices[n] = n;
                    bd.userData = m_indices + n;

                    float32 x = 0.0f;
                    //float32 x = RandomFloat(-0.02f, 0.02f);
                    //float32 x = i % 2 == 0 ? -0.025f : 0.025f;
                    bd.position.Set(xs[j] + x, 0.752f + 1.54f * i);
                    b2Body* body = m_world->CreateBody(&bd);

                    m_bodies[n] = body;

                    body->CreateFixture(&fd);
                }
            }

            m_bullet = NULL
        */
    }
}

impl VerticalStack {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case ',':
                if (m_bullet != NULL)
                {
                    m_world->DestroyBody(m_bullet);
                    m_bullet = NULL;
                }

                {
                    b2CircleShape shape;
                    shape.m_radius = 0.25f;

                    b2FixtureDef fd;
                    fd.shape = &shape;
                    fd.density = 20.0f;
                    fd.restitution = 0.05f;

                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.bullet = true;
                    bd.position.Set(-31.0f, 5.0f);

                    m_bullet = m_world->CreateBody(&bd);
                    m_bullet->CreateFixture(&fd);

                    m_bullet->SetLinearVelocity(b2Vec2(400.0f, 0.0f));
                }
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Press: (,) to launch a bullet.");
            m_textLine += 15;

            //if (m_stepCount == 300)
            //{
            //  if (m_bullet != NULL)
            //  {
            //      m_world->DestroyBody(m_bullet);
            //      m_bullet = NULL;
            //  }

            //  {
            //      b2CircleShape shape;
            //      shape.m_radius = 0.25f;

            //      b2FixtureDef fd;
            //      fd.shape = &shape;
            //      fd.density = 20.0f;
            //      fd.restitution = 0.05f;

            //      b2BodyDef bd;
            //      bd.type = b2_dynamicBody;
            //      bd.bullet = true;
            //      bd.position.Set(-31.0f, 5.0f);

            //      m_bullet = m_world->CreateBody(&bd);
            //      m_bullet->CreateFixture(&fd);

            //      m_bullet->SetLinearVelocity(b2Vec2(400.0f, 0.0f));
            //  }
            //}
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new VerticalStack;
        */
    }
}
