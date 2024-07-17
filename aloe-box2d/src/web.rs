crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/Web.h]

/**
   This tests distance joints, body destruction,
   and joint destruction.
  */
pub struct Web {
    base:   Test,
    bodies: *mut [b2Body; 4],
    joints: *mut [b2Joint; 8],
}

impl Default for Web {
    
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
                b2PolygonShape shape;
                shape.SetAsBox(0.5f, 0.5f);

                b2BodyDef bd;
                bd.type = b2_dynamicBody;

                bd.position.Set(-5.0f, 5.0f);
                m_bodies[0] = m_world->CreateBody(&bd);
                m_bodies[0]->CreateFixture(&shape, 5.0f);

                bd.position.Set(5.0f, 5.0f);
                m_bodies[1] = m_world->CreateBody(&bd);
                m_bodies[1]->CreateFixture(&shape, 5.0f);

                bd.position.Set(5.0f, 15.0f);
                m_bodies[2] = m_world->CreateBody(&bd);
                m_bodies[2]->CreateFixture(&shape, 5.0f);

                bd.position.Set(-5.0f, 15.0f);
                m_bodies[3] = m_world->CreateBody(&bd);
                m_bodies[3]->CreateFixture(&shape, 5.0f);

                b2DistanceJointDef jd;
                b2Vec2 p1, p2, d;

                jd.frequencyHz = 2.0f;
                jd.dampingRatio = 0.0f;

                jd.bodyA = ground;
                jd.bodyB = m_bodies[0];
                jd.localAnchorA.Set(-10.0f, 0.0f);
                jd.localAnchorB.Set(-0.5f, -0.5f);
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[0] = m_world->CreateJoint(&jd);

                jd.bodyA = ground;
                jd.bodyB = m_bodies[1];
                jd.localAnchorA.Set(10.0f, 0.0f);
                jd.localAnchorB.Set(0.5f, -0.5f);
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[1] = m_world->CreateJoint(&jd);

                jd.bodyA = ground;
                jd.bodyB = m_bodies[2];
                jd.localAnchorA.Set(10.0f, 20.0f);
                jd.localAnchorB.Set(0.5f, 0.5f);
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[2] = m_world->CreateJoint(&jd);

                jd.bodyA = ground;
                jd.bodyB = m_bodies[3];
                jd.localAnchorA.Set(-10.0f, 20.0f);
                jd.localAnchorB.Set(-0.5f, 0.5f);
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[3] = m_world->CreateJoint(&jd);

                jd.bodyA = m_bodies[0];
                jd.bodyB = m_bodies[1];
                jd.localAnchorA.Set(0.5f, 0.0f);
                jd.localAnchorB.Set(-0.5f, 0.0f);;
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[4] = m_world->CreateJoint(&jd);

                jd.bodyA = m_bodies[1];
                jd.bodyB = m_bodies[2];
                jd.localAnchorA.Set(0.0f, 0.5f);
                jd.localAnchorB.Set(0.0f, -0.5f);
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[5] = m_world->CreateJoint(&jd);

                jd.bodyA = m_bodies[2];
                jd.bodyB = m_bodies[3];
                jd.localAnchorA.Set(-0.5f, 0.0f);
                jd.localAnchorB.Set(0.5f, 0.0f);
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[6] = m_world->CreateJoint(&jd);

                jd.bodyA = m_bodies[3];
                jd.bodyB = m_bodies[0];
                jd.localAnchorA.Set(0.0f, -0.5f);
                jd.localAnchorB.Set(0.0f, 0.5f);
                p1 = jd.bodyA->GetWorldPoint(jd.localAnchorA);
                p2 = jd.bodyB->GetWorldPoint(jd.localAnchorB);
                d = p2 - p1;
                jd.length = d.Length();
                m_joints[7] = m_world->CreateJoint(&jd);
            
        */
    }
}

impl Web {

    pub fn keyboard(&mut self, key: u8)  {
        
        todo!();
        /*
            switch (key)
            {
            case 'b':
                for (int32 i = 0; i < 4; ++i)
                {
                    if (m_bodies[i])
                    {
                        m_world->DestroyBody(m_bodies[i]);
                        m_bodies[i] = NULL;
                        break;
                    }
                }
                break;

            case 'j':
                for (int32 i = 0; i < 8; ++i)
                {
                    if (m_joints[i])
                    {
                        m_world->DestroyJoint(m_joints[i]);
                        m_joints[i] = NULL;
                        break;
                    }
                }
                break;
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "This demonstrates a soft distance joint.");
            m_textLine += 15;
            m_debugDraw.DrawString(5, m_textLine, "Press: (b) to delete a body, (j) to delete a joint");
            m_textLine += 15;
        */
    }
    
    pub fn joint_destroyed(&mut self, joint: *mut b2Joint)  {
        
        todo!();
        /*
            for (int32 i = 0; i < 8; ++i)
            {
                if (m_joints[i] == joint)
                {
                    m_joints[i] = NULL;
                    break;
                }
            }
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Web;
        */
    }
}
