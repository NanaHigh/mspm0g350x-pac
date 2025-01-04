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
    iidx: Iidx,
    _reserved5: [u8; 0x04],
    imask: Imask,
    _reserved6: [u8; 0x04],
    ris: Ris,
    _reserved7: [u8; 0x04],
    mis: Mis,
    _reserved8: [u8; 0x04],
    iset: Iset,
    _reserved9: [u8; 0x04],
    iclr: Iclr,
    _reserved10: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved11: [u8; 0x18],
    desc: Desc,
    wwdtctl0: Wwdtctl0,
    wwdtctl1: Wwdtctl1,
    wwdtcntrst: Wwdtcntrst,
    wwdtstat: Wwdtstat,
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
    #[doc = "0x1100 - Window Watchdog Timer Control Register 0"]
    #[inline(always)]
    pub const fn wwdtctl0(&self) -> &Wwdtctl0 {
        &self.wwdtctl0
    }
    #[doc = "0x1104 - Window Watchdog Timer Control Register 0"]
    #[inline(always)]
    pub const fn wwdtctl1(&self) -> &Wwdtctl1 {
        &self.wwdtctl1
    }
    #[doc = "0x1108 - Window Watchdog Timer Counter Reset Register"]
    #[inline(always)]
    pub const fn wwdtcntrst(&self) -> &Wwdtcntrst {
        &self.wwdtcntrst
    }
    #[doc = "0x110c - Window Watchdog Timer Status Register"]
    #[inline(always)]
    pub const fn wwdtstat(&self) -> &Wwdtstat {
        &self.wwdtstat
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
#[doc = "WWDTCTL0 (rw) register accessor: Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtctl0`]
module"]
#[doc(alias = "WWDTCTL0")]
pub type Wwdtctl0 = crate::Reg<wwdtctl0::Wwdtctl0Spec>;
#[doc = "Window Watchdog Timer Control Register 0"]
pub mod wwdtctl0;
#[doc = "WWDTCTL1 (rw) register accessor: Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtctl1`]
module"]
#[doc(alias = "WWDTCTL1")]
pub type Wwdtctl1 = crate::Reg<wwdtctl1::Wwdtctl1Spec>;
#[doc = "Window Watchdog Timer Control Register 0"]
pub mod wwdtctl1;
#[doc = "WWDTCNTRST (rw) register accessor: Window Watchdog Timer Counter Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtcntrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtcntrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtcntrst`]
module"]
#[doc(alias = "WWDTCNTRST")]
pub type Wwdtcntrst = crate::Reg<wwdtcntrst::WwdtcntrstSpec>;
#[doc = "Window Watchdog Timer Counter Reset Register"]
pub mod wwdtcntrst;
#[doc = "WWDTSTAT (r) register accessor: Window Watchdog Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdtstat`]
module"]
#[doc(alias = "WWDTSTAT")]
pub type Wwdtstat = crate::Reg<wwdtstat::WwdtstatSpec>;
#[doc = "Window Watchdog Timer Status Register"]
pub mod wwdtstat;
