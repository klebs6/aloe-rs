crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/Pulleys.h]

pub struct Pulleys {
    base:   Test,
    joint1: *mut b2PulleyJoint,
}

impl Default for Pulleys {
    
    fn default() -> Self {
        todo!();
        /*


            float32 y = 16.0f;
            float32 L = 12.0f;
            float32 a = 1.0f;
            float32 b = 2.0f;

            b2Body* ground = NULL;
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);

                b2EdgeShape edge;
                edge.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                //ground->CreateFixture(&shape, 0.0f);

                b2CircleShape circle;
                circle.m_radius = 2.0f;

                circle.m_p.Set(-10.0f, y + b + L);
                ground->CreateFixture(&circle, 0.0f);

                circle.m_p.Set(10.0f, y + b + L);
                ground->CreateFixture(&circle, 0.0f);
            }

            {

                b2PolygonShape shape;
                shape.SetAsBox(a, b);

                b2BodyDef bd;
                bd.type = b2_dynamicBody;

                //bd.fixedRotation = true;
                bd.position.Set(-10.0f, y);
                b2Body* body1 = m_world->CreateBody(&bd);
                body1->CreateFixture(&shape, 5.0f);

                bd.position.Set(10.0f, y);
                b2Body* body2 = m_world->CreateBody(&bd);
                body2->CreateFixture(&shape, 5.0f);

                b2PulleyJointDef pulleyDef;
                b2Vec2 anchor1(-10.0f, y + b);
                b2Vec2 anchor2(10.0f, y + b);
                b2Vec2 groundAnchor1(-10.0f, y + b + L);
                b2Vec2 groundAnchor2(10.0f, y + b + L);
                pulleyDef.Initialize(body1, body2, groundAnchor1, groundAnchor2, anchor1, anchor2, 1.5f);

                m_joint1 = (b2PulleyJoint*)m_world->CreateJoint(&pulleyDef);
            
        */
    }
}

impl Pulleys {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 0:
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            float32 ratio = m_joint1->GetRatio();
            float32 L = m_joint1->GetLengthA() + ratio * m_joint1->GetLengthB();
            m_debugDraw.DrawString(5, m_textLine, "L1 + %4.2f * L2 = %4.2f", (float) ratio, (float) L);
            m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Pulleys;
        */
    }
}
