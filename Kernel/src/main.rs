// Copyright (c) ChefKiss Inc 2021-2023. Licensed under the Thou Shalt Not Profit License version 1.0. See LICENSE for details.

#![no_std]
#![no_main]
#![deny(warnings, clippy::cargo, clippy::nursery, unused_extern_crates)]
#![feature(
    asm_const,
    alloc_error_handler,
    const_size_of_val,
    naked_functions,
    const_mut_refs,
    sync_unsafe_cell
)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate log;

mod acpi;
mod intrs;
mod system;
mod timer;
mod utils;

#[no_mangle]
extern "C" fn kernel_main(boot_info: &'static sulphur_dioxide::BootInfo) -> ! {
    utils::logger::init();
    assert_eq!(boot_info.revision, sulphur_dioxide::CURRENT_REVISION);
    utils::init_core(boot_info);
    debug!("Copyright ChefKiss Inc 2021-2023.");

    let state = unsafe { &mut *crate::system::state::SYS_STATE.get() };
    state.terminal = boot_info.frame_buffer.map(|fb_info| {
        debug!("Got boot display: {:X?}", fb_info);
        let mut terminal = crate::system::terminal::Terminal::new(unsafe {
            paper_fb::fb::FrameBuffer::new(
                fb_info.base,
                fb_info.resolution.width,
                fb_info.resolution.height,
                fb_info.pitch,
                paper_fb::pixel::BitMask {
                    r: fb_info.pixel_bitmask.red,
                    g: fb_info.pixel_bitmask.green,
                    b: fb_info.pixel_bitmask.blue,
                    a: fb_info.pixel_bitmask.alpha,
                },
            )
        });
        terminal.clear();
        terminal
    });

    utils::init_paging(state);

    acpi::madt::setup(state);
    acpi::apic::setup(state);

    state.tkcache = Some(spin::Mutex::new(
        postcard::from_bytes(boot_info.dc_cache).unwrap(),
    ));
    state.scheduler = Some(spin::Mutex::new(system::proc::scheduler::Scheduler::new(
        &acpi::get_hpet(state),
    )));

    system::tkext::spawn_new_matches();

    debug!("I'm out of here!");
    system::proc::userland::setup();
    system::proc::scheduler::Scheduler::unmask();

    hlt_loop!();
}
