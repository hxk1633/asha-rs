use std::pin::Pin;
use simplersble::{self, Adapter, Peripheral};

use crate::asha_model;

fn get_adapter() -> Pin<Box<Adapter>> {
    println!(
        "Bluetooth enabled: {}",
        simplersble::Adapter::bluetooth_enabled().unwrap()
    );

    let mut adapters = simplersble::Adapter::get_adapters().unwrap();

    // If the adapter list is empty, print a message and exit
    if adapters.is_empty() {
        println!("No adapters found.");
    }

    for adapter in adapters.iter_mut() {
        println!("Adapter: {} [{}]", adapter.identifier().unwrap(), adapter.address().unwrap());
    }

    let adapter = adapters.pop().unwrap();
    adapter
}

fn scan(adapter: &mut Pin<Box<Adapter>>) -> Vec<Pin<Box<Peripheral>>> {
    adapter.set_callback_on_scan_found(Box::new(|peripheral| {
        println!(
            "Found device: {} [{}] {} dBm",
            peripheral.identifier().unwrap(),
            peripheral.address().unwrap(),
            peripheral.rssi().unwrap()
        );
    }));

    adapter.scan_for(5000).unwrap();
    println!("Scan complete.");

    println!("The following devices were found:");
    let results = adapter.scan_get_results().unwrap();
    for (i, peripheral) in results.iter().enumerate() {
        let connectable_str = "unknown";
        let peripheral_str = format!(
            "{} [{}] {} dBm",
            peripheral.identifier().unwrap(),
            peripheral.address().unwrap(),
            peripheral.rssi().unwrap()
        );

        println!("{}: {} {}", i, peripheral_str, connectable_str);
    }
    results
}

fn connect(results: &mut Vec<Pin<Box<Peripheral>>>) -> Pin<Box<Peripheral>> {
    // Prompt the user to select a device
    println!("Select a device to connect to:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let input = input.parse::<usize>().unwrap();

    // Get the selected device by moving it out of the scan results
    let mut peripheral = results.remove(input);

    peripheral.set_callback_on_connected(Box::new(|| {
        println!("Connected to device.");
    }));

    peripheral.set_callback_on_disconnected(Box::new(|| {
        println!("Disconnected from device.");
    }));

    // Connect to the device
    println!("Connecting to device...");
    peripheral.connect().unwrap();

    println!("Connected to device.");
    peripheral
}

fn select_characteristic(peripheral: &mut Pin<Box<Peripheral>>) -> (String, String){
    // Make a Vec of all service/characteristic pairs
    let mut service_characteristic_pairs = Vec::new();
    for service in peripheral.services().unwrap().iter() {
        for characteristic in service.characteristics().iter() {
            service_characteristic_pairs.push((service.uuid(), characteristic.uuid()));
        }
    }

    // Print the list of services and characteristics
    println!("The following services and characteristics were found:");
    for (i, (service, characteristic)) in service_characteristic_pairs.iter().enumerate() {
        println!("{}: {} {}", i, service, characteristic);
    }

    // Prompt the user to select a service/characteristic pair
    println!("Select a service/characteristic pair:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let input = input.parse::<usize>().unwrap();

    // Get the selected service/characteristic pair by moving it out of the Vec
    service_characteristic_pairs.remove(input)
}

pub fn notify() {
    let mut adapter = get_adapter();
    let mut results = scan(&mut adapter);
    let mut peripheral = connect(&mut results);

    let (service, characteristic) = select_characteristic(&mut peripheral);

    // Subscribe to the characteristic
    println!("Subscribing to characteristic...");
    peripheral.notify(&service, &characteristic, Box::new(|data| {
        println!("Received data: {:?}", data);
    })).unwrap();

    // Sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Unsubscribe from the characteristic
    println!("Unsubscribing from characteristic...");
    peripheral.unsubscribe(&service, &characteristic).unwrap();

    peripheral.disconnect().unwrap();
}

pub fn read() {
    let mut adapter = get_adapter();
    let mut results = scan(&mut adapter);
    let mut peripheral = connect(&mut results);

    let (service, characteristic) = select_characteristic(&mut peripheral);

    let value = peripheral.read(&service, &characteristic);
    println!("Value: {:?}", value);
    let read_only_properties = asha_model::ReadOnlyProperties::new(&value.unwrap());
    println!("{}", read_only_properties.unwrap());

    peripheral.disconnect().unwrap();
}