use std::borrow::{Borrow, BorrowMut};
use std::net::UdpSocket;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, sleep};
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

    let console_print_thread = spawn_console_print_thread(history.clone(), dips.clone());
    let udp_print_thread = spawn_udp_print_thread(history.clone(), dips.clone());



    console_print_thread.join().expect("TODO: panic message");
    udp_print_thread.join().expect("TODO: panic message");

}

fn spawn_console_print_thread(history: Arc<Mutex<CircularBuffer<f32>>>, dips: Arc<Mutex<usize>>) -> JoinHandle<()> {
    thread::spawn(move||{
        loop {
            let history_lock = history.lock().unwrap();
            if !history_lock.is_empty() {
                let latest_sample = history_lock.get_latest_samples(1);
                println!("Voltage:{:?}\tDip:{:?}", latest_sample, *dips.lock().unwrap());
            }
            drop(history_lock);
            sleep(Duration::from_secs(1));
        }
    })
}

fn spawn_udp_print_thread(history: Arc<Mutex<CircularBuffer<f32>>>, dips: Arc<Mutex<usize>>) -> JoinHandle<()> {
    let udp_socket = UdpSocket::bind("192.168.7.2:1234").expect("unable to bind to address");
    let mut buffer = [0;128];

    thread::spawn(move||{
        let mut last_msg = String::new();
        let mut msg = String::new();
        loop{
            let (size, sender_addr) = udp_socket.recv_from(buffer.as_mut_slice()).expect("unable to receive");
            let msg_raw = String::from_utf8(buffer[..size].to_vec()).expect("Unable to convert to String");
            buffer = [0;128];
            msg = msg_raw.trim_matches('\0').trim_matches('\n').to_owned();
            let mut msg_used = "";
            if msg.is_empty(){
                msg_used = last_msg.as_str();
            }else{
                msg_used = msg.as_str();
                last_msg = msg.clone();
            }

            let send = udp_socket.send_to(msg_used.as_bytes(), sender_addr).expect("couldn't send message");
        }
    })
}
