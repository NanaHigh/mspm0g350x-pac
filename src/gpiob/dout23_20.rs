#[doc = "Register `DOUT23_20` writer"]
pub type W = crate::W<Dout23_20Spec>;
#[doc = "This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout23_20Dio20 {
    #[doc = "0: ZERO"]
    Dout23_20Dio20Zero = 0,
    #[doc = "1: ONE"]
    Dout23_20Dio20One = 1,
}
impl From<Dout23_20Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dout23_20Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT23_20_DIO20` writer - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
pub type Dout23_20Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dout23_20Dio20>;
impl<'a, REG> Dout23_20Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout23_20_dio20_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio20::Dout23_20Dio20Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout23_20_dio20_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio20::Dout23_20Dio20One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout23_20Dio21 {
    #[doc = "0: ZERO"]
    Dout23_20Dio21Zero = 0,
    #[doc = "1: ONE"]
    Dout23_20Dio21One = 1,
}
impl From<Dout23_20Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dout23_20Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT23_20_DIO21` writer - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
pub type Dout23_20Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dout23_20Dio21>;
impl<'a, REG> Dout23_20Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout23_20_dio21_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio21::Dout23_20Dio21Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout23_20_dio21_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio21::Dout23_20Dio21One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout23_20Dio22 {
    #[doc = "0: ZERO"]
    Dout23_20Dio22Zero = 0,
    #[doc = "1: ONE"]
    Dout23_20Dio22One = 1,
}
impl From<Dout23_20Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dout23_20Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT23_20_DIO22` writer - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
pub type Dout23_20Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dout23_20Dio22>;
impl<'a, REG> Dout23_20Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout23_20_dio22_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio22::Dout23_20Dio22Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout23_20_dio22_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio22::Dout23_20Dio22One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout23_20Dio23 {
    #[doc = "0: ZERO"]
    Dout23_20Dio23Zero = 0,
    #[doc = "1: ONE"]
    Dout23_20Dio23One = 1,
}
impl From<Dout23_20Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dout23_20Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT23_20_DIO23` writer - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
pub type Dout23_20Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dout23_20Dio23>;
impl<'a, REG> Dout23_20Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout23_20_dio23_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio23::Dout23_20Dio23Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout23_20_dio23_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout23_20Dio23::Dout23_20Dio23One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout23_20_dio20(&mut self) -> Dout23_20Dio20W<Dout23_20Spec> {
        Dout23_20Dio20W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout23_20_dio21(&mut self) -> Dout23_20Dio21W<Dout23_20Spec> {
        Dout23_20Dio21W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout23_20_dio22(&mut self) -> Dout23_20Dio22W<Dout23_20Spec> {
        Dout23_20Dio22W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout23_20_dio23(&mut self) -> Dout23_20Dio23W<Dout23_20Spec> {
        Dout23_20Dio23W::new(self, 24)
    }
}
#[doc = "Data output 23 to 20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout23_20::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout23_20Spec;
impl crate::RegisterSpec for Dout23_20Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout23_20::W`](W) writer structure"]
impl crate::Writable for Dout23_20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT23_20 to value 0"]
impl crate::Resettable for Dout23_20Spec {
    const RESET_VALUE: u32 = 0;
}
