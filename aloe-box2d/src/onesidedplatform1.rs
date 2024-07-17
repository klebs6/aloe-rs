crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/OneSidedPlatform.h]
pub enum OneSidedPlatformState
{
    e_unknown,
    e_above,
    e_below
}

pub struct OneSidedPlatform {
    base:      Test,
    radius:    f32,
    top:       f32,
    bottom:    f32,
    state:     OneSidedPlatformState,
    platform:  *mut b2Fixture,
    character: *mut b2Fixture,
}

impl Default for OneSidedPlatform {
    
    fn default() -> Self {
        todo!();
        /*


            // Ground
            {
                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(-20.0f, 0.0f), b2Vec2(20.0f, 0.0f));
                ground->CreateFixture(&shape, 0.0f);
            }

            // Platform
            {
                b2BodyDef bd;
                bd.position.Set(0.0f, 10.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2PolygonShape shape;
                shape.SetAsBox(3.0f, 0.5f);
                m_platform = body->CreateFixture(&shape, 0.0f);

                m_bottom = 10.0f - 0.5f;
                m_top = 10.0f + 0.5f;
            }

            // Actor
            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(0.0f, 12.0f);
                b2Body* body = m_world->CreateBody(&bd);

                m_radius = 0.5f;
                b2CircleShape shape;
                shape.m_radius = m_radius;
                m_character = body->CreateFixture(&shape, 20.0f);

                body->SetLinearVelocity(b2Vec2(0.0f, -50.0f));

                m_state = e_unknown;
            
        */
    }
}

impl OneSidedPlatform {

    pub fn pre_solve(&mut self, 
        contact:      *mut b2Contact,
        old_manifold: *const b2Manifold)  {
        
        todo!();
        /*
            Test::PreSolve(contact, oldManifold);

            b2Fixture* fixtureA = contact->GetFixtureA();
            b2Fixture* fixtureB = contact->GetFixtureB();

            if (fixtureA != m_platform && fixtureA != m_character)
            {
                return;
            }

            if (fixtureB != m_platform && fixtureB != m_character)
            {
                return;
            }

            b2Vec2 position = m_character->GetBody()->GetPosition();

            if (position.y < m_top + m_radius - 3.0f * b2_linearSlop)
            {
                contact->SetEnabled(false);
            }
        */
    }
    
    pub fn step(&mut self, settings: *mut Settings)  {
        
        todo!();
        /*
            Test::Step(settings);
            m_debugDraw.DrawString(5, m_textLine, "Press: (c) create a shape, (d) destroy a shape.");
            m_textLine += 15;
        */
    }
    
    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new OneSidedPlatform;
        */
    }
}
