#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Enable {
    #[doc = "0: DISABLE"]
    Ctl1EnableDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1EnableEnable = 1,
}
impl From<Ctl1Enable> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_ENABLE` reader - SPI enable"]
pub type Ctl1EnableR = crate::BitReader<Ctl1Enable>;
impl Ctl1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Enable {
        match self.bits {
            false => Ctl1Enable::Ctl1EnableDisable,
            true => Ctl1Enable::Ctl1EnableEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_enable_disable(&self) -> bool {
        *self == Ctl1Enable::Ctl1EnableDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_enable_enable(&self) -> bool {
        *self == Ctl1Enable::Ctl1EnableEnable
    }
}
#[doc = "Field `CTL1_ENABLE` writer - SPI enable"]
pub type Ctl1EnableW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Enable>;
impl<'a, REG> Ctl1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_enable_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Enable::Ctl1EnableDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_enable_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Enable::Ctl1EnableEnable)
    }
}
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Lbm {
    #[doc = "0: DISABLE"]
    Ctl1LbmDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1LbmEnable = 1,
}
impl From<Ctl1Lbm> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_LBM` reader - Loop back mode"]
pub type Ctl1LbmR = crate::BitReader<Ctl1Lbm>;
impl Ctl1LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Lbm {
        match self.bits {
            false => Ctl1Lbm::Ctl1LbmDisable,
            true => Ctl1Lbm::Ctl1LbmEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_lbm_disable(&self) -> bool {
        *self == Ctl1Lbm::Ctl1LbmDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_lbm_enable(&self) -> bool {
        *self == Ctl1Lbm::Ctl1LbmEnable
    }
}
#[doc = "Field `CTL1_LBM` writer - Loop back mode"]
pub type Ctl1LbmW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Lbm>;
impl<'a, REG> Ctl1LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_lbm_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Lbm::Ctl1LbmDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_lbm_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Lbm::Ctl1LbmEnable)
    }
}
#[doc = "Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Ms {
    #[doc = "0: DISABLE"]
    Ctl1MsDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1MsEnable = 1,
}
impl From<Ctl1Ms> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Ms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_MS` reader - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
pub type Ctl1MsR = crate::BitReader<Ctl1Ms>;
impl Ctl1MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Ms {
        match self.bits {
            false => Ctl1Ms::Ctl1MsDisable,
            true => Ctl1Ms::Ctl1MsEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_ms_disable(&self) -> bool {
        *self == Ctl1Ms::Ctl1MsDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_ms_enable(&self) -> bool {
        *self == Ctl1Ms::Ctl1MsEnable
    }
}
#[doc = "Field `CTL1_MS` writer - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
pub type Ctl1MsW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Ms>;
impl<'a, REG> Ctl1MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_ms_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ms::Ctl1MsDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_ms_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ms::Ctl1MsEnable)
    }
}
#[doc = "Peripheral-mode: Data output disabled This bit is relevant only in the peripheral mode, CTL1.MS=1. In multiple-peripheral systems, it is possible for an SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the MISO lines from multiple peripherals could be tied together. To operate in such systems, this bitfield can be set if the SPI peripheral is not supposed to drive the MISO line:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Sod {
    #[doc = "0: DISABLE"]
    Ctl1SodDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1SodEnable = 1,
}
impl From<Ctl1Sod> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Sod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_SOD` reader - Peripheral-mode: Data output disabled This bit is relevant only in the peripheral mode, CTL1.MS=1. In multiple-peripheral systems, it is possible for an SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the MISO lines from multiple peripherals could be tied together. To operate in such systems, this bitfield can be set if the SPI peripheral is not supposed to drive the MISO line:"]
pub type Ctl1SodR = crate::BitReader<Ctl1Sod>;
impl Ctl1SodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Sod {
        match self.bits {
            false => Ctl1Sod::Ctl1SodDisable,
            true => Ctl1Sod::Ctl1SodEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_sod_disable(&self) -> bool {
        *self == Ctl1Sod::Ctl1SodDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_sod_enable(&self) -> bool {
        *self == Ctl1Sod::Ctl1SodEnable
    }
}
#[doc = "Field `CTL1_SOD` writer - Peripheral-mode: Data output disabled This bit is relevant only in the peripheral mode, CTL1.MS=1. In multiple-peripheral systems, it is possible for an SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the MISO lines from multiple peripherals could be tied together. To operate in such systems, this bitfield can be set if the SPI peripheral is not supposed to drive the MISO line:"]
pub type Ctl1SodW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Sod>;
impl<'a, REG> Ctl1SodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_sod_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Sod::Ctl1SodDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_sod_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Sod::Ctl1SodEnable)
    }
}
#[doc = "MSB first select. Controls the direction of the receive and transmit shift register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Msb {
    #[doc = "0: DISABLE"]
    Ctl1MsbDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1MsbEnable = 1,
}
impl From<Ctl1Msb> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Msb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_MSB` reader - MSB first select. Controls the direction of the receive and transmit shift register."]
pub type Ctl1MsbR = crate::BitReader<Ctl1Msb>;
impl Ctl1MsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Msb {
        match self.bits {
            false => Ctl1Msb::Ctl1MsbDisable,
            true => Ctl1Msb::Ctl1MsbEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_msb_disable(&self) -> bool {
        *self == Ctl1Msb::Ctl1MsbDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_msb_enable(&self) -> bool {
        *self == Ctl1Msb::Ctl1MsbEnable
    }
}
#[doc = "Field `CTL1_MSB` writer - MSB first select. Controls the direction of the receive and transmit shift register."]
pub type Ctl1MsbW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Msb>;
impl<'a, REG> Ctl1MsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_msb_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Msb::Ctl1MsbDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_msb_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Msb::Ctl1MsbEnable)
    }
}
#[doc = "Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Pren {
    #[doc = "0: DISABLE"]
    Ctl1PrenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1PrenEnable = 1,
}
impl From<Ctl1Pren> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Pren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_PREN` reader - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
pub type Ctl1PrenR = crate::BitReader<Ctl1Pren>;
impl Ctl1PrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Pren {
        match self.bits {
            false => Ctl1Pren::Ctl1PrenDisable,
            true => Ctl1Pren::Ctl1PrenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_pren_disable(&self) -> bool {
        *self == Ctl1Pren::Ctl1PrenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_pren_enable(&self) -> bool {
        *self == Ctl1Pren::Ctl1PrenEnable
    }
}
#[doc = "Field `CTL1_PREN` writer - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
pub type Ctl1PrenW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Pren>;
impl<'a, REG> Ctl1PrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_pren_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pren::Ctl1PrenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_pren_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pren::Ctl1PrenEnable)
    }
}
#[doc = "Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Pes {
    #[doc = "0: DISABLE"]
    Ctl1PesDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1PesEnable = 1,
}
impl From<Ctl1Pes> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Pes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_PES` reader - Even Parity Select"]
pub type Ctl1PesR = crate::BitReader<Ctl1Pes>;
impl Ctl1PesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Pes {
        match self.bits {
            false => Ctl1Pes::Ctl1PesDisable,
            true => Ctl1Pes::Ctl1PesEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_pes_disable(&self) -> bool {
        *self == Ctl1Pes::Ctl1PesDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_pes_enable(&self) -> bool {
        *self == Ctl1Pes::Ctl1PesEnable
    }
}
#[doc = "Field `CTL1_PES` writer - Even Parity Select"]
pub type Ctl1PesW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Pes>;
impl<'a, REG> Ctl1PesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_pes_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pes::Ctl1PesDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_pes_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pes::Ctl1PesEnable)
    }
}
#[doc = "Parity Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Pbs {
    #[doc = "0: DISABLE"]
    Ctl1PbsDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1PbsEnable = 1,
}
impl From<Ctl1Pbs> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Pbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_PBS` reader - Parity Bit Select"]
pub type Ctl1PbsR = crate::BitReader<Ctl1Pbs>;
impl Ctl1PbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Pbs {
        match self.bits {
            false => Ctl1Pbs::Ctl1PbsDisable,
            true => Ctl1Pbs::Ctl1PbsEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_pbs_disable(&self) -> bool {
        *self == Ctl1Pbs::Ctl1PbsDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_pbs_enable(&self) -> bool {
        *self == Ctl1Pbs::Ctl1PbsEnable
    }
}
#[doc = "Field `CTL1_PBS` writer - Parity Bit Select"]
pub type Ctl1PbsW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Pbs>;
impl<'a, REG> Ctl1PbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_pbs_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pbs::Ctl1PbsDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_pbs_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pbs::Ctl1PbsEnable)
    }
}
#[doc = "Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Pten {
    #[doc = "0: DISABLE"]
    Ctl1PtenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1PtenEnable = 1,
}
impl From<Ctl1Pten> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Pten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_PTEN` reader - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
pub type Ctl1PtenR = crate::BitReader<Ctl1Pten>;
impl Ctl1PtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Pten {
        match self.bits {
            false => Ctl1Pten::Ctl1PtenDisable,
            true => Ctl1Pten::Ctl1PtenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_pten_disable(&self) -> bool {
        *self == Ctl1Pten::Ctl1PtenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_pten_enable(&self) -> bool {
        *self == Ctl1Pten::Ctl1PtenEnable
    }
}
#[doc = "Field `CTL1_PTEN` writer - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
pub type Ctl1PtenW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Pten>;
impl<'a, REG> Ctl1PtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_pten_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pten::Ctl1PtenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_pten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Pten::Ctl1PtenEnable)
    }
}
#[doc = "Command/Data Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Cdenable {
    #[doc = "0: DISABLE"]
    Ctl1CdenableDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1CdenableEnable = 1,
}
impl From<Ctl1Cdenable> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Cdenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_CDENABLE` reader - Command/Data Mode enable"]
pub type Ctl1CdenableR = crate::BitReader<Ctl1Cdenable>;
impl Ctl1CdenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Cdenable {
        match self.bits {
            false => Ctl1Cdenable::Ctl1CdenableDisable,
            true => Ctl1Cdenable::Ctl1CdenableEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_cdenable_disable(&self) -> bool {
        *self == Ctl1Cdenable::Ctl1CdenableDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_cdenable_enable(&self) -> bool {
        *self == Ctl1Cdenable::Ctl1CdenableEnable
    }
}
#[doc = "Field `CTL1_CDENABLE` writer - Command/Data Mode enable"]
pub type Ctl1CdenableW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Cdenable>;
impl<'a, REG> Ctl1CdenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_cdenable_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Cdenable::Ctl1CdenableDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_cdenable_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Cdenable::Ctl1CdenableEnable)
    }
}
#[doc = "Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl1Cdmode {
    #[doc = "0: DATA"]
    Ctl1CdmodeData = 0,
    #[doc = "15: COMMAND"]
    Ctl1CdmodeCommand = 15,
}
impl From<Ctl1Cdmode> for u8 {
    #[inline(always)]
    fn from(variant: Ctl1Cdmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl1Cdmode {
    type Ux = u8;
}
impl crate::IsEnum for Ctl1Cdmode {}
#[doc = "Field `CTL1_CDMODE` reader - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
pub type Ctl1CdmodeR = crate::FieldReader<Ctl1Cdmode>;
impl Ctl1CdmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl1Cdmode> {
        match self.bits {
            0 => Some(Ctl1Cdmode::Ctl1CdmodeData),
            15 => Some(Ctl1Cdmode::Ctl1CdmodeCommand),
            _ => None,
        }
    }
    #[doc = "DATA"]
    #[inline(always)]
    pub fn is_ctl1_cdmode_data(&self) -> bool {
        *self == Ctl1Cdmode::Ctl1CdmodeData
    }
    #[doc = "COMMAND"]
    #[inline(always)]
    pub fn is_ctl1_cdmode_command(&self) -> bool {
        *self == Ctl1Cdmode::Ctl1CdmodeCommand
    }
}
#[doc = "Field `CTL1_CDMODE` writer - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
pub type Ctl1CdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctl1Cdmode>;
impl<'a, REG> Ctl1CdmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DATA"]
    #[inline(always)]
    pub fn ctl1_cdmode_data(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Cdmode::Ctl1CdmodeData)
    }
    #[doc = "COMMAND"]
    #[inline(always)]
    pub fn ctl1_cdmode_command(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Cdmode::Ctl1CdmodeCommand)
    }
}
#[doc = "Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl1Repeattx {
    #[doc = "0: DISABLE"]
    Ctl1RepeattxDisable = 0,
}
impl From<Ctl1Repeattx> for u8 {
    #[inline(always)]
    fn from(variant: Ctl1Repeattx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl1Repeattx {
    type Ux = u8;
}
impl crate::IsEnum for Ctl1Repeattx {}
#[doc = "Field `CTL1_REPEATTX` reader - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
pub type Ctl1RepeattxR = crate::FieldReader<Ctl1Repeattx>;
impl Ctl1RepeattxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl1Repeattx> {
        match self.bits {
            0 => Some(Ctl1Repeattx::Ctl1RepeattxDisable),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_repeattx_disable(&self) -> bool {
        *self == Ctl1Repeattx::Ctl1RepeattxDisable
    }
}
#[doc = "Field `CTL1_REPEATTX` writer - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
pub type Ctl1RepeattxW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctl1Repeattx>;
impl<'a, REG> Ctl1RepeattxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_repeattx_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Repeattx::Ctl1RepeattxDisable)
    }
}
#[doc = "Field `CTL1_RXTIMEOUT` reader - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
pub type Ctl1RxtimeoutR = crate::FieldReader;
#[doc = "Field `CTL1_RXTIMEOUT` writer - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
pub type Ctl1RxtimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - SPI enable"]
    #[inline(always)]
    pub fn ctl1_enable(&self) -> Ctl1EnableR {
        Ctl1EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loop back mode"]
    #[inline(always)]
    pub fn ctl1_lbm(&self) -> Ctl1LbmR {
        Ctl1LbmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
    #[inline(always)]
    pub fn ctl1_ms(&self) -> Ctl1MsR {
        Ctl1MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral-mode: Data output disabled This bit is relevant only in the peripheral mode, CTL1.MS=1. In multiple-peripheral systems, it is possible for an SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the MISO lines from multiple peripherals could be tied together. To operate in such systems, this bitfield can be set if the SPI peripheral is not supposed to drive the MISO line:"]
    #[inline(always)]
    pub fn ctl1_sod(&self) -> Ctl1SodR {
        Ctl1SodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    pub fn ctl1_msb(&self) -> Ctl1MsbR {
        Ctl1MsbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
    #[inline(always)]
    pub fn ctl1_pren(&self) -> Ctl1PrenR {
        Ctl1PrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Even Parity Select"]
    #[inline(always)]
    pub fn ctl1_pes(&self) -> Ctl1PesR {
        Ctl1PesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Bit Select"]
    #[inline(always)]
    pub fn ctl1_pbs(&self) -> Ctl1PbsR {
        Ctl1PbsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
    #[inline(always)]
    pub fn ctl1_pten(&self) -> Ctl1PtenR {
        Ctl1PtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Command/Data Mode enable"]
    #[inline(always)]
    pub fn ctl1_cdenable(&self) -> Ctl1CdenableR {
        Ctl1CdenableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
    #[inline(always)]
    pub fn ctl1_cdmode(&self) -> Ctl1CdmodeR {
        Ctl1CdmodeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
    #[inline(always)]
    pub fn ctl1_repeattx(&self) -> Ctl1RepeattxR {
        Ctl1RepeattxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
    #[inline(always)]
    pub fn ctl1_rxtimeout(&self) -> Ctl1RxtimeoutR {
        Ctl1RxtimeoutR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI enable"]
    #[inline(always)]
    pub fn ctl1_enable(&mut self) -> Ctl1EnableW<Ctl1Spec> {
        Ctl1EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Loop back mode"]
    #[inline(always)]
    pub fn ctl1_lbm(&mut self) -> Ctl1LbmW<Ctl1Spec> {
        Ctl1LbmW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller or peripheral mode select. This bit can be modified only when SPI is disabled, CTL1.ENABLE=0."]
    #[inline(always)]
    pub fn ctl1_ms(&mut self) -> Ctl1MsW<Ctl1Spec> {
        Ctl1MsW::new(self, 2)
    }
    #[doc = "Bit 3 - Peripheral-mode: Data output disabled This bit is relevant only in the peripheral mode, CTL1.MS=1. In multiple-peripheral systems, it is possible for an SPI controller to broadcast a message to all peripherals in the system while ensuring that only one peripheral drives data onto its serial output line. In such systems the MISO lines from multiple peripherals could be tied together. To operate in such systems, this bitfield can be set if the SPI peripheral is not supposed to drive the MISO line:"]
    #[inline(always)]
    pub fn ctl1_sod(&mut self) -> Ctl1SodW<Ctl1Spec> {
        Ctl1SodW::new(self, 3)
    }
    #[doc = "Bit 4 - MSB first select. Controls the direction of the receive and transmit shift register."]
    #[inline(always)]
    pub fn ctl1_msb(&mut self) -> Ctl1MsbW<Ctl1Spec> {
        Ctl1MsbW::new(self, 4)
    }
    #[doc = "Bit 5 - Parity receive enable If enabled, parity reception check will be done for both controller and peripheral modes In case of a parity miss-match the parity error flag RIS.PER will be set."]
    #[inline(always)]
    pub fn ctl1_pren(&mut self) -> Ctl1PrenW<Ctl1Spec> {
        Ctl1PrenW::new(self, 5)
    }
    #[doc = "Bit 6 - Even Parity Select"]
    #[inline(always)]
    pub fn ctl1_pes(&mut self) -> Ctl1PesW<Ctl1Spec> {
        Ctl1PesW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Bit Select"]
    #[inline(always)]
    pub fn ctl1_pbs(&mut self) -> Ctl1PbsW<Ctl1Spec> {
        Ctl1PbsW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity transmit enable If enabled, parity transmission will be done for both controller and peripheral modes."]
    #[inline(always)]
    pub fn ctl1_pten(&mut self) -> Ctl1PtenW<Ctl1Spec> {
        Ctl1PtenW::new(self, 8)
    }
    #[doc = "Bit 11 - Command/Data Mode enable"]
    #[inline(always)]
    pub fn ctl1_cdenable(&mut self) -> Ctl1CdenableW<Ctl1Spec> {
        Ctl1CdenableW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Command/Data Mode Value When CTL1.CDENABLE is 1, CS3 line is used as C/D signal to distinguish between Command (C/D low) and Data (C/D high) information. When a value is written into the CTL1.CDMODE bits, the C/D (CS3) line will go low for the given numbers of byte which are sent by the SPI, starting with the next value to be transmitted after which, C/D line will go high automatically 0: Manual mode with C/D signal as High 1-14: C/D is low while this number of bytes are being sent after which, this field sets to 0 and C/D goes high. Reading this field at any time returns the remaining number of command bytes. 15: Manual mode with C/D signal as Low."]
    #[inline(always)]
    pub fn ctl1_cdmode(&mut self) -> Ctl1CdmodeW<Ctl1Spec> {
        Ctl1CdmodeW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Counter to repeat last transfer 0: repeat last transfer is disabled. x: repeat the last transfer with the given number. The transfer will be started with writing a data into the TX Buffer. Sending the data will be repeated with the given value, so the data will be transferred X+1 times in total. The behavior is identical as if the data would be written into the TX Buffer that many times as defined by the value here. It can be used to clean a transfer or to pull a certain amount of data by a peripheral."]
    #[inline(always)]
    pub fn ctl1_repeattx(&mut self) -> Ctl1RepeattxW<Ctl1Spec> {
        Ctl1RepeattxW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Receive Timeout (only for Peripheral mode) Defines the number of Clock Cycles before after which the Receive Timeout flag RTOUT is set. The time is calculated using the control register for the clock selection and divider in the Controller mode configuration. A value of 0 disables this function."]
    #[inline(always)]
    pub fn ctl1_rxtimeout(&mut self) -> Ctl1RxtimeoutW<Ctl1Spec> {
        Ctl1RxtimeoutW::new(self, 24)
    }
}
#[doc = "SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0x04"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x04;
}
