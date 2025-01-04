#[doc = "Register `DOUT11_8` writer"]
pub type W = crate::W<Dout11_8Spec>;
#[doc = "This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout11_8Dio8 {
    #[doc = "0: ZERO"]
    Dout11_8Dio8Zero = 0,
    #[doc = "1: ONE"]
    Dout11_8Dio8One = 1,
}
impl From<Dout11_8Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dout11_8Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT11_8_DIO8` writer - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
pub type Dout11_8Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dout11_8Dio8>;
impl<'a, REG> Dout11_8Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout11_8_dio8_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio8::Dout11_8Dio8Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout11_8_dio8_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio8::Dout11_8Dio8One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout11_8Dio9 {
    #[doc = "0: ZERO"]
    Dout11_8Dio9Zero = 0,
    #[doc = "1: ONE"]
    Dout11_8Dio9One = 1,
}
impl From<Dout11_8Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dout11_8Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT11_8_DIO9` writer - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
pub type Dout11_8Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dout11_8Dio9>;
impl<'a, REG> Dout11_8Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout11_8_dio9_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio9::Dout11_8Dio9Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout11_8_dio9_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio9::Dout11_8Dio9One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout11_8Dio10 {
    #[doc = "0: ZERO"]
    Dout11_8Dio10Zero = 0,
    #[doc = "1: ONE"]
    Dout11_8Dio10One = 1,
}
impl From<Dout11_8Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dout11_8Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT11_8_DIO10` writer - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
pub type Dout11_8Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dout11_8Dio10>;
impl<'a, REG> Dout11_8Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout11_8_dio10_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio10::Dout11_8Dio10Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout11_8_dio10_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio10::Dout11_8Dio10One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout11_8Dio11 {
    #[doc = "0: ZERO"]
    Dout11_8Dio11Zero = 0,
    #[doc = "1: ONE"]
    Dout11_8Dio11One = 1,
}
impl From<Dout11_8Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dout11_8Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT11_8_DIO11` writer - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
pub type Dout11_8Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dout11_8Dio11>;
impl<'a, REG> Dout11_8Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout11_8_dio11_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio11::Dout11_8Dio11Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout11_8_dio11_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout11_8Dio11::Dout11_8Dio11One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout11_8_dio8(&mut self) -> Dout11_8Dio8W<Dout11_8Spec> {
        Dout11_8Dio8W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout11_8_dio9(&mut self) -> Dout11_8Dio9W<Dout11_8Spec> {
        Dout11_8Dio9W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout11_8_dio10(&mut self) -> Dout11_8Dio10W<Dout11_8Spec> {
        Dout11_8Dio10W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout11_8_dio11(&mut self) -> Dout11_8Dio11W<Dout11_8Spec> {
        Dout11_8Dio11W::new(self, 24)
    }
}
#[doc = "Data output 11 to 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout11_8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout11_8Spec;
impl crate::RegisterSpec for Dout11_8Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout11_8::W`](W) writer structure"]
impl crate::Writable for Dout11_8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT11_8 to value 0"]
impl crate::Resettable for Dout11_8Spec {
    const RESET_VALUE: u32 = 0;
}
