// Copyright (c) ChefKiss 2021-2024. Licensed under the Thou Shalt Not Profit License version 1.5. See LICENSE for details.

use amd64::paging::PAGE_SIZE;
use skyliftkit::{MemoryData, MemoryEntry};

pub struct BitmapAllocator {
    bitmap: &'static mut [u64],
    highest_addr: u64,
    pub free_pages: u64,
    pub total_pages: u64,
    last_index: u64,
}

impl BitmapAllocator {
    #[inline]
    pub fn new(mmap: &'static [MemoryEntry]) -> Self {
        let highest_addr = mmap
            .iter()
            .flat_map(|v| {
                let MemoryEntry::Usable(v) = v else {
                    return None;
                };
                Some(v.base + v.length)
            })
            .max()
            .unwrap();

        let bitmap_sz = (highest_addr / PAGE_SIZE) / 8 + 8; // Why + 8?
        debug!(
            "Highest usable address: {highest_addr:#X?}, Bitmap size: {bitmap_sz} bytes, {} \
             entries",
            bitmap_sz / 8
        );

        let mut bitmap: &mut [u64] = Default::default();

        let mut free_pages = 0;

        for v in mmap.iter().flat_map(|v| {
            let MemoryEntry::Usable(v) = v else {
                return None;
            };
            // Skip the first 2 MiB.
            Some(if v.base <= 0x20_0000 && v.base + v.length > 0x20_0000 {
                MemoryData::new(0x20_0000, v.length - 0x20_0000)
            } else {
                *v
            })
        }) {
            if v.length == 0 {
                continue;
            }

            debug!("Base: {:#X?}, End: {:#X?}", v.base, v.base + v.length);
            let v = if bitmap.is_empty() && v.length >= bitmap_sz {
                bitmap = unsafe {
                    core::slice::from_raw_parts_mut(
                        (v.base + amd64::paging::PHYS_VIRT_OFFSET) as *mut _,
                        (bitmap_sz / 8) as _,
                    )
                };
                bitmap.fill(!0u64);

                trace!("Bitmap is here");
                MemoryData::new(v.base + bitmap_sz, v.length - bitmap_sz)
            } else {
                v
            };

            let base = v.base / PAGE_SIZE;
            let count = v.length / PAGE_SIZE;
            debug!(
                "Base: {base:#X?}, Count: {count:#X?}, End: {}",
                base + count
            );
            for i in base..(base + count) {
                crate::bitmap::bit_reset(bitmap, i);
            }
            free_pages += count;
        }

        let total_pages = highest_addr / PAGE_SIZE;
        Self {
            bitmap,
            highest_addr,
            total_pages,
            free_pages,
            last_index: 0,
        }
    }

    unsafe fn internal_alloc(&mut self, count: u64, limit: u64) -> Option<*mut u8> {
        let mut n = 0;

        while self.last_index < limit {
            let set = crate::bitmap::bit_test(self.bitmap, self.last_index);
            self.last_index += 1;
            if set {
                n = 0;
                continue;
            }

            n += 1;

            if n == count {
                let page = self.last_index - count;

                for i in page..self.last_index {
                    crate::bitmap::bit_set(self.bitmap, i);
                }

                self.free_pages -= count;

                return Some((page * PAGE_SIZE) as *mut _);
            }
        }

        None
    }

    pub unsafe fn alloc(&mut self, count: u64) -> Option<*mut u8> {
        let l = self.last_index;

        self.internal_alloc(count, self.highest_addr / PAGE_SIZE)
            .or_else(|| {
                self.last_index = 0;
                self.internal_alloc(count, l)
            })
    }

    pub unsafe fn free(&mut self, ptr: *mut u8, count: u64) {
        let idx = ptr as u64 / PAGE_SIZE;

        for i in idx..(idx + count) {
            crate::bitmap::bit_reset(self.bitmap, i);
        }

        self.free_pages += count;
    }

    pub fn is_allocated(&self, ptr: *mut u8, count: u64) -> bool {
        let idx = ptr as u64 / PAGE_SIZE;

        for i in idx..(idx + count) {
            if !crate::bitmap::bit_test(self.bitmap, i) {
                return false;
            }
        }

        true
    }
}
