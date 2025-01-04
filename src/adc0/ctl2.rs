#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Data read-back format. Data is always stored in binary unsigned format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Df {
    #[doc = "0: UNSIGNED"]
    Ctl2DfUnsigned = 0,
    #[doc = "1: SIGNED"]
    Ctl2DfSigned = 1,
}
impl From<Ctl2Df> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Df) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_DF` reader - Data read-back format. Data is always stored in binary unsigned format."]
pub type Ctl2DfR = crate::BitReader<Ctl2Df>;
impl Ctl2DfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Df {
        match self.bits {
            false => Ctl2Df::Ctl2DfUnsigned,
            true => Ctl2Df::Ctl2DfSigned,
        }
    }
    #[doc = "UNSIGNED"]
    #[inline(always)]
    pub fn is_ctl2_df_unsigned(&self) -> bool {
        *self == Ctl2Df::Ctl2DfUnsigned
    }
    #[doc = "SIGNED"]
    #[inline(always)]
    pub fn is_ctl2_df_signed(&self) -> bool {
        *self == Ctl2Df::Ctl2DfSigned
    }
}
#[doc = "Field `CTL2_DF` writer - Data read-back format. Data is always stored in binary unsigned format."]
pub type Ctl2DfW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Df>;
impl<'a, REG> Ctl2DfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UNSIGNED"]
    #[inline(always)]
    pub fn ctl2_df_unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Df::Ctl2DfUnsigned)
    }
    #[doc = "SIGNED"]
    #[inline(always)]
    pub fn ctl2_df_signed(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Df::Ctl2DfSigned)
    }
}
#[doc = "Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Res {
    #[doc = "0: BIT_12"]
    Ctl2ResBit12 = 0,
    #[doc = "1: BIT_10"]
    Ctl2ResBit10 = 1,
    #[doc = "2: BIT_8"]
    Ctl2ResBit8 = 2,
}
impl From<Ctl2Res> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Res {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Res {}
#[doc = "Field `CTL2_RES` reader - Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
pub type Ctl2ResR = crate::FieldReader<Ctl2Res>;
impl Ctl2ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl2Res> {
        match self.bits {
            0 => Some(Ctl2Res::Ctl2ResBit12),
            1 => Some(Ctl2Res::Ctl2ResBit10),
            2 => Some(Ctl2Res::Ctl2ResBit8),
            _ => None,
        }
    }
    #[doc = "BIT_12"]
    #[inline(always)]
    pub fn is_ctl2_res_bit_12(&self) -> bool {
        *self == Ctl2Res::Ctl2ResBit12
    }
    #[doc = "BIT_10"]
    #[inline(always)]
    pub fn is_ctl2_res_bit_10(&self) -> bool {
        *self == Ctl2Res::Ctl2ResBit10
    }
    #[doc = "BIT_8"]
    #[inline(always)]
    pub fn is_ctl2_res_bit_8(&self) -> bool {
        *self == Ctl2Res::Ctl2ResBit8
    }
}
#[doc = "Field `CTL2_RES` writer - Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
pub type Ctl2ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl2Res>;
impl<'a, REG> Ctl2ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BIT_12"]
    #[inline(always)]
    pub fn ctl2_res_bit_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Res::Ctl2ResBit12)
    }
    #[doc = "BIT_10"]
    #[inline(always)]
    pub fn ctl2_res_bit_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Res::Ctl2ResBit10)
    }
    #[doc = "BIT_8"]
    #[inline(always)]
    pub fn ctl2_res_bit_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Res::Ctl2ResBit8)
    }
}
#[doc = "Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Dmaen {
    #[doc = "0: DISABLE"]
    Ctl2DmaenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl2DmaenEnable = 1,
}
impl From<Ctl2Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_DMAEN` reader - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
pub type Ctl2DmaenR = crate::BitReader<Ctl2Dmaen>;
impl Ctl2DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Dmaen {
        match self.bits {
            false => Ctl2Dmaen::Ctl2DmaenDisable,
            true => Ctl2Dmaen::Ctl2DmaenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl2_dmaen_disable(&self) -> bool {
        *self == Ctl2Dmaen::Ctl2DmaenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl2_dmaen_enable(&self) -> bool {
        *self == Ctl2Dmaen::Ctl2DmaenEnable
    }
}
#[doc = "Field `CTL2_DMAEN` writer - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
pub type Ctl2DmaenW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Dmaen>;
impl<'a, REG> Ctl2DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl2_dmaen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dmaen::Ctl2DmaenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl2_dmaen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dmaen::Ctl2DmaenEnable)
    }
}
#[doc = "Enable FIFO based operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Fifoen {
    #[doc = "0: DISABLE"]
    Ctl2FifoenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl2FifoenEnable = 1,
}
impl From<Ctl2Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_FIFOEN` reader - Enable FIFO based operation"]
pub type Ctl2FifoenR = crate::BitReader<Ctl2Fifoen>;
impl Ctl2FifoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Fifoen {
        match self.bits {
            false => Ctl2Fifoen::Ctl2FifoenDisable,
            true => Ctl2Fifoen::Ctl2FifoenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl2_fifoen_disable(&self) -> bool {
        *self == Ctl2Fifoen::Ctl2FifoenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl2_fifoen_enable(&self) -> bool {
        *self == Ctl2Fifoen::Ctl2FifoenEnable
    }
}
#[doc = "Field `CTL2_FIFOEN` writer - Enable FIFO based operation"]
pub type Ctl2FifoenW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Fifoen>;
impl<'a, REG> Ctl2FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl2_fifoen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoen::Ctl2FifoenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl2_fifoen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoen::Ctl2FifoenEnable)
    }
}
#[doc = "Number of ADC converted samples to be transferred on a DMA trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Sampcnt {
    #[doc = "0: MIN"]
    Ctl2SampcntMin = 0,
    #[doc = "24: MAX"]
    Ctl2SampcntMax = 24,
}
impl From<Ctl2Sampcnt> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Sampcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Sampcnt {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Sampcnt {}
#[doc = "Field `CTL2_SAMPCNT` reader - Number of ADC converted samples to be transferred on a DMA trigger"]
pub type Ctl2SampcntR = crate::FieldReader<Ctl2Sampcnt>;
impl Ctl2SampcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl2Sampcnt> {
        match self.bits {
            0 => Some(Ctl2Sampcnt::Ctl2SampcntMin),
            24 => Some(Ctl2Sampcnt::Ctl2SampcntMax),
            _ => None,
        }
    }
    #[doc = "MIN"]
    #[inline(always)]
    pub fn is_ctl2_sampcnt_min(&self) -> bool {
        *self == Ctl2Sampcnt::Ctl2SampcntMin
    }
    #[doc = "MAX"]
    #[inline(always)]
    pub fn is_ctl2_sampcnt_max(&self) -> bool {
        *self == Ctl2Sampcnt::Ctl2SampcntMax
    }
}
#[doc = "Field `CTL2_SAMPCNT` writer - Number of ADC converted samples to be transferred on a DMA trigger"]
pub type Ctl2SampcntW<'a, REG> = crate::FieldWriter<'a, REG, 5, Ctl2Sampcnt>;
impl<'a, REG> Ctl2SampcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MIN"]
    #[inline(always)]
    pub fn ctl2_sampcnt_min(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Sampcnt::Ctl2SampcntMin)
    }
    #[doc = "MAX"]
    #[inline(always)]
    pub fn ctl2_sampcnt_max(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Sampcnt::Ctl2SampcntMax)
    }
}
#[doc = "Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Startadd {
    #[doc = "0: ADDR_00"]
    Ctl2StartaddAddr00 = 0,
    #[doc = "1: ADDR_01"]
    Ctl2StartaddAddr01 = 1,
    #[doc = "2: ADDR_02"]
    Ctl2StartaddAddr02 = 2,
    #[doc = "3: ADDR_03"]
    Ctl2StartaddAddr03 = 3,
    #[doc = "4: ADDR_04"]
    Ctl2StartaddAddr04 = 4,
    #[doc = "5: ADDR_05"]
    Ctl2StartaddAddr05 = 5,
    #[doc = "6: ADDR_06"]
    Ctl2StartaddAddr06 = 6,
    #[doc = "7: ADDR_07"]
    Ctl2StartaddAddr07 = 7,
    #[doc = "8: ADDR_08"]
    Ctl2StartaddAddr08 = 8,
    #[doc = "9: ADDR_09"]
    Ctl2StartaddAddr09 = 9,
    #[doc = "10: ADDR_10"]
    Ctl2StartaddAddr10 = 10,
    #[doc = "11: ADDR_11"]
    Ctl2StartaddAddr11 = 11,
    #[doc = "12: ADDR_12"]
    Ctl2StartaddAddr12 = 12,
    #[doc = "13: ADDR_13"]
    Ctl2StartaddAddr13 = 13,
    #[doc = "14: ADDR_14"]
    Ctl2StartaddAddr14 = 14,
    #[doc = "15: ADDR_15"]
    Ctl2StartaddAddr15 = 15,
    #[doc = "16: ADDR_16"]
    Ctl2StartaddAddr16 = 16,
    #[doc = "17: ADDR_17"]
    Ctl2StartaddAddr17 = 17,
    #[doc = "18: ADDR_18"]
    Ctl2StartaddAddr18 = 18,
    #[doc = "19: ADDR_19"]
    Ctl2StartaddAddr19 = 19,
    #[doc = "20: ADDR_20"]
    Ctl2StartaddAddr20 = 20,
    #[doc = "21: ADDR_21"]
    Ctl2StartaddAddr21 = 21,
    #[doc = "22: ADDR_22"]
    Ctl2StartaddAddr22 = 22,
    #[doc = "23: ADDR_23"]
    Ctl2StartaddAddr23 = 23,
}
impl From<Ctl2Startadd> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Startadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Startadd {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Startadd {}
#[doc = "Field `CTL2_STARTADD` reader - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type Ctl2StartaddR = crate::FieldReader<Ctl2Startadd>;
impl Ctl2StartaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl2Startadd> {
        match self.bits {
            0 => Some(Ctl2Startadd::Ctl2StartaddAddr00),
            1 => Some(Ctl2Startadd::Ctl2StartaddAddr01),
            2 => Some(Ctl2Startadd::Ctl2StartaddAddr02),
            3 => Some(Ctl2Startadd::Ctl2StartaddAddr03),
            4 => Some(Ctl2Startadd::Ctl2StartaddAddr04),
            5 => Some(Ctl2Startadd::Ctl2StartaddAddr05),
            6 => Some(Ctl2Startadd::Ctl2StartaddAddr06),
            7 => Some(Ctl2Startadd::Ctl2StartaddAddr07),
            8 => Some(Ctl2Startadd::Ctl2StartaddAddr08),
            9 => Some(Ctl2Startadd::Ctl2StartaddAddr09),
            10 => Some(Ctl2Startadd::Ctl2StartaddAddr10),
            11 => Some(Ctl2Startadd::Ctl2StartaddAddr11),
            12 => Some(Ctl2Startadd::Ctl2StartaddAddr12),
            13 => Some(Ctl2Startadd::Ctl2StartaddAddr13),
            14 => Some(Ctl2Startadd::Ctl2StartaddAddr14),
            15 => Some(Ctl2Startadd::Ctl2StartaddAddr15),
            16 => Some(Ctl2Startadd::Ctl2StartaddAddr16),
            17 => Some(Ctl2Startadd::Ctl2StartaddAddr17),
            18 => Some(Ctl2Startadd::Ctl2StartaddAddr18),
            19 => Some(Ctl2Startadd::Ctl2StartaddAddr19),
            20 => Some(Ctl2Startadd::Ctl2StartaddAddr20),
            21 => Some(Ctl2Startadd::Ctl2StartaddAddr21),
            22 => Some(Ctl2Startadd::Ctl2StartaddAddr22),
            23 => Some(Ctl2Startadd::Ctl2StartaddAddr23),
            _ => None,
        }
    }
    #[doc = "ADDR_00"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_00(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr00
    }
    #[doc = "ADDR_01"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_01(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr01
    }
    #[doc = "ADDR_02"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_02(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr02
    }
    #[doc = "ADDR_03"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_03(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr03
    }
    #[doc = "ADDR_04"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_04(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr04
    }
    #[doc = "ADDR_05"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_05(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr05
    }
    #[doc = "ADDR_06"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_06(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr06
    }
    #[doc = "ADDR_07"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_07(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr07
    }
    #[doc = "ADDR_08"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_08(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr08
    }
    #[doc = "ADDR_09"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_09(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr09
    }
    #[doc = "ADDR_10"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_10(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr10
    }
    #[doc = "ADDR_11"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_11(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr11
    }
    #[doc = "ADDR_12"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_12(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr12
    }
    #[doc = "ADDR_13"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_13(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr13
    }
    #[doc = "ADDR_14"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_14(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr14
    }
    #[doc = "ADDR_15"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_15(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr15
    }
    #[doc = "ADDR_16"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_16(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr16
    }
    #[doc = "ADDR_17"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_17(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr17
    }
    #[doc = "ADDR_18"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_18(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr18
    }
    #[doc = "ADDR_19"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_19(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr19
    }
    #[doc = "ADDR_20"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_20(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr20
    }
    #[doc = "ADDR_21"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_21(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr21
    }
    #[doc = "ADDR_22"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_22(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr22
    }
    #[doc = "ADDR_23"]
    #[inline(always)]
    pub fn is_ctl2_startadd_addr_23(&self) -> bool {
        *self == Ctl2Startadd::Ctl2StartaddAddr23
    }
}
#[doc = "Field `CTL2_STARTADD` writer - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type Ctl2StartaddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Ctl2Startadd>;
impl<'a, REG> Ctl2StartaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADDR_00"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr00)
    }
    #[doc = "ADDR_01"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr01)
    }
    #[doc = "ADDR_02"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_02(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr02)
    }
    #[doc = "ADDR_03"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_03(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr03)
    }
    #[doc = "ADDR_04"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_04(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr04)
    }
    #[doc = "ADDR_05"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_05(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr05)
    }
    #[doc = "ADDR_06"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_06(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr06)
    }
    #[doc = "ADDR_07"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_07(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr07)
    }
    #[doc = "ADDR_08"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_08(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr08)
    }
    #[doc = "ADDR_09"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_09(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr09)
    }
    #[doc = "ADDR_10"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr10)
    }
    #[doc = "ADDR_11"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr11)
    }
    #[doc = "ADDR_12"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr12)
    }
    #[doc = "ADDR_13"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr13)
    }
    #[doc = "ADDR_14"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr14)
    }
    #[doc = "ADDR_15"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr15)
    }
    #[doc = "ADDR_16"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr16)
    }
    #[doc = "ADDR_17"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_17(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr17)
    }
    #[doc = "ADDR_18"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_18(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr18)
    }
    #[doc = "ADDR_19"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_19(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr19)
    }
    #[doc = "ADDR_20"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_20(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr20)
    }
    #[doc = "ADDR_21"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_21(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr21)
    }
    #[doc = "ADDR_22"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_22(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr22)
    }
    #[doc = "ADDR_23"]
    #[inline(always)]
    pub fn ctl2_startadd_addr_23(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Startadd::Ctl2StartaddAddr23)
    }
}
#[doc = "Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Endadd {
    #[doc = "0: ADDR_00"]
    Ctl2EndaddAddr00 = 0,
    #[doc = "1: ADDR_01"]
    Ctl2EndaddAddr01 = 1,
    #[doc = "2: ADDR_02"]
    Ctl2EndaddAddr02 = 2,
    #[doc = "3: ADDR_03"]
    Ctl2EndaddAddr03 = 3,
    #[doc = "4: ADDR_04"]
    Ctl2EndaddAddr04 = 4,
    #[doc = "5: ADDR_05"]
    Ctl2EndaddAddr05 = 5,
    #[doc = "6: ADDR_06"]
    Ctl2EndaddAddr06 = 6,
    #[doc = "7: ADDR_07"]
    Ctl2EndaddAddr07 = 7,
    #[doc = "8: ADDR_08"]
    Ctl2EndaddAddr08 = 8,
    #[doc = "9: ADDR_09"]
    Ctl2EndaddAddr09 = 9,
    #[doc = "10: ADDR_10"]
    Ctl2EndaddAddr10 = 10,
    #[doc = "11: ADDR_11"]
    Ctl2EndaddAddr11 = 11,
    #[doc = "12: ADDR_12"]
    Ctl2EndaddAddr12 = 12,
    #[doc = "13: ADDR_13"]
    Ctl2EndaddAddr13 = 13,
    #[doc = "14: ADDR_14"]
    Ctl2EndaddAddr14 = 14,
    #[doc = "15: ADDR_15"]
    Ctl2EndaddAddr15 = 15,
    #[doc = "16: ADDR_16"]
    Ctl2EndaddAddr16 = 16,
    #[doc = "17: ADDR_17"]
    Ctl2EndaddAddr17 = 17,
    #[doc = "18: ADDR_18"]
    Ctl2EndaddAddr18 = 18,
    #[doc = "19: ADDR_19"]
    Ctl2EndaddAddr19 = 19,
    #[doc = "20: ADDR_20"]
    Ctl2EndaddAddr20 = 20,
    #[doc = "21: ADDR_21"]
    Ctl2EndaddAddr21 = 21,
    #[doc = "22: ADDR_22"]
    Ctl2EndaddAddr22 = 22,
    #[doc = "23: ADDR_23"]
    Ctl2EndaddAddr23 = 23,
}
impl From<Ctl2Endadd> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Endadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Endadd {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Endadd {}
#[doc = "Field `CTL2_ENDADD` reader - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type Ctl2EndaddR = crate::FieldReader<Ctl2Endadd>;
impl Ctl2EndaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl2Endadd> {
        match self.bits {
            0 => Some(Ctl2Endadd::Ctl2EndaddAddr00),
            1 => Some(Ctl2Endadd::Ctl2EndaddAddr01),
            2 => Some(Ctl2Endadd::Ctl2EndaddAddr02),
            3 => Some(Ctl2Endadd::Ctl2EndaddAddr03),
            4 => Some(Ctl2Endadd::Ctl2EndaddAddr04),
            5 => Some(Ctl2Endadd::Ctl2EndaddAddr05),
            6 => Some(Ctl2Endadd::Ctl2EndaddAddr06),
            7 => Some(Ctl2Endadd::Ctl2EndaddAddr07),
            8 => Some(Ctl2Endadd::Ctl2EndaddAddr08),
            9 => Some(Ctl2Endadd::Ctl2EndaddAddr09),
            10 => Some(Ctl2Endadd::Ctl2EndaddAddr10),
            11 => Some(Ctl2Endadd::Ctl2EndaddAddr11),
            12 => Some(Ctl2Endadd::Ctl2EndaddAddr12),
            13 => Some(Ctl2Endadd::Ctl2EndaddAddr13),
            14 => Some(Ctl2Endadd::Ctl2EndaddAddr14),
            15 => Some(Ctl2Endadd::Ctl2EndaddAddr15),
            16 => Some(Ctl2Endadd::Ctl2EndaddAddr16),
            17 => Some(Ctl2Endadd::Ctl2EndaddAddr17),
            18 => Some(Ctl2Endadd::Ctl2EndaddAddr18),
            19 => Some(Ctl2Endadd::Ctl2EndaddAddr19),
            20 => Some(Ctl2Endadd::Ctl2EndaddAddr20),
            21 => Some(Ctl2Endadd::Ctl2EndaddAddr21),
            22 => Some(Ctl2Endadd::Ctl2EndaddAddr22),
            23 => Some(Ctl2Endadd::Ctl2EndaddAddr23),
            _ => None,
        }
    }
    #[doc = "ADDR_00"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_00(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr00
    }
    #[doc = "ADDR_01"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_01(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr01
    }
    #[doc = "ADDR_02"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_02(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr02
    }
    #[doc = "ADDR_03"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_03(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr03
    }
    #[doc = "ADDR_04"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_04(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr04
    }
    #[doc = "ADDR_05"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_05(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr05
    }
    #[doc = "ADDR_06"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_06(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr06
    }
    #[doc = "ADDR_07"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_07(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr07
    }
    #[doc = "ADDR_08"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_08(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr08
    }
    #[doc = "ADDR_09"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_09(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr09
    }
    #[doc = "ADDR_10"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_10(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr10
    }
    #[doc = "ADDR_11"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_11(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr11
    }
    #[doc = "ADDR_12"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_12(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr12
    }
    #[doc = "ADDR_13"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_13(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr13
    }
    #[doc = "ADDR_14"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_14(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr14
    }
    #[doc = "ADDR_15"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_15(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr15
    }
    #[doc = "ADDR_16"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_16(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr16
    }
    #[doc = "ADDR_17"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_17(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr17
    }
    #[doc = "ADDR_18"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_18(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr18
    }
    #[doc = "ADDR_19"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_19(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr19
    }
    #[doc = "ADDR_20"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_20(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr20
    }
    #[doc = "ADDR_21"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_21(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr21
    }
    #[doc = "ADDR_22"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_22(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr22
    }
    #[doc = "ADDR_23"]
    #[inline(always)]
    pub fn is_ctl2_endadd_addr_23(&self) -> bool {
        *self == Ctl2Endadd::Ctl2EndaddAddr23
    }
}
#[doc = "Field `CTL2_ENDADD` writer - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type Ctl2EndaddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Ctl2Endadd>;
impl<'a, REG> Ctl2EndaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADDR_00"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_00(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr00)
    }
    #[doc = "ADDR_01"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_01(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr01)
    }
    #[doc = "ADDR_02"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_02(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr02)
    }
    #[doc = "ADDR_03"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_03(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr03)
    }
    #[doc = "ADDR_04"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_04(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr04)
    }
    #[doc = "ADDR_05"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_05(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr05)
    }
    #[doc = "ADDR_06"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_06(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr06)
    }
    #[doc = "ADDR_07"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_07(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr07)
    }
    #[doc = "ADDR_08"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_08(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr08)
    }
    #[doc = "ADDR_09"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_09(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr09)
    }
    #[doc = "ADDR_10"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr10)
    }
    #[doc = "ADDR_11"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr11)
    }
    #[doc = "ADDR_12"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr12)
    }
    #[doc = "ADDR_13"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr13)
    }
    #[doc = "ADDR_14"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr14)
    }
    #[doc = "ADDR_15"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr15)
    }
    #[doc = "ADDR_16"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr16)
    }
    #[doc = "ADDR_17"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_17(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr17)
    }
    #[doc = "ADDR_18"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_18(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr18)
    }
    #[doc = "ADDR_19"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_19(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr19)
    }
    #[doc = "ADDR_20"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_20(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr20)
    }
    #[doc = "ADDR_21"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_21(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr21)
    }
    #[doc = "ADDR_22"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_22(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr22)
    }
    #[doc = "ADDR_23"]
    #[inline(always)]
    pub fn ctl2_endadd_addr_23(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Endadd::Ctl2EndaddAddr23)
    }
}
impl R {
    #[doc = "Bit 0 - Data read-back format. Data is always stored in binary unsigned format."]
    #[inline(always)]
    pub fn ctl2_df(&self) -> Ctl2DfR {
        Ctl2DfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
    #[inline(always)]
    pub fn ctl2_res(&self) -> Ctl2ResR {
        Ctl2ResR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
    #[inline(always)]
    pub fn ctl2_dmaen(&self) -> Ctl2DmaenR {
        Ctl2DmaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable FIFO based operation"]
    #[inline(always)]
    pub fn ctl2_fifoen(&self) -> Ctl2FifoenR {
        Ctl2FifoenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - Number of ADC converted samples to be transferred on a DMA trigger"]
    #[inline(always)]
    pub fn ctl2_sampcnt(&self) -> Ctl2SampcntR {
        Ctl2SampcntR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn ctl2_startadd(&self) -> Ctl2StartaddR {
        Ctl2StartaddR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn ctl2_endadd(&self) -> Ctl2EndaddR {
        Ctl2EndaddR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data read-back format. Data is always stored in binary unsigned format."]
    #[inline(always)]
    pub fn ctl2_df(&mut self) -> Ctl2DfW<Ctl2Spec> {
        Ctl2DfW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Resolution. These bits define the resolutoin of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
    #[inline(always)]
    pub fn ctl2_res(&mut self) -> Ctl2ResW<Ctl2Spec> {
        Ctl2ResW::new(self, 1)
    }
    #[doc = "Bit 8 - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
    #[inline(always)]
    pub fn ctl2_dmaen(&mut self) -> Ctl2DmaenW<Ctl2Spec> {
        Ctl2DmaenW::new(self, 8)
    }
    #[doc = "Bit 10 - Enable FIFO based operation"]
    #[inline(always)]
    pub fn ctl2_fifoen(&mut self) -> Ctl2FifoenW<Ctl2Spec> {
        Ctl2FifoenW::new(self, 10)
    }
    #[doc = "Bits 11:15 - Number of ADC converted samples to be transferred on a DMA trigger"]
    #[inline(always)]
    pub fn ctl2_sampcnt(&mut self) -> Ctl2SampcntW<Ctl2Spec> {
        Ctl2SampcntW::new(self, 11)
    }
    #[doc = "Bits 16:20 - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn ctl2_startadd(&mut self) -> Ctl2StartaddW<Ctl2Spec> {
        Ctl2StartaddW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn ctl2_endadd(&mut self) -> Ctl2EndaddW<Ctl2Spec> {
        Ctl2EndaddW::new(self, 24)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
