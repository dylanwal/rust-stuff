use std::time::Instant;
use serialport;
use std::time::Duration;
use std::io::{self, Write};
use std::str;
use serialport::SerialPort;

#[allow(dead_code)]
fn get_ports() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}

#[allow(dead_code)]
fn single_write_read(port: &mut Box<dyn SerialPort>) {
    let output = "1,2,3,4,5\r".as_bytes();
    port.write(output).expect("Write failed!");

    let mut serial_buf: Vec<u8> = vec![0; 32];
    let bytes_recvd = port.read(serial_buf.as_mut_slice()).expect("Found no data!");
    println!("{:?}", serial_buf);

    let msg_recvd = str::from_utf8(&serial_buf[..bytes_recvd]).unwrap();
    println!("{}", msg_recvd)
}


fn single_write_read_until2(port: &mut Box<dyn SerialPort>) {
    let output = "1,2,3,4,5\r".as_bytes();
    port.write(output).expect("Write failed!");


    let mut serial_buf: Vec<u8> = vec![0; 256]; // Use a larger buffer size

    let bytes_recvd = port.read(serial_buf.as_mut_slice()).expect("Found no data!");

    if bytes_recvd > 0 {
        let received_data = String::from_utf8_lossy(&serial_buf[..bytes_recvd]);
        let trimmed_data = received_data.trim();
        // println!("{}", trimmed_data);
    }

    // slower version of what is above that I wrote, but ChatGPT is better
    // let mut my_string = String::with_capacity(8);
    // let mut len_string = 0;
    // 'read_loop: loop {
    //     let mut serial_buf: Vec<u8> = vec![0; 1];
    //     let bytes_recvd = port.read(serial_buf.as_mut_slice()).expect("Found no data!");
    //     if bytes_recvd > 0 {
    //         if serial_buf[0] == 13 {
    //             // println!("{}", &my_string[0..len_string]);
    //             break 'read_loop;
    //         }
    //         my_string.push(serial_buf[0] as char);
    //         len_string += 1;
    //     }
    // }
}


fn serial_comm(n: u32) {
    let mut port = serialport::new("COM3", 9600)
        .timeout(Duration::from_millis(100))
        .open().expect("Failed to open port");

    for _ in 0..n {
        // single_write_read(&mut port);
        // single_write_read_until(&mut port)
        single_write_read_until2(&mut port)
    }
}


fn main() {
    let n: u32 = 5000;
    let now = Instant::now();
    serial_comm(n);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    print!("Hz: {:.1?}", n as f64/elapsed.as_secs_f64());
}
