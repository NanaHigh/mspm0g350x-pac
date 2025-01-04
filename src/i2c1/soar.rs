#[doc = "Register `SOAR` reader"]
pub type R = crate::R<SoarSpec>;
#[doc = "Register `SOAR` writer"]
pub type W = crate::W<SoarSpec>;
#[doc = "Field `SOAR_OAR` reader - I2C Slave Own Address: This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
pub type SoarOarR = crate::FieldReader<u16>;
#[doc = "Field `SOAR_OAR` writer - I2C Slave Own Address: This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
pub type SoarOarW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "I2C Slave Own Address Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoarOaren {
    #[doc = "0: DISABLE"]
    SoarOarenDisable = 0,
    #[doc = "1: ENABLE"]
    SoarOarenEnable = 1,
}
impl From<SoarOaren> for bool {
    #[inline(always)]
    fn from(variant: SoarOaren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOAR_OAREN` reader - I2C Slave Own Address Enable"]
pub type SoarOarenR = crate::BitReader<SoarOaren>;
impl SoarOarenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SoarOaren {
        match self.bits {
            false => SoarOaren::SoarOarenDisable,
            true => SoarOaren::SoarOarenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_soar_oaren_disable(&self) -> bool {
        *self == SoarOaren::SoarOarenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_soar_oaren_enable(&self) -> bool {
        *self == SoarOaren::SoarOarenEnable
    }
}
#[doc = "Field `SOAR_OAREN` writer - I2C Slave Own Address Enable"]
pub type SoarOarenW<'a, REG> = crate::BitWriter<'a, REG, SoarOaren>;
impl<'a, REG> SoarOarenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn soar_oaren_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SoarOaren::SoarOarenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn soar_oaren_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SoarOaren::SoarOarenEnable)
    }
}
#[doc = "This bit selects the adressing mode to be used in slave mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoarSmode {
    #[doc = "0: MODE7"]
    SoarSmodeMode7 = 0,
    #[doc = "1: MODE10"]
    SoarSmodeMode10 = 1,
}
impl From<SoarSmode> for bool {
    #[inline(always)]
    fn from(variant: SoarSmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOAR_SMODE` reader - This bit selects the adressing mode to be used in slave mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type SoarSmodeR = crate::BitReader<SoarSmode>;
impl SoarSmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SoarSmode {
        match self.bits {
            false => SoarSmode::SoarSmodeMode7,
            true => SoarSmode::SoarSmodeMode10,
        }
    }
    #[doc = "MODE7"]
    #[inline(always)]
    pub fn is_soar_smode_mode7(&self) -> bool {
        *self == SoarSmode::SoarSmodeMode7
    }
    #[doc = "MODE10"]
    #[inline(always)]
    pub fn is_soar_smode_mode10(&self) -> bool {
        *self == SoarSmode::SoarSmodeMode10
    }
}
#[doc = "Field `SOAR_SMODE` writer - This bit selects the adressing mode to be used in slave mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type SoarSmodeW<'a, REG> = crate::BitWriter<'a, REG, SoarSmode>;
impl<'a, REG> SoarSmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODE7"]
    #[inline(always)]
    pub fn soar_smode_mode7(self) -> &'a mut crate::W<REG> {
        self.variant(SoarSmode::SoarSmodeMode7)
    }
    #[doc = "MODE10"]
    #[inline(always)]
    pub fn soar_smode_mode10(self) -> &'a mut crate::W<REG> {
        self.variant(SoarSmode::SoarSmodeMode10)
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C Slave Own Address: This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn soar_oar(&self) -> SoarOarR {
        SoarOarR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - I2C Slave Own Address Enable"]
    #[inline(always)]
    pub fn soar_oaren(&self) -> SoarOarenR {
        SoarOarenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in slave mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn soar_smode(&self) -> SoarSmodeR {
        SoarSmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Slave Own Address: This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by I2CSOAR.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn soar_oar(&mut self) -> SoarOarW<SoarSpec> {
        SoarOarW::new(self, 0)
    }
    #[doc = "Bit 14 - I2C Slave Own Address Enable"]
    #[inline(always)]
    pub fn soar_oaren(&mut self) -> SoarOarenW<SoarSpec> {
        SoarOarenW::new(self, 14)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in slave mode. When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn soar_smode(&mut self) -> SoarSmodeW<SoarSpec> {
        SoarSmodeW::new(self, 15)
    }
}
#[doc = "I2C Slave Own Address\n\nYou can [`read`](crate::Reg::read) this register and get [`soar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoarSpec;
impl crate::RegisterSpec for SoarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soar::R`](R) reader structure"]
impl crate::Readable for SoarSpec {}
#[doc = "`write(|w| ..)` method takes [`soar::W`](W) writer structure"]
impl crate::Writable for SoarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOAR to value 0x4000"]
impl crate::Resettable for SoarSpec {
    const RESET_VALUE: u32 = 0x4000;
}
