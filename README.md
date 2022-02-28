# ebyte-rs

This driver requires a serial port, 2 output pins, and 1 input pin. It drives an ebyte E32 LoRa module.
I tested so far with an E32-433T30D, but they seem to be pretty similar across the board. Please report or PR if something doesn't work.

# Example

```rust
// aux: InputPin
// m0: OutputPin
// m1: OutputPin
// serial: Serial port
let ebyte = Ebyte::new(serial, m0, m1, aux, delay).unwrap();
let mut ebyte = ebyte.into_program_mode();
let mut params = ebyte.read_parameters().unwrap();
let ebyte = ebyte.into_normal_mode();

rprintln!("{:?}", params);
params.air_rate = AirBaudRate::Bps300;

let mut ebyte = ebyte.into_program_mode();
ebyte
    .set_parameters(&params, Persistence::Temporary)
    .unwrap();
let ebyte = ebyte.into_normal_mode();

let mut ebyte = ebyte.into_program_mode();
let params = ebyte.read_parameters().unwrap();
let mut ebyte = ebyte.into_normal_mode();
rprintln!("{:?}", params);

loop {
    delay_tim5.delay_ms(5000u32);
    ebyte.write_buffer(b"buffer").unwrap();
    led.toggle();
}
```

# Known limitations
* Driver is completely blocking and relies on blocking delay, blocking sometimes for 40ms
* AUX is not monitored while writing serial data. This would be important when filling the module buffer which has space for 512 bytes.
