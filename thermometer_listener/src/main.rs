mod thermometer_listener;

use std::cell::RefCell;
use std::rc::Rc;
use thermometer_listener::ThermometerListener;

#[tokio::main]
async fn main() {
    if let Some(listener) = ThermometerListener::new().await {
        // shared variable for received temperature measurement
        let last_measure = Rc::new(RefCell::new(String::from("N/A")));

        tokio::join!(
            loop_receive(listener, last_measure.clone()),
            loop_display(last_measure)
        );

    } else {
        println!("Unable to create ThermometerListener");
    }
}

async fn loop_receive(listener: ThermometerListener, last_measure: Rc<RefCell<String>>) {
    loop {
        let new_measure = listener.receive_measure().await;
        *last_measure.borrow_mut() = new_measure;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn loop_display(last_measure: Rc<RefCell<String>>) {
    loop {
        println!("last known measure: {}", last_measure.borrow());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
