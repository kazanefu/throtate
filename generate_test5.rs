use std::fs::File;
use std::io::Write;

fn main() {
    let mut f = File::create("src/courses_ron/test5.ron").unwrap();
    writeln!(f, "(").unwrap();
    writeln!(f, "    entities: [").unwrap();

    let mut x: f32 = 0.0;
    
    // Initial Spawn point enclosed
    writeln!(f, "        (x: {:.1}, y: -200.0, kind: Ground(width: 1000.0, height: 100.0)),", x).unwrap(); // starting floor
    writeln!(f, "        (x: {:.1}, y: 800.0, kind: Ground(width: 2000.0, height: 100.0)),", x).unwrap(); // starting ceiling
    writeln!(f, "        (x: {:.1}, y: 300.0, kind: Ground(width: 100.0, height: 1500.0)),", -800.0).unwrap(); // Start wall behind player
    
    x += 1000.0;
    
    // 200 segments => 200,000 units => VERY long course
    let num_segments = 200;
    let mut checkpoint_counter = 1;
    
    for i in 0..num_segments {
        let y_base = (i as f32 % 7.0) * 80.0 - 100.0; // Moderate elevation shifts between -100 and +380
        let ceiling_y = y_base + 900.0;
        let floor_y = y_base - 800.0;
        
        // Solid continuous bounding box for this segment!
        writeln!(f, "        // Segment {} Boundaries", i).unwrap();
        // Ceiling
        writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 1500.0, height: 100.0)),", x + 500.0, ceiling_y).unwrap();
        // Floor (acts as distance floor for pits, catching players that go out of bounds)
        writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 1500.0, height: 100.0)),", x + 500.0, floor_y).unwrap();
        
        // If passing between major elevations, add vertical walls so they can't fly out the gaps
        let prev_ceiling_y = if i == 0 { 800.0 } else { (((i - 1) as f32 % 7.0) * 80.0 - 100.0) + 900.0 };
        let prev_floor_y = if i == 0 { -200.0 - 800.0 } else { (((i - 1) as f32 % 7.0) * 80.0 - 100.0) - 800.0 };
        
        if (ceiling_y - prev_ceiling_y).abs() > 40.0 {
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 100.0, height: 600.0)),", x, (ceiling_y + prev_ceiling_y) / 2.0).unwrap();
        }
        if (floor_y - prev_floor_y).abs() > 40.0 {
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 100.0, height: 600.0)),", x, (floor_y + prev_floor_y) / 2.0).unwrap();
        }

        let seg_type = i % 5; 
        
        if seg_type == 0 {
            // The Gauntlet: Many Turrets
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 1000.0, height: 40.0)),", x + 500.0, y_base - 100.0).unwrap();
            for j in 0..7 {
                let tx = x + 150.0 * (j as f32);
                writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Turret(interval: 0.6, rotation: 0.0)),", tx, y_base).unwrap();
                writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Turret(interval: 0.8, rotation: 3.14)),", tx + 75.0, ceiling_y - 200.0).unwrap();
            }
        } else if seg_type == 1 {
            // Precision Jumps over death
            for j in 0..4 {
                let dx = x + 250.0 * (j as f32);
                writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 60.0, height: 20.0)),", dx, y_base).unwrap();
                writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Death),", dx + 125.0, y_base - 300.0).unwrap();
            }
        } else if seg_type == 2 {
            // Speed Wall Dash
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 400.0, height: 20.0)),", x + 200.0, y_base).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Breakable(required_speed: 120.0)),", x + 500.0, y_base + 50.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Breakable(required_speed: 120.0)),", x + 500.0, y_base + 100.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Breakable(required_speed: 120.0)),", x + 500.0, y_base + 150.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 400.0, height: 20.0)),", x + 800.0, y_base).unwrap();
            // A turret to punish waiting too long!
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Turret(interval: 1.0, rotation: -1.57)),", x + 800.0, y_base + 600.0).unwrap();
        } else if seg_type == 3 {
            // Vertical weave past turrets and blocks
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 120.0, height: 20.0)),", x + 100.0, y_base - 50.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Turret(interval: 0.5, rotation: -1.57)),", x + 250.0, ceiling_y - 300.0).unwrap();
            
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 120.0, height: 20.0)),", x + 400.0, y_base + 150.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Turret(interval: 0.6, rotation: 1.57)),", x + 550.0, y_base - 200.0).unwrap();
            
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 120.0, height: 20.0)),", x + 700.0, y_base + 50.0).unwrap();
            
            // Add some death catchers below
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Death),", x + 500.0, y_base - 500.0).unwrap();
        } else {
            // Breakable jumps over death
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 200.0, height: 20.0)),", x + 100.0, y_base).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Death),", x + 350.0, y_base - 200.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Breakable(required_speed: 100.0)),", x + 500.0, y_base + 100.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Death),", x + 650.0, y_base - 200.0).unwrap();
            writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Ground(width: 200.0, height: 20.0)),", x + 900.0, y_base).unwrap();
        }

        // Add regular checkpoints! (A checkpoint every segment)
        writeln!(f, "        (x: {:.1}, y: {:.1}, kind: Checkpoint(priority: {})),", x + 950.0, y_base, checkpoint_counter).unwrap();
        checkpoint_counter += 1;
        
        x += 1000.0;
    }
    
    // Final Goal Enclosure
    writeln!(f, "        (x: {:.1}, y: 0.0, kind: Ground(width: 1000.0, height: 100.0)),", x + 500.0).unwrap(); // Final stretch
    writeln!(f, "        (x: {:.1}, y: 150.0, kind: Goal),", x + 700.0).unwrap();
    writeln!(f, "        (x: {:.1}, y: 1000.0, kind: Ground(width: 1500.0, height: 200.0)),", x + 500.0).unwrap(); // Final Ceiling
    writeln!(f, "        (x: {:.1}, y: 500.0, kind: Ground(width: 200.0, height: 2000.0)),", x + 1100.0).unwrap(); // Final Wall so they can't go any further
    
    writeln!(f, "    ]").unwrap();
    writeln!(f, ")").unwrap();
}
