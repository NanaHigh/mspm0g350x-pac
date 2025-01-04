#[doc = "Register `DOUT15_12` writer"]
pub type W = crate::W<Dout15_12Spec>;
#[doc = "This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout15_12Dio12 {
    #[doc = "0: ZERO"]
    Dout15_12Dio12Zero = 0,
    #[doc = "1: ONE"]
    Dout15_12Dio12One = 1,
}
impl From<Dout15_12Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dout15_12Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT15_12_DIO12` writer - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
pub type Dout15_12Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dout15_12Dio12>;
impl<'a, REG> Dout15_12Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout15_12_dio12_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio12::Dout15_12Dio12Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout15_12_dio12_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio12::Dout15_12Dio12One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout15_12Dio13 {
    #[doc = "0: ZERO"]
    Dout15_12Dio13Zero = 0,
    #[doc = "1: ONE"]
    Dout15_12Dio13One = 1,
}
impl From<Dout15_12Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dout15_12Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT15_12_DIO13` writer - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
pub type Dout15_12Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dout15_12Dio13>;
impl<'a, REG> Dout15_12Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout15_12_dio13_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio13::Dout15_12Dio13Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout15_12_dio13_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio13::Dout15_12Dio13One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout15_12Dio14 {
    #[doc = "0: ZERO"]
    Dout15_12Dio14Zero = 0,
    #[doc = "1: ONE"]
    Dout15_12Dio14One = 1,
}
impl From<Dout15_12Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dout15_12Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT15_12_DIO14` writer - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
pub type Dout15_12Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dout15_12Dio14>;
impl<'a, REG> Dout15_12Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout15_12_dio14_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio14::Dout15_12Dio14Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout15_12_dio14_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio14::Dout15_12Dio14One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout15_12Dio15 {
    #[doc = "0: ZERO"]
    Dout15_12Dio15Zero = 0,
    #[doc = "1: ONE"]
    Dout15_12Dio15One = 1,
}
impl From<Dout15_12Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dout15_12Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT15_12_DIO15` writer - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
pub type Dout15_12Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dout15_12Dio15>;
impl<'a, REG> Dout15_12Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout15_12_dio15_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio15::Dout15_12Dio15Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout15_12_dio15_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout15_12Dio15::Dout15_12Dio15One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout15_12_dio12(&mut self) -> Dout15_12Dio12W<Dout15_12Spec> {
        Dout15_12Dio12W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout15_12_dio13(&mut self) -> Dout15_12Dio13W<Dout15_12Spec> {
        Dout15_12Dio13W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout15_12_dio14(&mut self) -> Dout15_12Dio14W<Dout15_12Spec> {
        Dout15_12Dio14W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout15_12_dio15(&mut self) -> Dout15_12Dio15W<Dout15_12Spec> {
        Dout15_12Dio15W::new(self, 24)
    }
}
#[doc = "Data output 15 to 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout15_12::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout15_12Spec;
impl crate::RegisterSpec for Dout15_12Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout15_12::W`](W) writer structure"]
impl crate::Writable for Dout15_12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT15_12 to value 0"]
impl crate::Resettable for Dout15_12Spec {
    const RESET_VALUE: u32 = 0;
}
