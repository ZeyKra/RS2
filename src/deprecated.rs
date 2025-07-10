use rdev::{listen, simulate, Event, EventType, Button};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Créer un flag partagé pour indiquer si l'auto-click est actif
    let is_clicking = Arc::new(Mutex::new(false));
    // Créer un flag pour indiquer si on simule un clic
    let is_simulating = Arc::new(Mutex::new(false));

    // Clone des flags pour l'envoyer au thread d'écoute
    let is_clicking_listener = Arc::clone(&is_clicking);
    let is_simulating_listener = Arc::clone(&is_simulating);

    // Lancer l'écoute des événements dans un thread séparé
    thread::spawn(move || {
        // Fonction de rappel pour gérer les événements
        let callback = move |event: Event| {
            // Verrouiller l'accès aux flags partagés
            let mut clicking = is_clicking_listener.lock().unwrap();
            let simulating = is_simulating_listener.lock().unwrap();

            // Ignorer les événements si on simule un clic
            if *simulating {
                return;
            }

            match event.event_type {
                // Si le bouton gauche de la souris est pressé, on active l'auto-clicker
                EventType::ButtonPress(Button::Left) => {
                    println!("Mouse button pressed: starting auto-click.");
                    *clicking = true;  // Activer l'auto-click
                }
                // Si le bouton gauche est relâché, on désactive l'auto-clicker
                EventType::ButtonRelease(Button::Left) => {
                    println!("Mouse button released: stopping auto-click.");
                    *clicking = false;  // Désactiver l'auto-click
                }
                _ => {}
            }
        };

        // Démarrer l'écoute des événements
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });

    // Lancer l'auto-clicker dans un autre thread
    let is_clicking_simulation = Arc::clone(&is_clicking);
    let is_simulating_simulation = Arc::clone(&is_simulating);
    thread::spawn(move || loop {
        // Vérifier si l'auto-click est activé
        let clicking = is_clicking_simulation.lock().unwrap();
        if *clicking {
            // Verrouiller le flag simulating avant de simuler le clic
            {
                let mut simulating = is_simulating_simulation.lock().unwrap();
                *simulating = true; // Activer le flag de simulation
            }

            // Simuler un clic gauche
            if let Err(error) = simulate(&EventType::ButtonPress(Button::Left)) {
                println!("Failed to simulate mouse click: {:?}", error);
            }
            if let Err(error) = simulate(&EventType::ButtonRelease(Button::Left)) {
                println!("Failed to simulate mouse release: {:?}", error);
            }

            // Désactiver le flag de simulation après le clic
            {
                let mut simulating = is_simulating_simulation.lock().unwrap();
                *simulating = false; // Désactiver le flag de simulation
            }

            // Attendre avant de refaire un clic (ajuster la durée pour la vitesse de clic)
            thread::sleep(Duration::from_millis(100));
        } else {
            // Si l'auto-click est désactivé, faire une petite pause avant de vérifier à nouveau
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Garder le programme en cours d'exécution
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
