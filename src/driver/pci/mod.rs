// Copyright (c) ChefKiss Inc 2021-2022.
// This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

use alloc::{boxed::Box, vec::Vec};

use acpi::tables::mcfg::{MCFGEntry, MCFG};
use modular_bitfield::prelude::*;
use num_enum::IntoPrimitive;

mod mmio;
mod pio;

#[derive(Debug, Default, Clone, Copy)]
pub struct PCIAddress {
    pub segment: u16,
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
}

#[bitfield(bits = 16)]
#[derive(Debug)]
#[repr(u16)]
pub struct PCICommand {
    pub pio: bool,
    pub mmio: bool,
    pub bus_master: bool,
    pub special_cycle: bool,
    pub mem_write_and_invl: bool,
    pub vga_palette_snoop: bool,
    pub parity_error_resp: bool,
    pub wait_cycle_ctl: bool,
    pub serr: bool,
    pub fast_back_to_back: bool,
    pub disable_intrs: bool,
    #[skip]
    __: B5,
}

#[allow(dead_code)]
#[derive(IntoPrimitive)]
#[repr(u8)]
pub enum PCICfgOffset {
    VendorId = 0x0,
    DeviceId = 0x2,
    Command = 0x4,
    Status = 0x6,
    RevisionId = 0x8,
    ProgIf = 0x9,
    ClassCode = 0xA,
    Subclass = 0xB,
    CacheLineSize = 0xC,
    LatencyTimer = 0xD,
    HeaderType = 0xE,
    Bist = 0xF,
    BaseAddr0 = 0x10,
    BaseAddr1 = 0x14,
    BaseAddr2 = 0x18,
    BaseAddr3 = 0x1C,
    BaseAddr4 = 0x20,
    BaseAddr5 = 0x24,
    CardBusCisPtr = 0x28,
    SubSystemVendorId = 0x2C,
    SubSystemId = 0x2E,
    ExpansionRomBase = 0x30,
    CapabilitiesPtr = 0x34,
    InterruptLine = 0x3C,
    InterruptPin = 0x3D,
    MinimumGrant = 0x3E,
    MaximumLatency = 0x3F,
}

pub trait PCIControllerIO: Sync {
    unsafe fn cfg_read8(&self, addr: PCIAddress, off: u8) -> u8;
    unsafe fn cfg_read16(&self, addr: PCIAddress, off: u8) -> u16;
    unsafe fn cfg_read32(&self, addr: PCIAddress, off: u8) -> u32;
    unsafe fn cfg_write8(&self, addr: PCIAddress, off: u8, value: u8);
    unsafe fn cfg_write16(&self, addr: PCIAddress, off: u8, value: u16);
    unsafe fn cfg_write32(&self, addr: PCIAddress, off: u8, value: u32);
}

#[derive(Clone)]
pub struct PCIDevice<T: PCIControllerIO + ?Sized> {
    addr: PCIAddress,
    io: Box<T>,
}

#[allow(dead_code)]
impl<T: PCIControllerIO + ?Sized> PCIDevice<T> {
    pub fn new(addr: PCIAddress, io: Box<T>) -> Self {
        Self { addr, io }
    }

    pub unsafe fn cfg_read8<A: Into<u8>, R: From<u8>>(&self, off: A) -> R {
        self.io.cfg_read8(self.addr, off.into()).into()
    }

    pub unsafe fn cfg_read16<A: Into<u8>, R: From<u16>>(&self, off: A) -> R {
        self.io.cfg_read16(self.addr, off.into()).into()
    }

    pub unsafe fn cfg_read32<A: Into<u8>, R: From<u32>>(&self, off: A) -> R {
        self.io.cfg_read32(self.addr, off.into()).into()
    }

    pub unsafe fn cfg_write8<A: Into<u8>, R: Into<u8>>(&self, off: A, value: R) {
        self.io.cfg_write8(self.addr, off.into(), value.into());
    }

    pub unsafe fn cfg_write16<A: Into<u8>, R: Into<u16>>(&self, off: A, value: R) {
        self.io.cfg_write16(self.addr, off.into(), value.into());
    }

    pub unsafe fn cfg_write32<A: Into<u8>, R: Into<u32>>(&self, off: A, value: R) {
        self.io.cfg_write32(self.addr, off.into(), value.into());
    }
}

pub struct PCIController {
    entries: Option<Vec<MCFGEntry>>,
}

impl PCIController {
    #[must_use]
    pub fn new(mcfg: Option<&'static MCFG>) -> Self {
        Self {
            entries: mcfg.map(|mcfg| mcfg.entries().to_vec()),
        }
    }

    pub fn find_ent(&self, addr: PCIAddress) -> &MCFGEntry {
        self.entries
            .as_ref()
            .unwrap()
            .iter()
            .find(|v| addr.segment == v.segment && (v.bus_start..=v.bus_end).contains(&addr.bus))
            .unwrap()
    }

    pub fn segment_count(&self) -> u16 {
        self.entries.as_ref().map_or(1, |entries| {
            entries
                .iter()
                .map(|v| v.segment)
                .max()
                .map_or_else(|| 1, |v| v + 1)
        })
    }

    pub fn get_io(&self, addr: PCIAddress) -> Box<dyn PCIControllerIO> {
        if self.entries.is_none() {
            Box::new(pio::PCIPortIO::new())
        } else {
            let entry = self.find_ent(addr);
            Box::new(mmio::PCIMemoryIO::new(*entry))
        }
    }
}

impl PCIController {
    pub fn find<
        T: PCIControllerIO + ?Sized,
        IO: FnMut(PCIAddress) -> Box<T>,
        P: FnMut(&PCIDevice<T>) -> bool,
    >(
        &self,
        mut io: IO,
        mut pred: P,
    ) -> Option<PCIDevice<T>> {
        for segment in 0..self.segment_count() {
            for bus in 0..=255 {
                for slot in 0..32 {
                    for func in 0..8 {
                        let addr = PCIAddress {
                            segment,
                            bus,
                            slot,
                            func,
                        };
                        let device = PCIDevice::new(addr, io(addr));

                        if pred(&device) {
                            return Some(device);
                        }
                    }
                }
            }
        }
        None
    }
}
