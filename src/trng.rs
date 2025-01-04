#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved2: [u8; 0x0c],
    gprcm_stat: GprcmStat,
    _reserved3: [u8; 0x0808],
    iidx: Iidx,
    _reserved4: [u8; 0x04],
    imask: Imask,
    _reserved5: [u8; 0x04],
    ris: Ris,
    _reserved6: [u8; 0x04],
    mis: Mis,
    _reserved7: [u8; 0x04],
    iset: Iset,
    _reserved8: [u8; 0x04],
    iclr: Iclr,
    _reserved9: [u8; 0xb0],
    desc: Desc,
    ctl: Ctl,
    stat: Stat,
    data_capture: DataCapture,
    test_results: TestResults,
    clkdivide: Clkdivide,
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
    #[doc = "0x10fc - Module descriptions"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1100 - Controls the command and decimation rate"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x1104 - Status register that informs health test results and last issued command"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x1108 - Captured word buffer of RNG data"]
    #[inline(always)]
    pub const fn data_capture(&self) -> &DataCapture {
        &self.data_capture
    }
    #[doc = "0x110c - Test results from TEST_ANA and TEST_DIG"]
    #[inline(always)]
    pub const fn test_results(&self) -> &TestResults {
        &self.test_results
    }
    #[doc = "0x1110 - Clock Divider"]
    #[inline(always)]
    pub const fn clkdivide(&self) -> &Clkdivide {
        &self.clkdivide
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
#[doc = "DESC (r) register accessor: Module descriptions\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module descriptions"]
pub mod desc;
#[doc = "CTL (rw) register accessor: Controls the command and decimation rate\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Controls the command and decimation rate"]
pub mod ctl;
#[doc = "STAT (r) register accessor: Status register that informs health test results and last issued command\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register that informs health test results and last issued command"]
pub mod stat;
#[doc = "DATA_CAPTURE (r) register accessor: Captured word buffer of RNG data\n\nYou can [`read`](crate::Reg::read) this register and get [`data_capture::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_capture`]
module"]
#[doc(alias = "DATA_CAPTURE")]
pub type DataCapture = crate::Reg<data_capture::DataCaptureSpec>;
#[doc = "Captured word buffer of RNG data"]
pub mod data_capture;
#[doc = "TEST_RESULTS (r) register accessor: Test results from TEST_ANA and TEST_DIG\n\nYou can [`read`](crate::Reg::read) this register and get [`test_results::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_results`]
module"]
#[doc(alias = "TEST_RESULTS")]
pub type TestResults = crate::Reg<test_results::TestResultsSpec>;
#[doc = "Test results from TEST_ANA and TEST_DIG"]
pub mod test_results;
#[doc = "CLKDIVIDE (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdivide::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdivide::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdivide`]
module"]
#[doc(alias = "CLKDIVIDE")]
pub type Clkdivide = crate::Reg<clkdivide::ClkdivideSpec>;
#[doc = "Clock Divider"]
pub mod clkdivide;
