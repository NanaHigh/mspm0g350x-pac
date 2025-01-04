#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0444],
    fpub_0: Fpub0,
    _reserved1: [u8; 0x03b8],
    pwren: Pwren,
    rstctl: Rstctl,
    clkcfg: Clkcfg,
    _reserved4: [u8; 0x08],
    stat: Stat,
    _reserved5: [u8; 0x07ec],
    clksel: Clksel,
    _reserved6: [u8; 0x18],
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
    _reserved18: [u8; 0x64],
    evt_mode: EvtMode,
    _reserved19: [u8; 0x18],
    desc: Desc,
    clkctl: Clkctl,
    dbgctl: Dbgctl,
    ctl: Ctl,
    sta: Sta,
    cal: Cal,
    tcmp: Tcmp,
    sec: Sec,
    min: Min,
    hour: Hour,
    day: Day,
    mon: Mon,
    year: Year,
    a1min: A1min,
    a1hour: A1hour,
    a1day: A1day,
    a2min: A2min,
    a2hour: A2hour,
    a2day: A2day,
    psctl: Psctl,
}
impl RegisterBlock {
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn fpub_0(&self) -> &Fpub0 {
        &self.fpub_0
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
    #[doc = "0x808 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn clkcfg(&self) -> &Clkcfg {
        &self.clkcfg
    }
    #[doc = "0x814 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x1004 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn clksel(&self) -> &Clksel {
        &self.clksel
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
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x10fc - RTC Descriptor Register"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1100 - RTC Clock Control Register"]
    #[inline(always)]
    pub const fn clkctl(&self) -> &Clkctl {
        &self.clkctl
    }
    #[doc = "0x1104 - RTC Module Debug Control Register"]
    #[inline(always)]
    pub const fn dbgctl(&self) -> &Dbgctl {
        &self.dbgctl
    }
    #[doc = "0x1108 - RTC Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x110c - RTC Status Register"]
    #[inline(always)]
    pub const fn sta(&self) -> &Sta {
        &self.sta
    }
    #[doc = "0x1110 - RTC Clock Offset Calibration Register"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
    #[doc = "0x1114 - RTC Temperature Compensation Register"]
    #[inline(always)]
    pub const fn tcmp(&self) -> &Tcmp {
        &self.tcmp
    }
    #[doc = "0x1118 - RTC Seconds Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn sec(&self) -> &Sec {
        &self.sec
    }
    #[doc = "0x111c - RTC Minutes Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn min(&self) -> &Min {
        &self.min
    }
    #[doc = "0x1120 - RTC Hours Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn hour(&self) -> &Hour {
        &self.hour
    }
    #[doc = "0x1124 - RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn day(&self) -> &Day {
        &self.day
    }
    #[doc = "0x1128 - RTC Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn mon(&self) -> &Mon {
        &self.mon
    }
    #[doc = "0x112c - RTC Year Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn year(&self) -> &Year {
        &self.year
    }
    #[doc = "0x1130 - RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn a1min(&self) -> &A1min {
        &self.a1min
    }
    #[doc = "0x1134 - RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn a1hour(&self) -> &A1hour {
        &self.a1hour
    }
    #[doc = "0x1138 - RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn a1day(&self) -> &A1day {
        &self.a1day
    }
    #[doc = "0x113c - RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn a2min(&self) -> &A2min {
        &self.a2min
    }
    #[doc = "0x1140 - RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn a2hour(&self) -> &A2hour {
        &self.a2hour
    }
    #[doc = "0x1144 - RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
    #[inline(always)]
    pub const fn a2day(&self) -> &A2day {
        &self.a2day
    }
    #[doc = "0x1148 - RTC Prescale Timer 0/1 Control Register"]
    #[inline(always)]
    pub const fn psctl(&self) -> &Psctl {
        &self.psctl
    }
}
#[doc = "FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpub_0`]
module"]
#[doc(alias = "FPUB_0")]
pub type Fpub0 = crate::Reg<fpub_0::Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod fpub_0;
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
#[doc = "CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg`]
module"]
#[doc(alias = "CLKCFG")]
pub type Clkcfg = crate::Reg<clkcfg::ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod clkcfg;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "CLKSEL (r) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"]
#[doc(alias = "CLKSEL")]
pub type Clksel = crate::Reg<clksel::ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod clksel;
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
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "DESC (r) register accessor: RTC Descriptor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "RTC Descriptor Register"]
pub mod desc;
#[doc = "CLKCTL (rw) register accessor: RTC Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctl`]
module"]
#[doc(alias = "CLKCTL")]
pub type Clkctl = crate::Reg<clkctl::ClkctlSpec>;
#[doc = "RTC Clock Control Register"]
pub mod clkctl;
#[doc = "DBGCTL (rw) register accessor: RTC Module Debug Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctl`]
module"]
#[doc(alias = "DBGCTL")]
pub type Dbgctl = crate::Reg<dbgctl::DbgctlSpec>;
#[doc = "RTC Module Debug Control Register"]
pub mod dbgctl;
#[doc = "CTL (rw) register accessor: RTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "RTC Control Register"]
pub mod ctl;
#[doc = "STA (r) register accessor: RTC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
#[doc(alias = "STA")]
pub type Sta = crate::Reg<sta::StaSpec>;
#[doc = "RTC Status Register"]
pub mod sta;
#[doc = "CAL (rw) register accessor: RTC Clock Offset Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`]
module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "RTC Clock Offset Calibration Register"]
pub mod cal;
#[doc = "TCMP (rw) register accessor: RTC Temperature Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmp`]
module"]
#[doc(alias = "TCMP")]
pub type Tcmp = crate::Reg<tcmp::TcmpSpec>;
#[doc = "RTC Temperature Compensation Register"]
pub mod tcmp;
#[doc = "SEC (rw) register accessor: RTC Seconds Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
#[doc(alias = "SEC")]
pub type Sec = crate::Reg<sec::SecSpec>;
#[doc = "RTC Seconds Register - Calendar Mode With Binary / BCD Format"]
pub mod sec;
#[doc = "MIN (rw) register accessor: RTC Minutes Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@min`]
module"]
#[doc(alias = "MIN")]
pub type Min = crate::Reg<min::MinSpec>;
#[doc = "RTC Minutes Register - Calendar Mode With Binary / BCD Format"]
pub mod min;
#[doc = "HOUR (rw) register accessor: RTC Hours Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hour`]
module"]
#[doc(alias = "HOUR")]
pub type Hour = crate::Reg<hour::HourSpec>;
#[doc = "RTC Hours Register - Calendar Mode With Binary / BCD Format"]
pub mod hour;
#[doc = "DAY (rw) register accessor: RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@day`]
module"]
#[doc(alias = "DAY")]
pub type Day = crate::Reg<day::DaySpec>;
#[doc = "RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
pub mod day;
#[doc = "MON (rw) register accessor: RTC Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`mon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mon`]
module"]
#[doc(alias = "MON")]
pub type Mon = crate::Reg<mon::MonSpec>;
#[doc = "RTC Month Register - Calendar Mode With Binary / BCD Format"]
pub mod mon;
#[doc = "YEAR (rw) register accessor: RTC Year Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@year`]
module"]
#[doc(alias = "YEAR")]
pub type Year = crate::Reg<year::YearSpec>;
#[doc = "RTC Year Register - Calendar Mode With Binary / BCD Format"]
pub mod year;
#[doc = "A1MIN (rw) register accessor: RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a1min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1min`]
module"]
#[doc(alias = "A1MIN")]
pub type A1min = crate::Reg<a1min::A1minSpec>;
#[doc = "RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod a1min;
#[doc = "A1HOUR (rw) register accessor: RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a1hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1hour`]
module"]
#[doc(alias = "A1HOUR")]
pub type A1hour = crate::Reg<a1hour::A1hourSpec>;
#[doc = "RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod a1hour;
#[doc = "A1DAY (rw) register accessor: RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a1day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a1day`]
module"]
#[doc(alias = "A1DAY")]
pub type A1day = crate::Reg<a1day::A1daySpec>;
#[doc = "RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
pub mod a1day;
#[doc = "A2MIN (rw) register accessor: RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a2min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2min`]
module"]
#[doc(alias = "A2MIN")]
pub type A2min = crate::Reg<a2min::A2minSpec>;
#[doc = "RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod a2min;
#[doc = "A2HOUR (rw) register accessor: RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a2hour::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2hour::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2hour`]
module"]
#[doc(alias = "A2HOUR")]
pub type A2hour = crate::Reg<a2hour::A2hourSpec>;
#[doc = "RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format"]
pub mod a2hour;
#[doc = "A2DAY (rw) register accessor: RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a2day::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2day::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a2day`]
module"]
#[doc(alias = "A2DAY")]
pub type A2day = crate::Reg<a2day::A2daySpec>;
#[doc = "RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format"]
pub mod a2day;
#[doc = "PSCTL (rw) register accessor: RTC Prescale Timer 0/1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psctl`]
module"]
#[doc(alias = "PSCTL")]
pub type Psctl = crate::Reg<psctl::PsctlSpec>;
#[doc = "RTC Prescale Timer 0/1 Control Register"]
pub mod psctl;
