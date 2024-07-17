crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Builds/Android/app/src/main/assets/Box2DTests/CompoundShapes.h]

/**
   TODO_ERIN test joints on compounds.
  */
pub struct CompoundShapes {
    base: Test,
}

impl Default for CompoundShapes {
    
    fn default() -> Self {
        todo!();
        /*


            {
                b2BodyDef bd;
                bd.position.Set(0.0f, 0.0f);
                b2Body* body = m_world->CreateBody(&bd);

                b2EdgeShape shape;
                shape.Set(b2Vec2(50.0f, 0.0f), b2Vec2(-50.0f, 0.0f));

                body->CreateFixture(&shape, 0.0f);
            }

            {
                b2CircleShape circle1;
                circle1.m_radius = 0.5f;
                circle1.m_p.Set(-0.5f, 0.5f);

                b2CircleShape circle2;
                circle2.m_radius = 0.5f;
                circle2.m_p.Set(0.5f, 0.5f);

                for (int i = 0; i < 10; ++i)
                {
                    float32 x = RandomFloat(-0.1f, 0.1f);
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(x + 5.0f, 1.05f + 2.5f * i);
                    bd.angle = RandomFloat(-b2_pi, b2_pi);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&circle1, 2.0f);
                    body->CreateFixture(&circle2, 0.0f);
                }
            }

            {
                b2PolygonShape polygon1;
                polygon1.SetAsBox(0.25f, 0.5f);

                b2PolygonShape polygon2;
                polygon2.SetAsBox(0.25f, 0.5f, b2Vec2(0.0f, -0.5f), 0.5f * b2_pi);

                for (int i = 0; i < 10; ++i)
                {
                    float32 x = RandomFloat(-0.1f, 0.1f);
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(x - 5.0f, 1.05f + 2.5f * i);
                    bd.angle = RandomFloat(-b2_pi, b2_pi);
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&polygon1, 2.0f);
                    body->CreateFixture(&polygon2, 2.0f);
                }
            }

            {
                b2Transform xf1;
                xf1.q.Set(0.3524f * b2_pi);
                xf1.p = xf1.q.GetXAxis();

                b2Vec2 vertices[3];

                b2PolygonShape triangle1;
                vertices[0] = b2Mul(xf1, b2Vec2(-1.0f, 0.0f));
                vertices[1] = b2Mul(xf1, b2Vec2(1.0f, 0.0f));
                vertices[2] = b2Mul(xf1, b2Vec2(0.0f, 0.5f));
                triangle1.Set(vertices, 3);

                b2Transform xf2;
                xf2.q.Set(-0.3524f * b2_pi);
                xf2.p = -xf2.q.GetXAxis();

                b2PolygonShape triangle2;
                vertices[0] = b2Mul(xf2, b2Vec2(-1.0f, 0.0f));
                vertices[1] = b2Mul(xf2, b2Vec2(1.0f, 0.0f));
                vertices[2] = b2Mul(xf2, b2Vec2(0.0f, 0.5f));
                triangle2.Set(vertices, 3);

                for (int32 i = 0; i < 10; ++i)
                {
                    float32 x = RandomFloat(-0.1f, 0.1f);
                    b2BodyDef bd;
                    bd.type = b2_dynamicBody;
                    bd.position.Set(x, 2.05f + 2.5f * i);
                    bd.angle = 0.0f;
                    b2Body* body = m_world->CreateBody(&bd);
                    body->CreateFixture(&triangle1, 2.0f);
                    body->CreateFixture(&triangle2, 2.0f);
                }
            }

            {
                b2PolygonShape bottom;
                bottom.SetAsBox( 1.5f, 0.15f );

                b2PolygonShape left;
                left.SetAsBox(0.15f, 2.7f, b2Vec2(-1.45f, 2.35f), 0.2f);

                b2PolygonShape right;
                right.SetAsBox(0.15f, 2.7f, b2Vec2(1.45f, 2.35f), -0.2f);

                b2BodyDef bd;
                bd.type = b2_dynamicBody;
                bd.position.Set( 0.0f, 2.0f );
                b2Body* body = m_world->CreateBody(&bd);
                body->CreateFixture(&bottom, 4.0f);
                body->CreateFixture(&left, 4.0f);
                body->CreateFixture(&right, 4.0f);
            
        */
    }
}

impl CompoundShapes {

    pub fn create() -> *mut Test {
        
        todo!();
        /*
            return new CompoundShapes;
        */
    }
}
