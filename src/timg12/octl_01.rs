#[doc = "Register `OCTL_01[%s]` reader"]
pub type R = crate::R<Octl01Spec>;
#[doc = "Register `OCTL_01[%s]` writer"]
pub type W = crate::W<Octl01Spec>;
#[doc = "CCP Output Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Octl01Ccpo {
    #[doc = "0: FUNCVAL"]
    Octl01CcpoFuncval = 0,
    #[doc = "1: LOAD"]
    Octl01CcpoLoad = 1,
    #[doc = "2: CMPVAL"]
    Octl01CcpoCmpval = 2,
    #[doc = "4: ZERO"]
    Octl01CcpoZero = 4,
    #[doc = "5: CAPCOND"]
    Octl01CcpoCapcond = 5,
    #[doc = "6: FAULTCOND"]
    Octl01CcpoFaultcond = 6,
    #[doc = "8: CC0_MIRROR_ALL"]
    Octl01CcpoCc0MirrorAll = 8,
    #[doc = "9: CC1_MIRROR_ALL"]
    Octl01CcpoCc1MirrorAll = 9,
    #[doc = "12: DEADBAND"]
    Octl01CcpoDeadband = 12,
    #[doc = "13: CNTDIR"]
    Octl01CcpoCntdir = 13,
}
impl From<Octl01Ccpo> for u8 {
    #[inline(always)]
    fn from(variant: Octl01Ccpo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Octl01Ccpo {
    type Ux = u8;
}
impl crate::IsEnum for Octl01Ccpo {}
#[doc = "Field `OCTL_01_CCPO` reader - CCP Output Source"]
pub type Octl01CcpoR = crate::FieldReader<Octl01Ccpo>;
impl Octl01CcpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Octl01Ccpo> {
        match self.bits {
            0 => Some(Octl01Ccpo::Octl01CcpoFuncval),
            1 => Some(Octl01Ccpo::Octl01CcpoLoad),
            2 => Some(Octl01Ccpo::Octl01CcpoCmpval),
            4 => Some(Octl01Ccpo::Octl01CcpoZero),
            5 => Some(Octl01Ccpo::Octl01CcpoCapcond),
            6 => Some(Octl01Ccpo::Octl01CcpoFaultcond),
            8 => Some(Octl01Ccpo::Octl01CcpoCc0MirrorAll),
            9 => Some(Octl01Ccpo::Octl01CcpoCc1MirrorAll),
            12 => Some(Octl01Ccpo::Octl01CcpoDeadband),
            13 => Some(Octl01Ccpo::Octl01CcpoCntdir),
            _ => None,
        }
    }
    #[doc = "FUNCVAL"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_funcval(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoFuncval
    }
    #[doc = "LOAD"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_load(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoLoad
    }
    #[doc = "CMPVAL"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_cmpval(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoCmpval
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_zero(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoZero
    }
    #[doc = "CAPCOND"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_capcond(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoCapcond
    }
    #[doc = "FAULTCOND"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_faultcond(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoFaultcond
    }
    #[doc = "CC0_MIRROR_ALL"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_cc0_mirror_all(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoCc0MirrorAll
    }
    #[doc = "CC1_MIRROR_ALL"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_cc1_mirror_all(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoCc1MirrorAll
    }
    #[doc = "DEADBAND"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_deadband(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoDeadband
    }
    #[doc = "CNTDIR"]
    #[inline(always)]
    pub fn is_octl_01_ccpo_cntdir(&self) -> bool {
        *self == Octl01Ccpo::Octl01CcpoCntdir
    }
}
#[doc = "Field `OCTL_01_CCPO` writer - CCP Output Source"]
pub type Octl01CcpoW<'a, REG> = crate::FieldWriter<'a, REG, 4, Octl01Ccpo>;
impl<'a, REG> Octl01CcpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FUNCVAL"]
    #[inline(always)]
    pub fn octl_01_ccpo_funcval(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoFuncval)
    }
    #[doc = "LOAD"]
    #[inline(always)]
    pub fn octl_01_ccpo_load(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoLoad)
    }
    #[doc = "CMPVAL"]
    #[inline(always)]
    pub fn octl_01_ccpo_cmpval(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoCmpval)
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn octl_01_ccpo_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoZero)
    }
    #[doc = "CAPCOND"]
    #[inline(always)]
    pub fn octl_01_ccpo_capcond(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoCapcond)
    }
    #[doc = "FAULTCOND"]
    #[inline(always)]
    pub fn octl_01_ccpo_faultcond(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoFaultcond)
    }
    #[doc = "CC0_MIRROR_ALL"]
    #[inline(always)]
    pub fn octl_01_ccpo_cc0_mirror_all(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoCc0MirrorAll)
    }
    #[doc = "CC1_MIRROR_ALL"]
    #[inline(always)]
    pub fn octl_01_ccpo_cc1_mirror_all(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoCc1MirrorAll)
    }
    #[doc = "DEADBAND"]
    #[inline(always)]
    pub fn octl_01_ccpo_deadband(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoDeadband)
    }
    #[doc = "CNTDIR"]
    #[inline(always)]
    pub fn octl_01_ccpo_cntdir(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpo::Octl01CcpoCntdir)
    }
}
#[doc = "CCP Output Invert The output as selected by CCPO is conditionally inverted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octl01Ccpoinv {
    #[doc = "0: NOINV"]
    Octl01CcpoinvNoinv = 0,
    #[doc = "1: INV"]
    Octl01CcpoinvInv = 1,
}
impl From<Octl01Ccpoinv> for bool {
    #[inline(always)]
    fn from(variant: Octl01Ccpoinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTL_01_CCPOINV` reader - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
pub type Octl01CcpoinvR = crate::BitReader<Octl01Ccpoinv>;
impl Octl01CcpoinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Octl01Ccpoinv {
        match self.bits {
            false => Octl01Ccpoinv::Octl01CcpoinvNoinv,
            true => Octl01Ccpoinv::Octl01CcpoinvInv,
        }
    }
    #[doc = "NOINV"]
    #[inline(always)]
    pub fn is_octl_01_ccpoinv_noinv(&self) -> bool {
        *self == Octl01Ccpoinv::Octl01CcpoinvNoinv
    }
    #[doc = "INV"]
    #[inline(always)]
    pub fn is_octl_01_ccpoinv_inv(&self) -> bool {
        *self == Octl01Ccpoinv::Octl01CcpoinvInv
    }
}
#[doc = "Field `OCTL_01_CCPOINV` writer - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
pub type Octl01CcpoinvW<'a, REG> = crate::BitWriter<'a, REG, Octl01Ccpoinv>;
impl<'a, REG> Octl01CcpoinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOINV"]
    #[inline(always)]
    pub fn octl_01_ccpoinv_noinv(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpoinv::Octl01CcpoinvNoinv)
    }
    #[doc = "INV"]
    #[inline(always)]
    pub fn octl_01_ccpoinv_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpoinv::Octl01CcpoinvInv)
    }
}
#[doc = "CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Octl01Ccpiv {
    #[doc = "0: LOW"]
    Octl01CcpivLow = 0,
    #[doc = "1: HIGH"]
    Octl01CcpivHigh = 1,
}
impl From<Octl01Ccpiv> for bool {
    #[inline(always)]
    fn from(variant: Octl01Ccpiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTL_01_CCPIV` reader - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
pub type Octl01CcpivR = crate::BitReader<Octl01Ccpiv>;
impl Octl01CcpivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Octl01Ccpiv {
        match self.bits {
            false => Octl01Ccpiv::Octl01CcpivLow,
            true => Octl01Ccpiv::Octl01CcpivHigh,
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn is_octl_01_ccpiv_low(&self) -> bool {
        *self == Octl01Ccpiv::Octl01CcpivLow
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn is_octl_01_ccpiv_high(&self) -> bool {
        *self == Octl01Ccpiv::Octl01CcpivHigh
    }
}
#[doc = "Field `OCTL_01_CCPIV` writer - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
pub type Octl01CcpivW<'a, REG> = crate::BitWriter<'a, REG, Octl01Ccpiv>;
impl<'a, REG> Octl01CcpivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOW"]
    #[inline(always)]
    pub fn octl_01_ccpiv_low(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpiv::Octl01CcpivLow)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn octl_01_ccpiv_high(self) -> &'a mut crate::W<REG> {
        self.variant(Octl01Ccpiv::Octl01CcpivHigh)
    }
}
impl R {
    #[doc = "Bits 0:3 - CCP Output Source"]
    #[inline(always)]
    pub fn octl_01_ccpo(&self) -> Octl01CcpoR {
        Octl01CcpoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
    #[inline(always)]
    pub fn octl_01_ccpoinv(&self) -> Octl01CcpoinvR {
        Octl01CcpoinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
    #[inline(always)]
    pub fn octl_01_ccpiv(&self) -> Octl01CcpivR {
        Octl01CcpivR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CCP Output Source"]
    #[inline(always)]
    pub fn octl_01_ccpo(&mut self) -> Octl01CcpoW<Octl01Spec> {
        Octl01CcpoW::new(self, 0)
    }
    #[doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."]
    #[inline(always)]
    pub fn octl_01_ccpoinv(&mut self) -> Octl01CcpoinvW<Octl01Spec> {
        Octl01CcpoinvW::new(self, 4)
    }
    #[doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."]
    #[inline(always)]
    pub fn octl_01_ccpiv(&mut self) -> Octl01CcpivW<Octl01Spec> {
        Octl01CcpivW::new(self, 5)
    }
}
#[doc = "CCP Output Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`octl_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Octl01Spec;
impl crate::RegisterSpec for Octl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`octl_01::R`](R) reader structure"]
impl crate::Readable for Octl01Spec {}
#[doc = "`write(|w| ..)` method takes [`octl_01::W`](W) writer structure"]
impl crate::Writable for Octl01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCTL_01[%s]
to value 0"]
impl crate::Resettable for Octl01Spec {
    const RESET_VALUE: u32 = 0;
}
