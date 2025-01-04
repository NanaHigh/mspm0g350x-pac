#[doc = "Register `DOUT7_4` writer"]
pub type W = crate::W<Dout7_4Spec>;
#[doc = "This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout7_4Dio4 {
    #[doc = "0: ZERO"]
    Dout7_4Dio4Zero = 0,
    #[doc = "1: ONE"]
    Dout7_4Dio4One = 1,
}
impl From<Dout7_4Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dout7_4Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT7_4_DIO4` writer - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
pub type Dout7_4Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dout7_4Dio4>;
impl<'a, REG> Dout7_4Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout7_4_dio4_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio4::Dout7_4Dio4Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout7_4_dio4_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio4::Dout7_4Dio4One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout7_4Dio5 {
    #[doc = "0: ZERO"]
    Dout7_4Dio5Zero = 0,
    #[doc = "1: ONE"]
    Dout7_4Dio5One = 1,
}
impl From<Dout7_4Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dout7_4Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT7_4_DIO5` writer - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
pub type Dout7_4Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dout7_4Dio5>;
impl<'a, REG> Dout7_4Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout7_4_dio5_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio5::Dout7_4Dio5Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout7_4_dio5_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio5::Dout7_4Dio5One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout7_4Dio6 {
    #[doc = "0: ZERO"]
    Dout7_4Dio6Zero = 0,
    #[doc = "1: ONE"]
    Dout7_4Dio6One = 1,
}
impl From<Dout7_4Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dout7_4Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT7_4_DIO6` writer - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
pub type Dout7_4Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dout7_4Dio6>;
impl<'a, REG> Dout7_4Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout7_4_dio6_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio6::Dout7_4Dio6Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout7_4_dio6_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio6::Dout7_4Dio6One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout7_4Dio7 {
    #[doc = "0: ZERO"]
    Dout7_4Dio7Zero = 0,
    #[doc = "1: ONE"]
    Dout7_4Dio7One = 1,
}
impl From<Dout7_4Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dout7_4Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT7_4_DIO7` writer - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
pub type Dout7_4Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dout7_4Dio7>;
impl<'a, REG> Dout7_4Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout7_4_dio7_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio7::Dout7_4Dio7Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout7_4_dio7_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout7_4Dio7::Dout7_4Dio7One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout7_4_dio4(&mut self) -> Dout7_4Dio4W<Dout7_4Spec> {
        Dout7_4Dio4W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout7_4_dio5(&mut self) -> Dout7_4Dio5W<Dout7_4Spec> {
        Dout7_4Dio5W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout7_4_dio6(&mut self) -> Dout7_4Dio6W<Dout7_4Spec> {
        Dout7_4Dio6W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout7_4_dio7(&mut self) -> Dout7_4Dio7W<Dout7_4Spec> {
        Dout7_4Dio7W::new(self, 24)
    }
}
#[doc = "Data output 7 to 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout7_4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout7_4Spec;
impl crate::RegisterSpec for Dout7_4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout7_4::W`](W) writer structure"]
impl crate::Writable for Dout7_4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT7_4 to value 0"]
impl crate::Resettable for Dout7_4Spec {
    const RESET_VALUE: u32 = 0;
}
