#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10e0],
    evt_mode: EvtMode,
    _reserved1: [u8; 0x18],
    desc: Desc,
    int_group0_iidx: IntGroup0Iidx,
    _reserved3: [u8; 0x04],
    int_group0_imask: IntGroup0Imask,
    _reserved4: [u8; 0x04],
    int_group0_ris: IntGroup0Ris,
    _reserved5: [u8; 0x04],
    int_group0_mis: IntGroup0Mis,
    _reserved6: [u8; 0x04],
    int_group0_iset: IntGroup0Iset,
    _reserved7: [u8; 0x04],
    int_group0_iclr: IntGroup0Iclr,
    _reserved8: [u8; 0x04],
    int_group1_iidx: IntGroup1Iidx,
    _reserved9: [u8; 0x04],
    int_group1_imask: IntGroup1Imask,
    _reserved10: [u8; 0x04],
    int_group1_ris: IntGroup1Ris,
    _reserved11: [u8; 0x04],
    int_group1_mis: IntGroup1Mis,
    _reserved12: [u8; 0x04],
    int_group1_iset: IntGroup1Iset,
    _reserved13: [u8; 0x04],
    int_group1_iclr: IntGroup1Iclr,
    _reserved14: [u8; 0x01a4],
    ctl: Ctl,
}
impl RegisterBlock {
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1100 - Interrupt index"]
    #[inline(always)]
    pub const fn int_group0_iidx(&self) -> &IntGroup0Iidx {
        &self.int_group0_iidx
    }
    #[doc = "0x1108 - Interrupt mask"]
    #[inline(always)]
    pub const fn int_group0_imask(&self) -> &IntGroup0Imask {
        &self.int_group0_imask
    }
    #[doc = "0x1110 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_group0_ris(&self) -> &IntGroup0Ris {
        &self.int_group0_ris
    }
    #[doc = "0x1118 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_group0_mis(&self) -> &IntGroup0Mis {
        &self.int_group0_mis
    }
    #[doc = "0x1120 - Interrupt set"]
    #[inline(always)]
    pub const fn int_group0_iset(&self) -> &IntGroup0Iset {
        &self.int_group0_iset
    }
    #[doc = "0x1128 - Interrupt clear"]
    #[inline(always)]
    pub const fn int_group0_iclr(&self) -> &IntGroup0Iclr {
        &self.int_group0_iclr
    }
    #[doc = "0x1130 - Interrupt index"]
    #[inline(always)]
    pub const fn int_group1_iidx(&self) -> &IntGroup1Iidx {
        &self.int_group1_iidx
    }
    #[doc = "0x1138 - Interrupt mask"]
    #[inline(always)]
    pub const fn int_group1_imask(&self) -> &IntGroup1Imask {
        &self.int_group1_imask
    }
    #[doc = "0x1140 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_group1_ris(&self) -> &IntGroup1Ris {
        &self.int_group1_ris
    }
    #[doc = "0x1148 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_group1_mis(&self) -> &IntGroup1Mis {
        &self.int_group1_mis
    }
    #[doc = "0x1150 - Interrupt set"]
    #[inline(always)]
    pub const fn int_group1_iset(&self) -> &IntGroup1Iset {
        &self.int_group1_iset
    }
    #[doc = "0x1158 - Interrupt clear"]
    #[inline(always)]
    pub const fn int_group1_iclr(&self) -> &IntGroup1Iclr {
        &self.int_group1_iclr
    }
    #[doc = "0x1300 - Prefetch/Cache control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
}
#[doc = "EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "INT_GROUP0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group0_iidx`]
module"]
#[doc(alias = "INT_GROUP0_IIDX")]
pub type IntGroup0Iidx = crate::Reg<int_group0_iidx::IntGroup0IidxSpec>;
#[doc = "Interrupt index"]
pub mod int_group0_iidx;
#[doc = "INT_GROUP0_IMASK (r) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_imask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group0_imask`]
module"]
#[doc(alias = "INT_GROUP0_IMASK")]
pub type IntGroup0Imask = crate::Reg<int_group0_imask::IntGroup0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod int_group0_imask;
#[doc = "INT_GROUP0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group0_ris`]
module"]
#[doc(alias = "INT_GROUP0_RIS")]
pub type IntGroup0Ris = crate::Reg<int_group0_ris::IntGroup0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod int_group0_ris;
#[doc = "INT_GROUP0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group0_mis`]
module"]
#[doc(alias = "INT_GROUP0_MIS")]
pub type IntGroup0Mis = crate::Reg<int_group0_mis::IntGroup0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod int_group0_mis;
#[doc = "INT_GROUP0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_group0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group0_iset`]
module"]
#[doc(alias = "INT_GROUP0_ISET")]
pub type IntGroup0Iset = crate::Reg<int_group0_iset::IntGroup0IsetSpec>;
#[doc = "Interrupt set"]
pub mod int_group0_iset;
#[doc = "INT_GROUP0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_group0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group0_iclr`]
module"]
#[doc(alias = "INT_GROUP0_ICLR")]
pub type IntGroup0Iclr = crate::Reg<int_group0_iclr::IntGroup0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod int_group0_iclr;
#[doc = "INT_GROUP1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group1_iidx`]
module"]
#[doc(alias = "INT_GROUP1_IIDX")]
pub type IntGroup1Iidx = crate::Reg<int_group1_iidx::IntGroup1IidxSpec>;
#[doc = "Interrupt index"]
pub mod int_group1_iidx;
#[doc = "INT_GROUP1_IMASK (r) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_imask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group1_imask`]
module"]
#[doc(alias = "INT_GROUP1_IMASK")]
pub type IntGroup1Imask = crate::Reg<int_group1_imask::IntGroup1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod int_group1_imask;
#[doc = "INT_GROUP1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group1_ris`]
module"]
#[doc(alias = "INT_GROUP1_RIS")]
pub type IntGroup1Ris = crate::Reg<int_group1_ris::IntGroup1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod int_group1_ris;
#[doc = "INT_GROUP1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group1_mis`]
module"]
#[doc(alias = "INT_GROUP1_MIS")]
pub type IntGroup1Mis = crate::Reg<int_group1_mis::IntGroup1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod int_group1_mis;
#[doc = "INT_GROUP1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_group1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group1_iset`]
module"]
#[doc(alias = "INT_GROUP1_ISET")]
pub type IntGroup1Iset = crate::Reg<int_group1_iset::IntGroup1IsetSpec>;
#[doc = "Interrupt set"]
pub mod int_group1_iset;
#[doc = "INT_GROUP1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_group1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_group1_iclr`]
module"]
#[doc(alias = "INT_GROUP1_ICLR")]
pub type IntGroup1Iclr = crate::Reg<int_group1_iclr::IntGroup1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod int_group1_iclr;
#[doc = "CTL (rw) register accessor: Prefetch/Cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Prefetch/Cache control"]
pub mod ctl;
