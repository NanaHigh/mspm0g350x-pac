#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1020],
    iidx: Iidx,
    _reserved1: [u8; 0x04],
    imask: Imask,
    _reserved2: [u8; 0x04],
    ris: Ris,
    _reserved3: [u8; 0x04],
    mis: Mis,
    _reserved4: [u8; 0x04],
    iset: Iset,
    _reserved5: [u8; 0x04],
    iclr: Iclr,
    _reserved6: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved7: [u8; 0x18],
    desc: Desc,
    txd: Txd,
    txctl: Txctl,
    rxd: Rxd,
    rxctl: Rxctl,
    _reserved12: [u8; 0xf0],
    special_auth: SpecialAuth,
    _reserved13: [u8; 0x0c],
    app_auth: AppAuth,
}
impl RegisterBlock {
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
    #[doc = "0x1100 - Transmit data register"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x1104 - Transmit control register"]
    #[inline(always)]
    pub const fn txctl(&self) -> &Txctl {
        &self.txctl
    }
    #[doc = "0x1108 - Receive data register"]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x110c - Receive control register"]
    #[inline(always)]
    pub const fn rxctl(&self) -> &Rxctl {
        &self.rxctl
    }
    #[doc = "0x1200 - Special enable authorization register"]
    #[inline(always)]
    pub const fn special_auth(&self) -> &SpecialAuth {
        &self.special_auth
    }
    #[doc = "0x1210 - Application CPU0 authorization register"]
    #[inline(always)]
    pub const fn app_auth(&self) -> &AppAuth {
        &self.app_auth
    }
}
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
#[doc = "TXD (r) register accessor: Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`txd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`]
module"]
#[doc(alias = "TXD")]
pub type Txd = crate::Reg<txd::TxdSpec>;
#[doc = "Transmit data register"]
pub mod txd;
#[doc = "TXCTL (r) register accessor: Transmit control register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctl`]
module"]
#[doc(alias = "TXCTL")]
pub type Txctl = crate::Reg<txctl::TxctlSpec>;
#[doc = "Transmit control register"]
pub mod txctl;
#[doc = "RXD (rw) register accessor: Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd`]
module"]
#[doc(alias = "RXD")]
pub type Rxd = crate::Reg<rxd::RxdSpec>;
#[doc = "Receive data register"]
pub mod rxd;
#[doc = "RXCTL (rw) register accessor: Receive control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctl`]
module"]
#[doc(alias = "RXCTL")]
pub type Rxctl = crate::Reg<rxctl::RxctlSpec>;
#[doc = "Receive control register"]
pub mod rxctl;
#[doc = "SPECIAL_AUTH (r) register accessor: Special enable authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`special_auth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@special_auth`]
module"]
#[doc(alias = "SPECIAL_AUTH")]
pub type SpecialAuth = crate::Reg<special_auth::SpecialAuthSpec>;
#[doc = "Special enable authorization register"]
pub mod special_auth;
#[doc = "APP_AUTH (r) register accessor: Application CPU0 authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`app_auth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_auth`]
module"]
#[doc(alias = "APP_AUTH")]
pub type AppAuth = crate::Reg<app_auth::AppAuthSpec>;
#[doc = "Application CPU0 authorization register"]
pub mod app_auth;
