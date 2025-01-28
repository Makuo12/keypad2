
/*!
# Platform-agnostic driver for 4X4 numeric keypads

Provides a driver for reading from standard 4X4 keypads

## Example

```rust
let rows = (
    gpiob.pb15.into_pull_up_input(&mut gpiob.crh),
    gpioa.pa7.into_pull_up_input(&mut gpioa.crl),
    gpiob.pb6.into_pull_up_input(&mut gpiob.crl),
    gpioa.pa9.into_pull_up_input(&mut gpioa.crh),
);

let cols = (
    gpioa.pa8.into_open_drain_output(&mut gpioa.crh),
    gpiob.pb5.into_open_drain_output(&mut gpiob.crl),
    gpioc.pc7.into_open_drain_output(&mut gpioc.crl),
    gpioc.pc6.into_open_drain_output(&mut gpioc.crl),
);

let mut keypad = Keypad::new(rows, cols);

let key = keypad.read_char(&mut delay);
if key != ' ' {
    ...
}
```
*/
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::delay::DelayNs;

use crate::utils::convert;
/// Defines a type that makes it easier to supply the four pins required for rows in the keypad.
/// These pins need to support the `embedded_hal::digital::v2::InputPin` trait
pub type Rows<R0, R1, R2, R3> = (R0, R1, R2, R3);

/// Defines a type that makes it easier to supply the four pins required for rows in the keypad
/// These pins need to support the `embedded_hal::digital::v2::OutputPin` trait
pub type Columns<C0, C1, C2, C3> = (C0, C1, C2, C3);

/// Manages the pins and the logic for scanning a keypad
pub struct Keypad4x4<
    R0: InputPin,
    R1: InputPin,
    R2: InputPin,
    R3: InputPin,
    C0: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
    C3: OutputPin,
> {
    rows: Rows<R0, R1, R2, R3>,
    columns: Columns<C0, C1, C2, C3>,
}

impl<
        R0: InputPin,
        R1: InputPin,
        R2: InputPin,
        R3: InputPin,
        C0: OutputPin,
        C1: OutputPin,
        C2: OutputPin,
        C3: OutputPin,
    > Keypad4x4<R0, R1, R2, R3, C0, C1, C2, C3>
{
    /// Create a new instance of this structure
    pub fn new(rows: Rows<R0, R1, R2, R3>, columns: Columns<C0, C1, C2, C3>) -> Self {
        Self { rows, columns }
    }

    /**
    Reads a character from the keypad. This method returns even if no keys are pressed.
    It will return:
    
    * `'0'` through `'9'`
    * `'*'`
    * `'#'`
    * `' '` if no keys are pressed.
    */
    pub fn read_char(&mut self, delay: &mut dyn DelayNs) -> char {
        let raw = self.read(delay);
        if raw != 0 {
            self.get_char(raw)
        } else {
            ' '
        }
    }

    // Performs a "raw" read of the keypad and returns a bit set for each key down. Note,
    // this doesn't mean this code supports multiple key presses.
    fn read(&mut self, delay: &mut dyn DelayNs) -> u16 {
        let mut res = 0;

        self.columns.0.set_low().unwrap_or_default();
        res |= self.read_column(delay) << 0;
        self.columns.0.set_high().unwrap_or_default();

        self.columns.1.set_low().unwrap_or_default();
        res |= self.read_column(delay) << 4;
        self.columns.1.set_high().unwrap_or_default();

        self.columns.2.set_low().unwrap_or_default();
        res |= self.read_column(delay) << 8;
        self.columns.2.set_high().unwrap_or_default();
        self.columns.3.set_low().unwrap_or_default();
        res |= self.read_column(delay) << 12;
        self.columns.3.set_high().unwrap_or_default();
        res
    }

    // Converts the raw value from the read() method into a character that corresponds to the
    // label on each key
    fn get_char(&self, raw_value: u16) -> char {
        let value = convert(raw_value);
        match value {
            -1 => '*',
            -2 => '#',
            -3 => 'A',
            -4 => 'B',
            -5 => 'C',
            -6 => 'D',
            _ => char::from_digit(value as u32, 10).unwrap(),
        }
    }

    fn read_column(&mut self, delay: &mut dyn DelayNs) -> u16 {
        let mut res = 0;

        delay.delay_ms(1u32);
        if self.rows.0.is_low().unwrap_or_default() {
            res |= 1 << 0;
        }
        if self.rows.1.is_low().unwrap_or_default() {
            res |= 1 << 1;
        }
        if self.rows.2.is_low().unwrap_or_default() {
            res |= 1 << 2;
        }
        if self.rows.3.is_low().unwrap_or_default() {
            res |= 1 << 3;
        }

        res
    }

    // Converts the raw value (2^N) from the read() method into a keypad digit. This will be
    //      0..9    digits
    //      -1      *
    //      -2      #
    //      -3      A
}