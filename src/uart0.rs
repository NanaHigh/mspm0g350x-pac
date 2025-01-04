#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    pwren: Pwren,
    rstctl: Rstctl,
    clkcfg: Clkcfg,
    _reserved3: [u8; 0x08],
    gprcm_stat: GprcmStat,
    _reserved4: [u8; 0x07e8],
    clkdiv: Clkdiv,
    _reserved5: [u8; 0x04],
    clksel: Clksel,
    _reserved6: [u8; 0x0c],
    pdbgctl: Pdbgctl,
    _reserved7: [u8; 0x04],
    int_event0_iidx: IntEvent0Iidx,
    _reserved8: [u8; 0x04],
    int_event0_imask: IntEvent0Imask,
    _reserved9: [u8; 0x04],
    int_event0_ris: IntEvent0Ris,
    _reserved10: [u8; 0x04],
    int_event0_mis: IntEvent0Mis,
    _reserved11: [u8; 0x04],
    int_event0_iset: IntEvent0Iset,
    _reserved12: [u8; 0x04],
    int_event0_iclr: IntEvent0Iclr,
    _reserved13: [u8; 0x04],
    int_event1_iidx: IntEvent1Iidx,
    _reserved14: [u8; 0x04],
    int_event1_imask: IntEvent1Imask,
    _reserved15: [u8; 0x04],
    int_event1_ris: IntEvent1Ris,
    _reserved16: [u8; 0x04],
    int_event1_mis: IntEvent1Mis,
    _reserved17: [u8; 0x04],
    int_event1_iset: IntEvent1Iset,
    _reserved18: [u8; 0x04],
    int_event1_iclr: IntEvent1Iclr,
    _reserved19: [u8; 0x04],
    int_event2_iidx: IntEvent2Iidx,
    _reserved20: [u8; 0x04],
    int_event2_imask: IntEvent2Imask,
    _reserved21: [u8; 0x04],
    int_event2_ris: IntEvent2Ris,
    _reserved22: [u8; 0x04],
    int_event2_mis: IntEvent2Mis,
    _reserved23: [u8; 0x04],
    int_event2_iset: IntEvent2Iset,
    _reserved24: [u8; 0x04],
    int_event2_iclr: IntEvent2Iclr,
    _reserved25: [u8; 0x34],
    evt_mode: EvtMode,
    intctl: Intctl,
    _reserved27: [u8; 0x18],
    ctl0: Ctl0,
    lcrh: Lcrh,
    stat: Stat,
    ifls: Ifls,
    ibrd: Ibrd,
    fbrd: Fbrd,
    gfctl: Gfctl,
    _reserved34: [u8; 0x04],
    txdata: Txdata,
    rxdata: Rxdata,
    _reserved36: [u8; 0x08],
    lincnt: Lincnt,
    linctl: Linctl,
    linc0: Linc0,
    linc1: Linc1,
    irctl: Irctl,
    _reserved41: [u8; 0x04],
    amask: Amask,
    addr: Addr,
    _reserved43: [u8; 0x10],
    clkdiv2: Clkdiv2,
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
    #[doc = "0x808 - Peripheral Clock Configuration Register"]
    #[inline(always)]
    pub const fn clkcfg(&self) -> &Clkcfg {
        &self.clkcfg
    }
    #[doc = "0x814 - Status Register"]
    #[inline(always)]
    pub const fn gprcm_stat(&self) -> &GprcmStat {
        &self.gprcm_stat
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x1008 - Clock Select for Ultra Low Power peripherals"]
    #[inline(always)]
    pub const fn clksel(&self) -> &Clksel {
        &self.clksel
    }
    #[doc = "0x1018 - Peripheral Debug Control"]
    #[inline(always)]
    pub const fn pdbgctl(&self) -> &Pdbgctl {
        &self.pdbgctl
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
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0x10e4 - Interrupt control register"]
    #[inline(always)]
    pub const fn intctl(&self) -> &Intctl {
        &self.intctl
    }
    #[doc = "0x1100 - UART Control Register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x1104 - UART Line Control Register"]
    #[inline(always)]
    pub const fn lcrh(&self) -> &Lcrh {
        &self.lcrh
    }
    #[doc = "0x1108 - UART Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x110c - UART Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x1110 - UART Integer Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn ibrd(&self) -> &Ibrd {
        &self.ibrd
    }
    #[doc = "0x1114 - UART Fractional Baud-Rate Divisor Register"]
    #[inline(always)]
    pub const fn fbrd(&self) -> &Fbrd {
        &self.fbrd
    }
    #[doc = "0x1118 - Glitch Filter Control"]
    #[inline(always)]
    pub const fn gfctl(&self) -> &Gfctl {
        &self.gfctl
    }
    #[doc = "0x1120 - UART Transmit Data Register"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x1124 - UART Receive Data Register"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x1130 - UART LIN Mode Counter Register"]
    #[inline(always)]
    pub const fn lincnt(&self) -> &Lincnt {
        &self.lincnt
    }
    #[doc = "0x1134 - UART LIN Mode Control Register"]
    #[inline(always)]
    pub const fn linctl(&self) -> &Linctl {
        &self.linctl
    }
    #[doc = "0x1138 - UART LIN Mode Capture 0 Register"]
    #[inline(always)]
    pub const fn linc0(&self) -> &Linc0 {
        &self.linc0
    }
    #[doc = "0x113c - UART LIN Mode Capture 1 Register"]
    #[inline(always)]
    pub const fn linc1(&self) -> &Linc1 {
        &self.linc1
    }
    #[doc = "0x1140 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn irctl(&self) -> &Irctl {
        &self.irctl
    }
    #[doc = "0x1148 - Self Address Mask Register"]
    #[inline(always)]
    pub const fn amask(&self) -> &Amask {
        &self.amask
    }
    #[doc = "0x114c - Self Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x1160 - Clock Divider"]
    #[inline(always)]
    pub const fn clkdiv2(&self) -> &Clkdiv2 {
        &self.clkdiv2
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
#[doc = "CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg`]
module"]
#[doc(alias = "CLKCFG")]
pub type Clkcfg = crate::Reg<clkcfg::ClkcfgSpec>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod clkcfg;
#[doc = "GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gprcm_stat`]
module"]
#[doc(alias = "GPRCM_STAT")]
pub type GprcmStat = crate::Reg<gprcm_stat::GprcmStatSpec>;
#[doc = "Status Register"]
pub mod gprcm_stat;
#[doc = "CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Divider"]
pub mod clkdiv;
#[doc = "CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"]
#[doc(alias = "CLKSEL")]
pub type Clksel = crate::Reg<clksel::ClkselSpec>;
#[doc = "Clock Select for Ultra Low Power peripherals"]
pub mod clksel;
#[doc = "PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbgctl`]
module"]
#[doc(alias = "PDBGCTL")]
pub type Pdbgctl = crate::Reg<pdbgctl::PdbgctlSpec>;
#[doc = "Peripheral Debug Control"]
pub mod pdbgctl;
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
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event Mode"]
pub mod evt_mode;
#[doc = "INTCTL (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intctl`]
module"]
#[doc(alias = "INTCTL")]
pub type Intctl = crate::Reg<intctl::IntctlSpec>;
#[doc = "Interrupt control register"]
pub mod intctl;
#[doc = "CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "UART Control Register 0"]
pub mod ctl0;
#[doc = "LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcrh`]
module"]
#[doc(alias = "LCRH")]
pub type Lcrh = crate::Reg<lcrh::LcrhSpec>;
#[doc = "UART Line Control Register"]
pub mod lcrh;
#[doc = "STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "UART Status Register"]
pub mod stat;
#[doc = "IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`]
module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "UART Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`]
module"]
#[doc(alias = "IBRD")]
pub type Ibrd = crate::Reg<ibrd::IbrdSpec>;
#[doc = "UART Integer Baud-Rate Divisor Register"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`]
module"]
#[doc(alias = "FBRD")]
pub type Fbrd = crate::Reg<fbrd::FbrdSpec>;
#[doc = "UART Fractional Baud-Rate Divisor Register"]
pub mod fbrd;
#[doc = "GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfctl`]
module"]
#[doc(alias = "GFCTL")]
pub type Gfctl = crate::Reg<gfctl::GfctlSpec>;
#[doc = "Glitch Filter Control"]
pub mod gfctl;
#[doc = "TXDATA (rw) register accessor: UART Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "UART Transmit Data Register"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "UART Receive Data Register"]
pub mod rxdata;
#[doc = "LINCNT (rw) register accessor: UART LIN Mode Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lincnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lincnt`]
module"]
#[doc(alias = "LINCNT")]
pub type Lincnt = crate::Reg<lincnt::LincntSpec>;
#[doc = "UART LIN Mode Counter Register"]
pub mod lincnt;
#[doc = "LINCTL (rw) register accessor: UART LIN Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linctl`]
module"]
#[doc(alias = "LINCTL")]
pub type Linctl = crate::Reg<linctl::LinctlSpec>;
#[doc = "UART LIN Mode Control Register"]
pub mod linctl;
#[doc = "LINC0 (rw) register accessor: UART LIN Mode Capture 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linc0`]
module"]
#[doc(alias = "LINC0")]
pub type Linc0 = crate::Reg<linc0::Linc0Spec>;
#[doc = "UART LIN Mode Capture 0 Register"]
pub mod linc0;
#[doc = "LINC1 (rw) register accessor: UART LIN Mode Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linc1`]
module"]
#[doc(alias = "LINC1")]
pub type Linc1 = crate::Reg<linc1::Linc1Spec>;
#[doc = "UART LIN Mode Capture 1 Register"]
pub mod linc1;
#[doc = "IRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irctl`]
module"]
#[doc(alias = "IRCTL")]
pub type Irctl = crate::Reg<irctl::IrctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod irctl;
#[doc = "AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`amask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amask`]
module"]
#[doc(alias = "AMASK")]
pub type Amask = crate::Reg<amask::AmaskSpec>;
#[doc = "Self Address Mask Register"]
pub mod amask;
#[doc = "ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Self Address Register"]
pub mod addr;
#[doc = "CLKDIV2 (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv2`]
module"]
#[doc(alias = "CLKDIV2")]
pub type Clkdiv2 = crate::Reg<clkdiv2::Clkdiv2Spec>;
#[doc = "Clock Divider"]
pub mod clkdiv2;
