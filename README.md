# ebyte-e32-rs

Platform-agnostic driver for Ebyte E32 LoRa modules.

Uses [embedded-hal](https://github.com/rust-embedded/embedded-hal) for interfacing with a serial port, an input pin (AUX), and two output pins(M0, M1).

## [CLI and GUI frontends for the driver (Linux)](https://github.com/barafael/ebyte-e32-ui)

## [Demo Project (STM32F411)](https://github.com/barafael/ebyte-e32-demo)

## Example

```rust, no_run
use ebyte_e32::{
    mode::Normal,
    parameters::{air_baudrate::AirBaudRate, Persistence},
    Ebyte,
};
use embedded_hal::{
    blocking::delay,
    digital::v2::{InputPin, OutputPin},
    serial,
};
use embedded_hal::digital::v2::ToggleableOutputPin;
use std::fmt::Debug;

pub fn simple_test_ebyte<S, AUX, M0, M1, D, LED>(
    mut ebyte: Ebyte<S, AUX, M0, M1, D, Normal>,
    mut led: LED,
    mut delay: impl delay::DelayMs<u32>,
) where
    S: serial::Read<u8> + serial::Write<u8>,
    <S as serial::Read<u8>>::Error: Debug,
    <S as serial::Write<u8>>::Error: Debug,
    AUX: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: delay::DelayMs<u32>,
    LED: ToggleableOutputPin,
    LED::Error: Debug,
{
    let model_data = ebyte.model_data().unwrap();
    let mut params = ebyte.parameters().unwrap();

    println!("{model_data:#?}");
    println!("{params:#?}");

    params.air_rate = AirBaudRate::Bps300;
    params.channel = 23;

    ebyte
        .set_parameters(&params, Persistence::Temporary)
        .unwrap();

    let params = ebyte.parameters().unwrap();
    println!("{:#?}", params);

    loop {
        delay.delay_ms(5000u32);
        println!("Sending it!");
        ebyte.write_buffer(b"it").unwrap();
        led.toggle().unwrap();
    }
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

## Module Graph

![modules](https://user-images.githubusercontent.com/6966738/202765422-106b0da6-9136-48f3-ab67-071cfa037c59.png)

## Dependency Graph

(Note this doesn't include the optional feature `value_enum`)

![dependencies](https://user-images.githubusercontent.com/6966738/202765372-d2616ffa-bdbc-4ad6-bdac-d88a307dd320.png)
