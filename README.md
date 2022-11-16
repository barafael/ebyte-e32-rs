# ebyte-e32-rs

Platform-agnostic driver for Ebyte E32 LoRa modules.

Uses [embedded-hal](https://github.com/rust-embedded/embedded-hal) for interfacing with a serial port, an input pin (AUX), and two output pins(M0, M1).

## [CLI and GUI frontends for the driver (Linux)](https://github.com/barafael/ebyte-e32-ui)

## [Demo Project (STM32F411)](https://github.com/barafael/ebyte-e32-demo)

## Example

```rust, no_run
let ebyte = Ebyte::new(serial, aux, m0, m1, delay).unwrap();
let model_data = ebyte.read_model_data().unwrap();
let mut params = ebyte.read_parameters().unwrap();

println!("{model_data:#?}");
println!("{params:#?}");

params.air_rate = AirBaudRate::Bps300;
params.channel = 23;

ebyte
    .set_parameters(&params, Persistence::Temporary)
    .unwrap();

let params = ebyte.read_parameters().unwrap();
println!("{:#?}", params);

loop {
    delay.delay_ms(5000u32);
    println!("Sending it!");
    ebyte.write_buffer(b"it").unwrap();
    led.toggle();
}
```

## Supported E32 LoRa Modules

I tested so far with an `E32-433T30D` and an `E32-868T20D`. According to the datasheet, the differences between the modules are minor, but do check: chapter 11 "E32 series" lists some differences. For example, `E32-170T30D` only supports baud rates up to 9.6k. Please report or PR if you find something that doesn't work.

## Crate Features

* `value_enum`: enable deriving `clap::ValueEnum` for the enums inside `ebyte_e32::parameters::Parameters`. This disables `no_std`, but one can make nice CLIs and GUIs with this: [ebyte-e32-ui](https://github.com/barafael/ebyte-e32-ui)

## Known Limitations

* Driver is completely blocking and relies on blocking delay, blocking sometimes for 40ms.
* AUX is not monitored while writing serial data. This would be important when filling the module buffer which has space for 512 bytes.
* Transmission power, frequency, baudrate and probably some other definitions are generally not applicable for every single E32 model. See datasheet for table with module specialties. E.g., some few modules do not support all air baud rates.
