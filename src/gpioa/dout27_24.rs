#[doc = "Register `DOUT27_24` writer"]
pub type W = crate::W<Dout27_24Spec>;
#[doc = "This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout27_24Dio24 {
    #[doc = "0: ZERO"]
    Dout27_24Dio24Zero = 0,
    #[doc = "1: ONE"]
    Dout27_24Dio24One = 1,
}
impl From<Dout27_24Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dout27_24Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT27_24_DIO24` writer - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
pub type Dout27_24Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dout27_24Dio24>;
impl<'a, REG> Dout27_24Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout27_24_dio24_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio24::Dout27_24Dio24Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout27_24_dio24_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio24::Dout27_24Dio24One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout27_24Dio25 {
    #[doc = "0: ZERO"]
    Dout27_24Dio25Zero = 0,
    #[doc = "1: ONE"]
    Dout27_24Dio25One = 1,
}
impl From<Dout27_24Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dout27_24Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT27_24_DIO25` writer - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
pub type Dout27_24Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dout27_24Dio25>;
impl<'a, REG> Dout27_24Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout27_24_dio25_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio25::Dout27_24Dio25Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout27_24_dio25_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio25::Dout27_24Dio25One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout27_24Dio26 {
    #[doc = "0: ZERO"]
    Dout27_24Dio26Zero = 0,
    #[doc = "1: ONE"]
    Dout27_24Dio26One = 1,
}
impl From<Dout27_24Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dout27_24Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT27_24_DIO26` writer - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
pub type Dout27_24Dio26W<'a, REG> = crate::BitWriter<'a, REG, Dout27_24Dio26>;
impl<'a, REG> Dout27_24Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout27_24_dio26_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio26::Dout27_24Dio26Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout27_24_dio26_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio26::Dout27_24Dio26One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout27_24Dio27 {
    #[doc = "0: ZERO"]
    Dout27_24Dio27Zero = 0,
    #[doc = "1: ONE"]
    Dout27_24Dio27One = 1,
}
impl From<Dout27_24Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dout27_24Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT27_24_DIO27` writer - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
pub type Dout27_24Dio27W<'a, REG> = crate::BitWriter<'a, REG, Dout27_24Dio27>;
impl<'a, REG> Dout27_24Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout27_24_dio27_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio27::Dout27_24Dio27Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout27_24_dio27_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout27_24Dio27::Dout27_24Dio27One)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout27_24_dio24(&mut self) -> Dout27_24Dio24W<Dout27_24Spec> {
        Dout27_24Dio24W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout27_24_dio25(&mut self) -> Dout27_24Dio25W<Dout27_24Spec> {
        Dout27_24Dio25W::new(self, 8)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout27_24_dio26(&mut self) -> Dout27_24Dio26W<Dout27_24Spec> {
        Dout27_24Dio26W::new(self, 16)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout27_24_dio27(&mut self) -> Dout27_24Dio27W<Dout27_24Spec> {
        Dout27_24Dio27W::new(self, 24)
    }
}
#[doc = "Data output 27 to 24\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout27_24::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout27_24Spec;
impl crate::RegisterSpec for Dout27_24Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dout27_24::W`](W) writer structure"]
impl crate::Writable for Dout27_24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT27_24 to value 0"]
impl crate::Resettable for Dout27_24Spec {
    const RESET_VALUE: u32 = 0;
}
