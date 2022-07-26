use super::cores::Core;
use super::cores::CoreType;
use super::memory::Memory;
use super::memory::MemoryMap;
use super::memory::MemoryMapped;

use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use ihex::Reader;
use ihex::Record;

pub enum DeviceType {
    ATtiny1626
}

#[allow(non_snake_case)]
pub struct Device {
    pub core: Core,
    pub flash: Rc<RefCell<dyn MemoryMapped>>,
    pub sram: Rc<RefCell<dyn MemoryMapped>>,
    pub mm: Rc<RefCell<dyn MemoryMapped>>,
    RAMEND: u16
}

impl Device {
    pub fn new(dt: DeviceType) -> Self {
        match dt {
            DeviceType::ATtiny1626 => {
                // Constants
                const RAMEND: u16 = 0x3BFF;

                //Memories
                let flash: Rc<RefCell<dyn MemoryMapped>> =  Rc::new(RefCell::new(Memory::new(16384, 0xFF, 0)));
                let sram: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new(2048, 0x00, 0)));
                let gpio: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new(4, 0x00, 0)));
                
                //Read only
                let syscfg: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00, 0x04], 0))); // Rev E (0x04?) is inital release
                let fuse: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00, 0x00, 0x7E, ], 0)));
                
                // Placeholder
                let eeprom: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 256], 0)));  // Should this read 0xFF?
                let userrow: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 0x80], 0))); // Should this read 0xFF?
                
                //TODO
                let cpu: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new(0x10, 0x00, 0)));
                let clkctrl: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new(0x1D, 0x00, 0)));
                let porta: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new(0x18, 0x00, 0)));
                let portb: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new(0x18, 0x00, 0)));
                let portc: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new(0x18, 0x00, 0)));

                // Not implemented
                let slpctrl: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 0x01], 0)));
                let bod: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 0x0C], 0))); 
                let twi: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 0x0F], 0)));
                let crcscan: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 0x03], 0))); 
                let ac0: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 0x08], 0))); 
                let nvmctrl: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(Memory::new_rom(vec![0x00; 0x09], 0))); 


                let mut mm = MemoryMap::new();
                //[0x0000] VPORTA 
                //[0x0004] VPORTB 
                //[0x0008] VPORTC 
                mm.add(0x001C, Rc::clone(&gpio));       //[0x001C] GPIO (DONE)
                mm.add(0x0030, Rc::clone(&cpu));        //[0x0030] CPU (TODO)
                //[0x0040] RSTCTRL 
                mm.add(0x0050, Rc::clone(&slpctrl));    //[0x0050] SLPCTRL (not implemented) 
                mm.add(0x0060, Rc::clone(&clkctrl));    //[0x0060] CLKCTRL (TODO)
                mm.add(0x0080, Rc::clone(&bod));        //[0x0080] BOD (not implemented) 
                //[0x00A0] VREF 
                //[0x0100] WDT 
                //[0x0110] CPUINT 
                mm.add(0x0120, Rc::clone(&crcscan));    //[0x0120] CRCSCAN (not implemented)
                //[0x0140] RTC 
                //[0x0180] EVSYS 
                //[0x01C0] CCL 
                mm.add(0x0400, Rc::clone(&porta));      //[0x0400] PORTA (TODO) 
                mm.add(0x0420, Rc::clone(&portb));      //[0x0420] PORTB (TODO) 
                mm.add(0x0440, Rc::clone(&portc));      //[0x0440] PORTC (TODO) 
                //[0x05E0] PORTMUX 
                //[0x0600] ADC0 
                mm.add(0x0680, Rc::clone(&ac0));        //[0x0680] AC0 (not implemented) 
                //[0x0800] USART0 
                //[0x0820] USART1 
                mm.add(0x08A0, Rc::clone(&twi));        //[0x08A0] TWI0 (not implemented)
                //[0x08C0] SPI0 
                //[0x0A00] TCA0 
                //[0x0A80] TCB0 
                //[0x0A90] TCB1 
                mm.add(0x0F00, Rc::clone(&syscfg));     //[0x0F00] SYSCFG (DONE)
                mm.add(0x08A0, Rc::clone(&nvmctrl));    //[0x1000] NVMCTRL (not implemented) 
                //[0x1100] SIGROW 
                mm.add(0x1280, Rc::clone(&fuse));       //[0x1280] FUSE 
                //[0x128A] LOCKBIT 
                mm.add(0x1300, Rc::clone(&userrow));    //[0x1300] USERROW
                mm.add(0x1400, Rc::clone(&eeprom));     //[0x1400] EEPROM (erased, read only)
                //[0x1500-0x33FF] RESERVED
                mm.add(0x3400, Rc::clone(&sram));       //[0x3400] SRAM
                //[0x????-0x3FFF] RESERVED (up to 3K SRAM) 
                //[0x4000-0x7FFF] RESERVED
                mm.add(0x8000, Rc::clone(&flash));      //[0x8000] FLASH
                //[0xBFFF-0xFFFF] RESERVED (up to 32K FLASH)

                let mm: Rc<RefCell<dyn MemoryMapped>> = Rc::new(RefCell::new(mm));

                Device {
                    core: Core::new(CoreType::AVRxt, Rc::clone(&mm), Rc::clone(&flash), RAMEND),
                    flash: flash,
                    sram: sram,
                    mm: mm,
                    RAMEND
                }
            }
            
            
        }
    }

    pub fn load_hex(&mut self, filename: &String) {
        let path = Path::new(filename);
        let display = path.display();
    
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
    
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read {}: {}", display, why),
            Ok(_) => {
                let hex = Reader::new(&s);
                for r in hex {
                    if let Record::Data{offset, value} = r.unwrap() {
                        println!("[HEX] 0x{:04X} Writing {} bytes.", offset, value.len());
                        let mut address = usize::from(offset);
                        for b in value {
                            self.flash.borrow_mut().write(address, b);
                            address += 1;
                        }
                    }
                }
            }
        };

    }

    pub fn tick(&mut self) -> bool {
        self.core.tick()
    }

    pub fn dump_regs(&self) {
        for i in 0..=31 {
            println!("[R{:02}] 0x{:02X}", i, self.core.get_r(i));
        }
    }

    pub fn dump_stack(&self) {
        let mut sp = self.core.get_sp();
        while sp < self.RAMEND {
            sp += 1;
            println!("[STACK+{:03X}] 0x{:02X}", self.RAMEND-sp, self.mm.borrow().read(usize::from(sp)).0)
        }
    }
}