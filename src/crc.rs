#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    pwren: Pwren,
    rstctl: Rstctl,
    _reserved2: [u8; 0x0c],
    stat: Stat,
    _reserved3: [u8; 0x08e4],
    desc: Desc,
    crcctrl: Crcctrl,
    crcseed: Crcseed,
    crcin: Crcin,
    crcout: Crcout,
    _reserved8: [u8; 0x06f0],
    crcin_idx: [CrcinIdx; 512],
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
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x1100 - CRC Control Register"]
    #[inline(always)]
    pub const fn crcctrl(&self) -> &Crcctrl {
        &self.crcctrl
    }
    #[doc = "0x1104 - CRC Seed Register"]
    #[inline(always)]
    pub const fn crcseed(&self) -> &Crcseed {
        &self.crcseed
    }
    #[doc = "0x1108 - CRC Input Data Register"]
    #[inline(always)]
    pub const fn crcin(&self) -> &Crcin {
        &self.crcin
    }
    #[doc = "0x110c - CRC Output Result Register"]
    #[inline(always)]
    pub const fn crcout(&self) -> &Crcout {
        &self.crcout
    }
    #[doc = "0x1800..0x2000 - CRC Input Data Array Register"]
    #[inline(always)]
    pub const fn crcin_idx(&self, n: usize) -> &CrcinIdx {
        &self.crcin_idx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x2000 - CRC Input Data Array Register"]
    #[inline(always)]
    pub fn crcin_idx_iter(&self) -> impl Iterator<Item = &CrcinIdx> {
        self.crcin_idx.iter()
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
#[doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Module Description"]
pub mod desc;
#[doc = "CRCCTRL (rw) register accessor: CRC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcctrl`]
module"]
#[doc(alias = "CRCCTRL")]
pub type Crcctrl = crate::Reg<crcctrl::CrcctrlSpec>;
#[doc = "CRC Control Register"]
pub mod crcctrl;
#[doc = "CRCSEED (w) register accessor: CRC Seed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcseed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcseed`]
module"]
#[doc(alias = "CRCSEED")]
pub type Crcseed = crate::Reg<crcseed::CrcseedSpec>;
#[doc = "CRC Seed Register"]
pub mod crcseed;
#[doc = "CRCIN (w) register accessor: CRC Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcin`]
module"]
#[doc(alias = "CRCIN")]
pub type Crcin = crate::Reg<crcin::CrcinSpec>;
#[doc = "CRC Input Data Register"]
pub mod crcin;
#[doc = "CRCOUT (r) register accessor: CRC Output Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcout`]
module"]
#[doc(alias = "CRCOUT")]
pub type Crcout = crate::Reg<crcout::CrcoutSpec>;
#[doc = "CRC Output Result Register"]
pub mod crcout;
#[doc = "CRCIN_IDX (w) register accessor: CRC Input Data Array Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcin_idx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcin_idx`]
module"]
#[doc(alias = "CRCIN_IDX")]
pub type CrcinIdx = crate::Reg<crcin_idx::CrcinIdxSpec>;
#[doc = "CRC Input Data Array Register"]
pub mod crcin_idx;
