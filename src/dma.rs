#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    fsub_0: Fsub0,
    fsub_1: Fsub1,
    _reserved2: [u8; 0x3c],
    fpub_1: Fpub1,
    _reserved3: [u8; 0x0bd0],
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
    dmaprio: Dmaprio,
    _reserved13: [u8; 0x0c],
    dmatctl: Dmatctl,
    _reserved14: [u8; 0xec],
    dmactl: Dmactl,
    dmasa: Dmasa,
    dmada: Dmada,
    dmasz: Dmasz,
}
impl RegisterBlock {
    #[doc = "0x400 - Subscriber Port 0"]
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
    pub const fn fpub_1(&self) -> &Fpub1 {
        &self.fpub_1
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
    #[doc = "0x1100 - DMA Channel Priority Control"]
    #[inline(always)]
    pub const fn dmaprio(&self) -> &Dmaprio {
        &self.dmaprio
    }
    #[doc = "0x1110 - DMA Trigger Select"]
    #[inline(always)]
    pub const fn dmatctl(&self) -> &Dmatctl {
        &self.dmatctl
    }
    #[doc = "0x1200 - DMA Channel Control"]
    #[inline(always)]
    pub const fn dmactl(&self) -> &Dmactl {
        &self.dmactl
    }
    #[doc = "0x1204 - DMA Channel Source Address"]
    #[inline(always)]
    pub const fn dmasa(&self) -> &Dmasa {
        &self.dmasa
    }
    #[doc = "0x1208 - DMA Channel Destination Address"]
    #[inline(always)]
    pub const fn dmada(&self) -> &Dmada {
        &self.dmada
    }
    #[doc = "0x120c - DMA Channel Size"]
    #[inline(always)]
    pub const fn dmasz(&self) -> &Dmasz {
        &self.dmasz
    }
}
#[doc = "FSUB_0 (rw) register accessor: Subscriber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_0`]
module"]
#[doc(alias = "FSUB_0")]
pub type Fsub0 = crate::Reg<fsub_0::Fsub0Spec>;
#[doc = "Subscriber Port 0"]
pub mod fsub_0;
#[doc = "FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_1`]
module"]
#[doc(alias = "FSUB_1")]
pub type Fsub1 = crate::Reg<fsub_1::Fsub1Spec>;
#[doc = "Subscriber Port 1"]
pub mod fsub_1;
#[doc = "FPUB_1 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpub_1`]
module"]
#[doc(alias = "FPUB_1")]
pub type Fpub1 = crate::Reg<fpub_1::Fpub1Spec>;
#[doc = "Publisher Port 0"]
pub mod fpub_1;
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
#[doc = "DMAPRIO (rw) register accessor: DMA Channel Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaprio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaprio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaprio`]
module"]
#[doc(alias = "DMAPRIO")]
pub type Dmaprio = crate::Reg<dmaprio::DmaprioSpec>;
#[doc = "DMA Channel Priority Control"]
pub mod dmaprio;
#[doc = "DMATCTL (rw) register accessor: DMA Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatctl`]
module"]
#[doc(alias = "DMATCTL")]
pub type Dmatctl = crate::Reg<dmatctl::DmatctlSpec>;
#[doc = "DMA Trigger Select"]
pub mod dmatctl;
#[doc = "DMACTL (rw) register accessor: DMA Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl`]
module"]
#[doc(alias = "DMACTL")]
pub type Dmactl = crate::Reg<dmactl::DmactlSpec>;
#[doc = "DMA Channel Control"]
pub mod dmactl;
#[doc = "DMASA (rw) register accessor: DMA Channel Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasa`]
module"]
#[doc(alias = "DMASA")]
pub type Dmasa = crate::Reg<dmasa::DmasaSpec>;
#[doc = "DMA Channel Source Address"]
pub mod dmasa;
#[doc = "DMADA (rw) register accessor: DMA Channel Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmada::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmada::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmada`]
module"]
#[doc(alias = "DMADA")]
pub type Dmada = crate::Reg<dmada::DmadaSpec>;
#[doc = "DMA Channel Destination Address"]
pub mod dmada;
#[doc = "DMASZ (rw) register accessor: DMA Channel Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasz`]
module"]
#[doc(alias = "DMASZ")]
pub type Dmasz = crate::Reg<dmasz::DmaszSpec>;
#[doc = "DMA Channel Size"]
pub mod dmasz;
