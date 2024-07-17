crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2PolygonContact.h]
pub struct b2PolygonContact {
    base: b2Contact,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2PolygonContact.cpp]
impl b2PolygonContact {

    pub fn create(&mut self, 
        fixturea:  *mut b2Fixture,
        _1:        i32,
        fixtureb:  *mut b2Fixture,
        _3:        i32,
        allocator: *mut b2BlockAllocator) -> *mut b2Contact {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2PolygonContact));
        return new (mem) b2PolygonContact(fixtureA, fixtureB);
        */
    }

    pub fn destroy(&mut self, 
        contact:   *mut b2Contact,
        allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            ((b2PolygonContact*)contact)->~b2PolygonContact();
        allocator->Free(contact, sizeof(b2PolygonContact));
        */
    }

    pub fn new(
        fixturea: *mut b2Fixture,
        fixtureb: *mut b2Fixture) -> Self {
    
        todo!();
        /*
        : b2contact(fixtureA, 0, fixtureB, 0),

            b2Assert(m_fixtureA->GetType() == b2Shape::e_polygon);
        b2Assert(m_fixtureB->GetType() == b2Shape::e_polygon);
        */
    }

    pub fn evaluate(&mut self, 
        manifold: *mut b2Manifold,
        xfa:      &b2Transform,
        xfb:      &b2Transform)  {
        
        todo!();
        /*
            b2CollidePolygons(  manifold,
                            (b2PolygonShape*)m_fixtureA->GetShape(), xfA,
                            (b2PolygonShape*)m_fixtureB->GetShape(), xfB);
        */
    }
}
