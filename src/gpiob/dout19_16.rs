#[doc = "Register `DOUT19_16` writer"]
pub type W = crate::W<Dout19_16Spec>;
#[doc = "This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout19_16Dio16 {
    #[doc = "0: ZERO"]
    Dout19_16Dio16Zero = 0,
    #[doc = "1: ONE"]
    Dout19_16Dio16One = 1,
}
impl From<Dout19_16Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dout19_16Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT19_16_DIO16` writer - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
pub type Dout19_16Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dout19_16Dio16>;
impl<'a, REG> Dout19_16Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout19_16_dio16_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio16::Dout19_16Dio16Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout19_16_dio16_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio16::Dout19_16Dio16One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout19_16Dio17 {
    #[doc = "0: ZERO"]
    Dout19_16Dio17Zero = 0,
    #[doc = "1: ONE"]
    Dout19_16Dio17One = 1,
}
impl From<Dout19_16Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dout19_16Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT19_16_DIO17` writer - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
pub type Dout19_16Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dout19_16Dio17>;
impl<'a, REG> Dout19_16Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout19_16_dio17_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio17::Dout19_16Dio17Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout19_16_dio17_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio17::Dout19_16Dio17One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout19_16Dio18 {
    #[doc = "0: ZERO"]
    Dout19_16Dio18Zero = 0,
    #[doc = "1: ONE"]
    Dout19_16Dio18One = 1,
}
impl From<Dout19_16Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dout19_16Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT19_16_DIO18` writer - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
pub type Dout19_16Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dout19_16Dio18>;
impl<'a, REG> Dout19_16Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout19_16_dio18_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio18::Dout19_16Dio18Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout19_16_dio18_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio18::Dout19_16Dio18One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout19_16Dio19 {
    #[doc = "0: ZERO"]
    Dout19_16Dio19Zero = 0,
    #[doc = "1: ONE"]
    Dout19_16Dio19One = 1,
}
impl From<Dout19_16Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dout19_16Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT19_16_DIO19` writer - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
pub type Dout19_16Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dout19_16Dio19>;
impl<'a, REG> Dout19_16Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout19_16_dio19_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio19::Dout19_16Dio19Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout19_16_dio19_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout19_16Dio19::Dout19_16Dio19One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout19_16_dio16(&mut self) -> Dout19_16Dio16W<Dout19_16Spec> {
        Dout19_16Dio16W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout19_16_dio17(&mut self) -> Dout19_16Dio17W<Dout19_16Spec> {
        Dout19_16Dio17W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout19_16_dio18(&mut self) -> Dout19_16Dio18W<Dout19_16Spec> {
        Dout19_16Dio18W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout19_16_dio19(&mut self) -> Dout19_16Dio19W<Dout19_16Spec> {
        Dout19_16Dio19W::new(self, 24)
    }
}
#[doc = "Data output 19 to 16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout19_16::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout19_16Spec;
impl crate::RegisterSpec for Dout19_16Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout19_16::W`](W) writer structure"]
impl crate::Writable for Dout19_16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT19_16 to value 0"]
impl crate::Resettable for Dout19_16Spec {
    const RESET_VALUE: u32 = 0;
}
