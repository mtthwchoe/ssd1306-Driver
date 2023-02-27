// Driver Code
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c;

pub struct I2cDisplay <I>
    where I: i2c::Write + i2c::Read,
{
    address: u8,
    peripheral: I,
    //ram: [[u8; 128]; 64],
}

pub const COMMAND: u8 = 0x00;
pub const DATA: u8 = 0x40;
pub const HORIZONTAL_ADDRESSING_MODE: u8 = 0x00;
pub const VERTICAL_ADDRESSING_MODE: u8 = 0x01;
pub const PAGE_ADDRESSING_MODE: u8 = 0x02;

impl<I> I2cDisplay<I>
    where I: i2c::Write + i2c::Read
{
    pub fn new(addr: u8, i2c: I) -> I2cDisplay<I>{
        //let x = [[0 as u8; 128]; 64];
        I2cDisplay {
            address: addr,
            peripheral: i2c,
            //ram: x,
        }
    }

    // Fundamental Commands
    pub fn set_contrast_control(&mut self, contrast: u8) {  //1 - 256
        let data = [COMMAND, 0x81, contrast];
        self.peripheral.write(self.address, &data);
    }

    pub fn entire_display_on(&mut self, on: bool) {
        let mut data = [COMMAND, 0];
        if on {
            data = [data[0], 0xA5];
        }
        else {
            data = [data[0], 0xA4];
        }
        self.peripheral.write(self.address, &data);

    }

    pub fn set_normal_inverse_display(&mut self, inverse: bool) {
        let data:[u8; 2];
        if inverse {
            data = [COMMAND, 0xA7];
        }
        else {
            data = [COMMAND, 0xA6];
        }
        self.peripheral.write(self.address, &data);
    }

    pub fn set_display_on_off(&mut self, on: bool) {
        let data: [u8; 2];
        if on {
            data = [COMMAND, 0xAF];
        }
        else {
            data = [COMMAND, 0xAE];
        }
        self.peripheral.write(self.address, &data);
    }

    // Scrolling Commands
    pub fn continuous_horizontal_scroll_setup() {

    }

    pub fn continuous_vertical_and_horizontal_scroll_setup() {

    }

    pub fn deactivate_scroll() {

    }

    pub fn activate_scroll() {

    }

    pub fn set_vertical_scroll_area() {

    }

    // Addressing Setting Commands
    pub fn set_lower_column_start_address(&mut self, mut address: u8) {         // for page addressing mode
        address = address.clamp(0x00, 0x0F);
        let data = [COMMAND, address];
        self.peripheral.write(self.address, &data);
    }

    pub fn set_higher_column_start_address(&mut self, mut address: u8) {        // for page addressing mode
        address = address.clamp(0x00, 0x0f);
        let data = [COMMAND, (0x10) | address];
        self.peripheral.write(self.address, &data);
    }

    pub fn set_memory_addressing_mode(&mut self, mut mode: u8) {
        mode = mode.clamp(0,3);
        let data = [COMMAND, 0x20 | mode];
        self.peripheral.write(self.address, &data);
    }

    pub fn set_column_address(&mut self, mut start_addr: u8, mut end_addr: u8) { // This command is only for horizontal or vertical addressing mode. 
        start_addr = start_addr.clamp(0, 127);
        end_addr = end_addr.clamp(0, 127);
        let data = [COMMAND, 0x21, start_addr, end_addr];
        self.peripheral.write(self.address, &data);
    }

    pub fn set_page_address(&mut self, mut start_addr: u8, mut end_addr: u8) { // This command is only for horizontal or vertical addressing mode. 
        start_addr = start_addr.clamp(0, 127);
        end_addr = end_addr.clamp(0, 127);
        let data = [COMMAND, 0x22, start_addr, end_addr];
        self.peripheral.write(self.address, &data);
    }

    pub fn set_page_start_address(&mut self, mut page: u8) {                   // for page addressing mode
        page = page.clamp(0,7);   // 7 pages in page addr mode
        let data = [COMMAND, 0b10110000 | page];
        self.peripheral.write(self.address, &data);
    }

    // Hardware Configuration Commands
    pub fn set_display_start_line(&mut self, mut start: u8) {       // rows 0-63
        start = start.clamp(0, 63);
        let data: [u8; 2] = [COMMAND, 0b01000000 | start];
        self.peripheral.write(self.address, &data);

    }

    pub fn set_segment_remap(&mut self, remap: bool) {
        if remap {
            let mut data = [COMMAND, 0xA1];
            self.peripheral.write(self.address, &mut data);
        }
        else {
            let mut data = [COMMAND, 0xA0];
            self.peripheral.write(self.address, &mut data);
        }
    }

    pub fn set_multiplex_ratio(&mut self, mut ratio: u8) { // valid entries range from 16 to 64
        ratio = ratio.clamp(15, 63);
        let mut data = [COMMAND, 0xA8, ratio];
        self.peripheral.write(self.address, &mut data);
    }

    pub fn set_com_output_scan_direction(&mut self, dir: bool) {
        if dir {
            let mut data = [COMMAND, 0xC0];
            self.peripheral.write(self.address, &mut data);
        }
        else {
            let mut data = [COMMAND, 0xC8];
            self.peripheral.write(self.address, &mut data);
        }
    }

    pub fn set_display_offset(&mut self, mut offset: u8) {          // offset rows by 0-63
        offset = offset.clamp(0,63);
        let mut data = [COMMAND, 0xD3, offset];
        self.peripheral.write(self.address, &mut data);
    }

    pub fn set_com_pins_hardware_configuration(&mut self) {  // dont understand fix later
        let data = [COMMAND, 0xDA, 0x12];
        self.peripheral.write(self.address, &data);
    }

    // Timing and Driving Scheme Setting Commands

    pub fn set_display_clock_divide_frequency(&mut self, mut fosc: u8, mut divide: u8) {
        divide = divide.clamp(0,15);
        fosc = fosc.clamp(0, 15);
        let data = [COMMAND, 0xD5, divide | (fosc<<4)];
        self.peripheral.write(self.address, &data);
    }

    pub fn set_pre_charge_period() {

    }

    pub fn set_vcom_deselect_level() {

    }

    pub fn nop() {

    }

    // Charge Pump Command Table

    pub fn charge_pump_enable(&mut self, on: bool) {
        let data: [u8; 3];
        if on {
            data = [COMMAND, 0x8D, 0x14];
        }
        else {
            data = [COMMAND, 0x8D, 0x10];
        }
        self.peripheral.write(self.address, &data);
    }

    // User Created
    pub fn setup_page_addressing_mode(&mut self) {
        self.set_memory_addressing_mode(PAGE_ADDRESSING_MODE);
        self.set_page_start_address(0);
        self.set_lower_column_start_address(0x00);
        self.set_higher_column_start_address(0x00);
    }

    pub fn start_column_start_address(&mut self, mut column: u8) {
        column = column.clamp(0, 127);
        self.set_lower_column_start_address(0x00);
        self.set_higher_column_start_address(0x00);
    }

    pub fn write_byte(&mut self, data: u8) {    // writes 8 bits to a page column
        let payload = [DATA, data];
        self.peripheral.write(self.address, &payload);
    }

    pub fn write_bytes(&mut self, data: &mut [u8]) {
        self.peripheral.write(self.address, data);
    }

    pub fn setup(&mut self) {
        self.set_multiplex_ratio(0x3F);
        self.set_display_offset(0);
        self.set_display_start_line(0);
        self.set_segment_remap(false);
        self.set_com_output_scan_direction(false);
        self.set_com_pins_hardware_configuration();
        self.set_contrast_control(0x7F);
        self.entire_display_on(false);
        self.set_normal_inverse_display(false);
        self.set_display_clock_divide_frequency(8, 0);
        self.charge_pump_enable(true);
        self.set_display_on_off(true);
    }
}