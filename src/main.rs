#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f411;

const FLASH_KEY1: u32 = 0x45670123;
const FLASH_KEY2: u32 = 0xCDEF89AB;
const FLASH_OPT_KEY1: u32 = 0x08192A3B;
const FLASH_OPT_KEY2: u32 = 0x4C5D6E7F;
const FLASH_SECTORS_MASK: u16 = 0xFF;

const STM32_BOOTLOADER_VECTOR_TABLE: *const u32 = 0x1FFF0000 as *const u32;

#[entry]
fn main() -> ! {
    // Do all flash register modification in critical section
    cortex_m::interrupt::free(|_| {
        let peripherals = stm32f411::Peripherals::take().unwrap();
        let flash = &peripherals.FLASH;
        let cr = &flash.cr;
        let sr = &flash.sr;
        let optcr = &flash.optcr;
        let keyr = &flash.keyr;
        let optkeyr = &flash.optkeyr;

        // Unlock flash
        if cr.read().lock().bit_is_set() {
            keyr.write(|w| w.key().bits(FLASH_KEY1));
            keyr.write(|w| w.key().bits(FLASH_KEY2));
            assert!(cr.read().lock().bit_is_clear());
        }

        // Unlock option bytes
        if optcr.read().optlock().bit_is_set() {
            optkeyr.write(|w| w.optkey().bits(FLASH_OPT_KEY1));
            optkeyr.write(|w| w.optkey().bits(FLASH_OPT_KEY2));
            assert!(optcr.read().optlock().bit_is_clear());
        }

        // Unprotect flash sectors if it is necessary
        if (optcr.read().n_wrp().bits() & FLASH_SECTORS_MASK) != FLASH_SECTORS_MASK {
            optcr.modify(|_, w| unsafe { w.n_wrp().bits(FLASH_SECTORS_MASK) });

            // Persist changes to option bytes
            optcr.modify(|_, w| w.optstrt().set_bit());
        }

        // Wait for flash operations to complete
        while sr.read().bsy().bit_is_set() {}

        // Lock option bytes again
        optcr.modify(|_, w| w.optlock().set_bit());
        assert!(optcr.read().optlock().bit_is_set());

        // Lock flash again
        cr.modify(|_, w| w.lock().set_bit());
        assert!(cr.read().lock().bit_is_set());
    });

    // Jump to STM32-DFU bootloader in ROM
    unsafe {
        asm::bootload(STM32_BOOTLOADER_VECTOR_TABLE);
    }
}
