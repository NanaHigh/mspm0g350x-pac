#[doc = "Register `DOUT31_28` writer"]
pub type W = crate::W<Dout31_28Spec>;
#[doc = "This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_28Dio28 {
    #[doc = "0: ZERO"]
    Dout31_28Dio28Zero = 0,
    #[doc = "1: ONE"]
    Dout31_28Dio28One = 1,
}
impl From<Dout31_28Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dout31_28Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_28_DIO28` writer - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
pub type Dout31_28Dio28W<'a, REG> = crate::BitWriter<'a, REG, Dout31_28Dio28>;
impl<'a, REG> Dout31_28Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_28_dio28_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio28::Dout31_28Dio28Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_28_dio28_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio28::Dout31_28Dio28One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_28Dio29 {
    #[doc = "0: ZERO"]
    Dout31_28Dio29Zero = 0,
    #[doc = "1: ONE"]
    Dout31_28Dio29One = 1,
}
impl From<Dout31_28Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dout31_28Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_28_DIO29` writer - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
pub type Dout31_28Dio29W<'a, REG> = crate::BitWriter<'a, REG, Dout31_28Dio29>;
impl<'a, REG> Dout31_28Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_28_dio29_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio29::Dout31_28Dio29Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_28_dio29_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio29::Dout31_28Dio29One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_28Dio30 {
    #[doc = "0: ZERO"]
    Dout31_28Dio30Zero = 0,
    #[doc = "1: ONE"]
    Dout31_28Dio30One = 1,
}
impl From<Dout31_28Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dout31_28Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_28_DIO30` writer - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
pub type Dout31_28Dio30W<'a, REG> = crate::BitWriter<'a, REG, Dout31_28Dio30>;
impl<'a, REG> Dout31_28Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_28_dio30_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio30::Dout31_28Dio30Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_28_dio30_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio30::Dout31_28Dio30One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_28Dio31 {
    #[doc = "0: ZERO"]
    Dout31_28Dio31Zero = 0,
    #[doc = "1: ONE"]
    Dout31_28Dio31One = 1,
}
impl From<Dout31_28Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dout31_28Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_28_DIO31` writer - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
pub type Dout31_28Dio31W<'a, REG> = crate::BitWriter<'a, REG, Dout31_28Dio31>;
impl<'a, REG> Dout31_28Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_28_dio31_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio31::Dout31_28Dio31Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_28_dio31_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_28Dio31::Dout31_28Dio31One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_28_dio28(&mut self) -> Dout31_28Dio28W<Dout31_28Spec> {
        Dout31_28Dio28W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_28_dio29(&mut self) -> Dout31_28Dio29W<Dout31_28Spec> {
        Dout31_28Dio29W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_28_dio30(&mut self) -> Dout31_28Dio30W<Dout31_28Spec> {
        Dout31_28Dio30W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_28_dio31(&mut self) -> Dout31_28Dio31W<Dout31_28Spec> {
        Dout31_28Dio31W::new(self, 24)
    }
}
#[doc = "Data output 31 to 28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout31_28::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout31_28Spec;
impl crate::RegisterSpec for Dout31_28Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout31_28::W`](W) writer structure"]
impl crate::Writable for Dout31_28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT31_28 to value 0"]
impl crate::Resettable for Dout31_28Spec {
    const RESET_VALUE: u32 = 0;
}
