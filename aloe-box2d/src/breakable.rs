crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Assets/Box2DTests/Breakable.h]

/**
   This is used to test sensor shapes.
  */
pub struct Breakable {
    base:             Test,
    body1:            *mut b2Body,
    velocity:         b2Vec2,
    angular_velocity: f32,
    shape1:           b2PolygonShape,
    shape2:           b2PolygonShape,
    piece1:           *mut b2Fixture,
    piece2:           *mut b2Fixture,
    broke:            bool,
    break_:           bool,
}

pub const BREAKABLE_E_COUNT: usize = 7;

impl Default for Breakable {
    
    fn default() -> Self {
        todo!();
        /*
            // Ground body
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            // Breakable dynamic body
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 40.0f);
                bd.angle = 0.25f * b2_pi;
                m_body1 = m_world->CreateBody(&bd);

                m_shape1.SetAsBox(0.5f, 0.5f, b2Vec2(-0.5f, 0.0f), 0.0f);
                m_piece1 = m_body1->CreateFixture(&m_shape1, 1.0f);

                m_shape2.SetAsBox(0.5f, 0.5f, b2Vec2(0.5f, 0.0f), 0.0f);
                m_piece2 = m_body1->CreateFixture(&m_shape2, 1.0f);
            }

            m_break = false;
            m_broke = false
        */
    }
}

impl Breakable {

    pub fn post_solve(&mut self, 
        contact: *mut b2Contact,
        impulse: *const b2ContactImpulse)  {
        
        todo!();
        /*
            if (m_broke)
            {
                // The body already broke.
                return;
            }

            // Should the body break?
            int32 count = contact->GetManifold()->pointCount;

            float32 maxImpulse = 0.0f;
            for (int32 i = 0; i < count; ++i)
            {
                maxImpulse = b2Max(maxImpulse, impulse->normalImpulses[i]);
            }

            if (maxImpulse > 40.0f)
            {
                // Flag the body for breaking.
                m_break = true;
            }
        */
    }
    
    pub fn break_(&mut self)  {
        
        todo!();
        /*
            // Create two bodies from one.
            b2Body* body1 = m_piece1->GetBody();
            b2Vec2 center = body1->GetWorldCenter();

            body1->DestroyFixture(m_piece2);
            m_piece2 = NULL;

            b2BodyDef bd;
            bd.type = b2_dynamicBody;
            bd.position = body1->GetPosition();
            bd.angle = body1->GetAngle();

            b2Body* body2 = m_world->CreateBody(&bd);
            m_piece2 = body2->CreateFixture(&m_shape2, 1.0f);

            // Compute consistent velocities for new bodies based on
            // cached velocity.
            b2Vec2 center1 = body1->GetWorldCenter();
            b2Vec2 center2 = body2->GetWorldCenter();

            b2Vec2 velocity1 = m_velocity + b2Cross(m_angularVelocity, center1 - center);
            b2Vec2 velocity2 = m_velocity + b2Cross(m_angularVelocity, center2 - center);

            body1->SetAngularVelocity(m_angularVelocity);
            body1->SetLinearVelocity(velocity1);

            body2->SetAngularVelocity(m_angularVelocity);
            body2->SetLinearVelocity(velocity2);
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            if (m_break)
            {
                Break();
                m_broke = true;
                m_break = false;
            }

            // Cache velocities to improve movement on breakage.
            if (m_broke == false)
            {
                m_velocity = m_body1->GetLinearVelocity();
                m_angularVelocity = m_body1->GetAngularVelocity();
            }

            Test::Step(settings);
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new Breakable;
        */
    }
}
