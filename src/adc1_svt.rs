#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0160],
    fifodata: Fifodata,
    _reserved1: [u8; 0x011c],
    memres: [Memres; 12],
}
impl RegisterBlock {
    #[doc = "0x160 - FIFO Data Register"]
    #[inline(always)]
    pub const fn fifodata(&self) -> &Fifodata {
        &self.fifodata
    }
    #[doc = "0x280..0x2b0 - Memory Result Register"]
    #[inline(always)]
    pub const fn memres(&self, n: usize) -> &Memres {
        &self.memres[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x2b0 - Memory Result Register"]
    #[inline(always)]
    pub fn memres_iter(&self) -> impl Iterator<Item = &Memres> {
        self.memres.iter()
    }
}
#[doc = "FIFODATA (r) register accessor: FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodata`]
module"]
#[doc(alias = "FIFODATA")]
pub type Fifodata = crate::Reg<fifodata::FifodataSpec>;
#[doc = "FIFO Data Register"]
pub mod fifodata;
#[doc = "MEMRES (r) register accessor: Memory Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memres::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memres`]
module"]
#[doc(alias = "MEMRES")]
pub type Memres = crate::Reg<memres::MemresSpec>;
#[doc = "Memory Result Register"]
pub mod memres;
