crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2WorldCallbacks.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2WorldCallbacks.cpp]

/**
  | Joints and fixtures are destroyed when their
  | associated body is destroyed. Implement this
  | listener so that you may nullify references to
  | these joints and shapes.
  */
pub trait b2DestructionListener {

    /**
      | Called when any joint is about to be destroyed
      | due to the destruction of one of its attached
      | bodies.
      |
      */
    fn say_goodbye_to_joint(&mut self, joint: *mut b2Joint);

    /**
      | Called when any fixture is about to be
      | destroyed due to the destruction of
      | its parent body.
      |
      */
    fn say_goodbye_to_fixture(&mut self, fixture: *mut b2Fixture);
}

/**
  | Implement this class to provide collision
  | filtering. In other words, you can implement
  | this class if you want finer control over
  | contact creation.
  */
pub trait b2ContactFilter {

    /**
      | Return true if contact calculations should be
      | performed between these two shapes.
      |
      | If you implement your own collision filter you
      | may want to build from this implementation.
      |
      | -----------
      | @warning
      | 
      | for performance reasons this is only
      | called when the AABBs begin to overlap.
      */
    fn should_collide(&mut self, 
        fixturea: *mut b2Fixture,
        fixtureb: *mut b2Fixture) -> bool {
        
        todo!();
        /*
            const b2Filter& filterA = fixtureA->GetFilterData();
        const b2Filter& filterB = fixtureB->GetFilterData();

        if (filterA.groupIndex == filterB.groupIndex && filterA.groupIndex != 0)
        {
            return filterA.groupIndex > 0;
        }

        bool collide = (filterA.maskBits & filterB.categoryBits) != 0 && (filterA.categoryBits & filterB.maskBits) != 0;
        return collide;
        */
    }
}

/**
  | Contact impulses for reporting. Impulses are
  | used instead of forces because sub-step forces
  | may approach infinity for rigid body
  | collisions. These match up one-to-one with the
  | contact points in b2Manifold.
  */
pub struct b2ContactImpulse
{
    normal_impulses:  [f32; b2_maxManifoldPoints],
    tangent_impulses: [f32; b2_maxManifoldPoints],
    count:            i32,
}

/**
  | Implement this class to get contact
  | information. You can use these results
  | for things like sounds and game logic.
  | You can also get contact results by traversing
  | the contact lists after the time step.
  | However, you might miss some contacts
  | because continuous physics leads to
  | sub-stepping.
  | 
  | Additionally you may receive multiple
  | callbacks for the same contact in a single
  | time step.
  | 
  | You should strive to make your callbacks
  | efficient because there may be many
  | callbacks per time step.
  | 
  | -----------
  | @warning
  | 
  | You cannot create/destroy Box2D entities
  | inside these callbacks.
  |
  */
pub trait b2ContactListener {

    /**
       Called when two fixtures begin to touch.
      */
    fn begin_contact(&mut self, contact: *mut b2Contact)  {
        
        todo!();
        /*
            B2_NOT_USED(contact);
        */
    }

    /**
       Called when two fixtures cease to touch.
      */
    fn end_contact(&mut self, contact: *mut b2Contact)  {
        
        todo!();
        /*
            B2_NOT_USED(contact);
        */
    }

    /**
      | This is called after a contact is updated.
      | This allows you to inspect a contact
      | before it goes to the solver. If you are
      | careful, you can modify the contact
      | manifold (e.g. disable contact).
      | 
      | A copy of the old manifold is provided
      | so that you can detect changes.
      | 
      | -----------
      | @note
      | 
      | this is called only for awake bodies.
      | ----------
      | @note
      | 
      | this is called even when the number of
      | contact points is zero.
      | ----------
      | @note
      | 
      | this is not called for sensors.
      | ----------
      | @note
      | 
      | if you set the number of contact points
      | to zero, you will not get an EndContact
      | callback. However, you may get a BeginContact
      | callback the next step.
      |
      */
    fn pre_solve(&mut self, 
        contact:      *mut b2Contact,
        old_manifold: *const b2Manifold)  {
        
        todo!();
        /*
            B2_NOT_USED(contact);
            B2_NOT_USED(oldManifold);
        */
    }

    /**
      | This lets you inspect a contact after
      | the solver is finished. This is useful
      | for inspecting impulses.
      | 
      | -----------
      | @note
      | 
      | the contact manifold does not include
      | time of impact impulses, which can be
      | arbitrarily large if the sub-step is
      | small. Hence the impulse is provided
      | explicitly in a separate data structure.
      | ----------
      | @note
      | 
      | this is only called for contacts that
      | are touching, solid, and awake.
      |
      */
    fn post_solve(&mut self, 
        contact: *mut b2Contact,
        impulse: *const b2ContactImpulse)  {
        
        todo!();
        /*
            B2_NOT_USED(contact);
            B2_NOT_USED(impulse);
        */
    }
}

/**
   Callback class for AABB queries.
   See b2World::Query
  */
pub trait b2QueryCallback {

    /**
      | Called for each fixture found in the query
      | AABB.
      |
      | @return false to terminate the query.
      */
    fn report_fixture(&mut self, fixture: *mut b2Fixture) -> bool;
}

/**
   Callback class for ray casts.
   See b2World::RayCast
  */
pub trait b2RayCastCallback {

    /**
      | Called for each fixture found in the
      | query. You control how the ray cast
      | proceeds by returning a float:
      |
      | return -1: ignore this fixture and
      | continue
      |
      | return 0: terminate the ray cast
      |
      | return fraction: clip the ray to this
      | point
      |
      | return 1: don't clip the ray and continue
      |
      | @param fixture the fixture hit by the ray
      |
      | @param point the point of initial
      | intersection
      |
      | @param normal the normal vector at the
      | point of intersection
      |
      | @return -1 to filter, 0 to terminate,
      | fraction to clip the ray for closest hit,
      | 1 to continue
      */
    fn report_fixture(&mut self, 
            fixture:  *mut b2Fixture,
            point:    &b2Vec2,
            normal:   &b2Vec2,
            fraction: f32) -> f32;
}
