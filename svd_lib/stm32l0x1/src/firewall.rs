#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    pub firewall_cssa: FIREWALL_CSSA,
    #[doc = "0x04 - Code segment length"]
    pub firewall_csl: FIREWALL_CSL,
    #[doc = "0x08 - Non-volatile data segment start address"]
    pub firewall_nvdssa: FIREWALL_NVDSSA,
    #[doc = "0x0c - Non-volatile data segment length"]
    pub firewall_nvdsl: FIREWALL_NVDSL,
    #[doc = "0x10 - Volatile data segment start address"]
    pub firewall_vdssa: FIREWALL_VDSSA,
    #[doc = "0x14 - Volatile data segment length"]
    pub firewall_vdsl: FIREWALL_VDSL,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Configuration register"]
    pub firewall_cr: FIREWALL_CR,
}
#[doc = "Code segment start address"]
pub struct FIREWALL_CSSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code segment start address"]
pub mod firewall_cssa;
#[doc = "Code segment length"]
pub struct FIREWALL_CSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code segment length"]
pub mod firewall_csl;
#[doc = "Non-volatile data segment start address"]
pub struct FIREWALL_NVDSSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-volatile data segment start address"]
pub mod firewall_nvdssa;
#[doc = "Non-volatile data segment length"]
pub struct FIREWALL_NVDSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-volatile data segment length"]
pub mod firewall_nvdsl;
#[doc = "Volatile data segment start address"]
pub struct FIREWALL_VDSSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Volatile data segment start address"]
pub mod firewall_vdssa;
#[doc = "Volatile data segment length"]
pub struct FIREWALL_VDSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Volatile data segment length"]
pub mod firewall_vdsl;
#[doc = "Configuration register"]
pub struct FIREWALL_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod firewall_cr;
