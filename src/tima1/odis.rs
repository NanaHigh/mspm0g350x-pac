#[doc = "Register `ODIS` reader"]
pub type R = crate::R<OdisSpec>;
#[doc = "Register `ODIS` writer"]
pub type W = crate::W<OdisSpec>;
#[doc = "Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OdisC0ccp0 {
    #[doc = "0: CCP_OUTPUT_OCTL"]
    OdisC0ccp0CcpOutputOctl = 0,
    #[doc = "1: CCP_OUTPUT_LOW"]
    OdisC0ccp0CcpOutputLow = 1,
}
impl From<OdisC0ccp0> for bool {
    #[inline(always)]
    fn from(variant: OdisC0ccp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODIS_C0CCP0` reader - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type OdisC0ccp0R = crate::BitReader<OdisC0ccp0>;
impl OdisC0ccp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OdisC0ccp0 {
        match self.bits {
            false => OdisC0ccp0::OdisC0ccp0CcpOutputOctl,
            true => OdisC0ccp0::OdisC0ccp0CcpOutputLow,
        }
    }
    #[doc = "CCP_OUTPUT_OCTL"]
    #[inline(always)]
    pub fn is_odis_c0ccp0_ccp_output_octl(&self) -> bool {
        *self == OdisC0ccp0::OdisC0ccp0CcpOutputOctl
    }
    #[doc = "CCP_OUTPUT_LOW"]
    #[inline(always)]
    pub fn is_odis_c0ccp0_ccp_output_low(&self) -> bool {
        *self == OdisC0ccp0::OdisC0ccp0CcpOutputLow
    }
}
#[doc = "Field `ODIS_C0CCP0` writer - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type OdisC0ccp0W<'a, REG> = crate::BitWriter<'a, REG, OdisC0ccp0>;
impl<'a, REG> OdisC0ccp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCP_OUTPUT_OCTL"]
    #[inline(always)]
    pub fn odis_c0ccp0_ccp_output_octl(self) -> &'a mut crate::W<REG> {
        self.variant(OdisC0ccp0::OdisC0ccp0CcpOutputOctl)
    }
    #[doc = "CCP_OUTPUT_LOW"]
    #[inline(always)]
    pub fn odis_c0ccp0_ccp_output_low(self) -> &'a mut crate::W<REG> {
        self.variant(OdisC0ccp0::OdisC0ccp0CcpOutputLow)
    }
}
#[doc = "Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OdisC0ccp1 {
    #[doc = "0: CCP_OUTPUT_OCTL"]
    OdisC0ccp1CcpOutputOctl = 0,
    #[doc = "1: CCP_OUTPUT_LOW"]
    OdisC0ccp1CcpOutputLow = 1,
}
impl From<OdisC0ccp1> for bool {
    #[inline(always)]
    fn from(variant: OdisC0ccp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODIS_C0CCP1` reader - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type OdisC0ccp1R = crate::BitReader<OdisC0ccp1>;
impl OdisC0ccp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OdisC0ccp1 {
        match self.bits {
            false => OdisC0ccp1::OdisC0ccp1CcpOutputOctl,
            true => OdisC0ccp1::OdisC0ccp1CcpOutputLow,
        }
    }
    #[doc = "CCP_OUTPUT_OCTL"]
    #[inline(always)]
    pub fn is_odis_c0ccp1_ccp_output_octl(&self) -> bool {
        *self == OdisC0ccp1::OdisC0ccp1CcpOutputOctl
    }
    #[doc = "CCP_OUTPUT_LOW"]
    #[inline(always)]
    pub fn is_odis_c0ccp1_ccp_output_low(&self) -> bool {
        *self == OdisC0ccp1::OdisC0ccp1CcpOutputLow
    }
}
#[doc = "Field `ODIS_C0CCP1` writer - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
pub type OdisC0ccp1W<'a, REG> = crate::BitWriter<'a, REG, OdisC0ccp1>;
impl<'a, REG> OdisC0ccp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCP_OUTPUT_OCTL"]
    #[inline(always)]
    pub fn odis_c0ccp1_ccp_output_octl(self) -> &'a mut crate::W<REG> {
        self.variant(OdisC0ccp1::OdisC0ccp1CcpOutputOctl)
    }
    #[doc = "CCP_OUTPUT_LOW"]
    #[inline(always)]
    pub fn odis_c0ccp1_ccp_output_low(self) -> &'a mut crate::W<REG> {
        self.variant(OdisC0ccp1::OdisC0ccp1CcpOutputLow)
    }
}
impl R {
    #[doc = "Bit 0 - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn odis_c0ccp0(&self) -> OdisC0ccp0R {
        OdisC0ccp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn odis_c0ccp1(&self) -> OdisC0ccp1R {
        OdisC0ccp1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter CCP0 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn odis_c0ccp0(&mut self) -> OdisC0ccp0W<OdisSpec> {
        OdisC0ccp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter CCP1 Disable Mask Defines whether CCP0 of Counter n is forced low or not"]
    #[inline(always)]
    pub fn odis_c0ccp1(&mut self) -> OdisC0ccp1W<OdisSpec> {
        OdisC0ccp1W::new(self, 1)
    }
}
#[doc = "Output Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`odis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdisSpec;
impl crate::RegisterSpec for OdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odis::R`](R) reader structure"]
impl crate::Readable for OdisSpec {}
#[doc = "`write(|w| ..)` method takes [`odis::W`](W) writer structure"]
impl crate::Writable for OdisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODIS to value 0"]
impl crate::Resettable for OdisSpec {
    const RESET_VALUE: u32 = 0;
}
