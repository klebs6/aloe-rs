crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/Tumbler.h]

pub const TUMBLER_E_COUNT: usize = 800;

pub struct Tumbler {
    base:  Test,
    joint: *mut b2RevoluteJoint,
    count: i32,
}

impl Default for Tumbler {
    
    fn default() -> Self {
        todo!();
        /*


            b2Body* ground = NULL;
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);
            }

            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.allowSleep = false;
                bd.position.Set(0.0f, 10.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2PolygonShape shape;
                shape.SetAsBox(0.5f, 10.0f, b2Vec2( 10.0f, 0.0f), 0.0);
                body->CreateFixture(&shape, 5.0f);
                shape.SetAsBox(0.5f, 10.0f, b2Vec2(-10.0f, 0.0f), 0.0);
                body->CreateFixture(&shape, 5.0f);
                shape.SetAsBox(10.0f, 0.5f, b2Vec2(0.0f, 10.0f), 0.0);
                body->CreateFixture(&shape, 5.0f);
                shape.SetAsBox(10.0f, 0.5f, b2Vec2(0.0f, -10.0f), 0.0);
                body->CreateFixture(&shape, 5.0f);

                b2RevoluteJointDef jd;
                jd.bodyA = ground;
                jd.bodyB = body;
                jd.localAnchorA.Set(0.0f, 10.0f);
                jd.localAnchorB.Set(0.0f, 0.0f);
                jd.referenceAngle = 0.0f;
                jd.motorSpeed = 0.05f * b2_pi;
                jd.maxMotorTorque = 1e8f;
                jd.enableMotor = true;
                m_joint = (b2RevoluteJoint*)m_world->CreateJoint(&jd);
            }

            m_count = 0
        */
    }
}

impl Tumbler {

    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);

            if (m_count < TUMBLER_E_COUNT)
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 10.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2PolygonShape shape;
                shape.SetAsBox(0.125f, 0.125f);
                body->CreateFixture(&shape, 1.0f);

                ++m_count;
            }
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Tumbler;
        */
    }
}
