use std::thread;
use std::time;
use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};
use std::io;

/// A water tank struct
/// There is flow into the water tank a size of the water tank and a level we want the water to be
/// There is an areal of the water tank, it is a box watertank with a hight
/// There is also a valve out of the water tank that controls the outflow
/// 
struct WaterTank {
    level: i64,         // the water level of the tank mm. 
    inflow_mean: f32,   // the mean inflow if the tank l/s
    inflow_stddev: f32, // the stddev of inflow of the tank l/s
    inflow: f64,        // the inflow right now
    areal: i64,         // the areal of the tank mm^2
    height: i64,        // the height of the tank mm
    outflow: f64,       // the outflow of the tank l/s
    set_level: i64,     // the wanted level of the tank mm, Real value? or 4 - 20 mA?
}

impl WaterTank {
    fn volume(&self) -> i64 {
        self.areal * self.height
    }

    fn volume_in_cubic_meters(&self) -> f32 {
        self.volume() as f32 / CUBIC_MM_TO_M as f32
    }

    fn volume_in_liters(&self) -> i64 {
        self.volume() / L_TO_CUBIC_MM
    }

    fn update_level(&mut self, seconds_passed: f32) {
        // water volume of the tank = areal * level
        // change in volume = volume + (inflow - outflow) * seconds_passed
        let volume = (self.areal * self.level) as f64 + ((self.inflow - self.outflow) * seconds_passed as f64 * L_TO_CUBIC_MM as f64);
        self.level = (volume / self.areal as f64) as i64;
    }

    fn update_inflow(&mut self) {
        let mut rng = thread_rng();
        let normal = Normal::new(self.inflow_mean, self.inflow_stddev).unwrap();
        let v = normal.sample(&mut rng);
        self.inflow = v as f64;
    }
}

// The ratio of cubic MM to M
const CUBIC_MM_TO_M: i64 = 1000000000;

// How many cubic MM in one liter, Thousand liters in one cubic M. 
const L_TO_CUBIC_MM: i64 = 1000000;

fn main() {
    let mut tank = WaterTank {
        level: 1000,
        areal: 1000000,
        height: 2000,
        inflow: 20.0,
        inflow_mean: 20.0,
        inflow_stddev: 3.0,
        outflow: 20.0,
        set_level: 1000,
    };
    // let mut t1 = time::SystemTime::now();
    // let start_time = time::SystemTime::now();
    

    let mut wtr2 = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_path("out.csv").unwrap();

    // When writing records without Serde, the header record is written just
    // like any other record.
    wtr.write_record(&["Seconds lapsed", "tank level", "tank inflow"]).unwrap();


    for sec in 0..1000 {
        // thread::sleep(time::Duration::from_millis(500));
        // let sec = t1.elapsed().unwrap().as_millis() as f32 / 1000.0;
        tank.update_inflow();
        tank.update_level(10.0);
        // tank.outflow = tank.outflow - 1.0;
        let mut data: Vec<String> = Vec::new();
        // data.push(format!("{}", start_time.elapsed().unwrap().as_secs_f32()));
        data.push(format!("{}", sec*10));
        data.push(format!("{}", tank.level as f32,));
        data.push(format!("{}", tank.inflow));
        wtr.write_record(&data).unwrap(); 
        // println!("{} secs, {} mm, {} l/s", start_time.elapsed().unwrap().as_secs_f32(), tank.level as f32, tank.inflow);
        // t1 = time::SystemTime::now();
    }

    wtr.flush().unwrap();

    // println!("Hello tank volume: {} mm³!", tank.volume());
    // println!("Hello tank volume: {} m³!", tank.volume_in_cubic_meters());
    // println!("Hello tank volume: {} liters!", tank.volume_in_liters());
}
