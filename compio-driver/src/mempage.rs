use compio_buf::IoBuf;
use io_uring_buf_ring::Buffer;
use memmap2::MmapMut;

pub(crate) struct MemoryPage {
    page: MmapMut,
}

impl MemoryPage {
    pub fn new(size: usize) -> Result<Self, std::io::Error> {
        if !size.is_power_of_two() || size < 4096 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Size is not power of two or less than 4096"));
        }

        let mut page = memmap2::MmapMut::map_anon(size)?;
        page.fill(0);

        Ok(Self { page })
    }
}

unsafe impl Buffer for MemoryPage {
    fn ptr(&self) -> *mut std::mem::MaybeUninit<u8> {
        self.page.as_ptr().cast_mut().cast()
    }

    fn len(&self) -> usize {
        self.page.len()
    }

    fn drop(self) {
        drop(self)
    }
}
