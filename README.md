# ebyte-e32-rs

This driver requires a serial port, 2 output pins, and 1 input pin. It drives an ebyte E32 LoRa module.
I tested so far with an E32-433T30D and an E32-868T20D. According to the datasheet, the differences between the modules are minor, but do check the datasheet: chapter 11 "E32 series" lists some differences. For example, E32-170T30D only supports baud rates up to 9.6k. Please report or PR if you find something that doesn't work.

# [Demo Project](https://github.com/barafael/ebyte-e32-demo)

# Example

```rust
// aux: InputPin
// m0: OutputPin
// m1: OutputPin
// serial: Serial port
let ebyte = Ebyte::new(serial, aux, m0, m1, delay).unwrap();
let model_data = ebyte.read_model_data().unwrap();
let mut params = ebyte.read_parameters().unwrap();

rprintln!("{:#?}", model_data);
rprintln!("{:#?}", params);

params.air_rate = AirBaudRate::Bps300;
params.channel = 23;

ebyte
    .set_parameters(&params, Persistence::Temporary)
    .unwrap();

let params = ebyte.read_parameters().unwrap();
rprintln!("{:#?}", params);

loop {
    delay_tim5.delay_ms(5000u32);
    rprintln!("Sending it!");
    ebyte.write_buffer(b"it").unwrap();
    led.toggle();
}
```

# Known limitations
* Driver is completely blocking and relies on blocking delay, blocking sometimes for 40ms
* AUX is not monitored while writing serial data. This would be important when filling the module buffer which has space for 512 bytes.
* Transmission power, frequency, baudrate and probably some other definitions are generally not applicable for every single E32 model. See datasheet for table with module specialties. E.g., some few modules do not support all air baud rates.
