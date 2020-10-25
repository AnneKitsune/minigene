// Run after ApplyEffectorsSystem
system!(RemoveOutdatedEffectorSystem<E: Send+Sync+'static>, |
        effectors: WriteStorage<'a, Comp<EffectorSet<E>>>,
        time: ReadExpect<'a, Time>| {
    for (mut eff) in (&mut effectors,).join() {
        (eff.0).0.effectors.retain(|e| {
            if let Some(mut d) = e.disable_in {
                d -= time.delta_seconds() as f64;
                d > 0.0
            } else {
                true
            }
        });
    }
});
