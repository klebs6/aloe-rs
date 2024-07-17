crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/BodyTypes.h]

pub struct BodyTypes {
    base:       Test,
    attachment: *mut b2Body,
    platform:   *mut b2Body,
    speed:      f32,
}

impl Default for BodyTypes {
    
    fn default() -> Self {
        todo!();
        /*
            b2Body* ground = NULL;
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-20.0f, 0.0f), b2Vec2(20.0f, 0.0f));

                b2FixtureDef fd;
                fd.shape = &shape;

                ground->CreateFixture(&fd);
            }

            // Define attachment
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 3.0f);
                m_attachment = m_world->CreateBody(&bd);

                b2PolygonShape shape;
                shape.SetAsBox(0.5f, 2.0f);
                m_attachment->CreateFixture(&shape, 2.0f);
            }

            // Define platform
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(-4.0f, 5.0f);
                m_platform = m_world->CreateBody(&bd);

                b2PolygonShape shape;
                shape.SetAsBox(0.5f, 4.0f, b2Vec2(4.0f, 0.0f), 0.5f * b2_pi);

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.friction = 0.6f;
                fd.density = 2.0f;
                m_platform->CreateFixture(&fd);

                b2RevoluteJointDef rjd;
                rjd.Initialize(m_attachment, m_platform, b2Vec2(0.0f, 5.0f));
                rjd.maxMotorTorque = 50.0f;
                rjd.enableMotor = true;
                m_world->CreateJoint(&rjd);

                b2PrismaticJointDef pjd;
                pjd.Initialize(ground, m_platform, b2Vec2(0.0f, 5.0f), b2Vec2(1.0f, 0.0f));

                pjd.maxMotorForce = 1000.0f;
                pjd.enableMotor = true;
                pjd.lowerTranslation = -10.0f;
                pjd.upperTranslation = 10.0f;
                pjd.enableLimit = true;

                m_world->CreateJoint(&pjd);

                m_speed = 3.0f;
            }

            // Create a payload
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 8.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2PolygonShape shape;
                shape.SetAsBox(0.75f, 0.75f);

                b2FixtureDef fd;
                fd.shape = &shape;
                fd.friction = 0.6f;
                fd.density = 2.0f;

                body->CreateFixture(&fd);
            
        */
    }
}

impl BodyTypes {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'd':
                m_platform->SetType(b2_dynamicBody);
                break;

            case 's':
                m_platform->SetType(b2_staticBody);
                break;

            case 'k':
                m_platform->SetType(b2_kinematicBody);
                m_platform->SetLinearVelocity(b2Vec2(-m_speed, 0.0f));
                m_platform->SetAngularVelocity(0.0f);
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            // Drive the kinematic body.
            if (m_platform->GetType() == b2_kinematicBody)
            {
                b2Vec2 p = m_platform->GetTransform().p;
                b2Vec2 v = m_platform->GetLinearVelocity();

                if ((p.x < -10.0f && v.x < 0.0f) ||
                    (p.x > 10.0f && v.x > 0.0f))
                {
                    v.x = -v.x;
                    m_platform->SetLinearVelocity(v);
                }
            }

            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Keys: (d) dynamic, (s) static, (k) kinematic");
            m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new BodyTypes;
        */
    }
}
