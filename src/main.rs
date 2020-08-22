extern crate serialport;

use serialport::prelude::*;
use std::time::Duration;


fn main() {
    println!("{:?}", serialport::available_ports());
    let s = SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };
    let mon = serialport::open_with_settings("COM3", &s).unwrap();
    while true {
            println!("{:?}", mon.is_empty(&self));
    }
}
