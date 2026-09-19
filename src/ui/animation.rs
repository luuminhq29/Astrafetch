pub fn breathing(frame:u64)->f64{let phase=(frame%120)as f64/120.0;0.5+0.5*(phase*std::f64::consts::TAU).sin()}
