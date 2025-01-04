#[doc = "Register `DOUT3_0` writer"]
pub type W = crate::W<Dout3_0Spec>;
#[doc = "This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout3_0Dio0 {
    #[doc = "0: ZERO"]
    Dout3_0Dio0Zero = 0,
    #[doc = "1: ONE"]
    Dout3_0Dio0One = 1,
}
impl From<Dout3_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dout3_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT3_0_DIO0` writer - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
pub type Dout3_0Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dout3_0Dio0>;
impl<'a, REG> Dout3_0Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout3_0_dio0_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio0::Dout3_0Dio0Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout3_0_dio0_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio0::Dout3_0Dio0One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout3_0Dio1 {
    #[doc = "0: ZERO"]
    Dout3_0Dio1Zero = 0,
    #[doc = "1: ONE"]
    Dout3_0Dio1One = 1,
}
impl From<Dout3_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dout3_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT3_0_DIO1` writer - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
pub type Dout3_0Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dout3_0Dio1>;
impl<'a, REG> Dout3_0Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout3_0_dio1_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio1::Dout3_0Dio1Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout3_0_dio1_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio1::Dout3_0Dio1One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout3_0Dio2 {
    #[doc = "0: ZERO"]
    Dout3_0Dio2Zero = 0,
    #[doc = "1: ONE"]
    Dout3_0Dio2One = 1,
}
impl From<Dout3_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dout3_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT3_0_DIO2` writer - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
pub type Dout3_0Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dout3_0Dio2>;
impl<'a, REG> Dout3_0Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout3_0_dio2_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio2::Dout3_0Dio2Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout3_0_dio2_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio2::Dout3_0Dio2One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout3_0Dio3 {
    #[doc = "0: ZERO"]
    Dout3_0Dio3Zero = 0,
    #[doc = "1: ONE"]
    Dout3_0Dio3One = 1,
}
impl From<Dout3_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dout3_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT3_0_DIO3` writer - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
pub type Dout3_0Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dout3_0Dio3>;
impl<'a, REG> Dout3_0Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout3_0_dio3_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio3::Dout3_0Dio3Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout3_0_dio3_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout3_0Dio3::Dout3_0Dio3One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout3_0_dio0(&mut self) -> Dout3_0Dio0W<Dout3_0Spec> {
        Dout3_0Dio0W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout3_0_dio1(&mut self) -> Dout3_0Dio1W<Dout3_0Spec> {
        Dout3_0Dio1W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout3_0_dio2(&mut self) -> Dout3_0Dio2W<Dout3_0Spec> {
        Dout3_0Dio2W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout3_0_dio3(&mut self) -> Dout3_0Dio3W<Dout3_0Spec> {
        Dout3_0Dio3W::new(self, 24)
    }
}
#[doc = "Data output 3 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout3_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout3_0Spec;
impl crate::RegisterSpec for Dout3_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout3_0::W`](W) writer structure"]
impl crate::Writable for Dout3_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT3_0 to value 0"]
impl crate::Resettable for Dout3_0Spec {
    const RESET_VALUE: u32 = 0;
}
