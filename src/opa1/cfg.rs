#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Chopping enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgChop {
    #[doc = "0: OFF"]
    CfgChopOff = 0,
    #[doc = "1: ON"]
    CfgChopOn = 1,
    #[doc = "2: AVGON"]
    CfgChopAvgon = 2,
}
impl From<CfgChop> for u8 {
    #[inline(always)]
    fn from(variant: CfgChop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgChop {
    type Ux = u8;
}
impl crate::IsEnum for CfgChop {}
#[doc = "Field `CFG_CHOP` reader - Chopping enable."]
pub type CfgChopR = crate::FieldReader<CfgChop>;
impl CfgChopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgChop> {
        match self.bits {
            0 => Some(CfgChop::CfgChopOff),
            1 => Some(CfgChop::CfgChopOn),
            2 => Some(CfgChop::CfgChopAvgon),
            _ => None,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_cfg_chop_off(&self) -> bool {
        *self == CfgChop::CfgChopOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_cfg_chop_on(&self) -> bool {
        *self == CfgChop::CfgChopOn
    }
    #[doc = "AVGON"]
    #[inline(always)]
    pub fn is_cfg_chop_avgon(&self) -> bool {
        *self == CfgChop::CfgChopAvgon
    }
}
#[doc = "Field `CFG_CHOP` writer - Chopping enable."]
pub type CfgChopW<'a, REG> = crate::FieldWriter<'a, REG, 2, CfgChop>;
impl<'a, REG> CfgChopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn cfg_chop_off(self) -> &'a mut crate::W<REG> {
        self.variant(CfgChop::CfgChopOff)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn cfg_chop_on(self) -> &'a mut crate::W<REG> {
        self.variant(CfgChop::CfgChopOn)
    }
    #[doc = "AVGON"]
    #[inline(always)]
    pub fn cfg_chop_avgon(self) -> &'a mut crate::W<REG> {
        self.variant(CfgChop::CfgChopAvgon)
    }
}
#[doc = "Enable output pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CfgOutpin {
    #[doc = "0: DISABLED"]
    CfgOutpinDisabled = 0,
    #[doc = "1: ENABLED"]
    CfgOutpinEnabled = 1,
}
impl From<CfgOutpin> for bool {
    #[inline(always)]
    fn from(variant: CfgOutpin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFG_OUTPIN` reader - Enable output pin"]
pub type CfgOutpinR = crate::BitReader<CfgOutpin>;
impl CfgOutpinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CfgOutpin {
        match self.bits {
            false => CfgOutpin::CfgOutpinDisabled,
            true => CfgOutpin::CfgOutpinEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_cfg_outpin_disabled(&self) -> bool {
        *self == CfgOutpin::CfgOutpinDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_cfg_outpin_enabled(&self) -> bool {
        *self == CfgOutpin::CfgOutpinEnabled
    }
}
#[doc = "Field `CFG_OUTPIN` writer - Enable output pin"]
pub type CfgOutpinW<'a, REG> = crate::BitWriter<'a, REG, CfgOutpin>;
impl<'a, REG> CfgOutpinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn cfg_outpin_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CfgOutpin::CfgOutpinDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn cfg_outpin_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CfgOutpin::CfgOutpinEnabled)
    }
}
#[doc = "Positive OA input selection. Please refer to the device specific datasheet for exact channels available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgPsel {
    #[doc = "0: NC"]
    CfgPselNc = 0,
    #[doc = "1: EXTPIN0"]
    CfgPselExtpin0 = 1,
    #[doc = "2: EXTPIN1"]
    CfgPselExtpin1 = 2,
    #[doc = "3: DAC12OUT"]
    CfgPselDac12out = 3,
    #[doc = "4: DAC8OUT"]
    CfgPselDac8out = 4,
    #[doc = "5: VREF"]
    CfgPselVref = 5,
    #[doc = "6: OANM1RTOP"]
    CfgPselOanm1rtop = 6,
    #[doc = "7: GPAMP_OUT_INT"]
    CfgPselGpampOutInt = 7,
    #[doc = "8: VSS"]
    CfgPselVss = 8,
}
impl From<CfgPsel> for u8 {
    #[inline(always)]
    fn from(variant: CfgPsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgPsel {
    type Ux = u8;
}
impl crate::IsEnum for CfgPsel {}
#[doc = "Field `CFG_PSEL` reader - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."]
pub type CfgPselR = crate::FieldReader<CfgPsel>;
impl CfgPselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgPsel> {
        match self.bits {
            0 => Some(CfgPsel::CfgPselNc),
            1 => Some(CfgPsel::CfgPselExtpin0),
            2 => Some(CfgPsel::CfgPselExtpin1),
            3 => Some(CfgPsel::CfgPselDac12out),
            4 => Some(CfgPsel::CfgPselDac8out),
            5 => Some(CfgPsel::CfgPselVref),
            6 => Some(CfgPsel::CfgPselOanm1rtop),
            7 => Some(CfgPsel::CfgPselGpampOutInt),
            8 => Some(CfgPsel::CfgPselVss),
            _ => None,
        }
    }
    #[doc = "NC"]
    #[inline(always)]
    pub fn is_cfg_psel_nc(&self) -> bool {
        *self == CfgPsel::CfgPselNc
    }
    #[doc = "EXTPIN0"]
    #[inline(always)]
    pub fn is_cfg_psel_extpin0(&self) -> bool {
        *self == CfgPsel::CfgPselExtpin0
    }
    #[doc = "EXTPIN1"]
    #[inline(always)]
    pub fn is_cfg_psel_extpin1(&self) -> bool {
        *self == CfgPsel::CfgPselExtpin1
    }
    #[doc = "DAC12OUT"]
    #[inline(always)]
    pub fn is_cfg_psel_dac12out(&self) -> bool {
        *self == CfgPsel::CfgPselDac12out
    }
    #[doc = "DAC8OUT"]
    #[inline(always)]
    pub fn is_cfg_psel_dac8out(&self) -> bool {
        *self == CfgPsel::CfgPselDac8out
    }
    #[doc = "VREF"]
    #[inline(always)]
    pub fn is_cfg_psel_vref(&self) -> bool {
        *self == CfgPsel::CfgPselVref
    }
    #[doc = "OANM1RTOP"]
    #[inline(always)]
    pub fn is_cfg_psel_oanm1rtop(&self) -> bool {
        *self == CfgPsel::CfgPselOanm1rtop
    }
    #[doc = "GPAMP_OUT_INT"]
    #[inline(always)]
    pub fn is_cfg_psel_gpamp_out_int(&self) -> bool {
        *self == CfgPsel::CfgPselGpampOutInt
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn is_cfg_psel_vss(&self) -> bool {
        *self == CfgPsel::CfgPselVss
    }
}
#[doc = "Field `CFG_PSEL` writer - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."]
pub type CfgPselW<'a, REG> = crate::FieldWriter<'a, REG, 4, CfgPsel>;
impl<'a, REG> CfgPselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NC"]
    #[inline(always)]
    pub fn cfg_psel_nc(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselNc)
    }
    #[doc = "EXTPIN0"]
    #[inline(always)]
    pub fn cfg_psel_extpin0(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselExtpin0)
    }
    #[doc = "EXTPIN1"]
    #[inline(always)]
    pub fn cfg_psel_extpin1(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselExtpin1)
    }
    #[doc = "DAC12OUT"]
    #[inline(always)]
    pub fn cfg_psel_dac12out(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselDac12out)
    }
    #[doc = "DAC8OUT"]
    #[inline(always)]
    pub fn cfg_psel_dac8out(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselDac8out)
    }
    #[doc = "VREF"]
    #[inline(always)]
    pub fn cfg_psel_vref(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselVref)
    }
    #[doc = "OANM1RTOP"]
    #[inline(always)]
    pub fn cfg_psel_oanm1rtop(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselOanm1rtop)
    }
    #[doc = "GPAMP_OUT_INT"]
    #[inline(always)]
    pub fn cfg_psel_gpamp_out_int(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselGpampOutInt)
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn cfg_psel_vss(self) -> &'a mut crate::W<REG> {
        self.variant(CfgPsel::CfgPselVss)
    }
}
#[doc = "Negative OA input selection. Please refer to the device specific datasheet for exact channels available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgNsel {
    #[doc = "0: NC"]
    CfgNselNc = 0,
    #[doc = "1: EXTPIN0"]
    CfgNselExtpin0 = 1,
    #[doc = "2: EXTPIN1"]
    CfgNselExtpin1 = 2,
    #[doc = "3: OANP1RBOT"]
    CfgNselOanp1rbot = 3,
    #[doc = "4: OANRTAP"]
    CfgNselOanrtap = 4,
    #[doc = "5: OANRTOP"]
    CfgNselOanrtop = 5,
    #[doc = "6: SPARE"]
    CfgNselSpare = 6,
}
impl From<CfgNsel> for u8 {
    #[inline(always)]
    fn from(variant: CfgNsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgNsel {
    type Ux = u8;
}
impl crate::IsEnum for CfgNsel {}
#[doc = "Field `CFG_NSEL` reader - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."]
pub type CfgNselR = crate::FieldReader<CfgNsel>;
impl CfgNselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgNsel> {
        match self.bits {
            0 => Some(CfgNsel::CfgNselNc),
            1 => Some(CfgNsel::CfgNselExtpin0),
            2 => Some(CfgNsel::CfgNselExtpin1),
            3 => Some(CfgNsel::CfgNselOanp1rbot),
            4 => Some(CfgNsel::CfgNselOanrtap),
            5 => Some(CfgNsel::CfgNselOanrtop),
            6 => Some(CfgNsel::CfgNselSpare),
            _ => None,
        }
    }
    #[doc = "NC"]
    #[inline(always)]
    pub fn is_cfg_nsel_nc(&self) -> bool {
        *self == CfgNsel::CfgNselNc
    }
    #[doc = "EXTPIN0"]
    #[inline(always)]
    pub fn is_cfg_nsel_extpin0(&self) -> bool {
        *self == CfgNsel::CfgNselExtpin0
    }
    #[doc = "EXTPIN1"]
    #[inline(always)]
    pub fn is_cfg_nsel_extpin1(&self) -> bool {
        *self == CfgNsel::CfgNselExtpin1
    }
    #[doc = "OANP1RBOT"]
    #[inline(always)]
    pub fn is_cfg_nsel_oanp1rbot(&self) -> bool {
        *self == CfgNsel::CfgNselOanp1rbot
    }
    #[doc = "OANRTAP"]
    #[inline(always)]
    pub fn is_cfg_nsel_oanrtap(&self) -> bool {
        *self == CfgNsel::CfgNselOanrtap
    }
    #[doc = "OANRTOP"]
    #[inline(always)]
    pub fn is_cfg_nsel_oanrtop(&self) -> bool {
        *self == CfgNsel::CfgNselOanrtop
    }
    #[doc = "SPARE"]
    #[inline(always)]
    pub fn is_cfg_nsel_spare(&self) -> bool {
        *self == CfgNsel::CfgNselSpare
    }
}
#[doc = "Field `CFG_NSEL` writer - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."]
pub type CfgNselW<'a, REG> = crate::FieldWriter<'a, REG, 3, CfgNsel>;
impl<'a, REG> CfgNselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NC"]
    #[inline(always)]
    pub fn cfg_nsel_nc(self) -> &'a mut crate::W<REG> {
        self.variant(CfgNsel::CfgNselNc)
    }
    #[doc = "EXTPIN0"]
    #[inline(always)]
    pub fn cfg_nsel_extpin0(self) -> &'a mut crate::W<REG> {
        self.variant(CfgNsel::CfgNselExtpin0)
    }
    #[doc = "EXTPIN1"]
    #[inline(always)]
    pub fn cfg_nsel_extpin1(self) -> &'a mut crate::W<REG> {
        self.variant(CfgNsel::CfgNselExtpin1)
    }
    #[doc = "OANP1RBOT"]
    #[inline(always)]
    pub fn cfg_nsel_oanp1rbot(self) -> &'a mut crate::W<REG> {
        self.variant(CfgNsel::CfgNselOanp1rbot)
    }
    #[doc = "OANRTAP"]
    #[inline(always)]
    pub fn cfg_nsel_oanrtap(self) -> &'a mut crate::W<REG> {
        self.variant(CfgNsel::CfgNselOanrtap)
    }
    #[doc = "OANRTOP"]
    #[inline(always)]
    pub fn cfg_nsel_oanrtop(self) -> &'a mut crate::W<REG> {
        self.variant(CfgNsel::CfgNselOanrtop)
    }
    #[doc = "SPARE"]
    #[inline(always)]
    pub fn cfg_nsel_spare(self) -> &'a mut crate::W<REG> {
        self.variant(CfgNsel::CfgNselSpare)
    }
}
#[doc = "MSEL Mux selection. Please refer to the device specific datasheet for exact channels available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CfgMsel {
    #[doc = "0: NC"]
    CfgMselNc = 0,
    #[doc = "1: EXTNPIN1"]
    CfgMselExtnpin1 = 1,
    #[doc = "2: VSS"]
    CfgMselVss = 2,
    #[doc = "3: DAC12OUT"]
    CfgMselDac12out = 3,
    #[doc = "4: OANM1RTOP"]
    CfgMselOanm1rtop = 4,
}
impl From<CfgMsel> for u8 {
    #[inline(always)]
    fn from(variant: CfgMsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CfgMsel {
    type Ux = u8;
}
impl crate::IsEnum for CfgMsel {}
#[doc = "Field `CFG_MSEL` reader - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."]
pub type CfgMselR = crate::FieldReader<CfgMsel>;
impl CfgMselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfgMsel> {
        match self.bits {
            0 => Some(CfgMsel::CfgMselNc),
            1 => Some(CfgMsel::CfgMselExtnpin1),
            2 => Some(CfgMsel::CfgMselVss),
            3 => Some(CfgMsel::CfgMselDac12out),
            4 => Some(CfgMsel::CfgMselOanm1rtop),
            _ => None,
        }
    }
    #[doc = "NC"]
    #[inline(always)]
    pub fn is_cfg_msel_nc(&self) -> bool {
        *self == CfgMsel::CfgMselNc
    }
    #[doc = "EXTNPIN1"]
    #[inline(always)]
    pub fn is_cfg_msel_extnpin1(&self) -> bool {
        *self == CfgMsel::CfgMselExtnpin1
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn is_cfg_msel_vss(&self) -> bool {
        *self == CfgMsel::CfgMselVss
    }
    #[doc = "DAC12OUT"]
    #[inline(always)]
    pub fn is_cfg_msel_dac12out(&self) -> bool {
        *self == CfgMsel::CfgMselDac12out
    }
    #[doc = "OANM1RTOP"]
    #[inline(always)]
    pub fn is_cfg_msel_oanm1rtop(&self) -> bool {
        *self == CfgMsel::CfgMselOanm1rtop
    }
}
#[doc = "Field `CFG_MSEL` writer - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."]
pub type CfgMselW<'a, REG> = crate::FieldWriter<'a, REG, 3, CfgMsel>;
impl<'a, REG> CfgMselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NC"]
    #[inline(always)]
    pub fn cfg_msel_nc(self) -> &'a mut crate::W<REG> {
        self.variant(CfgMsel::CfgMselNc)
    }
    #[doc = "EXTNPIN1"]
    #[inline(always)]
    pub fn cfg_msel_extnpin1(self) -> &'a mut crate::W<REG> {
        self.variant(CfgMsel::CfgMselExtnpin1)
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn cfg_msel_vss(self) -> &'a mut crate::W<REG> {
        self.variant(CfgMsel::CfgMselVss)
    }
    #[doc = "DAC12OUT"]
    #[inline(always)]
    pub fn cfg_msel_dac12out(self) -> &'a mut crate::W<REG> {
        self.variant(CfgMsel::CfgMselDac12out)
    }
    #[doc = "OANM1RTOP"]
    #[inline(always)]
    pub fn cfg_msel_oanm1rtop(self) -> &'a mut crate::W<REG> {
        self.variant(CfgMsel::CfgMselOanm1rtop)
    }
}
#[doc = "Field `CFG_GAIN` reader - Gain setting. Refer to TRM for enumeration information."]
pub type CfgGainR = crate::FieldReader;
#[doc = "Field `CFG_GAIN` writer - Gain setting. Refer to TRM for enumeration information."]
pub type CfgGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Chopping enable."]
    #[inline(always)]
    pub fn cfg_chop(&self) -> CfgChopR {
        CfgChopR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable output pin"]
    #[inline(always)]
    pub fn cfg_outpin(&self) -> CfgOutpinR {
        CfgOutpinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."]
    #[inline(always)]
    pub fn cfg_psel(&self) -> CfgPselR {
        CfgPselR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:9 - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."]
    #[inline(always)]
    pub fn cfg_nsel(&self) -> CfgNselR {
        CfgNselR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."]
    #[inline(always)]
    pub fn cfg_msel(&self) -> CfgMselR {
        CfgMselR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Gain setting. Refer to TRM for enumeration information."]
    #[inline(always)]
    pub fn cfg_gain(&self) -> CfgGainR {
        CfgGainR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Chopping enable."]
    #[inline(always)]
    pub fn cfg_chop(&mut self) -> CfgChopW<CfgSpec> {
        CfgChopW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable output pin"]
    #[inline(always)]
    pub fn cfg_outpin(&mut self) -> CfgOutpinW<CfgSpec> {
        CfgOutpinW::new(self, 2)
    }
    #[doc = "Bits 3:6 - Positive OA input selection. Please refer to the device specific datasheet for exact channels available."]
    #[inline(always)]
    pub fn cfg_psel(&mut self) -> CfgPselW<CfgSpec> {
        CfgPselW::new(self, 3)
    }
    #[doc = "Bits 7:9 - Negative OA input selection. Please refer to the device specific datasheet for exact channels available."]
    #[inline(always)]
    pub fn cfg_nsel(&mut self) -> CfgNselW<CfgSpec> {
        CfgNselW::new(self, 7)
    }
    #[doc = "Bits 10:12 - MSEL Mux selection. Please refer to the device specific datasheet for exact channels available."]
    #[inline(always)]
    pub fn cfg_msel(&mut self) -> CfgMselW<CfgSpec> {
        CfgMselW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Gain setting. Refer to TRM for enumeration information."]
    #[inline(always)]
    pub fn cfg_gain(&mut self) -> CfgGainW<CfgSpec> {
        CfgGainW::new(self, 13)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
