#[doc = "Register `IFCTL_23[%s]` reader"]
pub type R = crate::R<Ifctl23Spec>;
#[doc = "Register `IFCTL_23[%s]` writer"]
pub type W = crate::W<Ifctl23Spec>;
#[doc = "Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ifctl23Isel {
    #[doc = "0: CCPX_INPUT"]
    Ifctl23IselCcpxInput = 0,
    #[doc = "1: CCPX_INPUT_PAIR"]
    Ifctl23IselCcpxInputPair = 1,
    #[doc = "2: CCP0_INPUT"]
    Ifctl23IselCcp0Input = 2,
    #[doc = "3: TRIG_INPUT"]
    Ifctl23IselTrigInput = 3,
    #[doc = "4: CCP_XOR"]
    Ifctl23IselCcpXor = 4,
    #[doc = "5: FSUB0"]
    Ifctl23IselFsub0 = 5,
    #[doc = "6: FSUB1"]
    Ifctl23IselFsub1 = 6,
    #[doc = "7: COMP0"]
    Ifctl23IselComp0 = 7,
    #[doc = "8: COMP1"]
    Ifctl23IselComp1 = 8,
    #[doc = "9: COMP2"]
    Ifctl23IselComp2 = 9,
}
impl From<Ifctl23Isel> for u8 {
    #[inline(always)]
    fn from(variant: Ifctl23Isel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ifctl23Isel {
    type Ux = u8;
}
impl crate::IsEnum for Ifctl23Isel {}
#[doc = "Field `IFCTL_23_ISEL` reader - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
pub type Ifctl23IselR = crate::FieldReader<Ifctl23Isel>;
impl Ifctl23IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ifctl23Isel> {
        match self.bits {
            0 => Some(Ifctl23Isel::Ifctl23IselCcpxInput),
            1 => Some(Ifctl23Isel::Ifctl23IselCcpxInputPair),
            2 => Some(Ifctl23Isel::Ifctl23IselCcp0Input),
            3 => Some(Ifctl23Isel::Ifctl23IselTrigInput),
            4 => Some(Ifctl23Isel::Ifctl23IselCcpXor),
            5 => Some(Ifctl23Isel::Ifctl23IselFsub0),
            6 => Some(Ifctl23Isel::Ifctl23IselFsub1),
            7 => Some(Ifctl23Isel::Ifctl23IselComp0),
            8 => Some(Ifctl23Isel::Ifctl23IselComp1),
            9 => Some(Ifctl23Isel::Ifctl23IselComp2),
            _ => None,
        }
    }
    #[doc = "CCPX_INPUT"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_ccpx_input(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselCcpxInput
    }
    #[doc = "CCPX_INPUT_PAIR"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_ccpx_input_pair(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselCcpxInputPair
    }
    #[doc = "CCP0_INPUT"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_ccp0_input(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselCcp0Input
    }
    #[doc = "TRIG_INPUT"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_trig_input(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselTrigInput
    }
    #[doc = "CCP_XOR"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_ccp_xor(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselCcpXor
    }
    #[doc = "FSUB0"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_fsub0(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselFsub0
    }
    #[doc = "FSUB1"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_fsub1(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselFsub1
    }
    #[doc = "COMP0"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_comp0(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselComp0
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_comp1(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselComp1
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn is_ifctl_23_isel_comp2(&self) -> bool {
        *self == Ifctl23Isel::Ifctl23IselComp2
    }
}
#[doc = "Field `IFCTL_23_ISEL` writer - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
pub type Ifctl23IselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ifctl23Isel>;
impl<'a, REG> Ifctl23IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCPX_INPUT"]
    #[inline(always)]
    pub fn ifctl_23_isel_ccpx_input(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselCcpxInput)
    }
    #[doc = "CCPX_INPUT_PAIR"]
    #[inline(always)]
    pub fn ifctl_23_isel_ccpx_input_pair(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselCcpxInputPair)
    }
    #[doc = "CCP0_INPUT"]
    #[inline(always)]
    pub fn ifctl_23_isel_ccp0_input(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselCcp0Input)
    }
    #[doc = "TRIG_INPUT"]
    #[inline(always)]
    pub fn ifctl_23_isel_trig_input(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselTrigInput)
    }
    #[doc = "CCP_XOR"]
    #[inline(always)]
    pub fn ifctl_23_isel_ccp_xor(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselCcpXor)
    }
    #[doc = "FSUB0"]
    #[inline(always)]
    pub fn ifctl_23_isel_fsub0(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselFsub0)
    }
    #[doc = "FSUB1"]
    #[inline(always)]
    pub fn ifctl_23_isel_fsub1(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselFsub1)
    }
    #[doc = "COMP0"]
    #[inline(always)]
    pub fn ifctl_23_isel_comp0(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselComp0)
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn ifctl_23_isel_comp1(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselComp1)
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn ifctl_23_isel_comp2(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Isel::Ifctl23IselComp2)
    }
}
#[doc = "Input Inversion This bit controls whether the selected input is inverted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifctl23Inv {
    #[doc = "0: NOINVERT"]
    Ifctl23InvNoinvert = 0,
    #[doc = "1: INVERT"]
    Ifctl23InvInvert = 1,
}
impl From<Ifctl23Inv> for bool {
    #[inline(always)]
    fn from(variant: Ifctl23Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCTL_23_INV` reader - Input Inversion This bit controls whether the selected input is inverted."]
pub type Ifctl23InvR = crate::BitReader<Ifctl23Inv>;
impl Ifctl23InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifctl23Inv {
        match self.bits {
            false => Ifctl23Inv::Ifctl23InvNoinvert,
            true => Ifctl23Inv::Ifctl23InvInvert,
        }
    }
    #[doc = "NOINVERT"]
    #[inline(always)]
    pub fn is_ifctl_23_inv_noinvert(&self) -> bool {
        *self == Ifctl23Inv::Ifctl23InvNoinvert
    }
    #[doc = "INVERT"]
    #[inline(always)]
    pub fn is_ifctl_23_inv_invert(&self) -> bool {
        *self == Ifctl23Inv::Ifctl23InvInvert
    }
}
#[doc = "Field `IFCTL_23_INV` writer - Input Inversion This bit controls whether the selected input is inverted."]
pub type Ifctl23InvW<'a, REG> = crate::BitWriter<'a, REG, Ifctl23Inv>;
impl<'a, REG> Ifctl23InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOINVERT"]
    #[inline(always)]
    pub fn ifctl_23_inv_noinvert(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Inv::Ifctl23InvNoinvert)
    }
    #[doc = "INVERT"]
    #[inline(always)]
    pub fn ifctl_23_inv_invert(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Inv::Ifctl23InvInvert)
    }
}
#[doc = "Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ifctl23Fp {
    #[doc = "0: _3"]
    Ifctl23Fp_3 = 0,
    #[doc = "1: _5"]
    Ifctl23Fp_5 = 1,
    #[doc = "2: _8"]
    Ifctl23Fp_8 = 2,
}
impl From<Ifctl23Fp> for u8 {
    #[inline(always)]
    fn from(variant: Ifctl23Fp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ifctl23Fp {
    type Ux = u8;
}
impl crate::IsEnum for Ifctl23Fp {}
#[doc = "Field `IFCTL_23_FP` reader - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type Ifctl23FpR = crate::FieldReader<Ifctl23Fp>;
impl Ifctl23FpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ifctl23Fp> {
        match self.bits {
            0 => Some(Ifctl23Fp::Ifctl23Fp_3),
            1 => Some(Ifctl23Fp::Ifctl23Fp_5),
            2 => Some(Ifctl23Fp::Ifctl23Fp_8),
            _ => None,
        }
    }
    #[doc = "_3"]
    #[inline(always)]
    pub fn is_ifctl_23_fp__3(&self) -> bool {
        *self == Ifctl23Fp::Ifctl23Fp_3
    }
    #[doc = "_5"]
    #[inline(always)]
    pub fn is_ifctl_23_fp__5(&self) -> bool {
        *self == Ifctl23Fp::Ifctl23Fp_5
    }
    #[doc = "_8"]
    #[inline(always)]
    pub fn is_ifctl_23_fp__8(&self) -> bool {
        *self == Ifctl23Fp::Ifctl23Fp_8
    }
}
#[doc = "Field `IFCTL_23_FP` writer - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type Ifctl23FpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ifctl23Fp>;
impl<'a, REG> Ifctl23FpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_3"]
    #[inline(always)]
    pub fn ifctl_23_fp__3(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Fp::Ifctl23Fp_3)
    }
    #[doc = "_5"]
    #[inline(always)]
    pub fn ifctl_23_fp__5(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Fp::Ifctl23Fp_5)
    }
    #[doc = "_8"]
    #[inline(always)]
    pub fn ifctl_23_fp__8(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Fp::Ifctl23Fp_8)
    }
}
#[doc = "Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifctl23Cpv {
    #[doc = "0: CONSECUTIVE"]
    Ifctl23CpvConsecutive = 0,
    #[doc = "1: VOTING"]
    Ifctl23CpvVoting = 1,
}
impl From<Ifctl23Cpv> for bool {
    #[inline(always)]
    fn from(variant: Ifctl23Cpv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCTL_23_CPV` reader - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type Ifctl23CpvR = crate::BitReader<Ifctl23Cpv>;
impl Ifctl23CpvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifctl23Cpv {
        match self.bits {
            false => Ifctl23Cpv::Ifctl23CpvConsecutive,
            true => Ifctl23Cpv::Ifctl23CpvVoting,
        }
    }
    #[doc = "CONSECUTIVE"]
    #[inline(always)]
    pub fn is_ifctl_23_cpv_consecutive(&self) -> bool {
        *self == Ifctl23Cpv::Ifctl23CpvConsecutive
    }
    #[doc = "VOTING"]
    #[inline(always)]
    pub fn is_ifctl_23_cpv_voting(&self) -> bool {
        *self == Ifctl23Cpv::Ifctl23CpvVoting
    }
}
#[doc = "Field `IFCTL_23_CPV` writer - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type Ifctl23CpvW<'a, REG> = crate::BitWriter<'a, REG, Ifctl23Cpv>;
impl<'a, REG> Ifctl23CpvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CONSECUTIVE"]
    #[inline(always)]
    pub fn ifctl_23_cpv_consecutive(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Cpv::Ifctl23CpvConsecutive)
    }
    #[doc = "VOTING"]
    #[inline(always)]
    pub fn ifctl_23_cpv_voting(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Cpv::Ifctl23CpvVoting)
    }
}
#[doc = "Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ifctl23Fe {
    #[doc = "0: DISABLED"]
    Ifctl23FeDisabled = 0,
    #[doc = "1: ENABLED"]
    Ifctl23FeEnabled = 1,
}
impl From<Ifctl23Fe> for bool {
    #[inline(always)]
    fn from(variant: Ifctl23Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCTL_23_FE` reader - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
pub type Ifctl23FeR = crate::BitReader<Ifctl23Fe>;
impl Ifctl23FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ifctl23Fe {
        match self.bits {
            false => Ifctl23Fe::Ifctl23FeDisabled,
            true => Ifctl23Fe::Ifctl23FeEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ifctl_23_fe_disabled(&self) -> bool {
        *self == Ifctl23Fe::Ifctl23FeDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ifctl_23_fe_enabled(&self) -> bool {
        *self == Ifctl23Fe::Ifctl23FeEnabled
    }
}
#[doc = "Field `IFCTL_23_FE` writer - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
pub type Ifctl23FeW<'a, REG> = crate::BitWriter<'a, REG, Ifctl23Fe>;
impl<'a, REG> Ifctl23FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ifctl_23_fe_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Fe::Ifctl23FeDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ifctl_23_fe_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ifctl23Fe::Ifctl23FeEnabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
    #[inline(always)]
    pub fn ifctl_23_isel(&self) -> Ifctl23IselR {
        Ifctl23IselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Input Inversion This bit controls whether the selected input is inverted."]
    #[inline(always)]
    pub fn ifctl_23_inv(&self) -> Ifctl23InvR {
        Ifctl23InvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn ifctl_23_fp(&self) -> Ifctl23FpR {
        Ifctl23FpR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn ifctl_23_cpv(&self) -> Ifctl23CpvR {
        Ifctl23CpvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
    #[inline(always)]
    pub fn ifctl_23_fe(&self) -> Ifctl23FeR {
        Ifctl23FeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input Select (CCP0) This field selects the input source to the filter input. 4h-7h = Reserved"]
    #[inline(always)]
    pub fn ifctl_23_isel(&mut self) -> Ifctl23IselW<Ifctl23Spec> {
        Ifctl23IselW::new(self, 0)
    }
    #[doc = "Bit 7 - Input Inversion This bit controls whether the selected input is inverted."]
    #[inline(always)]
    pub fn ifctl_23_inv(&mut self) -> Ifctl23InvW<Ifctl23Spec> {
        Ifctl23InvW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Filter Period. This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn ifctl_23_fp(&mut self) -> Ifctl23FpW<Ifctl23Spec> {
        Ifctl23FpW::new(self, 8)
    }
    #[doc = "Bit 11 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn ifctl_23_cpv(&mut self) -> Ifctl23CpvW<Ifctl23Spec> {
        Ifctl23CpvW::new(self, 11)
    }
    #[doc = "Bit 12 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to the edge detect."]
    #[inline(always)]
    pub fn ifctl_23_fe(&mut self) -> Ifctl23FeW<Ifctl23Spec> {
        Ifctl23FeW::new(self, 12)
    }
}
#[doc = "Input Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifctl_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifctl_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ifctl23Spec;
impl crate::RegisterSpec for Ifctl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifctl_23::R`](R) reader structure"]
impl crate::Readable for Ifctl23Spec {}
#[doc = "`write(|w| ..)` method takes [`ifctl_23::W`](W) writer structure"]
impl crate::Writable for Ifctl23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCTL_23[%s]
to value 0"]
impl crate::Resettable for Ifctl23Spec {
    const RESET_VALUE: u32 = 0;
}
