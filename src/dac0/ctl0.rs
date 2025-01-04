#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "This bit enables the DAC module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Enable {
    #[doc = "0: CLR"]
    Ctl0EnableClr = 0,
    #[doc = "1: SET"]
    Ctl0EnableSet = 1,
}
impl From<Ctl0Enable> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_ENABLE` reader - This bit enables the DAC module."]
pub type Ctl0EnableR = crate::BitReader<Ctl0Enable>;
impl Ctl0EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Enable {
        match self.bits {
            false => Ctl0Enable::Ctl0EnableClr,
            true => Ctl0Enable::Ctl0EnableSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ctl0_enable_clr(&self) -> bool {
        *self == Ctl0Enable::Ctl0EnableClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ctl0_enable_set(&self) -> bool {
        *self == Ctl0Enable::Ctl0EnableSet
    }
}
#[doc = "Field `CTL0_ENABLE` writer - This bit enables the DAC module."]
pub type Ctl0EnableW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Enable>;
impl<'a, REG> Ctl0EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn ctl0_enable_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enable::Ctl0EnableClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn ctl0_enable_set(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enable::Ctl0EnableSet)
    }
}
#[doc = "These bits define the DAC output voltage resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Res {
    #[doc = "0: _8BITS"]
    Ctl0Res_8bits = 0,
    #[doc = "1: _12BITS"]
    Ctl0Res_12bits = 1,
}
impl From<Ctl0Res> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Res) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_RES` reader - These bits define the DAC output voltage resolution."]
pub type Ctl0ResR = crate::BitReader<Ctl0Res>;
impl Ctl0ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Res {
        match self.bits {
            false => Ctl0Res::Ctl0Res_8bits,
            true => Ctl0Res::Ctl0Res_12bits,
        }
    }
    #[doc = "_8BITS"]
    #[inline(always)]
    pub fn is_ctl0_res__8bits(&self) -> bool {
        *self == Ctl0Res::Ctl0Res_8bits
    }
    #[doc = "_12BITS"]
    #[inline(always)]
    pub fn is_ctl0_res__12bits(&self) -> bool {
        *self == Ctl0Res::Ctl0Res_12bits
    }
}
#[doc = "Field `CTL0_RES` writer - These bits define the DAC output voltage resolution."]
pub type Ctl0ResW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Res>;
impl<'a, REG> Ctl0ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "_8BITS"]
    #[inline(always)]
    pub fn ctl0_res__8bits(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Res::Ctl0Res_8bits)
    }
    #[doc = "_12BITS"]
    #[inline(always)]
    pub fn ctl0_res__12bits(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Res::Ctl0Res_12bits)
    }
}
#[doc = "This bit defines the DAC input data format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Dfm {
    #[doc = "0: BINARY"]
    Ctl0DfmBinary = 0,
    #[doc = "1: TWOS_COMP"]
    Ctl0DfmTwosComp = 1,
}
impl From<Ctl0Dfm> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Dfm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_DFM` reader - This bit defines the DAC input data format."]
pub type Ctl0DfmR = crate::BitReader<Ctl0Dfm>;
impl Ctl0DfmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Dfm {
        match self.bits {
            false => Ctl0Dfm::Ctl0DfmBinary,
            true => Ctl0Dfm::Ctl0DfmTwosComp,
        }
    }
    #[doc = "BINARY"]
    #[inline(always)]
    pub fn is_ctl0_dfm_binary(&self) -> bool {
        *self == Ctl0Dfm::Ctl0DfmBinary
    }
    #[doc = "TWOS_COMP"]
    #[inline(always)]
    pub fn is_ctl0_dfm_twos_comp(&self) -> bool {
        *self == Ctl0Dfm::Ctl0DfmTwosComp
    }
}
#[doc = "Field `CTL0_DFM` writer - This bit defines the DAC input data format."]
pub type Ctl0DfmW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Dfm>;
impl<'a, REG> Ctl0DfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BINARY"]
    #[inline(always)]
    pub fn ctl0_dfm_binary(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dfm::Ctl0DfmBinary)
    }
    #[doc = "TWOS_COMP"]
    #[inline(always)]
    pub fn ctl0_dfm_twos_comp(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dfm::Ctl0DfmTwosComp)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables the DAC module."]
    #[inline(always)]
    pub fn ctl0_enable(&self) -> Ctl0EnableR {
        Ctl0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - These bits define the DAC output voltage resolution."]
    #[inline(always)]
    pub fn ctl0_res(&self) -> Ctl0ResR {
        Ctl0ResR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit defines the DAC input data format."]
    #[inline(always)]
    pub fn ctl0_dfm(&self) -> Ctl0DfmR {
        Ctl0DfmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the DAC module."]
    #[inline(always)]
    pub fn ctl0_enable(&mut self) -> Ctl0EnableW<Ctl0Spec> {
        Ctl0EnableW::new(self, 0)
    }
    #[doc = "Bit 8 - These bits define the DAC output voltage resolution."]
    #[inline(always)]
    pub fn ctl0_res(&mut self) -> Ctl0ResW<Ctl0Spec> {
        Ctl0ResW::new(self, 8)
    }
    #[doc = "Bit 16 - This bit defines the DAC input data format."]
    #[inline(always)]
    pub fn ctl0_dfm(&mut self) -> Ctl0DfmW<Ctl0Spec> {
        Ctl0DfmW::new(self, 16)
    }
}
#[doc = "Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
