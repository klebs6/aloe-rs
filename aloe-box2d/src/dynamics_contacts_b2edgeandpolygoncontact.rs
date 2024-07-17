crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2EdgeAndPolygonContact.h]

pub struct b2EdgeAndPolygonContact {
    base: b2Contact,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2EdgeAndPolygonContact.cpp]
impl b2EdgeAndPolygonContact {

    pub fn create(&mut self, 
        fixturea:  *mut b2Fixture,
        _1:        i32,
        fixtureb:  *mut b2Fixture,
        _3:        i32,
        allocator: *mut b2BlockAllocator) -> *mut b2Contact {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2EdgeAndPolygonContact));
        return new (mem) b2EdgeAndPolygonContact(fixtureA, fixtureB);
        */
    }
    
    pub fn destroy(&mut self, 
        contact:   *mut b2Contact,
        allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            ((b2EdgeAndPolygonContact*)contact)->~b2EdgeAndPolygonContact();
        allocator->Free(contact, sizeof(b2EdgeAndPolygonContact));
        */
    }
    
    pub fn new(
        fixturea: *mut b2Fixture,
        fixtureb: *mut b2Fixture) -> Self {
    
        todo!();
        /*
        : b2contact(fixtureA, 0, fixtureB, 0),

            b2Assert(m_fixtureA->GetType() == b2Shape::e_edge);
        b2Assert(m_fixtureB->GetType() == b2Shape::e_polygon);
        */
    }
    
    pub fn evaluate(&mut self, 
        manifold: *mut b2Manifold,
        xfa:      &b2Transform,
        xfb:      &b2Transform)  {
        
        todo!();
        /*
            b2CollideEdgeAndPolygon(    manifold,
                                    (b2EdgeShape*)m_fixtureA->GetShape(), xfA,
                                    (b2PolygonShape*)m_fixtureB->GetShape(), xfB);
        */
    }
}
