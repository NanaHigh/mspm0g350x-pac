#[doc = "Register `SOAR2` reader"]
pub type R = crate::R<Soar2Spec>;
#[doc = "Register `SOAR2` writer"]
pub type W = crate::W<Soar2Spec>;
#[doc = "Field `SOAR2_OAR2` reader - I2C Slave Own Address 2 This field specifies the alternate OAR2 address."]
pub type Soar2Oar2R = crate::FieldReader;
#[doc = "Field `SOAR2_OAR2` writer - I2C Slave Own Address 2 This field specifies the alternate OAR2 address."]
pub type Soar2Oar2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "I2C Slave Own Address 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soar2Oar2en {
    #[doc = "0: DISABLE"]
    Soar2Oar2enDisable = 0,
    #[doc = "1: ENABLE"]
    Soar2Oar2enEnable = 1,
}
impl From<Soar2Oar2en> for bool {
    #[inline(always)]
    fn from(variant: Soar2Oar2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOAR2_OAR2EN` reader - I2C Slave Own Address 2 Enable"]
pub type Soar2Oar2enR = crate::BitReader<Soar2Oar2en>;
impl Soar2Oar2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soar2Oar2en {
        match self.bits {
            false => Soar2Oar2en::Soar2Oar2enDisable,
            true => Soar2Oar2en::Soar2Oar2enEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_soar2_oar2en_disable(&self) -> bool {
        *self == Soar2Oar2en::Soar2Oar2enDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_soar2_oar2en_enable(&self) -> bool {
        *self == Soar2Oar2en::Soar2Oar2enEnable
    }
}
#[doc = "Field `SOAR2_OAR2EN` writer - I2C Slave Own Address 2 Enable"]
pub type Soar2Oar2enW<'a, REG> = crate::BitWriter<'a, REG, Soar2Oar2en>;
impl<'a, REG> Soar2Oar2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn soar2_oar2en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Soar2Oar2en::Soar2Oar2enDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn soar2_oar2en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Soar2Oar2en::Soar2Oar2enEnable)
    }
}
#[doc = "Field `SOAR2_OAR2_MASK` reader - I2C Slave Own Address 2 Mask: This field specifies bits A6 through A0 of the slave address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
pub type Soar2Oar2MaskR = crate::FieldReader;
#[doc = "Field `SOAR2_OAR2_MASK` writer - I2C Slave Own Address 2 Mask: This field specifies bits A6 through A0 of the slave address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
pub type Soar2Oar2MaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2 This field specifies the alternate OAR2 address."]
    #[inline(always)]
    pub fn soar2_oar2(&self) -> Soar2Oar2R {
        Soar2Oar2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    pub fn soar2_oar2en(&self) -> Soar2Oar2enR {
        Soar2Oar2enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:22 - I2C Slave Own Address 2 Mask: This field specifies bits A6 through A0 of the slave address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
    #[inline(always)]
    pub fn soar2_oar2_mask(&self) -> Soar2Oar2MaskR {
        Soar2Oar2MaskR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2 This field specifies the alternate OAR2 address."]
    #[inline(always)]
    pub fn soar2_oar2(&mut self) -> Soar2Oar2W<Soar2Spec> {
        Soar2Oar2W::new(self, 0)
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    pub fn soar2_oar2en(&mut self) -> Soar2Oar2enW<Soar2Spec> {
        Soar2Oar2enW::new(self, 7)
    }
    #[doc = "Bits 16:22 - I2C Slave Own Address 2 Mask: This field specifies bits A6 through A0 of the slave address. The bits with value 1 in SOAR2.OAR2_MASK field will make the corresponding incoming address bits to match by default regardless of the value inside SOAR2.OAR2 i.e. corresponding SOAR2.OAR2 bit is a dont care."]
    #[inline(always)]
    pub fn soar2_oar2_mask(&mut self) -> Soar2Oar2MaskW<Soar2Spec> {
        Soar2Oar2MaskW::new(self, 16)
    }
}
#[doc = "I2C Slave Own Address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`soar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soar2Spec;
impl crate::RegisterSpec for Soar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soar2::R`](R) reader structure"]
impl crate::Readable for Soar2Spec {}
#[doc = "`write(|w| ..)` method takes [`soar2::W`](W) writer structure"]
impl crate::Writable for Soar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOAR2 to value 0"]
impl crate::Resettable for Soar2Spec {
    const RESET_VALUE: u32 = 0;
}
