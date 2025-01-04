#[doc = "Register `MSA` reader"]
pub type R = crate::R<MsaSpec>;
#[doc = "Register `MSA` writer"]
pub type W = crate::W<MsaSpec>;
#[doc = "Receive/Send The DIR bit specifies if the next master operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsaDir {
    #[doc = "0: TRANSMIT"]
    MsaDirTransmit = 0,
    #[doc = "1: RECEIVE"]
    MsaDirReceive = 1,
}
impl From<MsaDir> for bool {
    #[inline(always)]
    fn from(variant: MsaDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSA_DIR` reader - Receive/Send The DIR bit specifies if the next master operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
pub type MsaDirR = crate::BitReader<MsaDir>;
impl MsaDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsaDir {
        match self.bits {
            false => MsaDir::MsaDirTransmit,
            true => MsaDir::MsaDirReceive,
        }
    }
    #[doc = "TRANSMIT"]
    #[inline(always)]
    pub fn is_msa_dir_transmit(&self) -> bool {
        *self == MsaDir::MsaDirTransmit
    }
    #[doc = "RECEIVE"]
    #[inline(always)]
    pub fn is_msa_dir_receive(&self) -> bool {
        *self == MsaDir::MsaDirReceive
    }
}
#[doc = "Field `MSA_DIR` writer - Receive/Send The DIR bit specifies if the next master operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
pub type MsaDirW<'a, REG> = crate::BitWriter<'a, REG, MsaDir>;
impl<'a, REG> MsaDirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRANSMIT"]
    #[inline(always)]
    pub fn msa_dir_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(MsaDir::MsaDirTransmit)
    }
    #[doc = "RECEIVE"]
    #[inline(always)]
    pub fn msa_dir_receive(self) -> &'a mut crate::W<REG> {
        self.variant(MsaDir::MsaDirReceive)
    }
}
#[doc = "Field `MSA_SADDR` reader - I2C Slave Address This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
pub type MsaSaddrR = crate::FieldReader<u16>;
#[doc = "Field `MSA_SADDR` writer - I2C Slave Address This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
pub type MsaSaddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit selects the adressing mode to be used in master mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsaMmode {
    #[doc = "0: MODE7"]
    MsaMmodeMode7 = 0,
    #[doc = "1: MODE10"]
    MsaMmodeMode10 = 1,
}
impl From<MsaMmode> for bool {
    #[inline(always)]
    fn from(variant: MsaMmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSA_MMODE` reader - This bit selects the adressing mode to be used in master mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type MsaMmodeR = crate::BitReader<MsaMmode>;
impl MsaMmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsaMmode {
        match self.bits {
            false => MsaMmode::MsaMmodeMode7,
            true => MsaMmode::MsaMmodeMode10,
        }
    }
    #[doc = "MODE7"]
    #[inline(always)]
    pub fn is_msa_mmode_mode7(&self) -> bool {
        *self == MsaMmode::MsaMmodeMode7
    }
    #[doc = "MODE10"]
    #[inline(always)]
    pub fn is_msa_mmode_mode10(&self) -> bool {
        *self == MsaMmode::MsaMmodeMode10
    }
}
#[doc = "Field `MSA_MMODE` writer - This bit selects the adressing mode to be used in master mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
pub type MsaMmodeW<'a, REG> = crate::BitWriter<'a, REG, MsaMmode>;
impl<'a, REG> MsaMmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODE7"]
    #[inline(always)]
    pub fn msa_mmode_mode7(self) -> &'a mut crate::W<REG> {
        self.variant(MsaMmode::MsaMmodeMode7)
    }
    #[doc = "MODE10"]
    #[inline(always)]
    pub fn msa_mmode_mode10(self) -> &'a mut crate::W<REG> {
        self.variant(MsaMmode::MsaMmodeMode10)
    }
}
impl R {
    #[doc = "Bit 0 - Receive/Send The DIR bit specifies if the next master operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
    #[inline(always)]
    pub fn msa_dir(&self) -> MsaDirR {
        MsaDirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - I2C Slave Address This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn msa_saddr(&self) -> MsaSaddrR {
        MsaSaddrR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in master mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn msa_mmode(&self) -> MsaMmodeR {
        MsaMmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive/Send The DIR bit specifies if the next master operation is a Receive (High) or Transmit (Low). 0h = Transmit 1h = Receive"]
    #[inline(always)]
    pub fn msa_dir(&mut self) -> MsaDirW<MsaSpec> {
        MsaDirW::new(self, 0)
    }
    #[doc = "Bits 1:10 - I2C Slave Address This field specifies bits A9 through A0 of the slave address. In 7-bit addressing mode as selected by MSA.MODE bit, the top 3 bits are don't care"]
    #[inline(always)]
    pub fn msa_saddr(&mut self) -> MsaSaddrW<MsaSpec> {
        MsaSaddrW::new(self, 1)
    }
    #[doc = "Bit 15 - This bit selects the adressing mode to be used in master mode When 0, 7-bit addressing is used. When 1, 10-bit addressing is used."]
    #[inline(always)]
    pub fn msa_mmode(&mut self) -> MsaMmodeW<MsaSpec> {
        MsaMmodeW::new(self, 15)
    }
}
#[doc = "I2C Master Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsaSpec;
impl crate::RegisterSpec for MsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msa::R`](R) reader structure"]
impl crate::Readable for MsaSpec {}
#[doc = "`write(|w| ..)` method takes [`msa::W`](W) writer structure"]
impl crate::Writable for MsaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSA to value 0"]
impl crate::Resettable for MsaSpec {
    const RESET_VALUE: u32 = 0;
}
