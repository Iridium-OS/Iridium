// Copyright (c) ChefKiss Inc 2021-2022.
// This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

use alloc::vec::Vec;

use acpi::tables::SDTHeader;

pub mod apic;
pub mod ioapic;
pub mod madt;

#[derive(Debug)]
pub struct ACPIPlatform {
    pub version: u8,
    pub tables: Vec<&'static SDTHeader>,
}

impl ACPIPlatform {
    pub fn new(rsdp: &'static acpi::tables::rsdp::RSDP) -> Self {
        let mut tables = Vec::new();

        for ent in rsdp.as_type().iter() {
            if !ent.validate() {
                debug!("Invalid table: {ent:X?}");
                continue;
            }

            debug!("Table: {ent:#X?}");
            tables.push(ent);
        }

        Self {
            version: rsdp.revision,
            tables,
        }
    }

    pub fn find<T>(&self, signature: &str) -> Option<&'static T> {
        self.tables
            .iter()
            .find(|&a| a.signature() == signature)
            .map(|&v| unsafe { (v as *const SDTHeader).cast::<T>().as_ref().unwrap() })
    }
}

pub fn get_hpet(
    state: &mut crate::sys::state::SystemState,
) -> super::timer::hpet::HighPrecisionEventTimer {
    let acpi = state.acpi.get_mut().unwrap();
    let pml4 = state.pml4.get_mut().unwrap();

    acpi.find("HPET")
        .map(|v| unsafe {
            let addr = v as *const _ as u64;
            pml4.map_mmio(
                addr,
                addr - amd64::paging::PHYS_VIRT_OFFSET,
                1,
                amd64::paging::PageTableEntry::new()
                    .with_present(true)
                    .with_writable(true),
            );
            super::timer::hpet::HighPrecisionEventTimer::new(v)
        })
        .unwrap()
}
