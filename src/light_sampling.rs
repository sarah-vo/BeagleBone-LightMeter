use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::{fs, thread};
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use crate::circular_buffer::CircularBuffer;

const METER_DIR:&str = "/sys/bus/iio/devices/iio:device0/in_voltage1_raw";
const DIALER_DIR:&str = "/sys/bus/iio/devices/iio:device0/in_voltage0_raw";

const HYSTERESIS:f32 = 0.03;
const DIFFERENCE_THRESHOLD:f32 = 0.1;

pub fn spawn_sampling_thread(
    history: Arc<Mutex<CircularBuffer<f32>>>,
    dialer:Arc<Mutex<usize>>,
    dips: Arc<Mutex<usize>>,
    run_state: Arc<RwLock<bool>>
) -> JoinHandle<()>{
    thread::spawn(move||{
        while *run_state.read().unwrap(){
            sleep(Duration::from_millis(1));
            let mut history_lock = history.lock().unwrap();

            let mut light_meter_raw = fs::read_to_string(METER_DIR)
                .expect("Failed to read path");
            light_meter_raw.pop();
            let mut light_meter_voltage: f32 = light_meter_raw.parse().expect("Failed to parse to number");
            light_meter_voltage = light_meter_voltage / 4095.0;

            history_lock.push(light_meter_voltage);

            let dialer_lock = dialer.lock().unwrap();
            if history_lock.capacity != *dialer_lock && *dialer_lock > 0{
                history_lock.resize(*dialer_lock);
            }
            drop(dialer_lock);

            if !history_lock.is_empty(){
                calculate_dips(dips.clone(), &mut history_lock);
            }
        }
        println!("Thread light exited");

    })
}

fn calculate_dips(dips: Arc<Mutex<usize>>, history_lock: &mut MutexGuard<CircularBuffer<f32>>) {
    let mut dips_lock = dips.lock().expect("unable to lock dips");
    let history_buffer = history_lock.get_latest_samples(history_lock.size);
    let mut index = history_lock.size - 1;
    let mut session_dips: usize = 0;
    while index != 0 {
        let light_difference = history_buffer.get(index).unwrap() - history_buffer.get(index - 1).unwrap() + HYSTERESIS;
        if light_difference >= DIFFERENCE_THRESHOLD {
            session_dips += 1;
        }
        index -= 1;
    }
    *dips_lock = session_dips;
}

pub fn handle_dialer(dialer:Arc<Mutex<usize>>, run_state: Arc<RwLock<bool>>) -> JoinHandle<()>{
    thread::spawn(move||{
        while *run_state.read().unwrap() {
            sleep(Duration::from_secs(1));

            let mut dialer_lock = dialer.lock().unwrap();

            let mut raw_dialer_value = fs::read_to_string(DIALER_DIR)
                .expect("Failed to read path");
            raw_dialer_value.pop();
            let dialer_value:usize = raw_dialer_value.parse().expect("Failed to parse to number");
            if dialer_value != 0{ *dialer_lock = dialer_value + 1; }
        }
        println!("Thread dialer exited");
    })
}

