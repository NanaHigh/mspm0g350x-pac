#[doc = "Register `IFCTL_01[%s]` reader"]
pub type R = crate::R<Ifctl01Spec>;
#[doc = "Register `IFCTL_01[%s]` writer"]
pub type W = crate::W<Ifctl01Spec>;
#[doc = "Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ifctl01Isel {
    #[doc = "0: CCPX_INPUT"]
    Ifctl01IselCcpxInput = 0,
    #[doc = "1: CCPX_INPUT_PAIR"]
    Ifctl01IselCcpxInputPair = 1,
    #[doc = "2: CCP0_INPUT"]
    Ifctl01IselCcp0Input = 2,
    #[doc = "3: TRIG_INPUT"]
    Ifctl01IselTrigInput = 3,
    #[doc = "4: CCP_XOR"]
    Ifctl01IselCcpXor = 4,
    #[doc = "5: FSUB0"]
    Ifctl01IselFsub0 = 5,
    #[doc = "6: FSUB1"]
    Ifctl01IselFsub1 = 6,
    #[doc = "7: COMP0"]
    Ifctl01IselComp0 = 7,
    #[doc = "8: COMP1"]
    Ifctl01IselComp1 = 8,
    #[doc = "9: COMP2"]
    Ifctl01IselComp2 = 9,
}
impl From<Ifctl01Isel> for u8 {
    #[inline(always)]
    fn from(variant: Ifctl01Isel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ifctl01Isel {
    type Ux = u8;
}
impl crate::IsEnum for Ifctl01Isel {}
#[doc = "Field `IFCTL_01_ISEL` reader - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
pub type Ifctl01IselR = crate::FieldReader<Ifctl01Isel>;
impl Ifctl01IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ifctl01Isel> {
        match self.bits {
            0 => Some(Ifctl01Isel::Ifctl01IselCcpxInput),
            1 => Some(Ifctl01Isel::Ifctl01IselCcpxInputPair),
            2 => Some(Ifctl01Isel::Ifctl01IselCcp0Input),
            3 => Some(Ifctl01Isel::Ifctl01IselTrigInput),
            4 => Some(Ifctl01Isel::Ifctl01IselCcpXor),
            5 => Some(Ifctl01Isel::Ifctl01IselFsub0),
            6 => Some(Ifctl01Isel::Ifctl01IselFsub1),
            7 => Some(Ifctl01Isel::Ifctl01IselComp0),
            8 => Some(Ifctl01Isel::Ifctl01IselComp1),
            9 => Some(Ifctl01Isel::Ifctl01IselComp2),
            _ => None,
        }
    }
    #[doc = "CCPX_INPUT"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_ccpx_input(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselCcpxInput
    }
    #[doc = "CCPX_INPUT_PAIR"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_ccpx_input_pair(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselCcpxInputPair
    }
    #[doc = "CCP0_INPUT"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_ccp0_input(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselCcp0Input
    }
    #[doc = "TRIG_INPUT"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_trig_input(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselTrigInput
    }
    #[doc = "CCP_XOR"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_ccp_xor(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselCcpXor
    }
    #[doc = "FSUB0"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_fsub0(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselFsub0
    }
    #[doc = "FSUB1"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_fsub1(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselFsub1
    }
    #[doc = "COMP0"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_comp0(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselComp0
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_comp1(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselComp1
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn is_ifctl_01_isel_comp2(&self) -> bool {
        *self == Ifctl01Isel::Ifctl01IselComp2
    }
}
#[doc = "Field `IFCTL_01_ISEL` writer - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
pub type Ifctl01IselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ifctl01Isel>;
impl<'a, REG> Ifctl01IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCPX_INPUT"]
    #[inline(always)]
    pub fn ifctl_01_isel_ccpx_input(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselCcpxInput)
    }
    #[doc = "CCPX_INPUT_PAIR"]
    #[inline(always)]
    pub fn ifctl_01_isel_ccpx_input_pair(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselCcpxInputPair)
    }
    #[doc = "CCP0_INPUT"]
    #[inline(always)]
    pub fn ifctl_01_isel_ccp0_input(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselCcp0Input)
    }
    #[doc = "TRIG_INPUT"]
    #[inline(always)]
    pub fn ifctl_01_isel_trig_input(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselTrigInput)
    }
    #[doc = "CCP_XOR"]
    #[inline(always)]
    pub fn ifctl_01_isel_ccp_xor(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselCcpXor)
    }
    #[doc = "FSUB0"]
    #[inline(always)]
    pub fn ifctl_01_isel_fsub0(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselFsub0)
    }
    #[doc = "FSUB1"]
    #[inline(always)]
    pub fn ifctl_01_isel_fsub1(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselFsub1)
    }
    #[doc = "COMP0"]
    #[inline(always)]
    pub fn ifctl_01_isel_comp0(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselComp0)
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn ifctl_01_isel_comp1(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselComp1)
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn ifctl_01_isel_comp2(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Isel::Ifctl01IselComp2)
    }
}
#[doc = "Input Inversion This bit controls whether the selected input is inverted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifctl01Inv {
    #[doc = "0: NOINVERT"]
    Ifctl01InvNoinvert = 0,
    #[doc = "1: INVERT"]
    Ifctl01InvInvert = 1,
}
impl From<Ifctl01Inv> for bool {
    #[inline(always)]
    fn from(variant: Ifctl01Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCTL_01_INV` reader - Input Inversion This bit controls whether the selected input is inverted."]
pub type Ifctl01InvR = crate::BitReader<Ifctl01Inv>;
impl Ifctl01InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifctl01Inv {
        match self.bits {
            false => Ifctl01Inv::Ifctl01InvNoinvert,
            true => Ifctl01Inv::Ifctl01InvInvert,
        }
    }
    #[doc = "NOINVERT"]
    #[inline(always)]
    pub fn is_ifctl_01_inv_noinvert(&self) -> bool {
        *self == Ifctl01Inv::Ifctl01InvNoinvert
    }
    #[doc = "INVERT"]
    #[inline(always)]
    pub fn is_ifctl_01_inv_invert(&self) -> bool {
        *self == Ifctl01Inv::Ifctl01InvInvert
    }
}
#[doc = "Field `IFCTL_01_INV` writer - Input Inversion This bit controls whether the selected input is inverted."]
pub type Ifctl01InvW<'a, REG> = crate::BitWriter<'a, REG, Ifctl01Inv>;
impl<'a, REG> Ifctl01InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOINVERT"]
    #[inline(always)]
    pub fn ifctl_01_inv_noinvert(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Inv::Ifctl01InvNoinvert)
    }
    #[doc = "INVERT"]
    #[inline(always)]
    pub fn ifctl_01_inv_invert(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Inv::Ifctl01InvInvert)
    }
}
#[doc = "Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ifctl01Fp {
    #[doc = "0: _3"]
    Ifctl01Fp_3 = 0,
    #[doc = "1: _5"]
    Ifctl01Fp_5 = 1,
    #[doc = "2: _8"]
    Ifctl01Fp_8 = 2,
}
impl From<Ifctl01Fp> for u8 {
    #[inline(always)]
    fn from(variant: Ifctl01Fp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ifctl01Fp {
    type Ux = u8;
}
impl crate::IsEnum for Ifctl01Fp {}
#[doc = "Field `IFCTL_01_FP` reader - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type Ifctl01FpR = crate::FieldReader<Ifctl01Fp>;
impl Ifctl01FpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ifctl01Fp> {
        match self.bits {
            0 => Some(Ifctl01Fp::Ifctl01Fp_3),
            1 => Some(Ifctl01Fp::Ifctl01Fp_5),
            2 => Some(Ifctl01Fp::Ifctl01Fp_8),
            _ => None,
        }
    }
    #[doc = "_3"]
    #[inline(always)]
    pub fn is_ifctl_01_fp__3(&self) -> bool {
        *self == Ifctl01Fp::Ifctl01Fp_3
    }
    #[doc = "_5"]
    #[inline(always)]
    pub fn is_ifctl_01_fp__5(&self) -> bool {
        *self == Ifctl01Fp::Ifctl01Fp_5
    }
    #[doc = "_8"]
    #[inline(always)]
    pub fn is_ifctl_01_fp__8(&self) -> bool {
        *self == Ifctl01Fp::Ifctl01Fp_8
    }
}
#[doc = "Field `IFCTL_01_FP` writer - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type Ifctl01FpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ifctl01Fp>;
impl<'a, REG> Ifctl01FpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_3"]
    #[inline(always)]
    pub fn ifctl_01_fp__3(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Fp::Ifctl01Fp_3)
    }
    #[doc = "_5"]
    #[inline(always)]
    pub fn ifctl_01_fp__5(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Fp::Ifctl01Fp_5)
    }
    #[doc = "_8"]
    #[inline(always)]
    pub fn ifctl_01_fp__8(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Fp::Ifctl01Fp_8)
    }
}
#[doc = "Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifctl01Cpv {
    #[doc = "0: CONSECUTIVE"]
    Ifctl01CpvConsecutive = 0,
    #[doc = "1: VOTING"]
    Ifctl01CpvVoting = 1,
}
impl From<Ifctl01Cpv> for bool {
    #[inline(always)]
    fn from(variant: Ifctl01Cpv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCTL_01_CPV` reader - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type Ifctl01CpvR = crate::BitReader<Ifctl01Cpv>;
impl Ifctl01CpvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifctl01Cpv {
        match self.bits {
            false => Ifctl01Cpv::Ifctl01CpvConsecutive,
            true => Ifctl01Cpv::Ifctl01CpvVoting,
        }
    }
    #[doc = "CONSECUTIVE"]
    #[inline(always)]
    pub fn is_ifctl_01_cpv_consecutive(&self) -> bool {
        *self == Ifctl01Cpv::Ifctl01CpvConsecutive
    }
    #[doc = "VOTING"]
    #[inline(always)]
    pub fn is_ifctl_01_cpv_voting(&self) -> bool {
        *self == Ifctl01Cpv::Ifctl01CpvVoting
    }
}
#[doc = "Field `IFCTL_01_CPV` writer - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type Ifctl01CpvW<'a, REG> = crate::BitWriter<'a, REG, Ifctl01Cpv>;
impl<'a, REG> Ifctl01CpvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CONSECUTIVE"]
    #[inline(always)]
    pub fn ifctl_01_cpv_consecutive(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Cpv::Ifctl01CpvConsecutive)
    }
    #[doc = "VOTING"]
    #[inline(always)]
    pub fn ifctl_01_cpv_voting(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Cpv::Ifctl01CpvVoting)
    }
}
#[doc = "Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifctl01Fe {
    #[doc = "0: DISABLED"]
    Ifctl01FeDisabled = 0,
    #[doc = "1: ENABLED"]
    Ifctl01FeEnabled = 1,
}
impl From<Ifctl01Fe> for bool {
    #[inline(always)]
    fn from(variant: Ifctl01Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCTL_01_FE` reader - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
pub type Ifctl01FeR = crate::BitReader<Ifctl01Fe>;
impl Ifctl01FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifctl01Fe {
        match self.bits {
            false => Ifctl01Fe::Ifctl01FeDisabled,
            true => Ifctl01Fe::Ifctl01FeEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ifctl_01_fe_disabled(&self) -> bool {
        *self == Ifctl01Fe::Ifctl01FeDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ifctl_01_fe_enabled(&self) -> bool {
        *self == Ifctl01Fe::Ifctl01FeEnabled
    }
}
#[doc = "Field `IFCTL_01_FE` writer - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
pub type Ifctl01FeW<'a, REG> = crate::BitWriter<'a, REG, Ifctl01Fe>;
impl<'a, REG> Ifctl01FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ifctl_01_fe_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Fe::Ifctl01FeDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ifctl_01_fe_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl01Fe::Ifctl01FeEnabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
    #[inline(always)]
    pub fn ifctl_01_isel(&self) -> Ifctl01IselR {
        Ifctl01IselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Input Inversion This bit controls whether the selected input is inverted."]
    #[inline(always)]
    pub fn ifctl_01_inv(&self) -> Ifctl01InvR {
        Ifctl01InvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn ifctl_01_fp(&self) -> Ifctl01FpR {
        Ifctl01FpR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn ifctl_01_cpv(&self) -> Ifctl01CpvR {
        Ifctl01CpvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
    #[inline(always)]
    pub fn ifctl_01_fe(&self) -> Ifctl01FeR {
        Ifctl01FeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
    #[inline(always)]
    pub fn ifctl_01_isel(&mut self) -> Ifctl01IselW<Ifctl01Spec> {
        Ifctl01IselW::new(self, 0)
    }
    #[doc = "Bit 7 - Input Inversion This bit controls whether the selected input is inverted."]
    #[inline(always)]
    pub fn ifctl_01_inv(&mut self) -> Ifctl01InvW<Ifctl01Spec> {
        Ifctl01InvW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn ifctl_01_fp(&mut self) -> Ifctl01FpW<Ifctl01Spec> {
        Ifctl01FpW::new(self, 8)
    }
    #[doc = "Bit 11 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn ifctl_01_cpv(&mut self) -> Ifctl01CpvW<Ifctl01Spec> {
        Ifctl01CpvW::new(self, 11)
    }
    #[doc = "Bit 12 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
    #[inline(always)]
    pub fn ifctl_01_fe(&mut self) -> Ifctl01FeW<Ifctl01Spec> {
        Ifctl01FeW::new(self, 12)
    }
}
#[doc = "Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifctl_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifctl_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifctl01Spec;
impl crate::RegisterSpec for Ifctl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifctl_01::R`](R) reader structure"]
impl crate::Readable for Ifctl01Spec {}
#[doc = "`write(|w| ..)` method takes [`ifctl_01::W`](W) writer structure"]
impl crate::Writable for Ifctl01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCTL_01[%s]
to value 0"]
impl crate::Resettable for Ifctl01Spec {
    const RESET_VALUE: u32 = 0;
}
