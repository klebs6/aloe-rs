crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/SliderCrank.h]

/**
   A motor driven slider crank with joint
   friction.
  */
pub struct SliderCrank {
    base:   Test,
    joint1: *mut b2RevoluteJoint,
    joint2: *mut b2PrismaticJoint,
}

impl Default for SliderCrank {
    
    fn default() -> Self {
        todo!();
        /*


            b2Body* ground = NULL;
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            {
                b2Body* prevBody = ground;

                // Define crank.
                {
                    b2PolygonShape shape;
                    shape.SetAsBox(0.5f, 2.0f);

                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(0.0f, 7.0f);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&shape, 2.0f);

                    b2RevoluteJointDef rjd;
                    rjd.Initialize(prevBody, body, b2Vec2(0.0f, 5.0f));
                    rjd.motorSpeed = 1.0f * b2_pi;
                    rjd.maxMotorTorque = 10000.0f;
                    rjd.enableMotor = true;
                    m_joint1 = (b2RevoluteJoint*)m_world->CreateJoint(&rjd);

                    prevBody = body;
                }

                // Define follower.
                {
                    b2PolygonShape shape;
                    shape.SetAsBox(0.5f, 4.0f);

                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(0.0f, 13.0f);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&shape, 2.0f);

                    b2RevoluteJointDef rjd;
                    rjd.Initialize(prevBody, body, b2Vec2(0.0f, 9.0f));
                    rjd.enableMotor = false;
                    m_world->CreateJoint(&rjd);

                    prevBody = body;
                }

                // Define piston
                {
                    b2PolygonShape shape;
                    shape.SetAsBox(1.5f, 1.5f);

                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.fixedRotation = true;
                    bd.position.Set(0.0f, 17.0f);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&shape, 2.0f);

                    b2RevoluteJointDef rjd;
                    rjd.Initialize(prevBody, body, b2Vec2(0.0f, 17.0f));
                    m_world->CreateJoint(&rjd);

                    b2PrismaticJointDef pjd;
                    pjd.Initialize(ground, body, b2Vec2(0.0f, 17.0f), b2Vec2(0.0f, 1.0f));

                    pjd.maxMotorForce = 1000.0f;
                    pjd.enableMotor = true;

                    m_joint2 = (b2PrismaticJoint*)m_world->CreateJoint(&pjd);
                }

                // Create a payload
                {
                    b2PolygonShape shape;
                    shape.SetAsBox(1.5f, 1.5f);

                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(0.0f, 23.0f);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&shape, 2.0f);
                }
            
        */
    }
}

impl SliderCrank {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'f':
                m_joint2->EnableMotor(!m_joint2->IsMotorEnabled());
                m_joint2->GetBodyB()->SetAwake(true);
                break;

            case 'm':
                m_joint1->EnableMotor(!m_joint1->IsMotorEnabled());
                m_joint1->GetBodyB()->SetAwake(true);
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Keys: (f) toggle friction, (m) toggle motor");
            m_textLine += 15;
            float32 torque = m_joint1->GetMotorTorque(settings->hz);
            m_debugDraw.DrawString(5, m_textLine, "Motor Torque = %5.0f", (float) torque);
            m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new SliderCrank;
        */
    }
}
