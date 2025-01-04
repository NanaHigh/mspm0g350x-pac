#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved2: [u8; 0x0c],
    gprcm_stat: GprcmStat,
    _reserved3: [u8; 0x07f8],
    clkovr: Clkovr,
    _reserved4: [u8; 0x08],
    pwrctl: Pwrctl,
    _reserved5: [u8; 0xe0],
    ctl: Ctl,
    cfgbase: Cfgbase,
    cfg: Cfg,
    _reserved8: [u8; 0x0c],
    stat: Stat,
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
    pub const fn gprcm_stat(&self) -> &GprcmStat {
        &self.gprcm_stat
    }
    #[doc = "0x1010 - Clock Override"]
    #[inline(always)]
    pub const fn clkovr(&self) -> &Clkovr {
        &self.clkovr
    }
    #[doc = "0x101c - Power Control"]
    #[inline(always)]
    pub const fn pwrctl(&self) -> &Pwrctl {
        &self.pwrctl
    }
    #[doc = "0x1100 - Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x1104 - Configuration Base Register"]
    #[inline(always)]
    pub const fn cfgbase(&self) -> &Cfgbase {
        &self.cfgbase
    }
    #[doc = "0x1108 - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x1118 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
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
#[doc = "GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gprcm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gprcm_stat`]
module"]
#[doc(alias = "GPRCM_STAT")]
pub type GprcmStat = crate::Reg<gprcm_stat::GprcmStatSpec>;
#[doc = "Status Register"]
pub mod gprcm_stat;
#[doc = "CLKOVR (rw) register accessor: Clock Override\n\nYou can [`read`](crate::Reg::read) this register and get [`clkovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkovr`]
module"]
#[doc(alias = "CLKOVR")]
pub type Clkovr = crate::Reg<clkovr::ClkovrSpec>;
#[doc = "Clock Override"]
pub mod clkovr;
#[doc = "PWRCTL (rw) register accessor: Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctl`]
module"]
#[doc(alias = "PWRCTL")]
pub type Pwrctl = crate::Reg<pwrctl::PwrctlSpec>;
#[doc = "Power Control"]
pub mod pwrctl;
#[doc = "CTL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "CFGBASE (rw) register accessor: Configuration Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgbase`]
module"]
#[doc(alias = "CFGBASE")]
pub type Cfgbase = crate::Reg<cfgbase::CfgbaseSpec>;
#[doc = "Configuration Base Register"]
pub mod cfgbase;
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
