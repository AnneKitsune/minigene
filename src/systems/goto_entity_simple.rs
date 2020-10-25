system!(GotoEntitySimpleSystem, |entities: Entities<'a>,
                                 positions: WriteStorage<
    'a,
    Point,
>,
                                 gotos: ReadStorage<
    'a,
    GotoEntity,
>| {
    let mut v = vec![];
    for (e, _, goto) in (&*entities, &positions, &gotos).join() {
        v.push((e, goto.entity.clone(), goto.speed));
    }
    for (e, t, speed) in v {
        if let Some(target) = positions.get(t).map(|p| p.clone()) {
            let mut p = positions.get_mut(e).unwrap();
            // TODO improve when we have a Time struct
            for i in 0..(speed as usize) {
                let delta_x = target.x - p.x;
                let delta_y = target.y - p.y;
                if delta_x.abs() >= delta_y.abs() {
                    if delta_x > 0 {
                        p.x += 1;
                    } else if delta_x < 0 {
                        p.x -= 1;
                    }
                } else {
                    if delta_y > 0 {
                        p.y += 1;
                    } else if delta_y < 0 {
                        p.y -= 1;
                    }
                }
            }
        }
    }
});
