#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    fsub_0: Fsub0,
    fsub_1: Fsub1,
    _reserved2: [u8; 0x3c],
    fpub_0: Fpub0,
    fpub_1: Fpub1,
    _reserved4: [u8; 0x03b4],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved6: [u8; 0x0c],
    stat: Stat,
    _reserved7: [u8; 0x07e8],
    clkdiv: Clkdiv,
    _reserved8: [u8; 0x04],
    clksel: Clksel,
    _reserved9: [u8; 0x0c],
    pdbgctl: Pdbgctl,
    _reserved10: [u8; 0x04],
    iidx: Iidx,
    _reserved11: [u8; 0x04],
    imask: Imask,
    _reserved12: [u8; 0x04],
    ris: Ris,
    _reserved13: [u8; 0x04],
    mis: Mis,
    _reserved14: [u8; 0x04],
    iset: Iset,
    _reserved15: [u8; 0x04],
    iclr: Iclr,
    _reserved16: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved17: [u8; 0x18],
    desc: Desc,
    ccpd: Ccpd,
    odis: Odis,
    cclkctl: Cclkctl,
    _reserved21: [u8; 0x08],
    cttrigctl: Cttrigctl,
    _reserved22: [u8; 0x04],
    cttrig: Cttrig,
    _reserved23: [u8; 0x06e0],
    ctr: Ctr,
    ctrctl: Ctrctl,
    load: Load,
    _reserved26: [u8; 0x04],
    cc_01: [Cc01; 2],
    _reserved27: [u8; 0x18],
    ccctl_01: [Ccctl01; 2],
    _reserved28: [u8; 0x18],
    octl_01: [Octl01; 2],
    _reserved29: [u8; 0x18],
    ccact_01: [Ccact01; 2],
    _reserved30: [u8; 0x08],
    ifctl_01: [Ifctl01; 2],
    _reserved31: [u8; 0x28],
    tsel: Tsel,
}
impl RegisterBlock {
    #[doc = "0x400 - Subsciber Port 0"]
    #[inline(always)]
    pub const fn fsub_0(&self) -> &Fsub0 {
        &self.fsub_0
    }
    #[doc = "0x404 - Subscriber Port 1"]
    #[inline(always)]
    pub const fn fsub_1(&self) -> &Fsub1 {
        &self.fsub_1
    }
    #[doc = "0x444 - Publisher Port 0"]
    #[inline(always)]
    pub const fn fpub_0(&self) -> &Fpub0 {
        &self.fpub_0
    }
    #[doc = "0x448 - Publisher Port 1"]
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
    #[doc = "0x814 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
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
    pub const fn iidx(&self) -> &Iidx {
        &self.iidx
    }
    #[doc = "0x1028 - Interrupt mask"]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x1030 - Raw interrupt status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x1038 - Masked interrupt status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x1040 - Interrupt set"]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x1048 - Interrupt clear"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
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
    #[doc = "0x1100 - CCP Direction"]
    #[inline(always)]
    pub const fn ccpd(&self) -> &Ccpd {
        &self.ccpd
    }
    #[doc = "0x1104 - Output Disable"]
    #[inline(always)]
    pub const fn odis(&self) -> &Odis {
        &self.odis
    }
    #[doc = "0x1108 - Counter Clock Control Register"]
    #[inline(always)]
    pub const fn cclkctl(&self) -> &Cclkctl {
        &self.cclkctl
    }
    #[doc = "0x1114 - Timer Cross Trigger Control Register"]
    #[inline(always)]
    pub const fn cttrigctl(&self) -> &Cttrigctl {
        &self.cttrigctl
    }
    #[doc = "0x111c - Timer Cross Trigger Register"]
    #[inline(always)]
    pub const fn cttrig(&self) -> &Cttrig {
        &self.cttrig
    }
    #[doc = "0x1800 - Counter Register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x1804 - Counter Control Register"]
    #[inline(always)]
    pub const fn ctrctl(&self) -> &Ctrctl {
        &self.ctrctl
    }
    #[doc = "0x1808 - Load Register"]
    #[inline(always)]
    pub const fn load(&self) -> &Load {
        &self.load
    }
    #[doc = "0x1810..0x1818 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub const fn cc_01(&self, n: usize) -> &Cc01 {
        &self.cc_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1810..0x1818 - Capture or Compare Register 0 to Capture or Compare Register 1"]
    #[inline(always)]
    pub fn cc_01_iter(&self) -> impl Iterator<Item = &Cc01> {
        self.cc_01.iter()
    }
    #[doc = "0x1830..0x1838 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub const fn ccctl_01(&self, n: usize) -> &Ccctl01 {
        &self.ccctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1830..0x1838 - Capture or Compare Control Registers"]
    #[inline(always)]
    pub fn ccctl_01_iter(&self) -> impl Iterator<Item = &Ccctl01> {
        self.ccctl_01.iter()
    }
    #[doc = "0x1850..0x1858 - CCP Output Control Registers"]
    #[inline(always)]
    pub const fn octl_01(&self, n: usize) -> &Octl01 {
        &self.octl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1850..0x1858 - CCP Output Control Registers"]
    #[inline(always)]
    pub fn octl_01_iter(&self) -> impl Iterator<Item = &Octl01> {
        self.octl_01.iter()
    }
    #[doc = "0x1870..0x1878 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub const fn ccact_01(&self, n: usize) -> &Ccact01 {
        &self.ccact_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1870..0x1878 - Capture or Compare Action Registers"]
    #[inline(always)]
    pub fn ccact_01_iter(&self) -> impl Iterator<Item = &Ccact01> {
        self.ccact_01.iter()
    }
    #[doc = "0x1880..0x1888 - Input Filter Control Register"]
    #[inline(always)]
    pub const fn ifctl_01(&self, n: usize) -> &Ifctl01 {
        &self.ifctl_01[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1880..0x1888 - Input Filter Control Register"]
    #[inline(always)]
    pub fn ifctl_01_iter(&self) -> impl Iterator<Item = &Ifctl01> {
        self.ifctl_01.iter()
    }
    #[doc = "0x18b0 - Trigger Select"]
    #[inline(always)]
    pub const fn tsel(&self) -> &Tsel {
        &self.tsel
    }
}
#[doc = "FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_0`]
module"]
#[doc(alias = "FSUB_0")]
pub type Fsub0 = crate::Reg<fsub_0::Fsub0Spec>;
#[doc = "Subsciber Port 0"]
pub mod fsub_0;
#[doc = "FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_1`]
module"]
#[doc(alias = "FSUB_1")]
pub type Fsub1 = crate::Reg<fsub_1::Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod fsub_1;
#[doc = "FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpub_0`]
module"]
#[doc(alias = "FPUB_0")]
pub type Fpub0 = crate::Reg<fpub_0::Fpub0Spec>;
#[doc = "Publisher Port 0"]
pub mod fpub_0;
#[doc = "FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpub_1`]
module"]
#[doc(alias = "FPUB_1")]
pub type Fpub1 = crate::Reg<fpub_1::Fpub1Spec>;
#[doc = "Publisher Port 1"]
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
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
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
#[doc = "IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`]
module"]
#[doc(alias = "IIDX")]
pub type Iidx = crate::Reg<iidx::IidxSpec>;
#[doc = "Interrupt index"]
pub mod iidx;
#[doc = "IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod imask;
#[doc = "RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status"]
pub mod mis;
#[doc = "ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt set"]
pub mod iset;
#[doc = "ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear"]
pub mod iclr;
#[doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
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
#[doc = "CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`ccpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccpd`]
module"]
#[doc(alias = "CCPD")]
pub type Ccpd = crate::Reg<ccpd::CcpdSpec>;
#[doc = "CCP Direction"]
pub mod ccpd;
#[doc = "ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`odis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odis`]
module"]
#[doc(alias = "ODIS")]
pub type Odis = crate::Reg<odis::OdisSpec>;
#[doc = "Output Disable"]
pub mod odis;
#[doc = "CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclkctl`]
module"]
#[doc(alias = "CCLKCTL")]
pub type Cclkctl = crate::Reg<cclkctl::CclkctlSpec>;
#[doc = "Counter Clock Control Register"]
pub mod cclkctl;
#[doc = "CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cttrigctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrigctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cttrigctl`]
module"]
#[doc(alias = "CTTRIGCTL")]
pub type Cttrigctl = crate::Reg<cttrigctl::CttrigctlSpec>;
#[doc = "Timer Cross Trigger Control Register"]
pub mod cttrigctl;
#[doc = "CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cttrig`]
module"]
#[doc(alias = "CTTRIG")]
pub type Cttrig = crate::Reg<cttrig::CttrigSpec>;
#[doc = "Timer Cross Trigger Register"]
pub mod cttrig;
#[doc = "CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Counter Register"]
pub mod ctr;
#[doc = "CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrctl`]
module"]
#[doc(alias = "CTRCTL")]
pub type Ctrctl = crate::Reg<ctrctl::CtrctlSpec>;
#[doc = "Counter Control Register"]
pub mod ctrctl;
#[doc = "LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`]
module"]
#[doc(alias = "LOAD")]
pub type Load = crate::Reg<load::LoadSpec>;
#[doc = "Load Register"]
pub mod load;
#[doc = "CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_01`]
module"]
#[doc(alias = "CC_01")]
pub type Cc01 = crate::Reg<cc_01::Cc01Spec>;
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1"]
pub mod cc_01;
#[doc = "CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ccctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccctl_01`]
module"]
#[doc(alias = "CCCTL_01")]
pub type Ccctl01 = crate::Reg<ccctl_01::Ccctl01Spec>;
#[doc = "Capture or Compare Control Registers"]
pub mod ccctl_01;
#[doc = "OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`octl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octl_01`]
module"]
#[doc(alias = "OCTL_01")]
pub type Octl01 = crate::Reg<octl_01::Octl01Spec>;
#[doc = "CCP Output Control Registers"]
pub mod octl_01;
#[doc = "CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ccact_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccact_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccact_01`]
module"]
#[doc(alias = "CCACT_01")]
pub type Ccact01 = crate::Reg<ccact_01::Ccact01Spec>;
#[doc = "Capture or Compare Action Registers"]
pub mod ccact_01;
#[doc = "IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifctl_01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifctl_01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifctl_01`]
module"]
#[doc(alias = "IFCTL_01")]
pub type Ifctl01 = crate::Reg<ifctl_01::Ifctl01Spec>;
#[doc = "Input Filter Control Register"]
pub mod ifctl_01;
#[doc = "TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`tsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsel`]
module"]
#[doc(alias = "TSEL")]
pub type Tsel = crate::Reg<tsel::TselSpec>;
#[doc = "Trigger Select"]
pub mod tsel;
