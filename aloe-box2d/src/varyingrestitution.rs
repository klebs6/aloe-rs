crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/VaryingRestitution.h]

/**
   Note: even with a restitution of 1.0, there is
   some energy change due to position correction.
  */
pub struct VaryingRestitution {
    base: Test,
}

impl Default for VaryingRestitution {
    
    fn default() -> Self {
        todo!();
        /*
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            {
                b2CircleShape shape;
                shape.m_radius = 1.0f;

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.density = 1.0f;

                float32 restitution[7] = {0.0f, 0.1f, 0.3f, 0.5f, 0.75f, 0.9f, 1.0f};

                for (int32 i = 0; i < 7; ++i)
                {
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(-10.0f + 3.0f * i, 20.0f);

                    b2Body* body = m_world->CreateBody(&bd);

                    fd.restitution = restitution[i];
                    body->CreateFixture(&fd);
                }
            
        */
    }
}

impl VaryingRestitution {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new VaryingRestitution;
        */
    }
}
