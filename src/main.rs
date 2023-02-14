use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use crate::circular_buffer::CircularBuffer;
use crate::light_sampling::{handle_dialer, spawn_sampling_thread};

mod circular_buffer;
mod light_sampling;

const SIZE: usize = 500;

fn main() {
    let history:Arc<Mutex<CircularBuffer<f32>>> = Arc::new(Mutex::new(CircularBuffer::new(SIZE)));
    let dialer:Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let dips:Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    let dialer_thread = handle_dialer(dialer.clone());
    let sampling_thread = spawn_sampling_thread(history.clone(), dialer.clone(),dips.clone());

    loop{
        let history_clone = history.clone();
        let history_lock = history_clone.lock().unwrap();
        if !history_lock.is_empty() {
            let latest_sample = history_lock.get_latest_samples(1);
            println!("Voltage:{:?}\tDip:{:?}", latest_sample, *dips.clone().lock().unwrap());
        }
        drop(history_lock);
        sleep(Duration::from_secs(1));
    }

}
