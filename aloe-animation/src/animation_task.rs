crate::ix!();

#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct AnimationTask<'a> {
    component:         WeakReference<Component<'a>>,
    proxy:             ComponentSafePointer<'a, Component<'a>>,
    destination:       Rectangle<i32>,
    dest_alpha:        f64,
    ms_elapsed:        i32,
    ms_total:          i32,
    start_speed:       f64,
    mid_speed:         f64,
    end_speed:         f64,
    last_progress:     f64,
    left:              f64,
    top:               f64,
    right:             f64,
    bottom:            f64,
    alpha:             f64,
    is_moving:         bool,
    is_changing_alpha: bool,
}

impl<'a> Drop for AnimationTask<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            proxy.deleteAndZero();
         */
    }
}

impl<'a> AnimationTask<'a> {

    pub fn new(c: *mut Component<'a>) -> Self {
    
        todo!();
        /*
        : component(c),

        
        */
    }
    
    pub fn reset(
        &mut self, 
        final_bounds:                 &Rectangle<i32>,
        final_alpha:                  f32,
        milliseconds_to_spend_moving: i32,
        use_proxy_component:          bool,
        start_spd:                    f64,
        end_spd:                      f64)  {

        todo!();
        /*
            msElapsed = 0;
            msTotal = jmax (1, millisecondsToSpendMoving);
            lastProgress = 0;
            destination = finalBounds;
            destAlpha = finalAlpha;

            isMoving = (finalBounds != component->getBounds());
            isChangingAlpha = (finalAlpha != component->getAlpha());

            left    = component->getX();
            top     = component->getY();
            right   = component->getRight();
            bottom  = component->getBottom();
            alpha   = component->getAlpha();

            const double invTotalDistance = 4.0 / (startSpd + endSpd + 2.0);
            startSpeed = jmax (0.0, startSpd * invTotalDistance);
            midSpeed = invTotalDistance;
            endSpeed = jmax (0.0, endSpd * invTotalDistance);

            proxy.deleteAndZero();

            if (useProxyComponent)
                proxy = new AnimationTaskProxyComponent (*component);

            component->setVisible (! useProxyComponent);
        */
    }
    
    pub fn use_timeslice(&mut self, elapsed: i32) -> bool {
        
        todo!();
        /*
            if (auto* c = proxy != nullptr ? proxy.getComponent()
                                           : component.get())
            {
                msElapsed += elapsed;
                double newProgress = msElapsed / (double) msTotal;

                if (newProgress >= 0 && newProgress < 1.0)
                {
                    const WeakReference<AnimationTask> weakRef (this);
                    newProgress = timeToDistance (newProgress);
                    const double delta = (newProgress - lastProgress) / (1.0 - lastProgress);
                    jassert (newProgress >= lastProgress);
                    lastProgress = newProgress;

                    if (delta < 1.0)
                    {
                        bool stillBusy = false;

                        if (isMoving)
                        {
                            left   += (destination.getX()      - left)   * delta;
                            top    += (destination.getY()      - top)    * delta;
                            right  += (destination.getRight()  - right)  * delta;
                            bottom += (destination.getBottom() - bottom) * delta;

                            const Rectangle<int> newBounds (roundToInt (left),
                                                            roundToInt (top),
                                                            roundToInt (right - left),
                                                            roundToInt (bottom - top));

                            if (newBounds != destination)
                            {
                                c->setBounds (newBounds);
                                stillBusy = true;
                            }
                        }

                        // Check whether the animation was cancelled/deleted during
                        // a callback during the setBounds method
                        if (weakRef.wasObjectDeleted())
                            return false;

                        if (isChangingAlpha)
                        {
                            alpha += (destAlpha - alpha) * delta;
                            c->setAlpha ((float) alpha);
                            stillBusy = true;
                        }

                        if (stillBusy)
                            return true;
                    }
                }
            }

            moveToFinalDestination();
            return false;
        */
    }
    
    pub fn move_to_final_destination(&mut self)  {
        
        todo!();
        /*
            if (component != nullptr)
            {
                const WeakReference<AnimationTask> weakRef (this);
                component->setAlpha ((float) destAlpha);
                component->setBounds (destination);

                if (! weakRef.wasObjectDeleted())
                    if (proxy != nullptr)
                        component->setVisible (destAlpha > 0);
            }
        */
    }
    
    pub fn time_to_distance(&self, time: f64) -> f64 {
        
        todo!();
        /*
            return (time < 0.5) ? time * (startSpeed + time * (midSpeed - startSpeed))
                                : 0.5 * (startSpeed + 0.5 * (midSpeed - startSpeed))
                                    + (time - 0.5) * (midSpeed + (time - 0.5) * (endSpeed - midSpeed));
        */
    }
}

