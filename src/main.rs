#![no_std]
#![no_main]

use esp32_hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
};

use esp_println::println;
use esp_backtrace as _;

mod driver;

#[xtensa_lx_rt::entry]
fn main() -> ! {
    
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = esp32_hal::IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio15.into_push_pull_output();
    led.set_high().unwrap();
    
    let temp = peripherals.I2C0;
    let mut itoc = esp32_hal::i2c::I2C::new(
        temp,
        io.pins.gpio23,
        io.pins.gpio22,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    let mut delay = Delay::new(&clocks);
    let mut display = driver::I2cDisplay::new(0x3c, itoc);
    display.setup();
    display.setup_page_addressing_mode();

    display.entire_display_on(true);
    delay.delay_ms(2000 as u32);
    display.entire_display_on(false);

    let mut counter = 0;
    let mut page = 1;

    let mut data: [u8; 100] = [0xff;100];
    data[0] = 0x40;
    display.write_bytes(&data);

    loop {
        let mut data: [u8; 2] = [0x00;2];
        data[0] = 0x40;
        display.write_bytes(&data);
        delay.delay_ms(100 as u8);
        counter = counter + 1;
        if counter > 128 {
            counter = 0;
            display.set_page_start_address(page);
            display.set_lower_column_start_address(0x00);
            display.set_higher_column_start_address(0x00);
            page = page + 1;
        }
    }
}