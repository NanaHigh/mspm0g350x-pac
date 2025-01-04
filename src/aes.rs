#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved2: [u8; 0x0c],
    stat: Stat,
    _reserved3: [u8; 0x0800],
    pdbgctl: Pdbgctl,
    _reserved4: [u8; 0x04],
    int_event0_iidx: IntEvent0Iidx,
    _reserved5: [u8; 0x04],
    int_event0_imask: IntEvent0Imask,
    _reserved6: [u8; 0x04],
    int_event0_ris: IntEvent0Ris,
    _reserved7: [u8; 0x04],
    int_event0_mis: IntEvent0Mis,
    _reserved8: [u8; 0x04],
    int_event0_iset: IntEvent0Iset,
    _reserved9: [u8; 0x04],
    int_event0_iclr: IntEvent0Iclr,
    _reserved10: [u8; 0x04],
    int_event1_iidx: IntEvent1Iidx,
    _reserved11: [u8; 0x04],
    int_event1_imask: IntEvent1Imask,
    _reserved12: [u8; 0x04],
    int_event1_ris: IntEvent1Ris,
    _reserved13: [u8; 0x04],
    int_event1_mis: IntEvent1Mis,
    _reserved14: [u8; 0x04],
    int_event1_iset: IntEvent1Iset,
    _reserved15: [u8; 0x04],
    int_event1_iclr: IntEvent1Iclr,
    _reserved16: [u8; 0x04],
    int_event2_iidx: IntEvent2Iidx,
    _reserved17: [u8; 0x04],
    int_event2_imask: IntEvent2Imask,
    _reserved18: [u8; 0x04],
    int_event2_ris: IntEvent2Ris,
    _reserved19: [u8; 0x04],
    int_event2_mis: IntEvent2Mis,
    _reserved20: [u8; 0x04],
    int_event2_iset: IntEvent2Iset,
    _reserved21: [u8; 0x04],
    int_event2_iclr: IntEvent2Iclr,
    _reserved22: [u8; 0x04],
    int_event3_iidx: IntEvent3Iidx,
    _reserved23: [u8; 0x04],
    int_event3_imask: IntEvent3Imask,
    _reserved24: [u8; 0x04],
    int_event3_ris: IntEvent3Ris,
    _reserved25: [u8; 0x04],
    int_event3_mis: IntEvent3Mis,
    _reserved26: [u8; 0x04],
    int_event3_iset: IntEvent3Iset,
    _reserved27: [u8; 0x04],
    int_event3_iclr: IntEvent3Iclr,
    _reserved28: [u8; 0x04],
    evt_mode: EvtMode,
    _reserved29: [u8; 0x1c],
    aesactl0: Aesactl0,
    aesactl1: Aesactl1,
    aesastat: Aesastat,
    aesakey: Aesakey,
    aesadin: Aesadin,
    aesadout: Aesadout,
    aesaxdin: Aesaxdin,
    aesaxin: Aesaxin,
}
impl RegisterBlock {
    #[doc = "0x800 - Power enable"]
    #[inline(always)]
    pub const fn pwren(&self) -> &Pwren {
        &self.pwren
    }
    #[doc = "0x804 - Reset Control"]
    #[inline(always)]
    pub const fn rstctl(&self) -> &Rstctl {
        &self.rstctl
    }
    #[doc = "0x814 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn pdbgctl(&self) -> &Pdbgctl {
        &self.pdbgctl
    }
    #[doc = "0x1020 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn int_event0_iidx(&self) -> &IntEvent0Iidx {
        &self.int_event0_iidx
    }
    #[doc = "0x1028 - Interrupt mask"]
    #[inline(always)]
    pub const fn int_event0_imask(&self) -> &IntEvent0Imask {
        &self.int_event0_imask
    }
    #[doc = "0x1030 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_event0_ris(&self) -> &IntEvent0Ris {
        &self.int_event0_ris
    }
    #[doc = "0x1038 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_event0_mis(&self) -> &IntEvent0Mis {
        &self.int_event0_mis
    }
    #[doc = "0x1040 - Interrupt set"]
    #[inline(always)]
    pub const fn int_event0_iset(&self) -> &IntEvent0Iset {
        &self.int_event0_iset
    }
    #[doc = "0x1048 - Interrupt clear"]
    #[inline(always)]
    pub const fn int_event0_iclr(&self) -> &IntEvent0Iclr {
        &self.int_event0_iclr
    }
    #[doc = "0x1050 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn int_event1_iidx(&self) -> &IntEvent1Iidx {
        &self.int_event1_iidx
    }
    #[doc = "0x1058 - Interrupt mask"]
    #[inline(always)]
    pub const fn int_event1_imask(&self) -> &IntEvent1Imask {
        &self.int_event1_imask
    }
    #[doc = "0x1060 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_event1_ris(&self) -> &IntEvent1Ris {
        &self.int_event1_ris
    }
    #[doc = "0x1068 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_event1_mis(&self) -> &IntEvent1Mis {
        &self.int_event1_mis
    }
    #[doc = "0x1070 - Interrupt set"]
    #[inline(always)]
    pub const fn int_event1_iset(&self) -> &IntEvent1Iset {
        &self.int_event1_iset
    }
    #[doc = "0x1078 - Interrupt clear"]
    #[inline(always)]
    pub const fn int_event1_iclr(&self) -> &IntEvent1Iclr {
        &self.int_event1_iclr
    }
    #[doc = "0x1080 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn int_event2_iidx(&self) -> &IntEvent2Iidx {
        &self.int_event2_iidx
    }
    #[doc = "0x1088 - Interrupt mask"]
    #[inline(always)]
    pub const fn int_event2_imask(&self) -> &IntEvent2Imask {
        &self.int_event2_imask
    }
    #[doc = "0x1090 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_event2_ris(&self) -> &IntEvent2Ris {
        &self.int_event2_ris
    }
    #[doc = "0x1098 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_event2_mis(&self) -> &IntEvent2Mis {
        &self.int_event2_mis
    }
    #[doc = "0x10a0 - Interrupt set"]
    #[inline(always)]
    pub const fn int_event2_iset(&self) -> &IntEvent2Iset {
        &self.int_event2_iset
    }
    #[doc = "0x10a8 - Interrupt clear"]
    #[inline(always)]
    pub const fn int_event2_iclr(&self) -> &IntEvent2Iclr {
        &self.int_event2_iclr
    }
    #[doc = "0x10b0 - Interrupt Index Register"]
    #[inline(always)]
    pub const fn int_event3_iidx(&self) -> &IntEvent3Iidx {
        &self.int_event3_iidx
    }
    #[doc = "0x10b8 - Interrupt mask"]
    #[inline(always)]
    pub const fn int_event3_imask(&self) -> &IntEvent3Imask {
        &self.int_event3_imask
    }
    #[doc = "0x10c0 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_event3_ris(&self) -> &IntEvent3Ris {
        &self.int_event3_ris
    }
    #[doc = "0x10c8 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_event3_mis(&self) -> &IntEvent3Mis {
        &self.int_event3_mis
    }
    #[doc = "0x10d0 - Interrupt set"]
    #[inline(always)]
    pub const fn int_event3_iset(&self) -> &IntEvent3Iset {
        &self.int_event3_iset
    }
    #[doc = "0x10d8 - Interrupt clear"]
    #[inline(always)]
    pub const fn int_event3_iclr(&self) -> &IntEvent3Iclr {
        &self.int_event3_iclr
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x1100 - AES accelerator control register 0"]
    #[inline(always)]
    pub const fn aesactl0(&self) -> &Aesactl0 {
        &self.aesactl0
    }
    #[doc = "0x1104 - AES accelerator control register 1"]
    #[inline(always)]
    pub const fn aesactl1(&self) -> &Aesactl1 {
        &self.aesactl1
    }
    #[doc = "0x1108 - aes accelerator status register"]
    #[inline(always)]
    pub const fn aesastat(&self) -> &Aesastat {
        &self.aesastat
    }
    #[doc = "0x110c - aes accelerator key register"]
    #[inline(always)]
    pub const fn aesakey(&self) -> &Aesakey {
        &self.aesakey
    }
    #[doc = "0x1110 - aes accelerator data in register"]
    #[inline(always)]
    pub const fn aesadin(&self) -> &Aesadin {
        &self.aesadin
    }
    #[doc = "0x1114 - aes accelerator data out register"]
    #[inline(always)]
    pub const fn aesadout(&self) -> &Aesadout {
        &self.aesadout
    }
    #[doc = "0x1118 - aes accelerator xored data in register"]
    #[inline(always)]
    pub const fn aesaxdin(&self) -> &Aesaxdin {
        &self.aesaxdin
    }
    #[doc = "0x111c - aes accelerator xored data in register (no trigger)"]
    #[inline(always)]
    pub const fn aesaxin(&self) -> &Aesaxin {
        &self.aesaxin
    }
}
#[doc = "PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"]
#[doc(alias = "PWREN")]
pub type Pwren = crate::Reg<pwren::PwrenSpec>;
#[doc = "Power enable"]
pub mod pwren;
#[doc = "RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`]
module"]
#[doc(alias = "RSTCTL")]
pub type Rstctl = crate::Reg<rstctl::RstctlSpec>;
#[doc = "Reset Control"]
pub mod rstctl;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbgctl`]
module"]
#[doc(alias = "PDBGCTL")]
pub type Pdbgctl = crate::Reg<pdbgctl::PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod pdbgctl;
#[doc = "INT_EVENT0_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_iidx`]
module"]
#[doc(alias = "INT_EVENT0_IIDX")]
pub type IntEvent0Iidx = crate::Reg<int_event0_iidx::IntEvent0IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod int_event0_iidx;
#[doc = "INT_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_imask`]
module"]
#[doc(alias = "INT_EVENT0_IMASK")]
pub type IntEvent0Imask = crate::Reg<int_event0_imask::IntEvent0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod int_event0_imask;
#[doc = "INT_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_ris`]
module"]
#[doc(alias = "INT_EVENT0_RIS")]
pub type IntEvent0Ris = crate::Reg<int_event0_ris::IntEvent0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod int_event0_ris;
#[doc = "INT_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_mis`]
module"]
#[doc(alias = "INT_EVENT0_MIS")]
pub type IntEvent0Mis = crate::Reg<int_event0_mis::IntEvent0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod int_event0_mis;
#[doc = "INT_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_iset`]
module"]
#[doc(alias = "INT_EVENT0_ISET")]
pub type IntEvent0Iset = crate::Reg<int_event0_iset::IntEvent0IsetSpec>;
#[doc = "Interrupt set"]
pub mod int_event0_iset;
#[doc = "INT_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_iclr`]
module"]
#[doc(alias = "INT_EVENT0_ICLR")]
pub type IntEvent0Iclr = crate::Reg<int_event0_iclr::IntEvent0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod int_event0_iclr;
#[doc = "INT_EVENT1_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_iidx`]
module"]
#[doc(alias = "INT_EVENT1_IIDX")]
pub type IntEvent1Iidx = crate::Reg<int_event1_iidx::IntEvent1IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod int_event1_iidx;
#[doc = "INT_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_imask`]
module"]
#[doc(alias = "INT_EVENT1_IMASK")]
pub type IntEvent1Imask = crate::Reg<int_event1_imask::IntEvent1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod int_event1_imask;
#[doc = "INT_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_ris`]
module"]
#[doc(alias = "INT_EVENT1_RIS")]
pub type IntEvent1Ris = crate::Reg<int_event1_ris::IntEvent1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod int_event1_ris;
#[doc = "INT_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_mis`]
module"]
#[doc(alias = "INT_EVENT1_MIS")]
pub type IntEvent1Mis = crate::Reg<int_event1_mis::IntEvent1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod int_event1_mis;
#[doc = "INT_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_iset`]
module"]
#[doc(alias = "INT_EVENT1_ISET")]
pub type IntEvent1Iset = crate::Reg<int_event1_iset::IntEvent1IsetSpec>;
#[doc = "Interrupt set"]
pub mod int_event1_iset;
#[doc = "INT_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_iclr`]
module"]
#[doc(alias = "INT_EVENT1_ICLR")]
pub type IntEvent1Iclr = crate::Reg<int_event1_iclr::IntEvent1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod int_event1_iclr;
#[doc = "INT_EVENT2_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iidx`]
module"]
#[doc(alias = "INT_EVENT2_IIDX")]
pub type IntEvent2Iidx = crate::Reg<int_event2_iidx::IntEvent2IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod int_event2_iidx;
#[doc = "INT_EVENT2_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_imask`]
module"]
#[doc(alias = "INT_EVENT2_IMASK")]
pub type IntEvent2Imask = crate::Reg<int_event2_imask::IntEvent2ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod int_event2_imask;
#[doc = "INT_EVENT2_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_ris`]
module"]
#[doc(alias = "INT_EVENT2_RIS")]
pub type IntEvent2Ris = crate::Reg<int_event2_ris::IntEvent2RisSpec>;
#[doc = "Raw interrupt status"]
pub mod int_event2_ris;
#[doc = "INT_EVENT2_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_mis`]
module"]
#[doc(alias = "INT_EVENT2_MIS")]
pub type IntEvent2Mis = crate::Reg<int_event2_mis::IntEvent2MisSpec>;
#[doc = "Masked interrupt status"]
pub mod int_event2_mis;
#[doc = "INT_EVENT2_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iset`]
module"]
#[doc(alias = "INT_EVENT2_ISET")]
pub type IntEvent2Iset = crate::Reg<int_event2_iset::IntEvent2IsetSpec>;
#[doc = "Interrupt set"]
pub mod int_event2_iset;
#[doc = "INT_EVENT2_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iclr`]
module"]
#[doc(alias = "INT_EVENT2_ICLR")]
pub type IntEvent2Iclr = crate::Reg<int_event2_iclr::IntEvent2IclrSpec>;
#[doc = "Interrupt clear"]
pub mod int_event2_iclr;
#[doc = "INT_EVENT3_IIDX (r) register accessor: Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event3_iidx`]
module"]
#[doc(alias = "INT_EVENT3_IIDX")]
pub type IntEvent3Iidx = crate::Reg<int_event3_iidx::IntEvent3IidxSpec>;
#[doc = "Interrupt Index Register"]
pub mod int_event3_iidx;
#[doc = "INT_EVENT3_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event3_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event3_imask`]
module"]
#[doc(alias = "INT_EVENT3_IMASK")]
pub type IntEvent3Imask = crate::Reg<int_event3_imask::IntEvent3ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod int_event3_imask;
#[doc = "INT_EVENT3_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event3_ris`]
module"]
#[doc(alias = "INT_EVENT3_RIS")]
pub type IntEvent3Ris = crate::Reg<int_event3_ris::IntEvent3RisSpec>;
#[doc = "Raw interrupt status"]
pub mod int_event3_ris;
#[doc = "INT_EVENT3_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event3_mis`]
module"]
#[doc(alias = "INT_EVENT3_MIS")]
pub type IntEvent3Mis = crate::Reg<int_event3_mis::IntEvent3MisSpec>;
#[doc = "Masked interrupt status"]
pub mod int_event3_mis;
#[doc = "INT_EVENT3_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event3_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event3_iset`]
module"]
#[doc(alias = "INT_EVENT3_ISET")]
pub type IntEvent3Iset = crate::Reg<int_event3_iset::IntEvent3IsetSpec>;
#[doc = "Interrupt set"]
pub mod int_event3_iset;
#[doc = "INT_EVENT3_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event3_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event3_iclr`]
module"]
#[doc(alias = "INT_EVENT3_ICLR")]
pub type IntEvent3Iclr = crate::Reg<int_event3_iclr::IntEvent3IclrSpec>;
#[doc = "Interrupt clear"]
pub mod int_event3_iclr;
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "AESACTL0 (rw) register accessor: AES accelerator control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesactl0`]
module"]
#[doc(alias = "AESACTL0")]
pub type Aesactl0 = crate::Reg<aesactl0::Aesactl0Spec>;
#[doc = "AES accelerator control register 0"]
pub mod aesactl0;
#[doc = "AESACTL1 (rw) register accessor: AES accelerator control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesactl1`]
module"]
#[doc(alias = "AESACTL1")]
pub type Aesactl1 = crate::Reg<aesactl1::Aesactl1Spec>;
#[doc = "AES accelerator control register 1"]
pub mod aesactl1;
#[doc = "AESASTAT (rw) register accessor: aes accelerator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesastat`]
module"]
#[doc(alias = "AESASTAT")]
pub type Aesastat = crate::Reg<aesastat::AesastatSpec>;
#[doc = "aes accelerator status register"]
pub mod aesastat;
#[doc = "AESAKEY (w) register accessor: aes accelerator key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesakey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesakey`]
module"]
#[doc(alias = "AESAKEY")]
pub type Aesakey = crate::Reg<aesakey::AesakeySpec>;
#[doc = "aes accelerator key register"]
pub mod aesakey;
#[doc = "AESADIN (w) register accessor: aes accelerator data in register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadin`]
module"]
#[doc(alias = "AESADIN")]
pub type Aesadin = crate::Reg<aesadin::AesadinSpec>;
#[doc = "aes accelerator data in register"]
pub mod aesadin;
#[doc = "AESADOUT (r) register accessor: aes accelerator data out register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesadout`]
module"]
#[doc(alias = "AESADOUT")]
pub type Aesadout = crate::Reg<aesadout::AesadoutSpec>;
#[doc = "aes accelerator data out register"]
pub mod aesadout;
#[doc = "AESAXDIN (w) register accessor: aes accelerator xored data in register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxdin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesaxdin`]
module"]
#[doc(alias = "AESAXDIN")]
pub type Aesaxdin = crate::Reg<aesaxdin::AesaxdinSpec>;
#[doc = "aes accelerator xored data in register"]
pub mod aesaxdin;
#[doc = "AESAXIN (w) register accessor: aes accelerator xored data in register (no trigger)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesaxin`]
module"]
#[doc(alias = "AESAXIN")]
pub type Aesaxin = crate::Reg<aesaxin::AesaxinSpec>;
#[doc = "aes accelerator xored data in register (no trigger)"]
pub mod aesaxin;
