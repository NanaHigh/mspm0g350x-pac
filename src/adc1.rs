#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    fsub_0: Fsub0,
    _reserved1: [u8; 0x40],
    fpub_1: Fpub1,
    _reserved2: [u8; 0x03b8],
    pwren: Pwren,
    rstctl: Rstctl,
    clkcfg: Clkcfg,
    _reserved5: [u8; 0x08],
    stat: Stat,
    _reserved6: [u8; 0x0808],
    int_event0_iidx: IntEvent0Iidx,
    _reserved7: [u8; 0x04],
    int_event0_imask: IntEvent0Imask,
    _reserved8: [u8; 0x04],
    int_event0_ris: IntEvent0Ris,
    _reserved9: [u8; 0x04],
    int_event0_mis: IntEvent0Mis,
    _reserved10: [u8; 0x04],
    int_event0_iset: IntEvent0Iset,
    _reserved11: [u8; 0x04],
    int_event0_iclr: IntEvent0Iclr,
    _reserved12: [u8; 0x04],
    int_event1_iidx: IntEvent1Iidx,
    _reserved13: [u8; 0x04],
    int_event1_imask: IntEvent1Imask,
    _reserved14: [u8; 0x04],
    int_event1_ris: IntEvent1Ris,
    _reserved15: [u8; 0x04],
    int_event1_mis: IntEvent1Mis,
    _reserved16: [u8; 0x04],
    int_event1_iset: IntEvent1Iset,
    _reserved17: [u8; 0x04],
    int_event1_iclr: IntEvent1Iclr,
    _reserved18: [u8; 0x04],
    int_event2_iidx: IntEvent2Iidx,
    _reserved19: [u8; 0x04],
    int_event2_imask: IntEvent2Imask,
    _reserved20: [u8; 0x04],
    int_event2_ris: IntEvent2Ris,
    _reserved21: [u8; 0x04],
    int_event2_mis: IntEvent2Mis,
    _reserved22: [u8; 0x04],
    int_event2_iset: IntEvent2Iset,
    _reserved23: [u8; 0x04],
    int_event2_iclr: IntEvent2Iclr,
    _reserved24: [u8; 0x34],
    evt_mode: EvtMode,
    _reserved25: [u8; 0x18],
    desc: Desc,
    ctl0: Ctl0,
    ctl1: Ctl1,
    ctl2: Ctl2,
    _reserved29: [u8; 0x04],
    clkfreq: Clkfreq,
    scomp0: Scomp0,
    scomp1: Scomp1,
    _reserved32: [u8; 0x2c],
    wclow: Wclow,
    _reserved33: [u8; 0x04],
    wchigh: Wchigh,
    _reserved34: [u8; 0x2c],
    memctl: [Memctl; 12],
    _reserved35: [u8; 0x0190],
    status: Status,
}
impl RegisterBlock {
    #[doc = "0x400 - Subscriber Configuration Register."]
    #[inline(always)]
    pub const fn fsub_0(&self) -> &Fsub0 {
        &self.fsub_0
    }
    #[doc = "0x444 - Publisher Configuration Register."]
    #[inline(always)]
    pub const fn fpub_1(&self) -> &Fpub1 {
        &self.fpub_1
    }
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
    #[doc = "0x808 - ADC clock configuration Register"]
    #[inline(always)]
    pub const fn clkcfg(&self) -> &Clkcfg {
        &self.clkcfg
    }
    #[doc = "0x814 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x1020 - Interrupt index"]
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
    #[doc = "0x1050 - Interrupt index"]
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
    #[doc = "0x1080 - Interrupt index"]
    #[inline(always)]
    pub const fn int_event2_iidx(&self) -> &IntEvent2Iidx {
        &self.int_event2_iidx
    }
    #[doc = "0x1088 - Interrupt mask extension"]
    #[inline(always)]
    pub const fn int_event2_imask(&self) -> &IntEvent2Imask {
        &self.int_event2_imask
    }
    #[doc = "0x1090 - Raw interrupt status extension"]
    #[inline(always)]
    pub const fn int_event2_ris(&self) -> &IntEvent2Ris {
        &self.int_event2_ris
    }
    #[doc = "0x1098 - Masked interrupt status extension"]
    #[inline(always)]
    pub const fn int_event2_mis(&self) -> &IntEvent2Mis {
        &self.int_event2_mis
    }
    #[doc = "0x10a0 - Interrupt set extension"]
    #[inline(always)]
    pub const fn int_event2_iset(&self) -> &IntEvent2Iset {
        &self.int_event2_iset
    }
    #[doc = "0x10a8 - Interrupt clear extension"]
    #[inline(always)]
    pub const fn int_event2_iclr(&self) -> &IntEvent2Iclr {
        &self.int_event2_iclr
    }
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
    #[doc = "0x1100 - Control Register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x1104 - Control Register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x1108 - Control Register 2"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &Ctl2 {
        &self.ctl2
    }
    #[doc = "0x1110 - Sample Clock Frequency Range Register"]
    #[inline(always)]
    pub const fn clkfreq(&self) -> &Clkfreq {
        &self.clkfreq
    }
    #[doc = "0x1114 - Sample Time Compare 0 Register"]
    #[inline(always)]
    pub const fn scomp0(&self) -> &Scomp0 {
        &self.scomp0
    }
    #[doc = "0x1118 - Sample Time Compare 1 Register"]
    #[inline(always)]
    pub const fn scomp1(&self) -> &Scomp1 {
        &self.scomp1
    }
    #[doc = "0x1148 - Window Comparator Low Threshold Register"]
    #[inline(always)]
    pub const fn wclow(&self) -> &Wclow {
        &self.wclow
    }
    #[doc = "0x1150 - Window Comparator High Threshold Register"]
    #[inline(always)]
    pub const fn wchigh(&self) -> &Wchigh {
        &self.wchigh
    }
    #[doc = "0x1180..0x11b0 - Conversion Memory Control Register"]
    #[inline(always)]
    pub const fn memctl(&self, n: usize) -> &Memctl {
        &self.memctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1180..0x11b0 - Conversion Memory Control Register"]
    #[inline(always)]
    pub fn memctl_iter(&self) -> impl Iterator<Item = &Memctl> {
        self.memctl.iter()
    }
    #[doc = "0x1340 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "FSUB_0 (rw) register accessor: Subscriber Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_0`]
module"]
#[doc(alias = "FSUB_0")]
pub type Fsub0 = crate::Reg<fsub_0::Fsub0Spec>;
#[doc = "Subscriber Configuration Register."]
pub mod fsub_0;
#[doc = "FPUB_1 (rw) register accessor: Publisher Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpub_1`]
module"]
#[doc(alias = "FPUB_1")]
pub type Fpub1 = crate::Reg<fpub_1::Fpub1Spec>;
#[doc = "Publisher Configuration Register."]
pub mod fpub_1;
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
#[doc = "CLKCFG (rw) register accessor: ADC clock configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg`]
module"]
#[doc(alias = "CLKCFG")]
pub type Clkcfg = crate::Reg<clkcfg::ClkcfgSpec>;
#[doc = "ADC clock configuration Register"]
pub mod clkcfg;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "INT_EVENT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_iidx`]
module"]
#[doc(alias = "INT_EVENT0_IIDX")]
pub type IntEvent0Iidx = crate::Reg<int_event0_iidx::IntEvent0IidxSpec>;
#[doc = "Interrupt index"]
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
#[doc = "INT_EVENT1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_iidx`]
module"]
#[doc(alias = "INT_EVENT1_IIDX")]
pub type IntEvent1Iidx = crate::Reg<int_event1_iidx::IntEvent1IidxSpec>;
#[doc = "Interrupt index"]
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
#[doc = "INT_EVENT2_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iidx`]
module"]
#[doc(alias = "INT_EVENT2_IIDX")]
pub type IntEvent2Iidx = crate::Reg<int_event2_iidx::IntEvent2IidxSpec>;
#[doc = "Interrupt index"]
pub mod int_event2_iidx;
#[doc = "INT_EVENT2_IMASK (rw) register accessor: Interrupt mask extension\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_imask`]
module"]
#[doc(alias = "INT_EVENT2_IMASK")]
pub type IntEvent2Imask = crate::Reg<int_event2_imask::IntEvent2ImaskSpec>;
#[doc = "Interrupt mask extension"]
pub mod int_event2_imask;
#[doc = "INT_EVENT2_RIS (r) register accessor: Raw interrupt status extension\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_ris`]
module"]
#[doc(alias = "INT_EVENT2_RIS")]
pub type IntEvent2Ris = crate::Reg<int_event2_ris::IntEvent2RisSpec>;
#[doc = "Raw interrupt status extension"]
pub mod int_event2_ris;
#[doc = "INT_EVENT2_MIS (r) register accessor: Masked interrupt status extension\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_mis`]
module"]
#[doc(alias = "INT_EVENT2_MIS")]
pub type IntEvent2Mis = crate::Reg<int_event2_mis::IntEvent2MisSpec>;
#[doc = "Masked interrupt status extension"]
pub mod int_event2_mis;
#[doc = "INT_EVENT2_ISET (w) register accessor: Interrupt set extension\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iset`]
module"]
#[doc(alias = "INT_EVENT2_ISET")]
pub type IntEvent2Iset = crate::Reg<int_event2_iset::IntEvent2IsetSpec>;
#[doc = "Interrupt set extension"]
pub mod int_event2_iset;
#[doc = "INT_EVENT2_ICLR (w) register accessor: Interrupt clear extension\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iclr`]
module"]
#[doc(alias = "INT_EVENT2_ICLR")]
pub type IntEvent2Iclr = crate::Reg<int_event2_iclr::IntEvent2IclrSpec>;
#[doc = "Interrupt clear extension"]
pub mod int_event2_iclr;
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
#[doc = "CTL0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control Register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control Register 1"]
pub mod ctl1;
#[doc = "CTL2 (rw) register accessor: Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`]
module"]
#[doc(alias = "CTL2")]
pub type Ctl2 = crate::Reg<ctl2::Ctl2Spec>;
#[doc = "Control Register 2"]
pub mod ctl2;
#[doc = "CLKFREQ (rw) register accessor: Sample Clock Frequency Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkfreq`]
module"]
#[doc(alias = "CLKFREQ")]
pub type Clkfreq = crate::Reg<clkfreq::ClkfreqSpec>;
#[doc = "Sample Clock Frequency Range Register"]
pub mod clkfreq;
#[doc = "SCOMP0 (rw) register accessor: Sample Time Compare 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scomp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scomp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scomp0`]
module"]
#[doc(alias = "SCOMP0")]
pub type Scomp0 = crate::Reg<scomp0::Scomp0Spec>;
#[doc = "Sample Time Compare 0 Register"]
pub mod scomp0;
#[doc = "SCOMP1 (rw) register accessor: Sample Time Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scomp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scomp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scomp1`]
module"]
#[doc(alias = "SCOMP1")]
pub type Scomp1 = crate::Reg<scomp1::Scomp1Spec>;
#[doc = "Sample Time Compare 1 Register"]
pub mod scomp1;
#[doc = "WCLOW (rw) register accessor: Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wclow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wclow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wclow`]
module"]
#[doc(alias = "WCLOW")]
pub type Wclow = crate::Reg<wclow::WclowSpec>;
#[doc = "Window Comparator Low Threshold Register"]
pub mod wclow;
#[doc = "WCHIGH (rw) register accessor: Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wchigh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wchigh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wchigh`]
module"]
#[doc(alias = "WCHIGH")]
pub type Wchigh = crate::Reg<wchigh::WchighSpec>;
#[doc = "Window Comparator High Threshold Register"]
pub mod wchigh;
#[doc = "MEMCTL (rw) register accessor: Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctl`]
module"]
#[doc(alias = "MEMCTL")]
pub type Memctl = crate::Reg<memctl::MemctlSpec>;
#[doc = "Conversion Memory Control Register"]
pub mod memctl;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
