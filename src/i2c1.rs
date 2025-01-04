#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    pwren: Pwren,
    rstctl: Rstctl,
    clkcfg: Clkcfg,
    _reserved3: [u8; 0x08],
    stat: Stat,
    _reserved4: [u8; 0x07e8],
    clkdiv: Clkdiv,
    clksel: Clksel,
    _reserved6: [u8; 0x10],
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
    _reserved27: [u8; 0x14],
    desc: Desc,
    _reserved28: [u8; 0x0100],
    gfctl: Gfctl,
    timeout_ctl: TimeoutCtl,
    timeout_cnt: TimeoutCnt,
    _reserved31: [u8; 0x04],
    msa: Msa,
    mctr: Mctr,
    msr: Msr,
    mrxdata: Mrxdata,
    mtxdata: Mtxdata,
    mtpr: Mtpr,
    mcr: Mcr,
    _reserved38: [u8; 0x08],
    mbmon: Mbmon,
    mfifoctl: Mfifoctl,
    mfifosr: Mfifosr,
    master_i2cpecctl: MasterI2cpecctl,
    master_pecsr: MasterPecsr,
    _reserved43: [u8; 0x08],
    soar: Soar,
    soar2: Soar2,
    sctr: Sctr,
    ssr: Ssr,
    srxdata: Srxdata,
    stxdata: Stxdata,
    sackctl: Sackctl,
    sfifoctl: Sfifoctl,
    sfifosr: Sfifosr,
    slave_pecctl: SlavePecctl,
    slave_pecsr: SlavePecsr,
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
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x1000 - Clock Divider"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x1004 - Clock Select for Ultra Low Power peripherals"]
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
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1200 - I2C Glitch Filter Control"]
    #[inline(always)]
    pub const fn gfctl(&self) -> &Gfctl {
        &self.gfctl
    }
    #[doc = "0x1204 - I2C Timeout Count Control Register"]
    #[inline(always)]
    pub const fn timeout_ctl(&self) -> &TimeoutCtl {
        &self.timeout_ctl
    }
    #[doc = "0x1208 - I2C Timeout Count Register"]
    #[inline(always)]
    pub const fn timeout_cnt(&self) -> &TimeoutCnt {
        &self.timeout_cnt
    }
    #[doc = "0x1210 - I2C Master Slave Address Register"]
    #[inline(always)]
    pub const fn msa(&self) -> &Msa {
        &self.msa
    }
    #[doc = "0x1214 - I2C Master Control Register"]
    #[inline(always)]
    pub const fn mctr(&self) -> &Mctr {
        &self.mctr
    }
    #[doc = "0x1218 - I2C Master Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x121c - I2C Master RXData"]
    #[inline(always)]
    pub const fn mrxdata(&self) -> &Mrxdata {
        &self.mrxdata
    }
    #[doc = "0x1220 - I2C Master TXData"]
    #[inline(always)]
    pub const fn mtxdata(&self) -> &Mtxdata {
        &self.mtxdata
    }
    #[doc = "0x1224 - I2C Master Timer Period"]
    #[inline(always)]
    pub const fn mtpr(&self) -> &Mtpr {
        &self.mtpr
    }
    #[doc = "0x1228 - I2C Master Configuration"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x1234 - I2C Master Bus Monitor"]
    #[inline(always)]
    pub const fn mbmon(&self) -> &Mbmon {
        &self.mbmon
    }
    #[doc = "0x1238 - I2C Master FIFO Control"]
    #[inline(always)]
    pub const fn mfifoctl(&self) -> &Mfifoctl {
        &self.mfifoctl
    }
    #[doc = "0x123c - I2C Master FIFO Status Register"]
    #[inline(always)]
    pub const fn mfifosr(&self) -> &Mfifosr {
        &self.mfifosr
    }
    #[doc = "0x1240 - I2C master PEC control register"]
    #[inline(always)]
    pub const fn master_i2cpecctl(&self) -> &MasterI2cpecctl {
        &self.master_i2cpecctl
    }
    #[doc = "0x1244 - I2C master PEC status register"]
    #[inline(always)]
    pub const fn master_pecsr(&self) -> &MasterPecsr {
        &self.master_pecsr
    }
    #[doc = "0x1250 - I2C Slave Own Address"]
    #[inline(always)]
    pub const fn soar(&self) -> &Soar {
        &self.soar
    }
    #[doc = "0x1254 - I2C Slave Own Address 2"]
    #[inline(always)]
    pub const fn soar2(&self) -> &Soar2 {
        &self.soar2
    }
    #[doc = "0x1258 - I2C Slave Control Register"]
    #[inline(always)]
    pub const fn sctr(&self) -> &Sctr {
        &self.sctr
    }
    #[doc = "0x125c - I2C Slave Status Register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &Ssr {
        &self.ssr
    }
    #[doc = "0x1260 - I2C Slave RXData"]
    #[inline(always)]
    pub const fn srxdata(&self) -> &Srxdata {
        &self.srxdata
    }
    #[doc = "0x1264 - I2C Slave TXData"]
    #[inline(always)]
    pub const fn stxdata(&self) -> &Stxdata {
        &self.stxdata
    }
    #[doc = "0x1268 - I2C Slave ACK Control"]
    #[inline(always)]
    pub const fn sackctl(&self) -> &Sackctl {
        &self.sackctl
    }
    #[doc = "0x126c - I2C Slave FIFO Control"]
    #[inline(always)]
    pub const fn sfifoctl(&self) -> &Sfifoctl {
        &self.sfifoctl
    }
    #[doc = "0x1270 - I2C Slave FIFO Status Register"]
    #[inline(always)]
    pub const fn sfifosr(&self) -> &Sfifosr {
        &self.sfifosr
    }
    #[doc = "0x1274 - I2C Slave PEC control register"]
    #[inline(always)]
    pub const fn slave_pecctl(&self) -> &SlavePecctl {
        &self.slave_pecctl
    }
    #[doc = "0x1278 - I2C slave PEC status register"]
    #[inline(always)]
    pub const fn slave_pecsr(&self) -> &SlavePecsr {
        &self.slave_pecsr
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
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "GFCTL (rw) register accessor: I2C Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfctl`]
module"]
#[doc(alias = "GFCTL")]
pub type Gfctl = crate::Reg<gfctl::GfctlSpec>;
#[doc = "I2C Glitch Filter Control"]
pub mod gfctl;
#[doc = "TIMEOUT_CTL (rw) register accessor: I2C Timeout Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_ctl`]
module"]
#[doc(alias = "TIMEOUT_CTL")]
pub type TimeoutCtl = crate::Reg<timeout_ctl::TimeoutCtlSpec>;
#[doc = "I2C Timeout Count Control Register"]
pub mod timeout_ctl;
#[doc = "TIMEOUT_CNT (r) register accessor: I2C Timeout Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_cnt`]
module"]
#[doc(alias = "TIMEOUT_CNT")]
pub type TimeoutCnt = crate::Reg<timeout_cnt::TimeoutCntSpec>;
#[doc = "I2C Timeout Count Register"]
pub mod timeout_cnt;
#[doc = "MSA (rw) register accessor: I2C Master Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msa`]
module"]
#[doc(alias = "MSA")]
pub type Msa = crate::Reg<msa::MsaSpec>;
#[doc = "I2C Master Slave Address Register"]
pub mod msa;
#[doc = "MCTR (rw) register accessor: I2C Master Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctr`]
module"]
#[doc(alias = "MCTR")]
pub type Mctr = crate::Reg<mctr::MctrSpec>;
#[doc = "I2C Master Control Register"]
pub mod mctr;
#[doc = "MSR (r) register accessor: I2C Master Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "I2C Master Status Register"]
pub mod msr;
#[doc = "MRXDATA (r) register accessor: I2C Master RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`mrxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrxdata`]
module"]
#[doc(alias = "MRXDATA")]
pub type Mrxdata = crate::Reg<mrxdata::MrxdataSpec>;
#[doc = "I2C Master RXData"]
pub mod mrxdata;
#[doc = "MTXDATA (rw) register accessor: I2C Master TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`mtxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtxdata`]
module"]
#[doc(alias = "MTXDATA")]
pub type Mtxdata = crate::Reg<mtxdata::MtxdataSpec>;
#[doc = "I2C Master TXData"]
pub mod mtxdata;
#[doc = "MTPR (rw) register accessor: I2C Master Timer Period\n\nYou can [`read`](crate::Reg::read) this register and get [`mtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtpr`]
module"]
#[doc(alias = "MTPR")]
pub type Mtpr = crate::Reg<mtpr::MtprSpec>;
#[doc = "I2C Master Timer Period"]
pub mod mtpr;
#[doc = "MCR (rw) register accessor: I2C Master Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "I2C Master Configuration"]
pub mod mcr;
#[doc = "MBMON (r) register accessor: I2C Master Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`mbmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbmon`]
module"]
#[doc(alias = "MBMON")]
pub type Mbmon = crate::Reg<mbmon::MbmonSpec>;
#[doc = "I2C Master Bus Monitor"]
pub mod mbmon;
#[doc = "MFIFOCTL (rw) register accessor: I2C Master FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfifoctl`]
module"]
#[doc(alias = "MFIFOCTL")]
pub type Mfifoctl = crate::Reg<mfifoctl::MfifoctlSpec>;
#[doc = "I2C Master FIFO Control"]
pub mod mfifoctl;
#[doc = "MFIFOSR (r) register accessor: I2C Master FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfifosr`]
module"]
#[doc(alias = "MFIFOSR")]
pub type Mfifosr = crate::Reg<mfifosr::MfifosrSpec>;
#[doc = "I2C Master FIFO Status Register"]
pub mod mfifosr;
#[doc = "MASTER_I2CPECCTL (rw) register accessor: I2C master PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_i2cpecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_i2cpecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_i2cpecctl`]
module"]
#[doc(alias = "MASTER_I2CPECCTL")]
pub type MasterI2cpecctl = crate::Reg<master_i2cpecctl::MasterI2cpecctlSpec>;
#[doc = "I2C master PEC control register"]
pub mod master_i2cpecctl;
#[doc = "MASTER_PECSR (r) register accessor: I2C master PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_pecsr`]
module"]
#[doc(alias = "MASTER_PECSR")]
pub type MasterPecsr = crate::Reg<master_pecsr::MasterPecsrSpec>;
#[doc = "I2C master PEC status register"]
pub mod master_pecsr;
#[doc = "SOAR (rw) register accessor: I2C Slave Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`soar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soar`]
module"]
#[doc(alias = "SOAR")]
pub type Soar = crate::Reg<soar::SoarSpec>;
#[doc = "I2C Slave Own Address"]
pub mod soar;
#[doc = "SOAR2 (rw) register accessor: I2C Slave Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`soar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soar2`]
module"]
#[doc(alias = "SOAR2")]
pub type Soar2 = crate::Reg<soar2::Soar2Spec>;
#[doc = "I2C Slave Own Address 2"]
pub mod soar2;
#[doc = "SCTR (rw) register accessor: I2C Slave Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctr`]
module"]
#[doc(alias = "SCTR")]
pub type Sctr = crate::Reg<sctr::SctrSpec>;
#[doc = "I2C Slave Control Register"]
pub mod sctr;
#[doc = "SSR (r) register accessor: I2C Slave Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
#[doc(alias = "SSR")]
pub type Ssr = crate::Reg<ssr::SsrSpec>;
#[doc = "I2C Slave Status Register"]
pub mod ssr;
#[doc = "SRXDATA (r) register accessor: I2C Slave RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`srxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srxdata`]
module"]
#[doc(alias = "SRXDATA")]
pub type Srxdata = crate::Reg<srxdata::SrxdataSpec>;
#[doc = "I2C Slave RXData"]
pub mod srxdata;
#[doc = "STXDATA (rw) register accessor: I2C Slave TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`stxdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stxdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stxdata`]
module"]
#[doc(alias = "STXDATA")]
pub type Stxdata = crate::Reg<stxdata::StxdataSpec>;
#[doc = "I2C Slave TXData"]
pub mod stxdata;
#[doc = "SACKCTL (rw) register accessor: I2C Slave ACK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sackctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sackctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sackctl`]
module"]
#[doc(alias = "SACKCTL")]
pub type Sackctl = crate::Reg<sackctl::SackctlSpec>;
#[doc = "I2C Slave ACK Control"]
pub mod sackctl;
#[doc = "SFIFOCTL (rw) register accessor: I2C Slave FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifoctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfifoctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfifoctl`]
module"]
#[doc(alias = "SFIFOCTL")]
pub type Sfifoctl = crate::Reg<sfifoctl::SfifoctlSpec>;
#[doc = "I2C Slave FIFO Control"]
pub mod sfifoctl;
#[doc = "SFIFOSR (r) register accessor: I2C Slave FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfifosr`]
module"]
#[doc(alias = "SFIFOSR")]
pub type Sfifosr = crate::Reg<sfifosr::SfifosrSpec>;
#[doc = "I2C Slave FIFO Status Register"]
pub mod sfifosr;
#[doc = "SLAVE_PECCTL (rw) register accessor: I2C Slave PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_pecctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_pecctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_pecctl`]
module"]
#[doc(alias = "SLAVE_PECCTL")]
pub type SlavePecctl = crate::Reg<slave_pecctl::SlavePecctlSpec>;
#[doc = "I2C Slave PEC control register"]
pub mod slave_pecctl;
#[doc = "SLAVE_PECSR (r) register accessor: I2C slave PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_pecsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_pecsr`]
module"]
#[doc(alias = "SLAVE_PECSR")]
pub type SlavePecsr = crate::Reg<slave_pecsr::SlavePecsrSpec>;
#[doc = "I2C slave PEC status register"]
pub mod slave_pecsr;
