crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/CollisionFiltering.h]

/**
  | This is a test of collision filtering.
  | There is a triangle, a box, and a circle.
  | There are 6 shapes. 3 large and 3 small.
  | The 3 small ones always collide.
  | The 3 large ones never collide.
  | The boxes don't collide with triangles (except
  | if both are small).
  */
pub const k_smallGroup:        i16 = 1;
pub const k_largeGroup:        i16 = -1;
pub const k_defaultCategory:   u16 = 0x0001;
pub const k_triangleCategory:  u16 = 0x0002;
pub const k_boxCategory:       u16 = 0x0004;
pub const k_circleCategory:    u16 = 0x0008;
pub const k_triangleMask:      u16 = 0xFFFF;
pub const k_boxMask:           u16 = 0xFFFF ^ k_triangleCategory;
pub const k_circleMask:        u16 = 0xFFFF;

pub struct CollisionFiltering {
    base: Test,
}

impl Default for CollisionFiltering {
    
    fn default() -> Self {
        todo!();
        /*


            // Ground body
            {
                b2EdgeShape shape;
                shape.Set(b2Vec2(-40.0f, 0.0f), b2Vec2(40.0f, 0.0f));

                b2FixtureDef sd;
                sd.shape = &shape;
                sd.friction = 0.3f;

                b2BodyDef bd;
                b2Body* ground = m_world->CreateBody(&bd);
                ground->CreateFixture(&sd);
            }

            // Small triangle
            b2Vec2 vertices[3];
            vertices[0].Set(-1.0f, 0.0f);
            vertices[1].Set(1.0f, 0.0f);
            vertices[2].Set(0.0f, 2.0f);
            b2PolygonShape polygon;
            polygon.Set(vertices, 3);

            b2FixtureDef triangleShapeDef;
            triangleShapeDef.shape = &polygon;
            triangleShapeDef.density = 1.0f;

            triangleShapeDef.filter.groupIndex = k_smallGroup;
            triangleShapeDef.filter.categoryBits = k_triangleCategory;
            triangleShapeDef.filter.maskBits = k_triangleMask;

            b2BodyDef triangleBodyDef;
            triangleBodyDef.type = b2_dynamicBody;
            triangleBodyDef.position.Set(-5.0f, 2.0f);

            b2Body* body1 = m_world->CreateBody(&triangleBodyDef);
            body1->CreateFixture(&triangleShapeDef);

            // Large triangle (recycle definitions)
            vertices[0] *= 2.0f;
            vertices[1] *= 2.0f;
            vertices[2] *= 2.0f;
            polygon.Set(vertices, 3);
            triangleShapeDef.filter.groupIndex = k_largeGroup;
            triangleBodyDef.position.Set(-5.0f, 6.0f);
            triangleBodyDef.fixedRotation = true; // look at me!

            b2Body* body2 = m_world->CreateBody(&triangleBodyDef);
            body2->CreateFixture(&triangleShapeDef);

            {
                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set(-5.0f, 10.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2PolygonShape p;
                p.SetAsBox(0.5f, 1.0f);
                body->CreateFixture(&p, 1.0f);

                b2PrismaticJointDef jd;
                jd.bodyA = body2;
                jd.bodyB = body;
                jd.enableLimit = true;
                jd.localAnchorA.Set(0.0f, 4.0f);
                jd.localAnchorB.SetZero();
                jd.localAxisA.Set(0.0f, 1.0f);
                jd.lowerTranslation = -1.0f;
                jd.upperTranslation = 1.0f;

                m_world->CreateJoint(&jd);
            }

            // Small box
            polygon.SetAsBox(1.0f, 0.5f);
            b2FixtureDef boxShapeDef;
            boxShapeDef.shape = &polygon;
            boxShapeDef.density = 1.0f;
            boxShapeDef.restitution = 0.1f;

            boxShapeDef.filter.groupIndex = k_smallGroup;
            boxShapeDef.filter.categoryBits = k_boxCategory;
            boxShapeDef.filter.maskBits = k_boxMask;

            b2BodyDef boxBodyDef;
            boxBodyDef.type = b2_dynamicBody;
            boxBodyDef.position.Set(0.0f, 2.0f);

            b2Body* body3 = m_world->CreateBody(&boxBodyDef);
            body3->CreateFixture(&boxShapeDef);

            // Large box (recycle definitions)
            polygon.SetAsBox(2.0f, 1.0f);
            boxShapeDef.filter.groupIndex = k_largeGroup;
            boxBodyDef.position.Set(0.0f, 6.0f);

            b2Body* body4 = m_world->CreateBody(&boxBodyDef);
            body4->CreateFixture(&boxShapeDef);

            // Small circle
            b2CircleShape circle;
            circle.m_radius = 1.0f;

            b2FixtureDef circleShapeDef;
            circleShapeDef.shape = &circle;
            circleShapeDef.density = 1.0f;

            circleShapeDef.filter.groupIndex = k_smallGroup;
            circleShapeDef.filter.categoryBits = k_circleCategory;
            circleShapeDef.filter.maskBits = k_circleMask;

            b2BodyDef circleBodyDef;
            circleBodyDef.type = b2_dynamicBody;
            circleBodyDef.position.Set(5.0f, 2.0f);

            b2Body* body5 = m_world->CreateBody(&circleBodyDef);
            body5->CreateFixture(&circleShapeDef);

            // Large circle
            circle.m_radius *= 2.0f;
            circleShapeDef.filter.groupIndex = k_largeGroup;
            circleBodyDef.position.Set(5.0f, 6.0f);

            b2Body* body6 = m_world->CreateBody(&circleBodyDef);
            body6->CreateFixture(&circleShapeDef)
        */
    }
}

impl CollisionFiltering {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new CollisionFiltering;
        */
    }
}
