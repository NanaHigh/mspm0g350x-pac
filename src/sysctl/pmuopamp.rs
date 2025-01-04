#[doc = "Register `PMUOPAMP` reader"]
pub type R = crate::R<PmuopampSpec>;
#[doc = "Register `PMUOPAMP` writer"]
pub type W = crate::W<PmuopampSpec>;
#[doc = "Set ENABLE to turn on the GPAMP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuopampEnable {
    #[doc = "0: FALSE"]
    PmuopampEnableFalse = 0,
    #[doc = "1: TRUE"]
    PmuopampEnableTrue = 1,
}
impl From<PmuopampEnable> for bool {
    #[inline(always)]
    fn from(variant: PmuopampEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUOPAMP_ENABLE` reader - Set ENABLE to turn on the GPAMP."]
pub type PmuopampEnableR = crate::BitReader<PmuopampEnable>;
impl PmuopampEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuopampEnable {
        match self.bits {
            false => PmuopampEnable::PmuopampEnableFalse,
            true => PmuopampEnable::PmuopampEnableTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_pmuopamp_enable_false(&self) -> bool {
        *self == PmuopampEnable::PmuopampEnableFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_pmuopamp_enable_true(&self) -> bool {
        *self == PmuopampEnable::PmuopampEnableTrue
    }
}
#[doc = "Field `PMUOPAMP_ENABLE` writer - Set ENABLE to turn on the GPAMP."]
pub type PmuopampEnableW<'a, REG> = crate::BitWriter<'a, REG, PmuopampEnable>;
impl<'a, REG> PmuopampEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn pmuopamp_enable_false(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampEnable::PmuopampEnableFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn pmuopamp_enable_true(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampEnable::PmuopampEnableTrue)
    }
}
#[doc = "Set PCHENABLE to enable the positive channel input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuopampPchenable {
    #[doc = "0: FALSE"]
    PmuopampPchenableFalse = 0,
    #[doc = "1: TRUE"]
    PmuopampPchenableTrue = 1,
}
impl From<PmuopampPchenable> for bool {
    #[inline(always)]
    fn from(variant: PmuopampPchenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUOPAMP_PCHENABLE` reader - Set PCHENABLE to enable the positive channel input."]
pub type PmuopampPchenableR = crate::BitReader<PmuopampPchenable>;
impl PmuopampPchenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuopampPchenable {
        match self.bits {
            false => PmuopampPchenable::PmuopampPchenableFalse,
            true => PmuopampPchenable::PmuopampPchenableTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_pmuopamp_pchenable_false(&self) -> bool {
        *self == PmuopampPchenable::PmuopampPchenableFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_pmuopamp_pchenable_true(&self) -> bool {
        *self == PmuopampPchenable::PmuopampPchenableTrue
    }
}
#[doc = "Field `PMUOPAMP_PCHENABLE` writer - Set PCHENABLE to enable the positive channel input."]
pub type PmuopampPchenableW<'a, REG> = crate::BitWriter<'a, REG, PmuopampPchenable>;
impl<'a, REG> PmuopampPchenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn pmuopamp_pchenable_false(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampPchenable::PmuopampPchenableFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn pmuopamp_pchenable_true(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampPchenable::PmuopampPchenableTrue)
    }
}
#[doc = "NSEL selects the GPAMP negative channel input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmuopampNsel {
    #[doc = "0: SEL0"]
    PmuopampNselSel0 = 0,
    #[doc = "1: SEL1"]
    PmuopampNselSel1 = 1,
    #[doc = "2: SEL2"]
    PmuopampNselSel2 = 2,
    #[doc = "3: SEL3"]
    PmuopampNselSel3 = 3,
}
impl From<PmuopampNsel> for u8 {
    #[inline(always)]
    fn from(variant: PmuopampNsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmuopampNsel {
    type Ux = u8;
}
impl crate::IsEnum for PmuopampNsel {}
#[doc = "Field `PMUOPAMP_NSEL` reader - NSEL selects the GPAMP negative channel input."]
pub type PmuopampNselR = crate::FieldReader<PmuopampNsel>;
impl PmuopampNselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuopampNsel {
        match self.bits {
            0 => PmuopampNsel::PmuopampNselSel0,
            1 => PmuopampNsel::PmuopampNselSel1,
            2 => PmuopampNsel::PmuopampNselSel2,
            3 => PmuopampNsel::PmuopampNselSel3,
            _ => unreachable!(),
        }
    }
    #[doc = "SEL0"]
    #[inline(always)]
    pub fn is_pmuopamp_nsel_sel0(&self) -> bool {
        *self == PmuopampNsel::PmuopampNselSel0
    }
    #[doc = "SEL1"]
    #[inline(always)]
    pub fn is_pmuopamp_nsel_sel1(&self) -> bool {
        *self == PmuopampNsel::PmuopampNselSel1
    }
    #[doc = "SEL2"]
    #[inline(always)]
    pub fn is_pmuopamp_nsel_sel2(&self) -> bool {
        *self == PmuopampNsel::PmuopampNselSel2
    }
    #[doc = "SEL3"]
    #[inline(always)]
    pub fn is_pmuopamp_nsel_sel3(&self) -> bool {
        *self == PmuopampNsel::PmuopampNselSel3
    }
}
#[doc = "Field `PMUOPAMP_NSEL` writer - NSEL selects the GPAMP negative channel input."]
pub type PmuopampNselW<'a, REG> = crate::FieldWriter<'a, REG, 2, PmuopampNsel, crate::Safe>;
impl<'a, REG> PmuopampNselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SEL0"]
    #[inline(always)]
    pub fn pmuopamp_nsel_sel0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampNsel::PmuopampNselSel0)
    }
    #[doc = "SEL1"]
    #[inline(always)]
    pub fn pmuopamp_nsel_sel1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampNsel::PmuopampNselSel1)
    }
    #[doc = "SEL2"]
    #[inline(always)]
    pub fn pmuopamp_nsel_sel2(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampNsel::PmuopampNselSel2)
    }
    #[doc = "SEL3"]
    #[inline(always)]
    pub fn pmuopamp_nsel_sel3(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampNsel::PmuopampNselSel3)
    }
}
#[doc = "RRI selects the rail-to-rail input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmuopampRri {
    #[doc = "0: MODE0"]
    PmuopampRriMode0 = 0,
    #[doc = "1: MODE1"]
    PmuopampRriMode1 = 1,
    #[doc = "2: MODE2"]
    PmuopampRriMode2 = 2,
    #[doc = "3: MODE3"]
    PmuopampRriMode3 = 3,
}
impl From<PmuopampRri> for u8 {
    #[inline(always)]
    fn from(variant: PmuopampRri) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmuopampRri {
    type Ux = u8;
}
impl crate::IsEnum for PmuopampRri {}
#[doc = "Field `PMUOPAMP_RRI` reader - RRI selects the rail-to-rail input mode."]
pub type PmuopampRriR = crate::FieldReader<PmuopampRri>;
impl PmuopampRriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuopampRri {
        match self.bits {
            0 => PmuopampRri::PmuopampRriMode0,
            1 => PmuopampRri::PmuopampRriMode1,
            2 => PmuopampRri::PmuopampRriMode2,
            3 => PmuopampRri::PmuopampRriMode3,
            _ => unreachable!(),
        }
    }
    #[doc = "MODE0"]
    #[inline(always)]
    pub fn is_pmuopamp_rri_mode0(&self) -> bool {
        *self == PmuopampRri::PmuopampRriMode0
    }
    #[doc = "MODE1"]
    #[inline(always)]
    pub fn is_pmuopamp_rri_mode1(&self) -> bool {
        *self == PmuopampRri::PmuopampRriMode1
    }
    #[doc = "MODE2"]
    #[inline(always)]
    pub fn is_pmuopamp_rri_mode2(&self) -> bool {
        *self == PmuopampRri::PmuopampRriMode2
    }
    #[doc = "MODE3"]
    #[inline(always)]
    pub fn is_pmuopamp_rri_mode3(&self) -> bool {
        *self == PmuopampRri::PmuopampRriMode3
    }
}
#[doc = "Field `PMUOPAMP_RRI` writer - RRI selects the rail-to-rail input mode."]
pub type PmuopampRriW<'a, REG> = crate::FieldWriter<'a, REG, 2, PmuopampRri, crate::Safe>;
impl<'a, REG> PmuopampRriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MODE0"]
    #[inline(always)]
    pub fn pmuopamp_rri_mode0(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampRri::PmuopampRriMode0)
    }
    #[doc = "MODE1"]
    #[inline(always)]
    pub fn pmuopamp_rri_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampRri::PmuopampRriMode1)
    }
    #[doc = "MODE2"]
    #[inline(always)]
    pub fn pmuopamp_rri_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampRri::PmuopampRriMode2)
    }
    #[doc = "MODE3"]
    #[inline(always)]
    pub fn pmuopamp_rri_mode3(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampRri::PmuopampRriMode3)
    }
}
#[doc = "Set OUTENABLE to connect the GPAMP output signal to the GPAMP_OUT pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuopampOutenable {
    #[doc = "0: FALSE"]
    PmuopampOutenableFalse = 0,
    #[doc = "1: TRUE"]
    PmuopampOutenableTrue = 1,
}
impl From<PmuopampOutenable> for bool {
    #[inline(always)]
    fn from(variant: PmuopampOutenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMUOPAMP_OUTENABLE` reader - Set OUTENABLE to connect the GPAMP output signal to the GPAMP_OUT pin"]
pub type PmuopampOutenableR = crate::BitReader<PmuopampOutenable>;
impl PmuopampOutenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuopampOutenable {
        match self.bits {
            false => PmuopampOutenable::PmuopampOutenableFalse,
            true => PmuopampOutenable::PmuopampOutenableTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_pmuopamp_outenable_false(&self) -> bool {
        *self == PmuopampOutenable::PmuopampOutenableFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_pmuopamp_outenable_true(&self) -> bool {
        *self == PmuopampOutenable::PmuopampOutenableTrue
    }
}
#[doc = "Field `PMUOPAMP_OUTENABLE` writer - Set OUTENABLE to connect the GPAMP output signal to the GPAMP_OUT pin"]
pub type PmuopampOutenableW<'a, REG> = crate::BitWriter<'a, REG, PmuopampOutenable>;
impl<'a, REG> PmuopampOutenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn pmuopamp_outenable_false(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampOutenable::PmuopampOutenableFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn pmuopamp_outenable_true(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampOutenable::PmuopampOutenableTrue)
    }
}
#[doc = "CHOPCLKFREQ selects the GPAMP chopping clock frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmuopampChopclkfreq {
    #[doc = "0: CLK16KHZ"]
    PmuopampChopclkfreqClk16khz = 0,
    #[doc = "1: CLK8KHZ"]
    PmuopampChopclkfreqClk8khz = 1,
    #[doc = "2: CLK4KHZ"]
    PmuopampChopclkfreqClk4khz = 2,
    #[doc = "3: CLK2KHZ"]
    PmuopampChopclkfreqClk2khz = 3,
}
impl From<PmuopampChopclkfreq> for u8 {
    #[inline(always)]
    fn from(variant: PmuopampChopclkfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmuopampChopclkfreq {
    type Ux = u8;
}
impl crate::IsEnum for PmuopampChopclkfreq {}
#[doc = "Field `PMUOPAMP_CHOPCLKFREQ` reader - CHOPCLKFREQ selects the GPAMP chopping clock frequency"]
pub type PmuopampChopclkfreqR = crate::FieldReader<PmuopampChopclkfreq>;
impl PmuopampChopclkfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuopampChopclkfreq {
        match self.bits {
            0 => PmuopampChopclkfreq::PmuopampChopclkfreqClk16khz,
            1 => PmuopampChopclkfreq::PmuopampChopclkfreqClk8khz,
            2 => PmuopampChopclkfreq::PmuopampChopclkfreqClk4khz,
            3 => PmuopampChopclkfreq::PmuopampChopclkfreqClk2khz,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK16KHZ"]
    #[inline(always)]
    pub fn is_pmuopamp_chopclkfreq_clk16khz(&self) -> bool {
        *self == PmuopampChopclkfreq::PmuopampChopclkfreqClk16khz
    }
    #[doc = "CLK8KHZ"]
    #[inline(always)]
    pub fn is_pmuopamp_chopclkfreq_clk8khz(&self) -> bool {
        *self == PmuopampChopclkfreq::PmuopampChopclkfreqClk8khz
    }
    #[doc = "CLK4KHZ"]
    #[inline(always)]
    pub fn is_pmuopamp_chopclkfreq_clk4khz(&self) -> bool {
        *self == PmuopampChopclkfreq::PmuopampChopclkfreqClk4khz
    }
    #[doc = "CLK2KHZ"]
    #[inline(always)]
    pub fn is_pmuopamp_chopclkfreq_clk2khz(&self) -> bool {
        *self == PmuopampChopclkfreq::PmuopampChopclkfreqClk2khz
    }
}
#[doc = "Field `PMUOPAMP_CHOPCLKFREQ` writer - CHOPCLKFREQ selects the GPAMP chopping clock frequency"]
pub type PmuopampChopclkfreqW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PmuopampChopclkfreq, crate::Safe>;
impl<'a, REG> PmuopampChopclkfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK16KHZ"]
    #[inline(always)]
    pub fn pmuopamp_chopclkfreq_clk16khz(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampChopclkfreq::PmuopampChopclkfreqClk16khz)
    }
    #[doc = "CLK8KHZ"]
    #[inline(always)]
    pub fn pmuopamp_chopclkfreq_clk8khz(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampChopclkfreq::PmuopampChopclkfreqClk8khz)
    }
    #[doc = "CLK4KHZ"]
    #[inline(always)]
    pub fn pmuopamp_chopclkfreq_clk4khz(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampChopclkfreq::PmuopampChopclkfreqClk4khz)
    }
    #[doc = "CLK2KHZ"]
    #[inline(always)]
    pub fn pmuopamp_chopclkfreq_clk2khz(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampChopclkfreq::PmuopampChopclkfreqClk2khz)
    }
}
#[doc = "CHOPCLKMODE selects the GPAMP chopping mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmuopampChopclkmode {
    #[doc = "0: CHOPDISABLED"]
    PmuopampChopclkmodeChopdisabled = 0,
    #[doc = "1: REGCHOP"]
    PmuopampChopclkmodeRegchop = 1,
    #[doc = "2: ADCASSIST"]
    PmuopampChopclkmodeAdcassist = 2,
}
impl From<PmuopampChopclkmode> for u8 {
    #[inline(always)]
    fn from(variant: PmuopampChopclkmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmuopampChopclkmode {
    type Ux = u8;
}
impl crate::IsEnum for PmuopampChopclkmode {}
#[doc = "Field `PMUOPAMP_CHOPCLKMODE` reader - CHOPCLKMODE selects the GPAMP chopping mode."]
pub type PmuopampChopclkmodeR = crate::FieldReader<PmuopampChopclkmode>;
impl PmuopampChopclkmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PmuopampChopclkmode> {
        match self.bits {
            0 => Some(PmuopampChopclkmode::PmuopampChopclkmodeChopdisabled),
            1 => Some(PmuopampChopclkmode::PmuopampChopclkmodeRegchop),
            2 => Some(PmuopampChopclkmode::PmuopampChopclkmodeAdcassist),
            _ => None,
        }
    }
    #[doc = "CHOPDISABLED"]
    #[inline(always)]
    pub fn is_pmuopamp_chopclkmode_chopdisabled(&self) -> bool {
        *self == PmuopampChopclkmode::PmuopampChopclkmodeChopdisabled
    }
    #[doc = "REGCHOP"]
    #[inline(always)]
    pub fn is_pmuopamp_chopclkmode_regchop(&self) -> bool {
        *self == PmuopampChopclkmode::PmuopampChopclkmodeRegchop
    }
    #[doc = "ADCASSIST"]
    #[inline(always)]
    pub fn is_pmuopamp_chopclkmode_adcassist(&self) -> bool {
        *self == PmuopampChopclkmode::PmuopampChopclkmodeAdcassist
    }
}
#[doc = "Field `PMUOPAMP_CHOPCLKMODE` writer - CHOPCLKMODE selects the GPAMP chopping mode."]
pub type PmuopampChopclkmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PmuopampChopclkmode>;
impl<'a, REG> PmuopampChopclkmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CHOPDISABLED"]
    #[inline(always)]
    pub fn pmuopamp_chopclkmode_chopdisabled(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampChopclkmode::PmuopampChopclkmodeChopdisabled)
    }
    #[doc = "REGCHOP"]
    #[inline(always)]
    pub fn pmuopamp_chopclkmode_regchop(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampChopclkmode::PmuopampChopclkmodeRegchop)
    }
    #[doc = "ADCASSIST"]
    #[inline(always)]
    pub fn pmuopamp_chopclkmode_adcassist(self) -> &'a mut crate::W<REG> {
        self.variant(PmuopampChopclkmode::PmuopampChopclkmodeAdcassist)
    }
}
impl R {
    #[doc = "Bit 0 - Set ENABLE to turn on the GPAMP."]
    #[inline(always)]
    pub fn pmuopamp_enable(&self) -> PmuopampEnableR {
        PmuopampEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set PCHENABLE to enable the positive channel input."]
    #[inline(always)]
    pub fn pmuopamp_pchenable(&self) -> PmuopampPchenableR {
        PmuopampPchenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - NSEL selects the GPAMP negative channel input."]
    #[inline(always)]
    pub fn pmuopamp_nsel(&self) -> PmuopampNselR {
        PmuopampNselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RRI selects the rail-to-rail input mode."]
    #[inline(always)]
    pub fn pmuopamp_rri(&self) -> PmuopampRriR {
        PmuopampRriR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set OUTENABLE to connect the GPAMP output signal to the GPAMP_OUT pin"]
    #[inline(always)]
    pub fn pmuopamp_outenable(&self) -> PmuopampOutenableR {
        PmuopampOutenableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CHOPCLKFREQ selects the GPAMP chopping clock frequency"]
    #[inline(always)]
    pub fn pmuopamp_chopclkfreq(&self) -> PmuopampChopclkfreqR {
        PmuopampChopclkfreqR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CHOPCLKMODE selects the GPAMP chopping mode."]
    #[inline(always)]
    pub fn pmuopamp_chopclkmode(&self) -> PmuopampChopclkmodeR {
        PmuopampChopclkmodeR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set ENABLE to turn on the GPAMP."]
    #[inline(always)]
    pub fn pmuopamp_enable(&mut self) -> PmuopampEnableW<PmuopampSpec> {
        PmuopampEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Set PCHENABLE to enable the positive channel input."]
    #[inline(always)]
    pub fn pmuopamp_pchenable(&mut self) -> PmuopampPchenableW<PmuopampSpec> {
        PmuopampPchenableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - NSEL selects the GPAMP negative channel input."]
    #[inline(always)]
    pub fn pmuopamp_nsel(&mut self) -> PmuopampNselW<PmuopampSpec> {
        PmuopampNselW::new(self, 2)
    }
    #[doc = "Bits 4:5 - RRI selects the rail-to-rail input mode."]
    #[inline(always)]
    pub fn pmuopamp_rri(&mut self) -> PmuopampRriW<PmuopampSpec> {
        PmuopampRriW::new(self, 4)
    }
    #[doc = "Bit 6 - Set OUTENABLE to connect the GPAMP output signal to the GPAMP_OUT pin"]
    #[inline(always)]
    pub fn pmuopamp_outenable(&mut self) -> PmuopampOutenableW<PmuopampSpec> {
        PmuopampOutenableW::new(self, 6)
    }
    #[doc = "Bits 8:9 - CHOPCLKFREQ selects the GPAMP chopping clock frequency"]
    #[inline(always)]
    pub fn pmuopamp_chopclkfreq(&mut self) -> PmuopampChopclkfreqW<PmuopampSpec> {
        PmuopampChopclkfreqW::new(self, 8)
    }
    #[doc = "Bits 10:11 - CHOPCLKMODE selects the GPAMP chopping mode."]
    #[inline(always)]
    pub fn pmuopamp_chopclkmode(&mut self) -> PmuopampChopclkmodeW<PmuopampSpec> {
        PmuopampChopclkmodeW::new(self, 10)
    }
}
#[doc = "GPAMP control\n\nYou can [`read`](crate::Reg::read) this register and get [`pmuopamp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmuopamp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuopampSpec;
impl crate::RegisterSpec for PmuopampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmuopamp::R`](R) reader structure"]
impl crate::Readable for PmuopampSpec {}
#[doc = "`write(|w| ..)` method takes [`pmuopamp::W`](W) writer structure"]
impl crate::Writable for PmuopampSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUOPAMP to value 0"]
impl crate::Resettable for PmuopampSpec {
    const RESET_VALUE: u32 = 0;
}
