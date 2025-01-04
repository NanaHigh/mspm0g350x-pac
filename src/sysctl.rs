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
    _reserved6: [u8; 0x04],
    nmiiidx: Nmiiidx,
    _reserved7: [u8; 0x0c],
    nmiris: Nmiris,
    _reserved8: [u8; 0x0c],
    nmiiset: Nmiiset,
    _reserved9: [u8; 0x04],
    nmiiclr: Nmiiclr,
    _reserved10: [u8; 0x84],
    sysosccfg: Sysosccfg,
    _reserved11: [u8; 0x04],
    hsclken: Hsclken,
    _reserved12: [u8; 0x14],
    syspllcfg0: Syspllcfg0,
    syspllcfg1: Syspllcfg1,
    syspllparam0: Syspllparam0,
    syspllparam1: Syspllparam1,
    _reserved16: [u8; 0x0c],
    genclken: Genclken,
    pmodecfg: Pmodecfg,
    _reserved18: [u8; 0x0c],
    fcc: Fcc,
    _reserved19: [u8; 0x1c],
    sysosctrimuser: Sysosctrimuser,
    _reserved20: [u8; 0x04],
    sramboundary: Sramboundary,
    _reserved21: [u8; 0x04],
    systemcfg: Systemcfg,
    _reserved22: [u8; 0x7c],
    writelock: Writelock,
    _reserved23: [u8; 0x08],
    dederraddr: Dederraddr,
    _reserved24: [u8; 0x10],
    rstcause: Rstcause,
    _reserved25: [u8; 0xdc],
    resetlevel: Resetlevel,
    resetcmd: Resetcmd,
    borthreshold: Borthreshold,
    borclrcmd: Borclrcmd,
    sysoscfclctl: Sysoscfclctl,
    lfxtctl: Lfxtctl,
    exlfctl: Exlfctl,
    shdniorel: Shdniorel,
    exrstpin: Exrstpin,
    _reserved34: [u8; 0x04],
    swdcfg: Swdcfg,
    fcccmd: Fcccmd,
    _reserved36: [u8; 0x50],
    pmuopamp: Pmuopamp,
    _reserved37: [u8; 0x7c],
    shutdnstore0: Shutdnstore0,
    shutdnstore1: Shutdnstore1,
    shutdnstore2: Shutdnstore2,
    shutdnstore3: Shutdnstore3,
}
impl RegisterBlock {
    #[doc = "0x1020 - SYSCTL interrupt index"]
    #[inline(always)]
    pub const fn iidx(&self) -> &Iidx {
        &self.iidx
    }
    #[doc = "0x1028 - SYSCTL interrupt mask"]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x1030 - SYSCTL raw interrupt status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x1038 - SYSCTL masked interrupt status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x1040 - SYSCTL interrupt set"]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x1048 - SYSCTL interrupt clear"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x1050 - NMI interrupt index"]
    #[inline(always)]
    pub const fn nmiiidx(&self) -> &Nmiiidx {
        &self.nmiiidx
    }
    #[doc = "0x1060 - NMI raw interrupt status"]
    #[inline(always)]
    pub const fn nmiris(&self) -> &Nmiris {
        &self.nmiris
    }
    #[doc = "0x1070 - NMI interrupt set"]
    #[inline(always)]
    pub const fn nmiiset(&self) -> &Nmiiset {
        &self.nmiiset
    }
    #[doc = "0x1078 - NMI interrupt clear"]
    #[inline(always)]
    pub const fn nmiiclr(&self) -> &Nmiiclr {
        &self.nmiiclr
    }
    #[doc = "0x1100 - SYSOSC configuration"]
    #[inline(always)]
    pub const fn sysosccfg(&self) -> &Sysosccfg {
        &self.sysosccfg
    }
    #[doc = "0x1108 - High-speed clock (HSCLK) source enable/disable"]
    #[inline(always)]
    pub const fn hsclken(&self) -> &Hsclken {
        &self.hsclken
    }
    #[doc = "0x1120 - SYSPLL reference and output configuration"]
    #[inline(always)]
    pub const fn syspllcfg0(&self) -> &Syspllcfg0 {
        &self.syspllcfg0
    }
    #[doc = "0x1124 - SYSPLL reference and feedback divider"]
    #[inline(always)]
    pub const fn syspllcfg1(&self) -> &Syspllcfg1 {
        &self.syspllcfg1
    }
    #[doc = "0x1128 - SYSPLL PARAM0 (load from FACTORY region)"]
    #[inline(always)]
    pub const fn syspllparam0(&self) -> &Syspllparam0 {
        &self.syspllparam0
    }
    #[doc = "0x112c - SYSPLL PARAM1 (load from FACTORY region)"]
    #[inline(always)]
    pub const fn syspllparam1(&self) -> &Syspllparam1 {
        &self.syspllparam1
    }
    #[doc = "0x113c - General clock enable control"]
    #[inline(always)]
    pub const fn genclken(&self) -> &Genclken {
        &self.genclken
    }
    #[doc = "0x1140 - Power mode configuration"]
    #[inline(always)]
    pub const fn pmodecfg(&self) -> &Pmodecfg {
        &self.pmodecfg
    }
    #[doc = "0x1150 - Frequency clock counter (FCC) count"]
    #[inline(always)]
    pub const fn fcc(&self) -> &Fcc {
        &self.fcc
    }
    #[doc = "0x1170 - SYSOSC user-specified trim"]
    #[inline(always)]
    pub const fn sysosctrimuser(&self) -> &Sysosctrimuser {
        &self.sysosctrimuser
    }
    #[doc = "0x1178 - SRAM Write Boundary"]
    #[inline(always)]
    pub const fn sramboundary(&self) -> &Sramboundary {
        &self.sramboundary
    }
    #[doc = "0x1180 - System configuration"]
    #[inline(always)]
    pub const fn systemcfg(&self) -> &Systemcfg {
        &self.systemcfg
    }
    #[doc = "0x1200 - SYSCTL register write lockout"]
    #[inline(always)]
    pub const fn writelock(&self) -> &Writelock {
        &self.writelock
    }
    #[doc = "0x120c - Memory DED Address"]
    #[inline(always)]
    pub const fn dederraddr(&self) -> &Dederraddr {
        &self.dederraddr
    }
    #[doc = "0x1220 - Reset cause"]
    #[inline(always)]
    pub const fn rstcause(&self) -> &Rstcause {
        &self.rstcause
    }
    #[doc = "0x1300 - Reset level for application-triggered reset command"]
    #[inline(always)]
    pub const fn resetlevel(&self) -> &Resetlevel {
        &self.resetlevel
    }
    #[doc = "0x1304 - Execute an application-triggered reset command"]
    #[inline(always)]
    pub const fn resetcmd(&self) -> &Resetcmd {
        &self.resetcmd
    }
    #[doc = "0x1308 - BOR threshold selection"]
    #[inline(always)]
    pub const fn borthreshold(&self) -> &Borthreshold {
        &self.borthreshold
    }
    #[doc = "0x130c - Set the BOR threshold"]
    #[inline(always)]
    pub const fn borclrcmd(&self) -> &Borclrcmd {
        &self.borclrcmd
    }
    #[doc = "0x1310 - SYSOSC frequency correction loop (FCL) ROSC enable"]
    #[inline(always)]
    pub const fn sysoscfclctl(&self) -> &Sysoscfclctl {
        &self.sysoscfclctl
    }
    #[doc = "0x1314 - LFXT and LFCLK control"]
    #[inline(always)]
    pub const fn lfxtctl(&self) -> &Lfxtctl {
        &self.lfxtctl
    }
    #[doc = "0x1318 - LFCLK_IN and LFCLK control"]
    #[inline(always)]
    pub const fn exlfctl(&self) -> &Exlfctl {
        &self.exlfctl
    }
    #[doc = "0x131c - SHUTDOWN IO release control"]
    #[inline(always)]
    pub const fn shdniorel(&self) -> &Shdniorel {
        &self.shdniorel
    }
    #[doc = "0x1320 - Disable the reset function of the NRST pin"]
    #[inline(always)]
    pub const fn exrstpin(&self) -> &Exrstpin {
        &self.exrstpin
    }
    #[doc = "0x1328 - Disable the SWD function on the SWD pins"]
    #[inline(always)]
    pub const fn swdcfg(&self) -> &Swdcfg {
        &self.swdcfg
    }
    #[doc = "0x132c - Frequency clock counter start capture"]
    #[inline(always)]
    pub const fn fcccmd(&self) -> &Fcccmd {
        &self.fcccmd
    }
    #[doc = "0x1380 - GPAMP control"]
    #[inline(always)]
    pub const fn pmuopamp(&self) -> &Pmuopamp {
        &self.pmuopamp
    }
    #[doc = "0x1400 - Shutdown storage memory (byte 0)"]
    #[inline(always)]
    pub const fn shutdnstore0(&self) -> &Shutdnstore0 {
        &self.shutdnstore0
    }
    #[doc = "0x1404 - Shutdown storage memory (byte 1)"]
    #[inline(always)]
    pub const fn shutdnstore1(&self) -> &Shutdnstore1 {
        &self.shutdnstore1
    }
    #[doc = "0x1408 - Shutdown storage memory (byte 2)"]
    #[inline(always)]
    pub const fn shutdnstore2(&self) -> &Shutdnstore2 {
        &self.shutdnstore2
    }
    #[doc = "0x140c - Shutdown storage memory (byte 3)"]
    #[inline(always)]
    pub const fn shutdnstore3(&self) -> &Shutdnstore3 {
        &self.shutdnstore3
    }
}
#[doc = "IIDX (r) register accessor: SYSCTL interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`]
module"]
#[doc(alias = "IIDX")]
pub type Iidx = crate::Reg<iidx::IidxSpec>;
#[doc = "SYSCTL interrupt index"]
pub mod iidx;
#[doc = "IMASK (rw) register accessor: SYSCTL interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "SYSCTL interrupt mask"]
pub mod imask;
#[doc = "RIS (r) register accessor: SYSCTL raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "SYSCTL raw interrupt status"]
pub mod ris;
#[doc = "MIS (r) register accessor: SYSCTL masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "SYSCTL masked interrupt status"]
pub mod mis;
#[doc = "ISET (w) register accessor: SYSCTL interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "SYSCTL interrupt set"]
pub mod iset;
#[doc = "ICLR (w) register accessor: SYSCTL interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "SYSCTL interrupt clear"]
pub mod iclr;
#[doc = "NMIIIDX (r) register accessor: NMI interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiiidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiiidx`]
module"]
#[doc(alias = "NMIIIDX")]
pub type Nmiiidx = crate::Reg<nmiiidx::NmiiidxSpec>;
#[doc = "NMI interrupt index"]
pub mod nmiiidx;
#[doc = "NMIRIS (r) register accessor: NMI raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiris`]
module"]
#[doc(alias = "NMIRIS")]
pub type Nmiris = crate::Reg<nmiris::NmirisSpec>;
#[doc = "NMI raw interrupt status"]
pub mod nmiris;
#[doc = "NMIISET (w) register accessor: NMI interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiiset`]
module"]
#[doc(alias = "NMIISET")]
pub type Nmiiset = crate::Reg<nmiiset::NmiisetSpec>;
#[doc = "NMI interrupt set"]
pub mod nmiiset;
#[doc = "NMIICLR (w) register accessor: NMI interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiiclr`]
module"]
#[doc(alias = "NMIICLR")]
pub type Nmiiclr = crate::Reg<nmiiclr::NmiiclrSpec>;
#[doc = "NMI interrupt clear"]
pub mod nmiiclr;
#[doc = "SYSOSCCFG (rw) register accessor: SYSOSC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysosccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysosccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysosccfg`]
module"]
#[doc(alias = "SYSOSCCFG")]
pub type Sysosccfg = crate::Reg<sysosccfg::SysosccfgSpec>;
#[doc = "SYSOSC configuration"]
pub mod sysosccfg;
#[doc = "HSCLKEN (rw) register accessor: High-speed clock (HSCLK) source enable/disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsclken`]
module"]
#[doc(alias = "HSCLKEN")]
pub type Hsclken = crate::Reg<hsclken::HsclkenSpec>;
#[doc = "High-speed clock (HSCLK) source enable/disable"]
pub mod hsclken;
#[doc = "SYSPLLCFG0 (rw) register accessor: SYSPLL reference and output configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllcfg0`]
module"]
#[doc(alias = "SYSPLLCFG0")]
pub type Syspllcfg0 = crate::Reg<syspllcfg0::Syspllcfg0Spec>;
#[doc = "SYSPLL reference and output configuration"]
pub mod syspllcfg0;
#[doc = "SYSPLLCFG1 (rw) register accessor: SYSPLL reference and feedback divider\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllcfg1`]
module"]
#[doc(alias = "SYSPLLCFG1")]
pub type Syspllcfg1 = crate::Reg<syspllcfg1::Syspllcfg1Spec>;
#[doc = "SYSPLL reference and feedback divider"]
pub mod syspllcfg1;
#[doc = "SYSPLLPARAM0 (rw) register accessor: SYSPLL PARAM0 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllparam0`]
module"]
#[doc(alias = "SYSPLLPARAM0")]
pub type Syspllparam0 = crate::Reg<syspllparam0::Syspllparam0Spec>;
#[doc = "SYSPLL PARAM0 (load from FACTORY region)"]
pub mod syspllparam0;
#[doc = "SYSPLLPARAM1 (rw) register accessor: SYSPLL PARAM1 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syspllparam1`]
module"]
#[doc(alias = "SYSPLLPARAM1")]
pub type Syspllparam1 = crate::Reg<syspllparam1::Syspllparam1Spec>;
#[doc = "SYSPLL PARAM1 (load from FACTORY region)"]
pub mod syspllparam1;
#[doc = "GENCLKEN (rw) register accessor: General clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`genclken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genclken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genclken`]
module"]
#[doc(alias = "GENCLKEN")]
pub type Genclken = crate::Reg<genclken::GenclkenSpec>;
#[doc = "General clock enable control"]
pub mod genclken;
#[doc = "PMODECFG (rw) register accessor: Power mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pmodecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmodecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmodecfg`]
module"]
#[doc(alias = "PMODECFG")]
pub type Pmodecfg = crate::Reg<pmodecfg::PmodecfgSpec>;
#[doc = "Power mode configuration"]
pub mod pmodecfg;
#[doc = "FCC (r) register accessor: Frequency clock counter (FCC) count\n\nYou can [`read`](crate::Reg::read) this register and get [`fcc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcc`]
module"]
#[doc(alias = "FCC")]
pub type Fcc = crate::Reg<fcc::FccSpec>;
#[doc = "Frequency clock counter (FCC) count"]
pub mod fcc;
#[doc = "SYSOSCTRIMUSER (rw) register accessor: SYSOSC user-specified trim\n\nYou can [`read`](crate::Reg::read) this register and get [`sysosctrimuser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysosctrimuser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysosctrimuser`]
module"]
#[doc(alias = "SYSOSCTRIMUSER")]
pub type Sysosctrimuser = crate::Reg<sysosctrimuser::SysosctrimuserSpec>;
#[doc = "SYSOSC user-specified trim"]
pub mod sysosctrimuser;
#[doc = "SRAMBOUNDARY (rw) register accessor: SRAM Write Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`sramboundary::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramboundary::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramboundary`]
module"]
#[doc(alias = "SRAMBOUNDARY")]
pub type Sramboundary = crate::Reg<sramboundary::SramboundarySpec>;
#[doc = "SRAM Write Boundary"]
pub mod sramboundary;
#[doc = "SYSTEMCFG (rw) register accessor: System configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`systemcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systemcfg`]
module"]
#[doc(alias = "SYSTEMCFG")]
pub type Systemcfg = crate::Reg<systemcfg::SystemcfgSpec>;
#[doc = "System configuration"]
pub mod systemcfg;
#[doc = "WRITELOCK (rw) register accessor: SYSCTL register write lockout\n\nYou can [`read`](crate::Reg::read) this register and get [`writelock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writelock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writelock`]
module"]
#[doc(alias = "WRITELOCK")]
pub type Writelock = crate::Reg<writelock::WritelockSpec>;
#[doc = "SYSCTL register write lockout"]
pub mod writelock;
#[doc = "DEDERRADDR (r) register accessor: Memory DED Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dederraddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dederraddr`]
module"]
#[doc(alias = "DEDERRADDR")]
pub type Dederraddr = crate::Reg<dederraddr::DederraddrSpec>;
#[doc = "Memory DED Address"]
pub mod dederraddr;
#[doc = "RSTCAUSE (r) register accessor: Reset cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcause`]
module"]
#[doc(alias = "RSTCAUSE")]
pub type Rstcause = crate::Reg<rstcause::RstcauseSpec>;
#[doc = "Reset cause"]
pub mod rstcause;
#[doc = "RESETLEVEL (rw) register accessor: Reset level for application-triggered reset command\n\nYou can [`read`](crate::Reg::read) this register and get [`resetlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetlevel`]
module"]
#[doc(alias = "RESETLEVEL")]
pub type Resetlevel = crate::Reg<resetlevel::ResetlevelSpec>;
#[doc = "Reset level for application-triggered reset command"]
pub mod resetlevel;
#[doc = "RESETCMD (w) register accessor: Execute an application-triggered reset command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetcmd`]
module"]
#[doc(alias = "RESETCMD")]
pub type Resetcmd = crate::Reg<resetcmd::ResetcmdSpec>;
#[doc = "Execute an application-triggered reset command"]
pub mod resetcmd;
#[doc = "BORTHRESHOLD (rw) register accessor: BOR threshold selection\n\nYou can [`read`](crate::Reg::read) this register and get [`borthreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`borthreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@borthreshold`]
module"]
#[doc(alias = "BORTHRESHOLD")]
pub type Borthreshold = crate::Reg<borthreshold::BorthresholdSpec>;
#[doc = "BOR threshold selection"]
pub mod borthreshold;
#[doc = "BORCLRCMD (w) register accessor: Set the BOR threshold\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`borclrcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@borclrcmd`]
module"]
#[doc(alias = "BORCLRCMD")]
pub type Borclrcmd = crate::Reg<borclrcmd::BorclrcmdSpec>;
#[doc = "Set the BOR threshold"]
pub mod borclrcmd;
#[doc = "SYSOSCFCLCTL (w) register accessor: SYSOSC frequency correction loop (FCL) ROSC enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscfclctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysoscfclctl`]
module"]
#[doc(alias = "SYSOSCFCLCTL")]
pub type Sysoscfclctl = crate::Reg<sysoscfclctl::SysoscfclctlSpec>;
#[doc = "SYSOSC frequency correction loop (FCL) ROSC enable"]
pub mod sysoscfclctl;
#[doc = "LFXTCTL (w) register accessor: LFXT and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfxtctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfxtctl`]
module"]
#[doc(alias = "LFXTCTL")]
pub type Lfxtctl = crate::Reg<lfxtctl::LfxtctlSpec>;
#[doc = "LFXT and LFCLK control"]
pub mod lfxtctl;
#[doc = "EXLFCTL (w) register accessor: LFCLK_IN and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exlfctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exlfctl`]
module"]
#[doc(alias = "EXLFCTL")]
pub type Exlfctl = crate::Reg<exlfctl::ExlfctlSpec>;
#[doc = "LFCLK_IN and LFCLK control"]
pub mod exlfctl;
#[doc = "SHDNIOREL (w) register accessor: SHUTDOWN IO release control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shdniorel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdniorel`]
module"]
#[doc(alias = "SHDNIOREL")]
pub type Shdniorel = crate::Reg<shdniorel::ShdniorelSpec>;
#[doc = "SHUTDOWN IO release control"]
pub mod shdniorel;
#[doc = "EXRSTPIN (w) register accessor: Disable the reset function of the NRST pin\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrstpin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exrstpin`]
module"]
#[doc(alias = "EXRSTPIN")]
pub type Exrstpin = crate::Reg<exrstpin::ExrstpinSpec>;
#[doc = "Disable the reset function of the NRST pin"]
pub mod exrstpin;
#[doc = "SWDCFG (w) register accessor: Disable the SWD function on the SWD pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdcfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swdcfg`]
module"]
#[doc(alias = "SWDCFG")]
pub type Swdcfg = crate::Reg<swdcfg::SwdcfgSpec>;
#[doc = "Disable the SWD function on the SWD pins"]
pub mod swdcfg;
#[doc = "FCCCMD (w) register accessor: Frequency clock counter start capture\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcccmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcccmd`]
module"]
#[doc(alias = "FCCCMD")]
pub type Fcccmd = crate::Reg<fcccmd::FcccmdSpec>;
#[doc = "Frequency clock counter start capture"]
pub mod fcccmd;
#[doc = "PMUOPAMP (rw) register accessor: GPAMP control\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuopamp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuopamp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmuopamp`]
module"]
#[doc(alias = "PMUOPAMP")]
pub type Pmuopamp = crate::Reg<pmuopamp::PmuopampSpec>;
#[doc = "GPAMP control"]
pub mod pmuopamp;
#[doc = "SHUTDNSTORE0 (rw) register accessor: Shutdown storage memory (byte 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore0`]
module"]
#[doc(alias = "SHUTDNSTORE0")]
pub type Shutdnstore0 = crate::Reg<shutdnstore0::Shutdnstore0Spec>;
#[doc = "Shutdown storage memory (byte 0)"]
pub mod shutdnstore0;
#[doc = "SHUTDNSTORE1 (rw) register accessor: Shutdown storage memory (byte 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore1`]
module"]
#[doc(alias = "SHUTDNSTORE1")]
pub type Shutdnstore1 = crate::Reg<shutdnstore1::Shutdnstore1Spec>;
#[doc = "Shutdown storage memory (byte 1)"]
pub mod shutdnstore1;
#[doc = "SHUTDNSTORE2 (rw) register accessor: Shutdown storage memory (byte 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore2`]
module"]
#[doc(alias = "SHUTDNSTORE2")]
pub type Shutdnstore2 = crate::Reg<shutdnstore2::Shutdnstore2Spec>;
#[doc = "Shutdown storage memory (byte 2)"]
pub mod shutdnstore2;
#[doc = "SHUTDNSTORE3 (rw) register accessor: Shutdown storage memory (byte 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdnstore3`]
module"]
#[doc(alias = "SHUTDNSTORE3")]
pub type Shutdnstore3 = crate::Reg<shutdnstore3::Shutdnstore3Spec>;
#[doc = "Shutdown storage memory (byte 3)"]
pub mod shutdnstore3;
