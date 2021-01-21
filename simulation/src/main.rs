/// A water tank struct
/// There is flow into the water tank a size of the water tank and a level we want the water to be
/// There is an areal of the water tank, it is a box watertank with a hight
/// There is also a valve out of the water tank that controls the outflow
/// 
struct WaterTank {
    level: i32,     // the water level of the tank mm. 
    inflow: f32,    // the outflow if the tank l/s
    areal: i32,     // the areal of the tank mm^2
    height: i32,    // the height of the tank mm
    outflow: f32,   // the outflow of the tank l/s
    set_level: i32, // the wanted level of the tank mm, Real value? or 4 - 20 mA?
}

impl WaterTank {
    fn volume(&self) -> i32 {
        self.areal * self.height
    }

    fn volume_in_cubic_meters(&self) -> f32 {
        self.volume() as f32 / CUBIC_MM_TO_M as f32
    }

    fn volume_in_liters(&self) -> i32 {
        self.volume() / L_TO_CUBIC_MM
    }

    fn update_level(&mut self, seconds_passed: i32) {
        // water volume of the tank = areal * level
        // change in volume = volume + (inflow - outflow) * seconds_passed
        let volume = (self.areal * self.level) as f32 + ((self.inflow - self.outflow) * seconds_passed as f32 * L_TO_CUBIC_MM as f32);
        println!("{} vs {}", (volume / self.areal as f32), (volume / self.areal as f32) as i32);
        self.level = (volume / self.areal as f32) as i32;
    }
}

// The ratio of cubic MM to M
const CUBIC_MM_TO_M: i32 = 1000000000;

// How many cubic MM in one liter, Thousand liters in one cubic M. 
const L_TO_CUBIC_MM: i32 = 1000000;

fn main() {
    let mut tank = WaterTank {
        level: 1000,
        areal: 1000000,
        height: 2000,
        inflow: 20.0,
        outflow: 20.0,
        set_level: 1000,
    };

    for sec in 0..10 {
        tank.update_level(sec);
        tank.outflow = tank.outflow - 1.0;
        println!("Tank level after {} secs: {}", sec, tank.level as f32);
    }

    println!("Hello tank volume: {} mm²!", tank.volume());
    println!("Hello tank volume: {} m²!", tank.volume_in_cubic_meters());
    println!("Hello tank volume: {} liters!", tank.volume_in_liters());
}
