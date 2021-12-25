use std::num;

struct Target {
    x: (i32, i32),
    y: (i32, i32)
}

fn main() {
    // let target = Target { x: (20,30), y: (-10,-5) };
    let target = Target { x: (265,287), y:(-103,-58) };
    let min_dx = (2.0 * f64::from(target.x.0)).sqrt() as i32;

    let mut best = (0,0,0); // x,y,peak

    let mut x0 = min_dx;     // Initital velocity
    while x0 < min_dx + 3 {     // Try x0-values
        let mut y0 = 1;
        while y0 <= -target.y.0 {     // Try y0-values
            println!("Trying {},{} - ", x0, y0);
            let mut pos = (0, 0);
            let mut vel = (x0, y0);

            while  pos.0 <= target.x.1 && pos.1 >= target.y.0 {
                // print!("({},{})({},{}) ", pos.0, pos.1, vel.0, vel.1);

                let peak_y = (pos.1+1)*pos.1/2; 
                if pos.0 >= target.x.0 && pos.1 <= target.y.1 {
                    println!("Hit at {},{} for initial x:{}, y:{}. Peak: {}", pos.0, pos.1, x0,y0, peak_y);

                    if best.2 < peak_y {
                        best = (x0,y0,peak_y);
                    }
                    break;
                } 
                pos.0 = pos.0 + vel.0;
                pos.1 = pos.1 + vel.1; 
                vel.0 = if vel.0 > 0 { vel.0 - 1 } else { 0 };
                vel.1 -= 1;
            }
            // println!();
            y0 += 1;
        }
        x0 += 1;
    }
    println!("Part 1: Best trajectory for {},{} reaches {}", best.0, best.1, best.2);

    let mut hitcount = 0;
    let mut x0 = min_dx;     // Initital velocity
    while x0 <= target.x.1 {     // Try x0-values
        let mut y0 = target.y.0;
        while y0 <= -target.y.0 {     // Try y0-values
            // println!("Trying {},{} - ", x0, y0);
            let mut pos = (0, 0);
            let mut vel = (x0, y0);

            while  pos.0 <= target.x.1 && pos.1 >= target.y.0 {
                // print!("({},{})({},{}) ", pos.0, pos.1, vel.0, vel.1);

                let peak_y = (pos.1+1)*pos.1/2; 
                if pos.0 >= target.x.0 && pos.1 <= target.y.1 {
                    println!("Hit at {},{} for initial x:{}, y:{}. Peak: {}", pos.0, pos.1, x0,y0, peak_y);
                    hitcount += 1;
                    break;
                } 
                pos.0 = pos.0 + vel.0;
                pos.1 = pos.1 + vel.1; 
                vel.0 = if vel.0 > 0 { vel.0 - 1 } else { 0 };
                vel.1 -= 1;
            }
            // println!();
            y0 += 1;
        }
        x0 += 1;
    }
    println!("Part 2: A total of {} trajectories hit the target", hitcount);
}
// Try x from sqrt(2*min x in target)
// Try x till x > target xmax when y < ymax
// Try y till |dy| > target ymax-ymin

