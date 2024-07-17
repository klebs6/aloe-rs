crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/Chain.h]

pub struct Chain {
    base: Test,
}

impl Default for Chain {
    
    fn default() -> Self {
        todo!();
        /*
            b2Body* ground = {};
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            {
                b2PolygonShape shape;
                shape.SetAsBox(0.6f, 0.125f);

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.density = 20.0f;
                fd.friction = 0.2f;

                b2RevoluteJointDef jd;
                jd.collideConnected = false;

                const float32 y = 25.0f;
                b2Body* prevBody = ground;
                for (int i = 0; i < 30; ++i)
                {
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(0.5f + i, y);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&fd);

                    b2Vec2 anchor(float32(i), y);
                    jd.Initialize(prevBody, body, anchor);
                    m_world->CreateJoint(&jd);

                    prevBody = body;
                }
            
        */
    }
}

impl Chain {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Chain;
        */
    }
}
