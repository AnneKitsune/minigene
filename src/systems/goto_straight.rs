system!(GotoStraightSystem, |positions: WriteStorage<'a, Point>,
                             gotos: ReadStorage<
    'a,
    GotoStraight,
>| {
    for (mut p, goto) in (&mut positions, &gotos).join() {
        // TODO improve when we have a Time struct
        for i in 0..(goto.speed as usize) {
            let delta_x = goto.target.x - p.x;
            let delta_y = goto.target.y - p.y;
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
});