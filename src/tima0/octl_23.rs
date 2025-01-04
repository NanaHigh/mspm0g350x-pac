#[doc = "Register `OCTL_23[%s]` reader"]
pub type R = crate::R<Octl23Spec>;
#[doc = "Register `OCTL_23[%s]` writer"]
pub type W = crate::W<Octl23Spec>;
#[doc = "CCP Output Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Octl23Ccpo {
    #[doc = "0: FUNCVAL"]
    Octl23CcpoFuncval = 0,
    #[doc = "1: LOAD"]
    Octl23CcpoLoad = 1,
    #[doc = "2: CMPVAL"]
    Octl23CcpoCmpval = 2,
    #[doc = "4: ZERO"]
    Octl23CcpoZero = 4,
    #[doc = "5: CAPCOND"]
    Octl23CcpoCapcond = 5,
    #[doc = "6: FAULTCOND"]
    Octl23CcpoFaultcond = 6,
    #[doc = "8: CC0_MIRROR_ALL"]
    Octl23CcpoCc0MirrorAll = 8,
    #[doc = "9: CC1_MIRROR_ALL"]
    Octl23CcpoCc1MirrorAll = 9,
    #[doc = "12: DEADBAND"]
    Octl23CcpoDeadband = 12,
    #[doc = "13: CNTDIR"]
    Octl23CcpoCntdir = 13,
}
impl From<Octl23Ccpo> for u8 {
    #[inline(always)]
    fn from(variant: Octl23Ccpo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Octl23Ccpo {
    type Ux = u8;
}
impl crate::IsEnum for Octl23Ccpo {}
#[doc = "Field `OCTL_23_CCPO` reader - CCP Output Source"]
pub type Octl23CcpoR = crate::FieldReader<Octl23Ccpo>;
impl Octl23CcpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Octl23Ccpo> {
        match self.bits {
            0 => Some(Octl23Ccpo::Octl23CcpoFuncval),
            1 => Some(Octl23Ccpo::Octl23CcpoLoad),
            2 => Some(Octl23Ccpo::Octl23CcpoCmpval),
            4 => Some(Octl23Ccpo::Octl23CcpoZero),
            5 => Some(Octl23Ccpo::Octl23CcpoCapcond),
            6 => Some(Octl23Ccpo::Octl23CcpoFaultcond),
            8 => Some(Octl23Ccpo::Octl23CcpoCc0MirrorAll),
            9 => Some(Octl23Ccpo::Octl23CcpoCc1MirrorAll),
            12 => Some(Octl23Ccpo::Octl23CcpoDeadband),
            13 => Some(Octl23Ccpo::Octl23CcpoCntdir),
            _ => None,
        }
    }
    #[doc = "FUNCVAL"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_funcval(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoFuncval
    }
    #[doc = "LOAD"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_load(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoLoad
    }
    #[doc = "CMPVAL"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_cmpval(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoCmpval
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_zero(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoZero
    }
    #[doc = "CAPCOND"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_capcond(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoCapcond
    }
    #[doc = "FAULTCOND"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_faultcond(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoFaultcond
    }
    #[doc = "CC0_MIRROR_ALL"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_cc0_mirror_all(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoCc0MirrorAll
    }
    #[doc = "CC1_MIRROR_ALL"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_cc1_mirror_all(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoCc1MirrorAll
    }
    #[doc = "DEADBAND"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_deadband(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoDeadband
    }
    #[doc = "CNTDIR"]
    #[inline(always)]
    pub fn is_octl_23_ccpo_cntdir(&self) -> bool {
        *self == Octl23Ccpo::Octl23CcpoCntdir
    }
}
#[doc = "Field `OCTL_23_CCPO` writer - CCP Output Source"]
pub type Octl23CcpoW<'a, REG> = crate::FieldWriter<'a, REG, 4, Octl23Ccpo>;
impl<'a, REG> Octl23CcpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FUNCVAL"]
    #[inline(always)]
    pub fn octl_23_ccpo_funcval(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoFuncval)
    }
    #[doc = "LOAD"]
    #[inline(always)]
    pub fn octl_23_ccpo_load(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoLoad)
    }
    #[doc = "CMPVAL"]
    #[inline(always)]
    pub fn octl_23_ccpo_cmpval(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoCmpval)
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn octl_23_ccpo_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoZero)
    }
    #[doc = "CAPCOND"]
    #[inline(always)]
    pub fn octl_23_ccpo_capcond(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoCapcond)
    }
    #[doc = "FAULTCOND"]
    #[inline(always)]
    pub fn octl_23_ccpo_faultcond(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoFaultcond)
    }
    #[doc = "CC0_MIRROR_ALL"]
    #[inline(always)]
    pub fn octl_23_ccpo_cc0_mirror_all(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoCc0MirrorAll)
    }
    #[doc = "CC1_MIRROR_ALL"]
    #[inline(always)]
    pub fn octl_23_ccpo_cc1_mirror_all(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoCc1MirrorAll)
    }
    #[doc = "DEADBAND"]
    #[inline(always)]
    pub fn octl_23_ccpo_deadband(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoDeadband)
    }
    #[doc = "CNTDIR"]
    #[inline(always)]
    pub fn octl_23_ccpo_cntdir(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpo::Octl23CcpoCntdir)
    }
}
#[doc = "CCP Output Invert The output as selected by CCPO is conditionally inverted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octl23Ccpoinv {
    #[doc = "0: NOINV"]
    Octl23CcpoinvNoinv = 0,
    #[doc = "1: INV"]
    Octl23CcpoinvInv = 1,
}
impl From<Octl23Ccpoinv> for bool {
    #[inline(always)]
    fn from(variant: Octl23Ccpoinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTL_23_CCPOINV` reader - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
pub type Octl23CcpoinvR = crate::BitReader<Octl23Ccpoinv>;
impl Octl23CcpoinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Octl23Ccpoinv {
        match self.bits {
            false => Octl23Ccpoinv::Octl23CcpoinvNoinv,
            true => Octl23Ccpoinv::Octl23CcpoinvInv,
        }
    }
    #[doc = "NOINV"]
    #[inline(always)]
    pub fn is_octl_23_ccpoinv_noinv(&self) -> bool {
        *self == Octl23Ccpoinv::Octl23CcpoinvNoinv
    }
    #[doc = "INV"]
    #[inline(always)]
    pub fn is_octl_23_ccpoinv_inv(&self) -> bool {
        *self == Octl23Ccpoinv::Octl23CcpoinvInv
    }
}
#[doc = "Field `OCTL_23_CCPOINV` writer - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
pub type Octl23CcpoinvW<'a, REG> = crate::BitWriter<'a, REG, Octl23Ccpoinv>;
impl<'a, REG> Octl23CcpoinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOINV"]
    #[inline(always)]
    pub fn octl_23_ccpoinv_noinv(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpoinv::Octl23CcpoinvNoinv)
    }
    #[doc = "INV"]
    #[inline(always)]
    pub fn octl_23_ccpoinv_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpoinv::Octl23CcpoinvInv)
    }
}
#[doc = "CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octl23Ccpiv {
    #[doc = "0: LOW"]
    Octl23CcpivLow = 0,
    #[doc = "1: HIGH"]
    Octl23CcpivHigh = 1,
}
impl From<Octl23Ccpiv> for bool {
    #[inline(always)]
    fn from(variant: Octl23Ccpiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTL_23_CCPIV` reader - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
pub type Octl23CcpivR = crate::BitReader<Octl23Ccpiv>;
impl Octl23CcpivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Octl23Ccpiv {
        match self.bits {
            false => Octl23Ccpiv::Octl23CcpivLow,
            true => Octl23Ccpiv::Octl23CcpivHigh,
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn is_octl_23_ccpiv_low(&self) -> bool {
        *self == Octl23Ccpiv::Octl23CcpivLow
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn is_octl_23_ccpiv_high(&self) -> bool {
        *self == Octl23Ccpiv::Octl23CcpivHigh
    }
}
#[doc = "Field `OCTL_23_CCPIV` writer - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
pub type Octl23CcpivW<'a, REG> = crate::BitWriter<'a, REG, Octl23Ccpiv>;
impl<'a, REG> Octl23CcpivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOW"]
    #[inline(always)]
    pub fn octl_23_ccpiv_low(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpiv::Octl23CcpivLow)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn octl_23_ccpiv_high(self) -> &'a mut crate::W<REG> {
        self.variant(Octl23Ccpiv::Octl23CcpivHigh)
    }
}
impl R {
    #[doc = "Bits 0:3 - CCP Output Source"]
    #[inline(always)]
    pub fn octl_23_ccpo(&self) -> Octl23CcpoR {
        Octl23CcpoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
    #[inline(always)]
    pub fn octl_23_ccpoinv(&self) -> Octl23CcpoinvR {
        Octl23CcpoinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
    #[inline(always)]
    pub fn octl_23_ccpiv(&self) -> Octl23CcpivR {
        Octl23CcpivR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CCP Output Source"]
    #[inline(always)]
    pub fn octl_23_ccpo(&mut self) -> Octl23CcpoW<Octl23Spec> {
        Octl23CcpoW::new(self, 0)
    }
    #[doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
    #[inline(always)]
    pub fn octl_23_ccpoinv(&mut self) -> Octl23CcpoinvW<Octl23Spec> {
        Octl23CcpoinvW::new(self, 4)
    }
    #[doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
    #[inline(always)]
    pub fn octl_23_ccpiv(&mut self) -> Octl23CcpivW<Octl23Spec> {
        Octl23CcpivW::new(self, 5)
    }
}
#[doc = "CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`octl_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Octl23Spec;
impl crate::RegisterSpec for Octl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octl_23::R`](R) reader structure"]
impl crate::Readable for Octl23Spec {}
#[doc = "`write(|w| ..)` method takes [`octl_23::W`](W) writer structure"]
impl crate::Writable for Octl23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCTL_23[%s]
to value 0"]
impl crate::Resettable for Octl23Spec {
    const RESET_VALUE: u32 = 0;
}
