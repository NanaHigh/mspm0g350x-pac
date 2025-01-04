#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved2: [u8; 0x0c],
    stat: Stat,
    _reserved3: [u8; 0x08e8],
    ctl: Ctl,
    _reserved4: [u8; 0x14],
    op2: Op2,
    op1: Op1,
    res1: Res1,
    res2: Res2,
    _reserved8: [u8; 0x08],
    status: Status,
    _reserved9: [u8; 0x0c],
    statusclr: Statusclr,
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
    #[doc = "0x1100 - Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x1118 - Operand 2 register."]
    #[inline(always)]
    pub const fn op2(&self) -> &Op2 {
        &self.op2
    }
    #[doc = "0x111c - Operand 1 register."]
    #[inline(always)]
    pub const fn op1(&self) -> &Op1 {
        &self.op1
    }
    #[doc = "0x1120 - Result 1 register."]
    #[inline(always)]
    pub const fn res1(&self) -> &Res1 {
        &self.res1
    }
    #[doc = "0x1124 - Result 2 register."]
    #[inline(always)]
    pub const fn res2(&self) -> &Res2 {
        &self.res2
    }
    #[doc = "0x1130 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1140 - Status flag clear register"]
    #[inline(always)]
    pub const fn statusclr(&self) -> &Statusclr {
        &self.statusclr
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
#[doc = "CTL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "OP2 (rw) register accessor: Operand 2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`op2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2`]
module"]
#[doc(alias = "OP2")]
pub type Op2 = crate::Reg<op2::Op2Spec>;
#[doc = "Operand 2 register."]
pub mod op2;
#[doc = "OP1 (rw) register accessor: Operand 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`op1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op1`]
module"]
#[doc(alias = "OP1")]
pub type Op1 = crate::Reg<op1::Op1Spec>;
#[doc = "Operand 1 register."]
pub mod op1;
#[doc = "RES1 (rw) register accessor: Result 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`res1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res1`]
module"]
#[doc(alias = "RES1")]
pub type Res1 = crate::Reg<res1::Res1Spec>;
#[doc = "Result 1 register."]
pub mod res1;
#[doc = "RES2 (rw) register accessor: Result 2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res2`]
module"]
#[doc(alias = "RES2")]
pub type Res2 = crate::Reg<res2::Res2Spec>;
#[doc = "Result 2 register."]
pub mod res2;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "STATUSCLR (w) register accessor: Status flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusclr`]
module"]
#[doc(alias = "STATUSCLR")]
pub type Statusclr = crate::Reg<statusclr::StatusclrSpec>;
#[doc = "Status flag clear register"]
pub mod statusclr;
