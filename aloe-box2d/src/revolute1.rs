crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/Revolute.h]

pub struct Revolute {
    base:  Test,
    ball:  *mut b2Body,
    joint: *mut b2RevoluteJoint,
}

impl Default for Revolute {
    
    fn default() -> Self {
        todo!();
        /*
            b2Body* ground = NULL;
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));

                b2FixtureDef fd;
                fd.shape = &shape;
                //fd.filter.categoryBits = 2;

                ground->CreateFixture(&fd);
            }

            {
                b2CircleShape shape;
                shape.m_radius = 0.5f;

                b2BodyDef bd;
                bd.type = b2_dynamicBody;

                b2RevoluteJointDef rjd;

                bd.position.Set(-10.0f, 20.0f);
                b2Body* body = m_world->CreateBody(&bd);
                body->CreateFixture(&shape, 5.0f);

                float32 w = 100.0f;
                body->SetAngularVelocity(w);
                body->SetLinearVelocity(b2Vec2(-8.0f * w, 0.0f));

                rjd.Initialize(ground, body, b2Vec2(-10.0f, 12.0f));
                rjd.motorSpeed = 1.0f * b2_pi;
                rjd.maxMotorTorque = 10000.0f;
                rjd.enableMotor = false;
                rjd.lowerAngle = -0.25f * b2_pi;
                rjd.upperAngle = 0.5f * b2_pi;
                rjd.enableLimit = true;
                rjd.collideConnected = true;

                m_joint = (b2RevoluteJoint*)m_world->CreateJoint(&rjd);
            }

            {
                b2CircleShape circle_shape;
                circle_shape.m_radius = 3.0f;

                b2BodyDef circle_bd;
                circle_bd.type = b2_dynamicBody;
                circle_bd.position.Set(5.0f, 30.0f);

                b2FixtureDef fd;
                fd.density = 5.0f;
                fd.filter.maskBits = 1;
                fd.shape = &circle_shape;

                m_ball = m_world->CreateBody(&circle_bd);
                m_ball->CreateFixture(&fd);

                b2PolygonShape polygon_shape;
                polygon_shape.SetAsBox(10.0f, 0.2f, b2Vec2 (-10.0f, 0.0f), 0.0f);

                b2BodyDef polygon_bd;
                polygon_bd.position.Set(20.0f, 10.0f);
                polygon_bd.type = b2_dynamicBody;
                polygon_bd.bullet = true;
                b2Body* polygon_body = m_world->CreateBody(&polygon_bd);
                polygon_body->CreateFixture(&polygon_shape, 2.0f);

                b2RevoluteJointDef rjd;
                rjd.Initialize(ground, polygon_body, b2Vec2(20.0f, 10.0f));
                rjd.lowerAngle = -0.25f * b2_pi;
                rjd.upperAngle = 0.0f * b2_pi;
                rjd.enableLimit = true;
                m_world->CreateJoint(&rjd);
            }

            // Tests mass computation of a small object far from the origin
            {
                b2BodyDef bodyDef;
                bodyDef.type = b2_dynamicBody;
                b2Body* body = m_world->CreateBody(&bodyDef);

                b2PolygonShape polyShape;
                b2Vec2 verts[3];
                verts[0].Set( 17.63f, 36.31f );
                verts[1].Set( 17.52f, 36.69f );
                verts[2].Set( 17.19f, 36.36f );
                polyShape.Set(verts, 3);

                b2FixtureDef polyFixtureDef;
                polyFixtureDef.shape = &polyShape;
                polyFixtureDef.density = 1;

                body->CreateFixture(&polyFixtureDef);   //assertion hits inside here
            
        */
    }
}

impl Revolute {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'l':
                m_joint->EnableLimit(!m_joint->IsLimitEnabled());
                break;

            case 'm':
                m_joint->EnableMotor(!m_joint->IsMotorEnabled());
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Keys: (l) limits, (m) motor");
            m_textLine += 15;

            //if (m_stepCount == 360)
            //{
            //  m_ball->SetTransform(b2Vec2(0.0f, 0.5f), 0.0f);
            //}

            //float32 torque1 = m_joint1->GetMotorTorque();
            //m_debugDraw.DrawString(5, m_textLine, "Motor Torque = %4.0f, %4.0f : Motor Force = %4.0f", (float) torque1, (float) torque2, (float) force3);
            //m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Revolute;
        */
    }
}
