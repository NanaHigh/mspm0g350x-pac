#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    fsub_0: Fsub0,
    fsub_1: Fsub1,
}
impl RegisterBlock {
    #[doc = "0x400 - Subscriber Port 0"]
    #[inline(always)]
    pub const fn fsub_0(&self) -> &Fsub0 {
        &self.fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn fsub_1(&self) -> &Fsub1 {
        &self.fsub_1
    }
}
#[doc = "FSUB_0 (rw) register accessor: Subscriber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_0`]
module"]
#[doc(alias = "FSUB_0")]
pub type Fsub0 = crate::Reg<fsub_0::Fsub0Spec>;
#[doc = "Subscriber Port 0"]
pub mod fsub_0;
#[doc = "FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_1`]
module"]
#[doc(alias = "FSUB_1")]
pub type Fsub1 = crate::Reg<fsub_1::Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod fsub_1;
