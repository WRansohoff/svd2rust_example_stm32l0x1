#![no_std]
#![no_main]

// Halt when the program panics.
extern crate panic_halt;

// Includes.
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32l0x1;

#[entry]
fn main() -> ! {
  // Set up SysTick peripheral.
  let cm_p = cortex_m::Peripherals::take().unwrap();
  let mut syst = cm_p.SYST;
  syst.set_clock_source( SystClkSource::Core );
  // ~1s period; STM32L0 boots to a ~2.1MHz internal oscillator.
  syst.set_reload( 2_097_000 );
  syst.enable_counter();

  // Set up GPIO pin B3 as push-pull output.
  let p = stm32l0x1::Peripherals::take().unwrap();
  let rcc = p.RCC;
  rcc.iopenr.write( |w| w.iopben().set_bit() );
  let gpiob = p.GPIOB;
  unsafe { gpiob.moder.write( |w| w.mode3().bits( 0x01 ) ); }
  gpiob.otyper.write( |w| w.ot3().clear_bit() );

  // Restart the SysTick counter.
  syst.clear_current();

  // Main loop.
  loop {
    // Toggle the LED every SysTick tick.
    while !syst.has_wrapped() {};
    gpiob.odr.write( |w| w.od3().set_bit() );
    while !syst.has_wrapped() {};
    gpiob.odr.write( |w| w.od3().clear_bit() );
  }
}
