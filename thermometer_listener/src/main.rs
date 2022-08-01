mod thermometer_listener;

use std::cell::RefCell;
use std::rc::Rc;
use thermometer_listener::ThermometerListener;

#[tokio::main]
async fn main() {
    if let Some(listener) = ThermometerListener::new().await {
        // shared variable for received temperature measurement
        // let last_measure = Arc::new(Mutex::new(String::from("N/A")));
        let last_measure = Rc::new(RefCell::new(String::from("N/A")));
        tokio::join!(
            loop_receive(listener, last_measure.clone()),
            loop_display(last_measure)
        );
        // starting a  with thermometer listener
        // let last_measure_clone = last_measure.clone();
        // let _ = std::thread::spawn(move || loop {
        //     let new_measure = listener.receive_measure();
        //     let mut lg = last_measure_clone.lock().unwrap();

        //     *lg = new_measure;

        //     std::mem::drop(lg);
        // });

        // displaying last known measurement
        // loop {
        //     let lg = last_measure.lock().unwrap();
        //     println!("last known measure: {}", lg);
        //     std::mem::drop(lg);
        //     std::thread::sleep(std::time::Duration::from_secs(1))
        // }
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
