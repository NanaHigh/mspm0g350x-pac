#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &gt; 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Refmode {
    #[doc = "0: STATIC"]
    Ctl2RefmodeStatic = 0,
    #[doc = "1: SAMPLED"]
    Ctl2RefmodeSampled = 1,
}
impl From<Ctl2Refmode> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Refmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_REFMODE` reader - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &gt; 0."]
pub type Ctl2RefmodeR = crate::BitReader<Ctl2Refmode>;
impl Ctl2RefmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Refmode {
        match self.bits {
            false => Ctl2Refmode::Ctl2RefmodeStatic,
            true => Ctl2Refmode::Ctl2RefmodeSampled,
        }
    }
    #[doc = "STATIC"]
    #[inline(always)]
    pub fn is_ctl2_refmode_static(&self) -> bool {
        *self == Ctl2Refmode::Ctl2RefmodeStatic
    }
    #[doc = "SAMPLED"]
    #[inline(always)]
    pub fn is_ctl2_refmode_sampled(&self) -> bool {
        *self == Ctl2Refmode::Ctl2RefmodeSampled
    }
}
#[doc = "Field `CTL2_REFMODE` writer - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &gt; 0."]
pub type Ctl2RefmodeW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Refmode>;
impl<'a, REG> Ctl2RefmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STATIC"]
    #[inline(always)]
    pub fn ctl2_refmode_static(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refmode::Ctl2RefmodeStatic)
    }
    #[doc = "SAMPLED"]
    #[inline(always)]
    pub fn ctl2_refmode_sampled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refmode::Ctl2RefmodeSampled)
    }
}
#[doc = "These bits select the reference source for the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Refsrc {
    #[doc = "0: OFF"]
    Ctl2RefsrcOff = 0,
    #[doc = "1: VDDA_DAC"]
    Ctl2RefsrcVddaDac = 1,
    #[doc = "2: VREF_DAC"]
    Ctl2RefsrcVrefDac = 2,
    #[doc = "3: VREF"]
    Ctl2RefsrcVref = 3,
}
impl From<Ctl2Refsrc> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Refsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Refsrc {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Refsrc {}
#[doc = "Field `CTL2_REFSRC` reader - These bits select the reference source for the comparator."]
pub type Ctl2RefsrcR = crate::FieldReader<Ctl2Refsrc>;
impl Ctl2RefsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Refsrc {
        match self.bits {
            0 => Ctl2Refsrc::Ctl2RefsrcOff,
            1 => Ctl2Refsrc::Ctl2RefsrcVddaDac,
            2 => Ctl2Refsrc::Ctl2RefsrcVrefDac,
            3 => Ctl2Refsrc::Ctl2RefsrcVref,
            _ => unreachable!(),
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_ctl2_refsrc_off(&self) -> bool {
        *self == Ctl2Refsrc::Ctl2RefsrcOff
    }
    #[doc = "VDDA_DAC"]
    #[inline(always)]
    pub fn is_ctl2_refsrc_vdda_dac(&self) -> bool {
        *self == Ctl2Refsrc::Ctl2RefsrcVddaDac
    }
    #[doc = "VREF_DAC"]
    #[inline(always)]
    pub fn is_ctl2_refsrc_vref_dac(&self) -> bool {
        *self == Ctl2Refsrc::Ctl2RefsrcVrefDac
    }
    #[doc = "VREF"]
    #[inline(always)]
    pub fn is_ctl2_refsrc_vref(&self) -> bool {
        *self == Ctl2Refsrc::Ctl2RefsrcVref
    }
}
#[doc = "Field `CTL2_REFSRC` writer - These bits select the reference source for the comparator."]
pub type Ctl2RefsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl2Refsrc, crate::Safe>;
impl<'a, REG> Ctl2RefsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn ctl2_refsrc_off(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refsrc::Ctl2RefsrcOff)
    }
    #[doc = "VDDA_DAC"]
    #[inline(always)]
    pub fn ctl2_refsrc_vdda_dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refsrc::Ctl2RefsrcVddaDac)
    }
    #[doc = "VREF_DAC"]
    #[inline(always)]
    pub fn ctl2_refsrc_vref_dac(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refsrc::Ctl2RefsrcVrefDac)
    }
    #[doc = "VREF"]
    #[inline(always)]
    pub fn ctl2_refsrc_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refsrc::Ctl2RefsrcVref)
    }
}
#[doc = "This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Refsel {
    #[doc = "0: POSITIVE"]
    Ctl2RefselPositive = 0,
    #[doc = "1: NEGATIVE"]
    Ctl2RefselNegative = 1,
}
impl From<Ctl2Refsel> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Refsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_REFSEL` reader - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
pub type Ctl2RefselR = crate::BitReader<Ctl2Refsel>;
impl Ctl2RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Refsel {
        match self.bits {
            false => Ctl2Refsel::Ctl2RefselPositive,
            true => Ctl2Refsel::Ctl2RefselNegative,
        }
    }
    #[doc = "POSITIVE"]
    #[inline(always)]
    pub fn is_ctl2_refsel_positive(&self) -> bool {
        *self == Ctl2Refsel::Ctl2RefselPositive
    }
    #[doc = "NEGATIVE"]
    #[inline(always)]
    pub fn is_ctl2_refsel_negative(&self) -> bool {
        *self == Ctl2Refsel::Ctl2RefselNegative
    }
}
#[doc = "Field `CTL2_REFSEL` writer - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
pub type Ctl2RefselW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Refsel>;
impl<'a, REG> Ctl2RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "POSITIVE"]
    #[inline(always)]
    pub fn ctl2_refsel_positive(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refsel::Ctl2RefselPositive)
    }
    #[doc = "NEGATIVE"]
    #[inline(always)]
    pub fn ctl2_refsel_negative(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Refsel::Ctl2RefselNegative)
    }
}
#[doc = "These bits select the blanking source for the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Blanksrc {
    #[doc = "0: DISABLE"]
    Ctl2BlanksrcDisable = 0,
    #[doc = "1: BLANKSRC1"]
    Ctl2BlanksrcBlanksrc1 = 1,
    #[doc = "2: BLANKSRC2"]
    Ctl2BlanksrcBlanksrc2 = 2,
    #[doc = "3: BLANKSRC3"]
    Ctl2BlanksrcBlanksrc3 = 3,
    #[doc = "4: BLANKSRC4"]
    Ctl2BlanksrcBlanksrc4 = 4,
    #[doc = "5: BLANKSRC5"]
    Ctl2BlanksrcBlanksrc5 = 5,
    #[doc = "6: BLANKSRC6"]
    Ctl2BlanksrcBlanksrc6 = 6,
}
impl From<Ctl2Blanksrc> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Blanksrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Blanksrc {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Blanksrc {}
#[doc = "Field `CTL2_BLANKSRC` reader - These bits select the blanking source for the comparator."]
pub type Ctl2BlanksrcR = crate::FieldReader<Ctl2Blanksrc>;
impl Ctl2BlanksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl2Blanksrc> {
        match self.bits {
            0 => Some(Ctl2Blanksrc::Ctl2BlanksrcDisable),
            1 => Some(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc1),
            2 => Some(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc2),
            3 => Some(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc3),
            4 => Some(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc4),
            5 => Some(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc5),
            6 => Some(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc6),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl2_blanksrc_disable(&self) -> bool {
        *self == Ctl2Blanksrc::Ctl2BlanksrcDisable
    }
    #[doc = "BLANKSRC1"]
    #[inline(always)]
    pub fn is_ctl2_blanksrc_blanksrc1(&self) -> bool {
        *self == Ctl2Blanksrc::Ctl2BlanksrcBlanksrc1
    }
    #[doc = "BLANKSRC2"]
    #[inline(always)]
    pub fn is_ctl2_blanksrc_blanksrc2(&self) -> bool {
        *self == Ctl2Blanksrc::Ctl2BlanksrcBlanksrc2
    }
    #[doc = "BLANKSRC3"]
    #[inline(always)]
    pub fn is_ctl2_blanksrc_blanksrc3(&self) -> bool {
        *self == Ctl2Blanksrc::Ctl2BlanksrcBlanksrc3
    }
    #[doc = "BLANKSRC4"]
    #[inline(always)]
    pub fn is_ctl2_blanksrc_blanksrc4(&self) -> bool {
        *self == Ctl2Blanksrc::Ctl2BlanksrcBlanksrc4
    }
    #[doc = "BLANKSRC5"]
    #[inline(always)]
    pub fn is_ctl2_blanksrc_blanksrc5(&self) -> bool {
        *self == Ctl2Blanksrc::Ctl2BlanksrcBlanksrc5
    }
    #[doc = "BLANKSRC6"]
    #[inline(always)]
    pub fn is_ctl2_blanksrc_blanksrc6(&self) -> bool {
        *self == Ctl2Blanksrc::Ctl2BlanksrcBlanksrc6
    }
}
#[doc = "Field `CTL2_BLANKSRC` writer - These bits select the blanking source for the comparator."]
pub type Ctl2BlanksrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctl2Blanksrc>;
impl<'a, REG> Ctl2BlanksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl2_blanksrc_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Blanksrc::Ctl2BlanksrcDisable)
    }
    #[doc = "BLANKSRC1"]
    #[inline(always)]
    pub fn ctl2_blanksrc_blanksrc1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc1)
    }
    #[doc = "BLANKSRC2"]
    #[inline(always)]
    pub fn ctl2_blanksrc_blanksrc2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc2)
    }
    #[doc = "BLANKSRC3"]
    #[inline(always)]
    pub fn ctl2_blanksrc_blanksrc3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc3)
    }
    #[doc = "BLANKSRC4"]
    #[inline(always)]
    pub fn ctl2_blanksrc_blanksrc4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc4)
    }
    #[doc = "BLANKSRC5"]
    #[inline(always)]
    pub fn ctl2_blanksrc_blanksrc5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc5)
    }
    #[doc = "BLANKSRC6"]
    #[inline(always)]
    pub fn ctl2_blanksrc_blanksrc6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Blanksrc::Ctl2BlanksrcBlanksrc6)
    }
}
#[doc = "This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Dacctl {
    #[doc = "0: COMPOUT_SEL"]
    Ctl2DacctlCompoutSel = 0,
    #[doc = "1: DACSW_SEL"]
    Ctl2DacctlDacswSel = 1,
}
impl From<Ctl2Dacctl> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Dacctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_DACCTL` reader - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
pub type Ctl2DacctlR = crate::BitReader<Ctl2Dacctl>;
impl Ctl2DacctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Dacctl {
        match self.bits {
            false => Ctl2Dacctl::Ctl2DacctlCompoutSel,
            true => Ctl2Dacctl::Ctl2DacctlDacswSel,
        }
    }
    #[doc = "COMPOUT_SEL"]
    #[inline(always)]
    pub fn is_ctl2_dacctl_compout_sel(&self) -> bool {
        *self == Ctl2Dacctl::Ctl2DacctlCompoutSel
    }
    #[doc = "DACSW_SEL"]
    #[inline(always)]
    pub fn is_ctl2_dacctl_dacsw_sel(&self) -> bool {
        *self == Ctl2Dacctl::Ctl2DacctlDacswSel
    }
}
#[doc = "Field `CTL2_DACCTL` writer - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
pub type Ctl2DacctlW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Dacctl>;
impl<'a, REG> Ctl2DacctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMPOUT_SEL"]
    #[inline(always)]
    pub fn ctl2_dacctl_compout_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dacctl::Ctl2DacctlCompoutSel)
    }
    #[doc = "DACSW_SEL"]
    #[inline(always)]
    pub fn ctl2_dacctl_dacsw_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dacctl::Ctl2DacctlDacswSel)
    }
}
#[doc = "This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Dacsw {
    #[doc = "0: DACCODE0_SEL"]
    Ctl2DacswDaccode0Sel = 0,
    #[doc = "1: DACCODE1_SEL"]
    Ctl2DacswDaccode1Sel = 1,
}
impl From<Ctl2Dacsw> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Dacsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_DACSW` reader - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
pub type Ctl2DacswR = crate::BitReader<Ctl2Dacsw>;
impl Ctl2DacswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Dacsw {
        match self.bits {
            false => Ctl2Dacsw::Ctl2DacswDaccode0Sel,
            true => Ctl2Dacsw::Ctl2DacswDaccode1Sel,
        }
    }
    #[doc = "DACCODE0_SEL"]
    #[inline(always)]
    pub fn is_ctl2_dacsw_daccode0_sel(&self) -> bool {
        *self == Ctl2Dacsw::Ctl2DacswDaccode0Sel
    }
    #[doc = "DACCODE1_SEL"]
    #[inline(always)]
    pub fn is_ctl2_dacsw_daccode1_sel(&self) -> bool {
        *self == Ctl2Dacsw::Ctl2DacswDaccode1Sel
    }
}
#[doc = "Field `CTL2_DACSW` writer - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
pub type Ctl2DacswW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Dacsw>;
impl<'a, REG> Ctl2DacswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DACCODE0_SEL"]
    #[inline(always)]
    pub fn ctl2_dacsw_daccode0_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dacsw::Ctl2DacswDaccode0Sel)
    }
    #[doc = "DACCODE1_SEL"]
    #[inline(always)]
    pub fn ctl2_dacsw_daccode1_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dacsw::Ctl2DacswDaccode1Sel)
    }
}
#[doc = "Enable sampled mode of comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Sampmode {
    #[doc = "0: DISABLE"]
    Ctl2SampmodeDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl2SampmodeEnable = 1,
}
impl From<Ctl2Sampmode> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Sampmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_SAMPMODE` reader - Enable sampled mode of comparator."]
pub type Ctl2SampmodeR = crate::BitReader<Ctl2Sampmode>;
impl Ctl2SampmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Sampmode {
        match self.bits {
            false => Ctl2Sampmode::Ctl2SampmodeDisable,
            true => Ctl2Sampmode::Ctl2SampmodeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl2_sampmode_disable(&self) -> bool {
        *self == Ctl2Sampmode::Ctl2SampmodeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl2_sampmode_enable(&self) -> bool {
        *self == Ctl2Sampmode::Ctl2SampmodeEnable
    }
}
#[doc = "Field `CTL2_SAMPMODE` writer - Enable sampled mode of comparator."]
pub type Ctl2SampmodeW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Sampmode>;
impl<'a, REG> Ctl2SampmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl2_sampmode_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Sampmode::Ctl2SampmodeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl2_sampmode_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Sampmode::Ctl2SampmodeEnable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &gt; 0."]
    #[inline(always)]
    pub fn ctl2_refmode(&self) -> Ctl2RefmodeR {
        Ctl2RefmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - These bits select the reference source for the comparator."]
    #[inline(always)]
    pub fn ctl2_refsrc(&self) -> Ctl2RefsrcR {
        Ctl2RefsrcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 7 - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
    #[inline(always)]
    pub fn ctl2_refsel(&self) -> Ctl2RefselR {
        Ctl2RefselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - These bits select the blanking source for the comparator."]
    #[inline(always)]
    pub fn ctl2_blanksrc(&self) -> Ctl2BlanksrcR {
        Ctl2BlanksrcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
    #[inline(always)]
    pub fn ctl2_dacctl(&self) -> Ctl2DacctlR {
        Ctl2DacctlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
    #[inline(always)]
    pub fn ctl2_dacsw(&self) -> Ctl2DacswR {
        Ctl2DacswR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable sampled mode of comparator."]
    #[inline(always)]
    pub fn ctl2_sampmode(&self) -> Ctl2SampmodeR {
        Ctl2SampmodeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &gt; 0."]
    #[inline(always)]
    pub fn ctl2_refmode(&mut self) -> Ctl2RefmodeW<Ctl2Spec> {
        Ctl2RefmodeW::new(self, 0)
    }
    #[doc = "Bits 3:4 - These bits select the reference source for the comparator."]
    #[inline(always)]
    pub fn ctl2_refsrc(&mut self) -> Ctl2RefsrcW<Ctl2Spec> {
        Ctl2RefsrcW::new(self, 3)
    }
    #[doc = "Bit 7 - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."]
    #[inline(always)]
    pub fn ctl2_refsel(&mut self) -> Ctl2RefselW<Ctl2Spec> {
        Ctl2RefselW::new(self, 7)
    }
    #[doc = "Bits 8:10 - These bits select the blanking source for the comparator."]
    #[inline(always)]
    pub fn ctl2_blanksrc(&mut self) -> Ctl2BlanksrcW<Ctl2Spec> {
        Ctl2BlanksrcW::new(self, 8)
    }
    #[doc = "Bit 16 - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."]
    #[inline(always)]
    pub fn ctl2_dacctl(&mut self) -> Ctl2DacctlW<Ctl2Spec> {
        Ctl2DacctlW::new(self, 16)
    }
    #[doc = "Bit 17 - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."]
    #[inline(always)]
    pub fn ctl2_dacsw(&mut self) -> Ctl2DacswW<Ctl2Spec> {
        Ctl2DacswW::new(self, 17)
    }
    #[doc = "Bit 24 - Enable sampled mode of comparator."]
    #[inline(always)]
    pub fn ctl2_sampmode(&mut self) -> Ctl2SampmodeW<Ctl2Spec> {
        Ctl2SampmodeW::new(self, 24)
    }
}
#[doc = "Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0;
}
