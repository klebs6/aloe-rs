crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/Gears.h]

pub struct Gears {
    base:   Test,
    joint1: *mut b2RevoluteJoint,
    joint2: *mut b2RevoluteJoint,
    joint3: *mut b2PrismaticJoint,
    joint4: *mut b2GearJoint,
    joint5: *mut b2GearJoint,
}

impl Default for Gears {
    
    fn default() -> Self {
        todo!();
        /*


            b2Body* ground = NULL;
            {
                b2BodyDef bd;
                ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(50.0f, 0.0f), b2Vec2(-50.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            // Gears co
            {
                b2CircleShape circle1;
                circle1.m_radius = 1.0f;

                b2PolygonShape box;
                box.SetAsBox(0.5f, 5.0f);

                b2CircleShape circle2;
                circle2.m_radius = 2.0f;

                b2BodyDef bd1;
                bd1.type = b2_staticBody;
                bd1.position.Set(10.0f, 9.0f);
                b2Body* body1 = m_world->CreateBody(&bd1);
                body1->CreateFixture(&circle1, 0.0f);

                b2BodyDef bd2;
                bd2.type = b2_dynamicBody;
                bd2.position.Set(10.0f, 8.0f);
                b2Body* body2 = m_world->CreateBody(&bd2);
                body2->CreateFixture(&box, 5.0f);

                b2BodyDef bd3;
                bd3.type = b2_dynamicBody;
                bd3.position.Set(10.0f, 6.0f);
                b2Body* body3 = m_world->CreateBody(&bd3);
                body3->CreateFixture(&circle2, 5.0f);

                b2RevoluteJointDef jd1;
                jd1.Initialize(body2, body1, bd1.position);
                b2Joint* joint1 = m_world->CreateJoint(&jd1);

                b2RevoluteJointDef jd2;
                jd2.Initialize(body2, body3, bd3.position);
                b2Joint* joint2 = m_world->CreateJoint(&jd2);

                b2GearJointDef jd4;
                jd4.bodyA = body1;
                jd4.bodyB = body3;
                jd4.joint1 = joint1;
                jd4.joint2 = joint2;
                jd4.ratio = circle2.m_radius / circle1.m_radius;
                m_world->CreateJoint(&jd4);
            }

            {
                b2CircleShape circle1;
                circle1.m_radius = 1.0f;

                b2CircleShape circle2;
                circle2.m_radius = 2.0f;

                b2PolygonShape box;
                box.SetAsBox(0.5f, 5.0f);

                b2BodyDef bd1;
                bd1.type = b2_dynamicBody;
                bd1.position.Set(-3.0f, 12.0f);
                b2Body* body1 = m_world->CreateBody(&bd1);
                body1->CreateFixture(&circle1, 5.0f);

                b2RevoluteJointDef jd1;
                jd1.bodyA = ground;
                jd1.bodyB = body1;
                jd1.localAnchorA = ground->GetLocalPoint(bd1.position);
                jd1.localAnchorB = body1->GetLocalPoint(bd1.position);
                jd1.referenceAngle = body1->GetAngle() - ground->GetAngle();
                m_joint1 = (b2RevoluteJoint*)m_world->CreateJoint(&jd1);

                b2BodyDef bd2;
                bd2.type = b2_dynamicBody;
                bd2.position.Set(0.0f, 12.0f);
                b2Body* body2 = m_world->CreateBody(&bd2);
                body2->CreateFixture(&circle2, 5.0f);

                b2RevoluteJointDef jd2;
                jd2.Initialize(ground, body2, bd2.position);
                m_joint2 = (b2RevoluteJoint*)m_world->CreateJoint(&jd2);

                b2BodyDef bd3;
                bd3.type = b2_dynamicBody;
                bd3.position.Set(2.5f, 12.0f);
                b2Body* body3 = m_world->CreateBody(&bd3);
                body3->CreateFixture(&box, 5.0f);

                b2PrismaticJointDef jd3;
                jd3.Initialize(ground, body3, bd3.position, b2Vec2(0.0f, 1.0f));
                jd3.lowerTranslation = -5.0f;
                jd3.upperTranslation = 5.0f;
                jd3.enableLimit = true;

                m_joint3 = (b2PrismaticJoint*)m_world->CreateJoint(&jd3);

                b2GearJointDef jd4;
                jd4.bodyA = body1;
                jd4.bodyB = body2;
                jd4.joint1 = m_joint1;
                jd4.joint2 = m_joint2;
                jd4.ratio = circle2.m_radius / circle1.m_radius;
                m_joint4 = (b2GearJoint*)m_world->CreateJoint(&jd4);

                b2GearJointDef jd5;
                jd5.bodyA = body2;
                jd5.bodyB = body3;
                jd5.joint1 = m_joint2;
                jd5.joint2 = m_joint3;
                jd5.ratio = -1.0f / circle2.m_radius;
                m_joint5 = (b2GearJoint*)m_world->CreateJoint(&jd5);
            
        */
    }
}

impl Gears {

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

            float32 ratio, value;

            ratio = m_joint4->GetRatio();
            value = m_joint1->GetJointAngle() + ratio * m_joint2->GetJointAngle();
            m_debugDraw.DrawString(5, m_textLine, "theta1 + %4.2f * theta2 = %4.2f", (float) ratio, (float) value);
            m_textLine += 15;

            ratio = m_joint5->GetRatio();
            value = m_joint2->GetJointAngle() + ratio * m_joint3->GetJointTranslation();
            m_debugDraw.DrawString(5, m_textLine, "theta2 + %4.2f * delta = %4.2f", (float) ratio, (float) value);
            m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Gears;
        */
    }
}
