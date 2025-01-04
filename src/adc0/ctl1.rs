#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Sample trigger source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Trigsrc {
    #[doc = "0: SOFTWARE"]
    Ctl1TrigsrcSoftware = 0,
    #[doc = "1: EVENT"]
    Ctl1TrigsrcEvent = 1,
}
impl From<Ctl1Trigsrc> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Trigsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_TRIGSRC` reader - Sample trigger source"]
pub type Ctl1TrigsrcR = crate::BitReader<Ctl1Trigsrc>;
impl Ctl1TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Trigsrc {
        match self.bits {
            false => Ctl1Trigsrc::Ctl1TrigsrcSoftware,
            true => Ctl1Trigsrc::Ctl1TrigsrcEvent,
        }
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_ctl1_trigsrc_software(&self) -> bool {
        *self == Ctl1Trigsrc::Ctl1TrigsrcSoftware
    }
    #[doc = "EVENT"]
    #[inline(always)]
    pub fn is_ctl1_trigsrc_event(&self) -> bool {
        *self == Ctl1Trigsrc::Ctl1TrigsrcEvent
    }
}
#[doc = "Field `CTL1_TRIGSRC` writer - Sample trigger source"]
pub type Ctl1TrigsrcW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Trigsrc>;
impl<'a, REG> Ctl1TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn ctl1_trigsrc_software(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Trigsrc::Ctl1TrigsrcSoftware)
    }
    #[doc = "EVENT"]
    #[inline(always)]
    pub fn ctl1_trigsrc_event(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Trigsrc::Ctl1TrigsrcEvent)
    }
}
#[doc = "Start of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Sc {
    #[doc = "0: STOP"]
    Ctl1ScStop = 0,
    #[doc = "1: START"]
    Ctl1ScStart = 1,
}
impl From<Ctl1Sc> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Sc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_SC` reader - Start of conversion"]
pub type Ctl1ScR = crate::BitReader<Ctl1Sc>;
impl Ctl1ScR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Sc {
        match self.bits {
            false => Ctl1Sc::Ctl1ScStop,
            true => Ctl1Sc::Ctl1ScStart,
        }
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub fn is_ctl1_sc_stop(&self) -> bool {
        *self == Ctl1Sc::Ctl1ScStop
    }
    #[doc = "START"]
    #[inline(always)]
    pub fn is_ctl1_sc_start(&self) -> bool {
        *self == Ctl1Sc::Ctl1ScStart
    }
}
#[doc = "Field `CTL1_SC` writer - Start of conversion"]
pub type Ctl1ScW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Sc>;
impl<'a, REG> Ctl1ScW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP"]
    #[inline(always)]
    pub fn ctl1_sc_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Sc::Ctl1ScStop)
    }
    #[doc = "START"]
    #[inline(always)]
    pub fn ctl1_sc_start(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Sc::Ctl1ScStart)
    }
}
#[doc = "Conversion sequence mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl1Conseq {
    #[doc = "0: SINGLE"]
    Ctl1ConseqSingle = 0,
    #[doc = "1: SEQUENCE"]
    Ctl1ConseqSequence = 1,
    #[doc = "2: REPEATSINGLE"]
    Ctl1ConseqRepeatsingle = 2,
    #[doc = "3: REPEATSEQUENCE"]
    Ctl1ConseqRepeatsequence = 3,
}
impl From<Ctl1Conseq> for u8 {
    #[inline(always)]
    fn from(variant: Ctl1Conseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl1Conseq {
    type Ux = u8;
}
impl crate::IsEnum for Ctl1Conseq {}
#[doc = "Field `CTL1_CONSEQ` reader - Conversion sequence mode"]
pub type Ctl1ConseqR = crate::FieldReader<Ctl1Conseq>;
impl Ctl1ConseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Conseq {
        match self.bits {
            0 => Ctl1Conseq::Ctl1ConseqSingle,
            1 => Ctl1Conseq::Ctl1ConseqSequence,
            2 => Ctl1Conseq::Ctl1ConseqRepeatsingle,
            3 => Ctl1Conseq::Ctl1ConseqRepeatsequence,
            _ => unreachable!(),
        }
    }
    #[doc = "SINGLE"]
    #[inline(always)]
    pub fn is_ctl1_conseq_single(&self) -> bool {
        *self == Ctl1Conseq::Ctl1ConseqSingle
    }
    #[doc = "SEQUENCE"]
    #[inline(always)]
    pub fn is_ctl1_conseq_sequence(&self) -> bool {
        *self == Ctl1Conseq::Ctl1ConseqSequence
    }
    #[doc = "REPEATSINGLE"]
    #[inline(always)]
    pub fn is_ctl1_conseq_repeatsingle(&self) -> bool {
        *self == Ctl1Conseq::Ctl1ConseqRepeatsingle
    }
    #[doc = "REPEATSEQUENCE"]
    #[inline(always)]
    pub fn is_ctl1_conseq_repeatsequence(&self) -> bool {
        *self == Ctl1Conseq::Ctl1ConseqRepeatsequence
    }
}
#[doc = "Field `CTL1_CONSEQ` writer - Conversion sequence mode"]
pub type Ctl1ConseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl1Conseq, crate::Safe>;
impl<'a, REG> Ctl1ConseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SINGLE"]
    #[inline(always)]
    pub fn ctl1_conseq_single(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Conseq::Ctl1ConseqSingle)
    }
    #[doc = "SEQUENCE"]
    #[inline(always)]
    pub fn ctl1_conseq_sequence(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Conseq::Ctl1ConseqSequence)
    }
    #[doc = "REPEATSINGLE"]
    #[inline(always)]
    pub fn ctl1_conseq_repeatsingle(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Conseq::Ctl1ConseqRepeatsingle)
    }
    #[doc = "REPEATSEQUENCE"]
    #[inline(always)]
    pub fn ctl1_conseq_repeatsequence(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Conseq::Ctl1ConseqRepeatsequence)
    }
}
#[doc = "Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Sampmode {
    #[doc = "0: AUTO"]
    Ctl1SampmodeAuto = 0,
    #[doc = "1: MANUAL"]
    Ctl1SampmodeManual = 1,
}
impl From<Ctl1Sampmode> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Sampmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_SAMPMODE` reader - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
pub type Ctl1SampmodeR = crate::BitReader<Ctl1Sampmode>;
impl Ctl1SampmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Sampmode {
        match self.bits {
            false => Ctl1Sampmode::Ctl1SampmodeAuto,
            true => Ctl1Sampmode::Ctl1SampmodeManual,
        }
    }
    #[doc = "AUTO"]
    #[inline(always)]
    pub fn is_ctl1_sampmode_auto(&self) -> bool {
        *self == Ctl1Sampmode::Ctl1SampmodeAuto
    }
    #[doc = "MANUAL"]
    #[inline(always)]
    pub fn is_ctl1_sampmode_manual(&self) -> bool {
        *self == Ctl1Sampmode::Ctl1SampmodeManual
    }
}
#[doc = "Field `CTL1_SAMPMODE` writer - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
pub type Ctl1SampmodeW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Sampmode>;
impl<'a, REG> Ctl1SampmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUTO"]
    #[inline(always)]
    pub fn ctl1_sampmode_auto(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Sampmode::Ctl1SampmodeAuto)
    }
    #[doc = "MANUAL"]
    #[inline(always)]
    pub fn ctl1_sampmode_manual(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Sampmode::Ctl1SampmodeManual)
    }
}
#[doc = "Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl1Avgn {
    #[doc = "0: DISABLE"]
    Ctl1AvgnDisable = 0,
    #[doc = "1: AVG_2"]
    Ctl1AvgnAvg2 = 1,
    #[doc = "2: AVG_4"]
    Ctl1AvgnAvg4 = 2,
    #[doc = "3: AVG_8"]
    Ctl1AvgnAvg8 = 3,
    #[doc = "4: AVG_16"]
    Ctl1AvgnAvg16 = 4,
    #[doc = "5: AVG_32"]
    Ctl1AvgnAvg32 = 5,
    #[doc = "6: AVG_64"]
    Ctl1AvgnAvg64 = 6,
    #[doc = "7: AVG_128"]
    Ctl1AvgnAvg128 = 7,
}
impl From<Ctl1Avgn> for u8 {
    #[inline(always)]
    fn from(variant: Ctl1Avgn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl1Avgn {
    type Ux = u8;
}
impl crate::IsEnum for Ctl1Avgn {}
#[doc = "Field `CTL1_AVGN` reader - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
pub type Ctl1AvgnR = crate::FieldReader<Ctl1Avgn>;
impl Ctl1AvgnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Avgn {
        match self.bits {
            0 => Ctl1Avgn::Ctl1AvgnDisable,
            1 => Ctl1Avgn::Ctl1AvgnAvg2,
            2 => Ctl1Avgn::Ctl1AvgnAvg4,
            3 => Ctl1Avgn::Ctl1AvgnAvg8,
            4 => Ctl1Avgn::Ctl1AvgnAvg16,
            5 => Ctl1Avgn::Ctl1AvgnAvg32,
            6 => Ctl1Avgn::Ctl1AvgnAvg64,
            7 => Ctl1Avgn::Ctl1AvgnAvg128,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_avgn_disable(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnDisable
    }
    #[doc = "AVG_2"]
    #[inline(always)]
    pub fn is_ctl1_avgn_avg_2(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnAvg2
    }
    #[doc = "AVG_4"]
    #[inline(always)]
    pub fn is_ctl1_avgn_avg_4(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnAvg4
    }
    #[doc = "AVG_8"]
    #[inline(always)]
    pub fn is_ctl1_avgn_avg_8(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnAvg8
    }
    #[doc = "AVG_16"]
    #[inline(always)]
    pub fn is_ctl1_avgn_avg_16(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnAvg16
    }
    #[doc = "AVG_32"]
    #[inline(always)]
    pub fn is_ctl1_avgn_avg_32(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnAvg32
    }
    #[doc = "AVG_64"]
    #[inline(always)]
    pub fn is_ctl1_avgn_avg_64(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnAvg64
    }
    #[doc = "AVG_128"]
    #[inline(always)]
    pub fn is_ctl1_avgn_avg_128(&self) -> bool {
        *self == Ctl1Avgn::Ctl1AvgnAvg128
    }
}
#[doc = "Field `CTL1_AVGN` writer - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
pub type Ctl1AvgnW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctl1Avgn, crate::Safe>;
impl<'a, REG> Ctl1AvgnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_avgn_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnDisable)
    }
    #[doc = "AVG_2"]
    #[inline(always)]
    pub fn ctl1_avgn_avg_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnAvg2)
    }
    #[doc = "AVG_4"]
    #[inline(always)]
    pub fn ctl1_avgn_avg_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnAvg4)
    }
    #[doc = "AVG_8"]
    #[inline(always)]
    pub fn ctl1_avgn_avg_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnAvg8)
    }
    #[doc = "AVG_16"]
    #[inline(always)]
    pub fn ctl1_avgn_avg_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnAvg16)
    }
    #[doc = "AVG_32"]
    #[inline(always)]
    pub fn ctl1_avgn_avg_32(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnAvg32)
    }
    #[doc = "AVG_64"]
    #[inline(always)]
    pub fn ctl1_avgn_avg_64(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnAvg64)
    }
    #[doc = "AVG_128"]
    #[inline(always)]
    pub fn ctl1_avgn_avg_128(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgn::Ctl1AvgnAvg128)
    }
}
#[doc = "Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropirately result will be truncated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl1Avgd {
    #[doc = "0: SHIFT0"]
    Ctl1AvgdShift0 = 0,
    #[doc = "1: SHIFT1"]
    Ctl1AvgdShift1 = 1,
    #[doc = "2: SHIFT2"]
    Ctl1AvgdShift2 = 2,
    #[doc = "3: SHIFT3"]
    Ctl1AvgdShift3 = 3,
    #[doc = "4: SHIFT4"]
    Ctl1AvgdShift4 = 4,
    #[doc = "5: SHIFT5"]
    Ctl1AvgdShift5 = 5,
    #[doc = "6: SHIFT6"]
    Ctl1AvgdShift6 = 6,
    #[doc = "7: SHIFT7"]
    Ctl1AvgdShift7 = 7,
}
impl From<Ctl1Avgd> for u8 {
    #[inline(always)]
    fn from(variant: Ctl1Avgd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl1Avgd {
    type Ux = u8;
}
impl crate::IsEnum for Ctl1Avgd {}
#[doc = "Field `CTL1_AVGD` reader - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropirately result will be truncated."]
pub type Ctl1AvgdR = crate::FieldReader<Ctl1Avgd>;
impl Ctl1AvgdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Avgd {
        match self.bits {
            0 => Ctl1Avgd::Ctl1AvgdShift0,
            1 => Ctl1Avgd::Ctl1AvgdShift1,
            2 => Ctl1Avgd::Ctl1AvgdShift2,
            3 => Ctl1Avgd::Ctl1AvgdShift3,
            4 => Ctl1Avgd::Ctl1AvgdShift4,
            5 => Ctl1Avgd::Ctl1AvgdShift5,
            6 => Ctl1Avgd::Ctl1AvgdShift6,
            7 => Ctl1Avgd::Ctl1AvgdShift7,
            _ => unreachable!(),
        }
    }
    #[doc = "SHIFT0"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift0(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift0
    }
    #[doc = "SHIFT1"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift1(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift1
    }
    #[doc = "SHIFT2"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift2(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift2
    }
    #[doc = "SHIFT3"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift3(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift3
    }
    #[doc = "SHIFT4"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift4(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift4
    }
    #[doc = "SHIFT5"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift5(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift5
    }
    #[doc = "SHIFT6"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift6(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift6
    }
    #[doc = "SHIFT7"]
    #[inline(always)]
    pub fn is_ctl1_avgd_shift7(&self) -> bool {
        *self == Ctl1Avgd::Ctl1AvgdShift7
    }
}
#[doc = "Field `CTL1_AVGD` writer - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropirately result will be truncated."]
pub type Ctl1AvgdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctl1Avgd, crate::Safe>;
impl<'a, REG> Ctl1AvgdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SHIFT0"]
    #[inline(always)]
    pub fn ctl1_avgd_shift0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift0)
    }
    #[doc = "SHIFT1"]
    #[inline(always)]
    pub fn ctl1_avgd_shift1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift1)
    }
    #[doc = "SHIFT2"]
    #[inline(always)]
    pub fn ctl1_avgd_shift2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift2)
    }
    #[doc = "SHIFT3"]
    #[inline(always)]
    pub fn ctl1_avgd_shift3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift3)
    }
    #[doc = "SHIFT4"]
    #[inline(always)]
    pub fn ctl1_avgd_shift4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift4)
    }
    #[doc = "SHIFT5"]
    #[inline(always)]
    pub fn ctl1_avgd_shift5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift5)
    }
    #[doc = "SHIFT6"]
    #[inline(always)]
    pub fn ctl1_avgd_shift6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift6)
    }
    #[doc = "SHIFT7"]
    #[inline(always)]
    pub fn ctl1_avgd_shift7(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Avgd::Ctl1AvgdShift7)
    }
}
impl R {
    #[doc = "Bit 0 - Sample trigger source"]
    #[inline(always)]
    pub fn ctl1_trigsrc(&self) -> Ctl1TrigsrcR {
        Ctl1TrigsrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Start of conversion"]
    #[inline(always)]
    pub fn ctl1_sc(&self) -> Ctl1ScR {
        Ctl1ScR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Conversion sequence mode"]
    #[inline(always)]
    pub fn ctl1_conseq(&self) -> Ctl1ConseqR {
        Ctl1ConseqR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
    #[inline(always)]
    pub fn ctl1_sampmode(&self) -> Ctl1SampmodeR {
        Ctl1SampmodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
    #[inline(always)]
    pub fn ctl1_avgn(&self) -> Ctl1AvgnR {
        Ctl1AvgnR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropirately result will be truncated."]
    #[inline(always)]
    pub fn ctl1_avgd(&self) -> Ctl1AvgdR {
        Ctl1AvgdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sample trigger source"]
    #[inline(always)]
    pub fn ctl1_trigsrc(&mut self) -> Ctl1TrigsrcW<Ctl1Spec> {
        Ctl1TrigsrcW::new(self, 0)
    }
    #[doc = "Bit 8 - Start of conversion"]
    #[inline(always)]
    pub fn ctl1_sc(&mut self) -> Ctl1ScW<Ctl1Spec> {
        Ctl1ScW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Conversion sequence mode"]
    #[inline(always)]
    pub fn ctl1_conseq(&mut self) -> Ctl1ConseqW<Ctl1Spec> {
        Ctl1ConseqW::new(self, 16)
    }
    #[doc = "Bit 20 - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
    #[inline(always)]
    pub fn ctl1_sampmode(&mut self) -> Ctl1SampmodeW<Ctl1Spec> {
        Ctl1SampmodeW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
    #[inline(always)]
    pub fn ctl1_avgn(&mut self) -> Ctl1AvgnW<Ctl1Spec> {
        Ctl1AvgnW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropirately result will be truncated."]
    #[inline(always)]
    pub fn ctl1_avgd(&mut self) -> Ctl1AvgdW<Ctl1Spec> {
        Ctl1AvgdW::new(self, 28)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}
