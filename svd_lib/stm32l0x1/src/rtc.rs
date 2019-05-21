#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    pub tr: TR,
    #[doc = "0x04 - RTC date register"]
    pub dr: DR,
    #[doc = "0x08 - RTC control register"]
    pub cr: CR,
    #[doc = "0x0c - RTC initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - RTC prescaler register"]
    pub prer: PRER,
    #[doc = "0x14 - RTC wakeup timer register"]
    pub wutr: WUTR,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - RTC alarm A register"]
    pub alrmar: ALRMAR,
    #[doc = "0x20 - RTC alarm B register"]
    pub alrmbr: ALRMBR,
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - RTC sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - RTC shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - RTC timestamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - RTC timestamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - RTC time-stamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - RTC calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - RTC tamper configuration register"]
    pub tampcr: TAMPCR,
    #[doc = "0x44 - RTC alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - RTC alarm B sub second register"]
    pub alrmbssr: ALRMBSSR,
    #[doc = "0x4c - option register"]
    pub or: OR,
    #[doc = "0x50 - RTC backup registers"]
    pub bkp0r: BKP0R,
    #[doc = "0x54 - RTC backup registers"]
    pub bkp1r: BKP1R,
    #[doc = "0x58 - RTC backup registers"]
    pub bkp2r: BKP2R,
    #[doc = "0x5c - RTC backup registers"]
    pub bkp3r: BKP3R,
    #[doc = "0x60 - RTC backup registers"]
    pub bkp4r: BKP4R,
}
#[doc = "RTC time register"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC time register"]
pub mod tr;
#[doc = "RTC date register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC date register"]
pub mod dr;
#[doc = "RTC control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC control register"]
pub mod cr;
#[doc = "RTC initialization and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC initialization and status register"]
pub mod isr;
#[doc = "RTC prescaler register"]
pub struct PRER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC prescaler register"]
pub mod prer;
#[doc = "RTC wakeup timer register"]
pub struct WUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC wakeup timer register"]
pub mod wutr;
#[doc = "RTC alarm A register"]
pub struct ALRMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC alarm A register"]
pub mod alrmar;
#[doc = "RTC alarm B register"]
pub struct ALRMBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC alarm B register"]
pub mod alrmbr;
#[doc = "write protection register"]
pub struct WPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "write protection register"]
pub mod wpr;
#[doc = "RTC sub second register"]
pub struct SSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "RTC shift control register"]
pub struct SHIFTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC shift control register"]
pub mod shiftr;
#[doc = "RTC timestamp time register"]
pub struct TSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC timestamp time register"]
pub mod tstr;
#[doc = "RTC timestamp date register"]
pub struct TSDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC timestamp date register"]
pub mod tsdr;
#[doc = "RTC time-stamp sub second register"]
pub struct TSSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC time-stamp sub second register"]
pub mod tsssr;
#[doc = "RTC calibration register"]
pub struct CALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC calibration register"]
pub mod calr;
#[doc = "RTC tamper configuration register"]
pub struct TAMPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC tamper configuration register"]
pub mod tampcr;
#[doc = "RTC alarm A sub second register"]
pub struct ALRMASSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC alarm A sub second register"]
pub mod alrmassr;
#[doc = "RTC alarm B sub second register"]
pub struct ALRMBSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC alarm B sub second register"]
pub mod alrmbssr;
#[doc = "option register"]
pub struct OR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "option register"]
pub mod or;
#[doc = "RTC backup registers"]
pub struct BKP0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC backup registers"]
pub mod bkp0r;
#[doc = "RTC backup registers"]
pub struct BKP1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC backup registers"]
pub mod bkp1r;
#[doc = "RTC backup registers"]
pub struct BKP2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC backup registers"]
pub mod bkp2r;
#[doc = "RTC backup registers"]
pub struct BKP3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC backup registers"]
pub mod bkp3r;
#[doc = "RTC backup registers"]
pub struct BKP4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC backup registers"]
pub mod bkp4r;
