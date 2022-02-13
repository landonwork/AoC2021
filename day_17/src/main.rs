type Bounds = ((i32, i32), (i32, i32));
type Vector = (i32, i32);

fn main() {
    println!("Part 1: {}", 162*161/2);

    let bounds = ((56, 76), (-162, -134));
    println!("Part 2: {}", part2(bounds));
}

fn part2(bounds: Bounds) -> i32 {

    let mut count = 0;
    
    for x in 11..=(bounds.0.1) {
        for y in (bounds.1.0)..=13041 {
            count += acertou((x,y), bounds);
        }
    }

    count
}

fn acertou(mut vel: Vector, bounds: Bounds) -> i32 {

    let mut pos = (0,0);

    loop {

        // Check
        if pos.0 > bounds.0.1 || pos.1 < bounds.1.0 {
            return 0;
        }

        if pos.0 >= bounds.0.0 && pos.1 <= bounds.1.1 {
            return 1;
        }

        // Step
        pos = (pos.0 + vel.0, pos.1 + vel.1);
        vel = (std::cmp::max(0, vel.0 - 1), vel.1 - 1);
    }

}
