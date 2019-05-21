#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x10 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x14 - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x18 - Clock interrupt clear register"]
    pub cicr: CICR,
    #[doc = "0x1c - GPIO reset register"]
    pub ioprstr: IOPRSTR,
    #[doc = "0x20 - AHB peripheral reset register"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x28 - APB1 peripheral reset register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x2c - GPIO clock enable register"]
    pub iopenr: IOPENR,
    #[doc = "0x30 - AHB peripheral clock enable register"]
    pub ahbenr: AHBENR,
    #[doc = "0x34 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    #[doc = "0x38 - APB1 peripheral clock enable register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x3c - GPIO clock enable in sleep mode register"]
    pub iopsmen: IOPSMEN,
    #[doc = "0x40 - AHB peripheral clock enable in sleep mode register"]
    pub ahbsmenr: AHBSMENR,
    #[doc = "0x44 - APB2 peripheral clock enable in sleep mode register"]
    pub apb2smenr: APB2SMENR,
    #[doc = "0x48 - APB1 peripheral clock enable in sleep mode register"]
    pub apb1smenr: APB1SMENR,
    #[doc = "0x4c - Clock configuration register"]
    pub ccipr: CCIPR,
    #[doc = "0x50 - Control and status register"]
    pub csr: CSR,
}
#[doc = "Clock control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock control register"]
pub mod cr;
#[doc = "Internal clock sources calibration register"]
pub struct ICSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "Clock configuration register"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "Clock interrupt enable register"]
pub struct CIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "Clock interrupt flag register"]
pub struct CIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "Clock interrupt clear register"]
pub struct CICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "GPIO reset register"]
pub struct IOPRSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO reset register"]
pub mod ioprstr;
#[doc = "AHB peripheral reset register"]
pub struct AHBRSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB peripheral reset register"]
pub mod ahbrstr;
#[doc = "APB2 peripheral reset register"]
pub struct APB2RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB1 peripheral reset register"]
pub struct APB1RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "GPIO clock enable register"]
pub struct IOPENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO clock enable register"]
pub mod iopenr;
#[doc = "AHB peripheral clock enable register"]
pub struct AHBENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB peripheral clock enable register"]
pub mod ahbenr;
#[doc = "APB2 peripheral clock enable register"]
pub struct APB2ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "APB1 peripheral clock enable register"]
pub struct APB1ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "GPIO clock enable in sleep mode register"]
pub struct IOPSMEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO clock enable in sleep mode register"]
pub mod iopsmen;
#[doc = "AHB peripheral clock enable in sleep mode register"]
pub struct AHBSMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB peripheral clock enable in sleep mode register"]
pub mod ahbsmenr;
#[doc = "APB2 peripheral clock enable in sleep mode register"]
pub struct APB2SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 peripheral clock enable in sleep mode register"]
pub mod apb2smenr;
#[doc = "APB1 peripheral clock enable in sleep mode register"]
pub struct APB1SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral clock enable in sleep mode register"]
pub mod apb1smenr;
#[doc = "Clock configuration register"]
pub struct CCIPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock configuration register"]
pub mod ccipr;
#[doc = "Control and status register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control and status register"]
pub mod csr;
