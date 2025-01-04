#[doc = "Register `PINCM[%s]` reader"]
pub type R = crate::R<PincmSpec>;
#[doc = "Register `PINCM[%s]` writer"]
pub type W = crate::W<PincmSpec>;
#[doc = "Field `PINCM_PF` reader - P channel Function selection bits"]
pub type PincmPfR = crate::FieldReader;
#[doc = "Field `PINCM_PF` writer - P channel Function selection bits"]
pub type PincmPfW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Peripheral is Connected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmPc {
    #[doc = "0: UNCONNECTED"]
    PincmPcUnconnected = 0,
    #[doc = "1: CONNECTED"]
    PincmPcConnected = 1,
}
impl From<PincmPc> for bool {
    #[inline(always)]
    fn from(variant: PincmPc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_PC` reader - Peripheral is Connected"]
pub type PincmPcR = crate::BitReader<PincmPc>;
impl PincmPcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmPc {
        match self.bits {
            false => PincmPc::PincmPcUnconnected,
            true => PincmPc::PincmPcConnected,
        }
    }
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn is_pincm_pc_unconnected(&self) -> bool {
        *self == PincmPc::PincmPcUnconnected
    }
    #[doc = "CONNECTED"]
    #[inline(always)]
    pub fn is_pincm_pc_connected(&self) -> bool {
        *self == PincmPc::PincmPcConnected
    }
}
#[doc = "Field `PINCM_PC` writer - Peripheral is Connected"]
pub type PincmPcW<'a, REG> = crate::BitWriter<'a, REG, PincmPc>;
impl<'a, REG> PincmPcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn pincm_pc_unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(PincmPc::PincmPcUnconnected)
    }
    #[doc = "CONNECTED"]
    #[inline(always)]
    pub fn pincm_pc_connected(self) -> &'a mut crate::W<REG> {
        self.variant(PincmPc::PincmPcConnected)
    }
}
#[doc = "This has the IOPAD WAKEUP signal as status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmWakestat {
    #[doc = "0: DISABLE"]
    PincmWakestatDisable = 0,
    #[doc = "1: ENABLE"]
    PincmWakestatEnable = 1,
}
impl From<PincmWakestat> for bool {
    #[inline(always)]
    fn from(variant: PincmWakestat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_WAKESTAT` reader - This has the IOPAD WAKEUP signal as status bit."]
pub type PincmWakestatR = crate::BitReader<PincmWakestat>;
impl PincmWakestatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmWakestat {
        match self.bits {
            false => PincmWakestat::PincmWakestatDisable,
            true => PincmWakestat::PincmWakestatEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_wakestat_disable(&self) -> bool {
        *self == PincmWakestat::PincmWakestatDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_wakestat_enable(&self) -> bool {
        *self == PincmWakestat::PincmWakestatEnable
    }
}
#[doc = "Pull Down control selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmPipd {
    #[doc = "0: DISABLE"]
    PincmPipdDisable = 0,
    #[doc = "1: ENABLE"]
    PincmPipdEnable = 1,
}
impl From<PincmPipd> for bool {
    #[inline(always)]
    fn from(variant: PincmPipd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_PIPD` reader - Pull Down control selection"]
pub type PincmPipdR = crate::BitReader<PincmPipd>;
impl PincmPipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmPipd {
        match self.bits {
            false => PincmPipd::PincmPipdDisable,
            true => PincmPipd::PincmPipdEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_pipd_disable(&self) -> bool {
        *self == PincmPipd::PincmPipdDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_pipd_enable(&self) -> bool {
        *self == PincmPipd::PincmPipdEnable
    }
}
#[doc = "Field `PINCM_PIPD` writer - Pull Down control selection"]
pub type PincmPipdW<'a, REG> = crate::BitWriter<'a, REG, PincmPipd>;
impl<'a, REG> PincmPipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pincm_pipd_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmPipd::PincmPipdDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pincm_pipd_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmPipd::PincmPipdEnable)
    }
}
#[doc = "Pull Up control selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmPipu {
    #[doc = "0: DISABLE"]
    PincmPipuDisable = 0,
    #[doc = "1: ENABLE"]
    PincmPipuEnable = 1,
}
impl From<PincmPipu> for bool {
    #[inline(always)]
    fn from(variant: PincmPipu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_PIPU` reader - Pull Up control selection"]
pub type PincmPipuR = crate::BitReader<PincmPipu>;
impl PincmPipuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmPipu {
        match self.bits {
            false => PincmPipu::PincmPipuDisable,
            true => PincmPipu::PincmPipuEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_pipu_disable(&self) -> bool {
        *self == PincmPipu::PincmPipuDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_pipu_enable(&self) -> bool {
        *self == PincmPipu::PincmPipuEnable
    }
}
#[doc = "Field `PINCM_PIPU` writer - Pull Up control selection"]
pub type PincmPipuW<'a, REG> = crate::BitWriter<'a, REG, PincmPipu>;
impl<'a, REG> PincmPipuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pincm_pipu_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmPipu::PincmPipuDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pincm_pipu_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmPipu::PincmPipuEnable)
    }
}
#[doc = "Input Enable Control Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmInena {
    #[doc = "0: DISABLE"]
    PincmInenaDisable = 0,
    #[doc = "1: ENABLE"]
    PincmInenaEnable = 1,
}
impl From<PincmInena> for bool {
    #[inline(always)]
    fn from(variant: PincmInena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_INENA` reader - Input Enable Control Selection"]
pub type PincmInenaR = crate::BitReader<PincmInena>;
impl PincmInenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmInena {
        match self.bits {
            false => PincmInena::PincmInenaDisable,
            true => PincmInena::PincmInenaEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_inena_disable(&self) -> bool {
        *self == PincmInena::PincmInenaDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_inena_enable(&self) -> bool {
        *self == PincmInena::PincmInenaEnable
    }
}
#[doc = "Field `PINCM_INENA` writer - Input Enable Control Selection"]
pub type PincmInenaW<'a, REG> = crate::BitWriter<'a, REG, PincmInena>;
impl<'a, REG> PincmInenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pincm_inena_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmInena::PincmInenaDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pincm_inena_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmInena::PincmInenaEnable)
    }
}
#[doc = "Hystersis Enable Control Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmHysten {
    #[doc = "0: DISABLE"]
    PincmHystenDisable = 0,
    #[doc = "1: ENABLE"]
    PincmHystenEnable = 1,
}
impl From<PincmHysten> for bool {
    #[inline(always)]
    fn from(variant: PincmHysten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_HYSTEN` reader - Hystersis Enable Control Selection"]
pub type PincmHystenR = crate::BitReader<PincmHysten>;
impl PincmHystenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmHysten {
        match self.bits {
            false => PincmHysten::PincmHystenDisable,
            true => PincmHysten::PincmHystenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_hysten_disable(&self) -> bool {
        *self == PincmHysten::PincmHystenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_hysten_enable(&self) -> bool {
        *self == PincmHysten::PincmHystenEnable
    }
}
#[doc = "Field `PINCM_HYSTEN` writer - Hystersis Enable Control Selection"]
pub type PincmHystenW<'a, REG> = crate::BitWriter<'a, REG, PincmHysten>;
impl<'a, REG> PincmHystenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pincm_hysten_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmHysten::PincmHystenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pincm_hysten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmHysten::PincmHystenEnable)
    }
}
#[doc = "Drive strength control selection, for HS IOCELL only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmDrv {
    #[doc = "0: DRVVAL0"]
    PincmDrvDrvval0 = 0,
    #[doc = "1: DRVVAL1"]
    PincmDrvDrvval1 = 1,
}
impl From<PincmDrv> for bool {
    #[inline(always)]
    fn from(variant: PincmDrv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_DRV` reader - Drive strength control selection, for HS IOCELL only"]
pub type PincmDrvR = crate::BitReader<PincmDrv>;
impl PincmDrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmDrv {
        match self.bits {
            false => PincmDrv::PincmDrvDrvval0,
            true => PincmDrv::PincmDrvDrvval1,
        }
    }
    #[doc = "DRVVAL0"]
    #[inline(always)]
    pub fn is_pincm_drv_drvval0(&self) -> bool {
        *self == PincmDrv::PincmDrvDrvval0
    }
    #[doc = "DRVVAL1"]
    #[inline(always)]
    pub fn is_pincm_drv_drvval1(&self) -> bool {
        *self == PincmDrv::PincmDrvDrvval1
    }
}
#[doc = "Field `PINCM_DRV` writer - Drive strength control selection, for HS IOCELL only"]
pub type PincmDrvW<'a, REG> = crate::BitWriter<'a, REG, PincmDrv>;
impl<'a, REG> PincmDrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DRVVAL0"]
    #[inline(always)]
    pub fn pincm_drv_drvval0(self) -> &'a mut crate::W<REG> {
        self.variant(PincmDrv::PincmDrvDrvval0)
    }
    #[doc = "DRVVAL1"]
    #[inline(always)]
    pub fn pincm_drv_drvval1(self) -> &'a mut crate::W<REG> {
        self.variant(PincmDrv::PincmDrvDrvval1)
    }
}
#[doc = "High output value will tri-state the output when this bit is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmHiz1 {
    #[doc = "0: DISABLE"]
    PincmHiz1Disable = 0,
    #[doc = "1: ENABLE"]
    PincmHiz1Enable = 1,
}
impl From<PincmHiz1> for bool {
    #[inline(always)]
    fn from(variant: PincmHiz1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_HIZ1` reader - High output value will tri-state the output when this bit is enabled"]
pub type PincmHiz1R = crate::BitReader<PincmHiz1>;
impl PincmHiz1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmHiz1 {
        match self.bits {
            false => PincmHiz1::PincmHiz1Disable,
            true => PincmHiz1::PincmHiz1Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_hiz1_disable(&self) -> bool {
        *self == PincmHiz1::PincmHiz1Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_hiz1_enable(&self) -> bool {
        *self == PincmHiz1::PincmHiz1Enable
    }
}
#[doc = "Field `PINCM_HIZ1` writer - High output value will tri-state the output when this bit is enabled"]
pub type PincmHiz1W<'a, REG> = crate::BitWriter<'a, REG, PincmHiz1>;
impl<'a, REG> PincmHiz1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pincm_hiz1_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmHiz1::PincmHiz1Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pincm_hiz1_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmHiz1::PincmHiz1Enable)
    }
}
#[doc = "Data inversion selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmInv {
    #[doc = "0: DISABLE"]
    PincmInvDisable = 0,
    #[doc = "1: ENABLE"]
    PincmInvEnable = 1,
}
impl From<PincmInv> for bool {
    #[inline(always)]
    fn from(variant: PincmInv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_INV` reader - Data inversion selection"]
pub type PincmInvR = crate::BitReader<PincmInv>;
impl PincmInvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmInv {
        match self.bits {
            false => PincmInv::PincmInvDisable,
            true => PincmInv::PincmInvEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_inv_disable(&self) -> bool {
        *self == PincmInv::PincmInvDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_inv_enable(&self) -> bool {
        *self == PincmInv::PincmInvEnable
    }
}
#[doc = "Field `PINCM_INV` writer - Data inversion selection"]
pub type PincmInvW<'a, REG> = crate::BitWriter<'a, REG, PincmInv>;
impl<'a, REG> PincmInvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pincm_inv_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmInv::PincmInvDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pincm_inv_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmInv::PincmInvEnable)
    }
}
#[doc = "Wakeup Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmWuen {
    #[doc = "0: DISABLE"]
    PincmWuenDisable = 0,
    #[doc = "1: ENABLE"]
    PincmWuenEnable = 1,
}
impl From<PincmWuen> for bool {
    #[inline(always)]
    fn from(variant: PincmWuen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_WUEN` reader - Wakeup Enable bit"]
pub type PincmWuenR = crate::BitReader<PincmWuen>;
impl PincmWuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmWuen {
        match self.bits {
            false => PincmWuen::PincmWuenDisable,
            true => PincmWuen::PincmWuenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pincm_wuen_disable(&self) -> bool {
        *self == PincmWuen::PincmWuenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pincm_wuen_enable(&self) -> bool {
        *self == PincmWuen::PincmWuenEnable
    }
}
#[doc = "Field `PINCM_WUEN` writer - Wakeup Enable bit"]
pub type PincmWuenW<'a, REG> = crate::BitWriter<'a, REG, PincmWuen>;
impl<'a, REG> PincmWuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pincm_wuen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmWuen::PincmWuenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pincm_wuen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmWuen::PincmWuenEnable)
    }
}
#[doc = "Wakeup Compare Value bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmWcomp {
    #[doc = "0: MATCH0"]
    PincmWcompMatch0 = 0,
    #[doc = "1: MATCH1"]
    PincmWcompMatch1 = 1,
}
impl From<PincmWcomp> for bool {
    #[inline(always)]
    fn from(variant: PincmWcomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM_WCOMP` reader - Wakeup Compare Value bit"]
pub type PincmWcompR = crate::BitReader<PincmWcomp>;
impl PincmWcompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PincmWcomp {
        match self.bits {
            false => PincmWcomp::PincmWcompMatch0,
            true => PincmWcomp::PincmWcompMatch1,
        }
    }
    #[doc = "MATCH0"]
    #[inline(always)]
    pub fn is_pincm_wcomp_match0(&self) -> bool {
        *self == PincmWcomp::PincmWcompMatch0
    }
    #[doc = "MATCH1"]
    #[inline(always)]
    pub fn is_pincm_wcomp_match1(&self) -> bool {
        *self == PincmWcomp::PincmWcompMatch1
    }
}
#[doc = "Field `PINCM_WCOMP` writer - Wakeup Compare Value bit"]
pub type PincmWcompW<'a, REG> = crate::BitWriter<'a, REG, PincmWcomp>;
impl<'a, REG> PincmWcompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MATCH0"]
    #[inline(always)]
    pub fn pincm_wcomp_match0(self) -> &'a mut crate::W<REG> {
        self.variant(PincmWcomp::PincmWcompMatch0)
    }
    #[doc = "MATCH1"]
    #[inline(always)]
    pub fn pincm_wcomp_match1(self) -> &'a mut crate::W<REG> {
        self.variant(PincmWcomp::PincmWcompMatch1)
    }
}
impl R {
    #[doc = "Bits 0:5 - P channel Function selection bits"]
    #[inline(always)]
    pub fn pincm_pf(&self) -> PincmPfR {
        PincmPfR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Peripheral is Connected"]
    #[inline(always)]
    pub fn pincm_pc(&self) -> PincmPcR {
        PincmPcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - This has the IOPAD WAKEUP signal as status bit."]
    #[inline(always)]
    pub fn pincm_wakestat(&self) -> PincmWakestatR {
        PincmWakestatR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Pull Down control selection"]
    #[inline(always)]
    pub fn pincm_pipd(&self) -> PincmPipdR {
        PincmPipdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pull Up control selection"]
    #[inline(always)]
    pub fn pincm_pipu(&self) -> PincmPipuR {
        PincmPipuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input Enable Control Selection"]
    #[inline(always)]
    pub fn pincm_inena(&self) -> PincmInenaR {
        PincmInenaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Hystersis Enable Control Selection"]
    #[inline(always)]
    pub fn pincm_hysten(&self) -> PincmHystenR {
        PincmHystenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Drive strength control selection, for HS IOCELL only"]
    #[inline(always)]
    pub fn pincm_drv(&self) -> PincmDrvR {
        PincmDrvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - High output value will tri-state the output when this bit is enabled"]
    #[inline(always)]
    pub fn pincm_hiz1(&self) -> PincmHiz1R {
        PincmHiz1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Data inversion selection"]
    #[inline(always)]
    pub fn pincm_inv(&self) -> PincmInvR {
        PincmInvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wakeup Enable bit"]
    #[inline(always)]
    pub fn pincm_wuen(&self) -> PincmWuenR {
        PincmWuenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wakeup Compare Value bit"]
    #[inline(always)]
    pub fn pincm_wcomp(&self) -> PincmWcompR {
        PincmWcompR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - P channel Function selection bits"]
    #[inline(always)]
    pub fn pincm_pf(&mut self) -> PincmPfW<PincmSpec> {
        PincmPfW::new(self, 0)
    }
    #[doc = "Bit 7 - Peripheral is Connected"]
    #[inline(always)]
    pub fn pincm_pc(&mut self) -> PincmPcW<PincmSpec> {
        PincmPcW::new(self, 7)
    }
    #[doc = "Bit 16 - Pull Down control selection"]
    #[inline(always)]
    pub fn pincm_pipd(&mut self) -> PincmPipdW<PincmSpec> {
        PincmPipdW::new(self, 16)
    }
    #[doc = "Bit 17 - Pull Up control selection"]
    #[inline(always)]
    pub fn pincm_pipu(&mut self) -> PincmPipuW<PincmSpec> {
        PincmPipuW::new(self, 17)
    }
    #[doc = "Bit 18 - Input Enable Control Selection"]
    #[inline(always)]
    pub fn pincm_inena(&mut self) -> PincmInenaW<PincmSpec> {
        PincmInenaW::new(self, 18)
    }
    #[doc = "Bit 19 - Hystersis Enable Control Selection"]
    #[inline(always)]
    pub fn pincm_hysten(&mut self) -> PincmHystenW<PincmSpec> {
        PincmHystenW::new(self, 19)
    }
    #[doc = "Bit 20 - Drive strength control selection, for HS IOCELL only"]
    #[inline(always)]
    pub fn pincm_drv(&mut self) -> PincmDrvW<PincmSpec> {
        PincmDrvW::new(self, 20)
    }
    #[doc = "Bit 25 - High output value will tri-state the output when this bit is enabled"]
    #[inline(always)]
    pub fn pincm_hiz1(&mut self) -> PincmHiz1W<PincmSpec> {
        PincmHiz1W::new(self, 25)
    }
    #[doc = "Bit 26 - Data inversion selection"]
    #[inline(always)]
    pub fn pincm_inv(&mut self) -> PincmInvW<PincmSpec> {
        PincmInvW::new(self, 26)
    }
    #[doc = "Bit 27 - Wakeup Enable bit"]
    #[inline(always)]
    pub fn pincm_wuen(&mut self) -> PincmWuenW<PincmSpec> {
        PincmWuenW::new(self, 27)
    }
    #[doc = "Bit 28 - Wakeup Compare Value bit"]
    #[inline(always)]
    pub fn pincm_wcomp(&mut self) -> PincmWcompW<PincmSpec> {
        PincmWcompW::new(self, 28)
    }
}
#[doc = "Pin Control Management Register in SECCFG region\n\nYou can [`read`](crate::Reg::read) this register and get [`pincm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PincmSpec;
impl crate::RegisterSpec for PincmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pincm::R`](R) reader structure"]
impl crate::Readable for PincmSpec {}
#[doc = "`write(|w| ..)` method takes [`pincm::W`](W) writer structure"]
impl crate::Writable for PincmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PINCM[%s]
to value 0"]
impl crate::Resettable for PincmSpec {
    const RESET_VALUE: u32 = 0;
}
