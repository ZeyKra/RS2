use rdev::{listen, Event, EventType, simulate, Button};

use std::io;
use std::{thread, time};
use std::sync::mpsc::{self, Sender};

use rand::Rng;

use rdev::Key::{F8};

fn main() {


    static mut ENABLED: bool = false;
    println!("Entrez le nombre de cps :");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la saisie");

    let mut cps: i32 = input.trim().parse().expect("Mauvais");
    let mut cps_delay: i32 = if cps > 0 { 1000 / cps } else { 1000 };
    println!("Le delay est de {} ms", cps_delay);

    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    // Définissez le callback avec une closure
    let callback = move |event: Event| {
        if let EventType::KeyPress(_key) = event.event_type{
            if _key == F8 {
                if let Err(err) = tx_clone.send(event) {
                    eprintln!("Failed to send event: {}", err);
                }
            }
        }
    };



    let mut randomizer_amount: f32 = 0.4;
    let mut randomizer: bool = false;
    // Lancez un thread pour écouter les événements
    thread::spawn( move ||
    unsafe {
        loop {
            if ENABLED {
                let mut rng = rand::thread_rng();
                let _delay: f32 = if randomizer {
                    cps_delay as f32 + (cps_delay as f32 * rng.gen_range(0.00..=randomizer_amount))
                } else {
                    cps_delay as f32
                };
                let now = time::Instant::now();
                thread::sleep(time::Duration::from_millis(_delay as u64));
                send(&EventType::ButtonPress(Button::Left));
                send(&EventType::ButtonRelease(Button::Left));
                println!("Clic {:?} | {} cps", now.elapsed(), 1000.0 / now.elapsed().as_millis() as f32);
            }

        }
    });

    thread::spawn(move || {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });


    // Thread principal pour traiter les événements
    for received_event in rx {
        eprintln!("event reçu {:?}", received_event);
        unsafe {
            ENABLED = if ENABLED == false { true } else { false };
            println!("Statu de ENABLED : {}", ENABLED);
        }

    }

}

fn send(event_type: &EventType) {

    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }

    // Let ths OS catchup (at least macOS)
}