#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "This bit enables the VREF module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Enable {
    #[doc = "0: DISABLE"]
    Ctl0EnableDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0EnableEnable = 1,
}
impl From<Ctl0Enable> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_ENABLE` reader - This bit enables the VREF module."]
pub type Ctl0EnableR = crate::BitReader<Ctl0Enable>;
impl Ctl0EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Enable {
        match self.bits {
            false => Ctl0Enable::Ctl0EnableDisable,
            true => Ctl0Enable::Ctl0EnableEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_enable_disable(&self) -> bool {
        *self == Ctl0Enable::Ctl0EnableDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_enable_enable(&self) -> bool {
        *self == Ctl0Enable::Ctl0EnableEnable
    }
}
#[doc = "Field `CTL0_ENABLE` writer - This bit enables the VREF module."]
pub type Ctl0EnableW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Enable>;
impl<'a, REG> Ctl0EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_enable_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enable::Ctl0EnableDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_enable_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enable::Ctl0EnableEnable)
    }
}
#[doc = "These bits configure output buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Bufconfig {
    #[doc = "0: HV"]
    Ctl0BufconfigOutput2p5v = 0,
    #[doc = "1: LV"]
    Ctl0BufconfigOutput1p4v = 1,
}
impl From<Ctl0Bufconfig> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Bufconfig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_BUFCONFIG` reader - These bits configure output buffer."]
pub type Ctl0BufconfigR = crate::BitReader<Ctl0Bufconfig>;
impl Ctl0BufconfigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Bufconfig {
        match self.bits {
            false => Ctl0Bufconfig::Ctl0BufconfigOutput2p5v,
            true => Ctl0Bufconfig::Ctl0BufconfigOutput1p4v,
        }
    }
    #[doc = "HV"]
    #[inline(always)]
    pub fn is_ctl0_bufconfig_output2p5v(&self) -> bool {
        *self == Ctl0Bufconfig::Ctl0BufconfigOutput2p5v
    }
    #[doc = "LV"]
    #[inline(always)]
    pub fn is_ctl0_bufconfig_output1p4v(&self) -> bool {
        *self == Ctl0Bufconfig::Ctl0BufconfigOutput1p4v
    }
}
#[doc = "Field `CTL0_BUFCONFIG` writer - These bits configure output buffer."]
pub type Ctl0BufconfigW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Bufconfig>;
impl<'a, REG> Ctl0BufconfigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HV"]
    #[inline(always)]
    pub fn ctl0_bufconfig_output2p5v(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Bufconfig::Ctl0BufconfigOutput2p5v)
    }
    #[doc = "LV"]
    #[inline(always)]
    pub fn ctl0_bufconfig_output1p4v(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Bufconfig::Ctl0BufconfigOutput1p4v)
    }
}
#[doc = "This bit enable sample and hold mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Shmode {
    #[doc = "0: DISABLE"]
    Ctl0ShmodeDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0ShmodeEnable = 1,
}
impl From<Ctl0Shmode> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Shmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_SHMODE` reader - This bit enable sample and hold mode"]
pub type Ctl0ShmodeR = crate::BitReader<Ctl0Shmode>;
impl Ctl0ShmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Shmode {
        match self.bits {
            false => Ctl0Shmode::Ctl0ShmodeDisable,
            true => Ctl0Shmode::Ctl0ShmodeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_shmode_disable(&self) -> bool {
        *self == Ctl0Shmode::Ctl0ShmodeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_shmode_enable(&self) -> bool {
        *self == Ctl0Shmode::Ctl0ShmodeEnable
    }
}
#[doc = "Field `CTL0_SHMODE` writer - This bit enable sample and hold mode"]
pub type Ctl0ShmodeW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Shmode>;
impl<'a, REG> Ctl0ShmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_shmode_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Shmode::Ctl0ShmodeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_shmode_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Shmode::Ctl0ShmodeEnable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables the VREF module."]
    #[inline(always)]
    pub fn ctl0_enable(&self) -> Ctl0EnableR {
        Ctl0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - These bits configure output buffer."]
    #[inline(always)]
    pub fn ctl0_bufconfig(&self) -> Ctl0BufconfigR {
        Ctl0BufconfigR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit enable sample and hold mode"]
    #[inline(always)]
    pub fn ctl0_shmode(&self) -> Ctl0ShmodeR {
        Ctl0ShmodeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the VREF module."]
    #[inline(always)]
    pub fn ctl0_enable(&mut self) -> Ctl0EnableW<Ctl0Spec> {
        Ctl0EnableW::new(self, 0)
    }
    #[doc = "Bit 7 - These bits configure output buffer."]
    #[inline(always)]
    pub fn ctl0_bufconfig(&mut self) -> Ctl0BufconfigW<Ctl0Spec> {
        Ctl0BufconfigW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit enable sample and hold mode"]
    #[inline(always)]
    pub fn ctl0_shmode(&mut self) -> Ctl0ShmodeW<Ctl0Spec> {
        Ctl0ShmodeW::new(self, 8)
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
