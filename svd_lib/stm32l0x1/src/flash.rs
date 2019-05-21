#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Program/erase control register"]
    pub pecr: PECR,
    #[doc = "0x08 - Power down key register"]
    pub pdkeyr: PDKEYR,
    #[doc = "0x0c - Program/erase key register"]
    pub pekeyr: PEKEYR,
    #[doc = "0x10 - Program memory key register"]
    pub prgkeyr: PRGKEYR,
    #[doc = "0x14 - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x18 - Status register"]
    pub sr: SR,
    #[doc = "0x1c - Option byte register"]
    pub obr: OBR,
    #[doc = "0x20 - Write protection register"]
    pub wrpr: WRPR,
}
#[doc = "Access control register"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access control register"]
pub mod acr;
#[doc = "Program/erase control register"]
pub struct PECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program/erase control register"]
pub mod pecr;
#[doc = "Power down key register"]
pub struct PDKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "Program/erase key register"]
pub struct PEKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program/erase key register"]
pub mod pekeyr;
#[doc = "Program memory key register"]
pub struct PRGKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program memory key register"]
pub mod prgkeyr;
#[doc = "Option byte key register"]
pub struct OPTKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "Status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Option byte register"]
pub struct OBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Option byte register"]
pub mod obr;
#[doc = "Write protection register"]
pub struct WRPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write protection register"]
pub mod wrpr;
