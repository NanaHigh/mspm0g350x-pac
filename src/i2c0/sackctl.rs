#[doc = "Register `SACKCTL` reader"]
pub type R = crate::R<SackctlSpec>;
#[doc = "Register `SACKCTL` writer"]
pub type W = crate::W<SackctlSpec>;
#[doc = "I2C Slave ACK Override Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SackctlAckoen {
    #[doc = "0: DISABLE"]
    SackctlAckoenDisable = 0,
    #[doc = "1: ENABLE"]
    SackctlAckoenEnable = 1,
}
impl From<SackctlAckoen> for bool {
    #[inline(always)]
    fn from(variant: SackctlAckoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKCTL_ACKOEN` reader - I2C Slave ACK Override Enable"]
pub type SackctlAckoenR = crate::BitReader<SackctlAckoen>;
impl SackctlAckoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SackctlAckoen {
        match self.bits {
            false => SackctlAckoen::SackctlAckoenDisable,
            true => SackctlAckoen::SackctlAckoenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_disable(&self) -> bool {
        *self == SackctlAckoen::SackctlAckoenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_enable(&self) -> bool {
        *self == SackctlAckoen::SackctlAckoenEnable
    }
}
#[doc = "Field `SACKCTL_ACKOEN` writer - I2C Slave ACK Override Enable"]
pub type SackctlAckoenW<'a, REG> = crate::BitWriter<'a, REG, SackctlAckoen>;
impl<'a, REG> SackctlAckoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoen::SackctlAckoenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoen::SackctlAckoenEnable)
    }
}
#[doc = "I2C Slave ACK Override Value Note: for General Call this bit will be ignored if set to NACK and slave continues to receive data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SackctlAckoval {
    #[doc = "0: DISABLE"]
    SackctlAckovalDisable = 0,
    #[doc = "1: ENABLE"]
    SackctlAckovalEnable = 1,
}
impl From<SackctlAckoval> for bool {
    #[inline(always)]
    fn from(variant: SackctlAckoval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKCTL_ACKOVAL` reader - I2C Slave ACK Override Value Note: for General Call this bit will be ignored if set to NACK and slave continues to receive data."]
pub type SackctlAckovalR = crate::BitReader<SackctlAckoval>;
impl SackctlAckovalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SackctlAckoval {
        match self.bits {
            false => SackctlAckoval::SackctlAckovalDisable,
            true => SackctlAckoval::SackctlAckovalEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoval_disable(&self) -> bool {
        *self == SackctlAckoval::SackctlAckovalDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoval_enable(&self) -> bool {
        *self == SackctlAckoval::SackctlAckovalEnable
    }
}
#[doc = "Field `SACKCTL_ACKOVAL` writer - I2C Slave ACK Override Value Note: for General Call this bit will be ignored if set to NACK and slave continues to receive data."]
pub type SackctlAckovalW<'a, REG> = crate::BitWriter<'a, REG, SackctlAckoval>;
impl<'a, REG> SackctlAckovalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sackctl_ackoval_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoval::SackctlAckovalDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sackctl_ackoval_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoval::SackctlAckovalEnable)
    }
}
#[doc = "When set this bit will automatically turn on the Slave ACKOEN field following a Start Condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SackctlAckoenOnStart {
    #[doc = "0: DISABLE"]
    SackctlAckoenOnStartDisable = 0,
    #[doc = "1: ENABLE"]
    SackctlAckoenOnStartEnable = 1,
}
impl From<SackctlAckoenOnStart> for bool {
    #[inline(always)]
    fn from(variant: SackctlAckoenOnStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKCTL_ACKOEN_ON_START` reader - When set this bit will automatically turn on the Slave ACKOEN field following a Start Condition."]
pub type SackctlAckoenOnStartR = crate::BitReader<SackctlAckoenOnStart>;
impl SackctlAckoenOnStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SackctlAckoenOnStart {
        match self.bits {
            false => SackctlAckoenOnStart::SackctlAckoenOnStartDisable,
            true => SackctlAckoenOnStart::SackctlAckoenOnStartEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_on_start_disable(&self) -> bool {
        *self == SackctlAckoenOnStart::SackctlAckoenOnStartDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_on_start_enable(&self) -> bool {
        *self == SackctlAckoenOnStart::SackctlAckoenOnStartEnable
    }
}
#[doc = "Field `SACKCTL_ACKOEN_ON_START` writer - When set this bit will automatically turn on the Slave ACKOEN field following a Start Condition."]
pub type SackctlAckoenOnStartW<'a, REG> = crate::BitWriter<'a, REG, SackctlAckoenOnStart>;
impl<'a, REG> SackctlAckoenOnStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_on_start_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoenOnStart::SackctlAckoenOnStartDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_on_start_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoenOnStart::SackctlAckoenOnStartEnable)
    }
}
#[doc = "When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing SLAVE_SACKCTL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SackctlAckoenOnPecnext {
    #[doc = "0: DISABLE"]
    SackctlAckoenOnPecnextDisable = 0,
    #[doc = "1: ENABLE"]
    SackctlAckoenOnPecnextEnable = 1,
}
impl From<SackctlAckoenOnPecnext> for bool {
    #[inline(always)]
    fn from(variant: SackctlAckoenOnPecnext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKCTL_ACKOEN_ON_PECNEXT` reader - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing SLAVE_SACKCTL."]
pub type SackctlAckoenOnPecnextR = crate::BitReader<SackctlAckoenOnPecnext>;
impl SackctlAckoenOnPecnextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SackctlAckoenOnPecnext {
        match self.bits {
            false => SackctlAckoenOnPecnext::SackctlAckoenOnPecnextDisable,
            true => SackctlAckoenOnPecnext::SackctlAckoenOnPecnextEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_on_pecnext_disable(&self) -> bool {
        *self == SackctlAckoenOnPecnext::SackctlAckoenOnPecnextDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_on_pecnext_enable(&self) -> bool {
        *self == SackctlAckoenOnPecnext::SackctlAckoenOnPecnextEnable
    }
}
#[doc = "Field `SACKCTL_ACKOEN_ON_PECNEXT` writer - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing SLAVE_SACKCTL."]
pub type SackctlAckoenOnPecnextW<'a, REG> = crate::BitWriter<'a, REG, SackctlAckoenOnPecnext>;
impl<'a, REG> SackctlAckoenOnPecnextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecnext_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoenOnPecnext::SackctlAckoenOnPecnextDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecnext_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoenOnPecnext::SackctlAckoenOnPecnextEnable)
    }
}
#[doc = "When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the received PEC byte.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SackctlAckoenOnPecdone {
    #[doc = "0: DISABLE"]
    SackctlAckoenOnPecdoneDisable = 0,
    #[doc = "1: ENABLE"]
    SackctlAckoenOnPecdoneEnable = 1,
}
impl From<SackctlAckoenOnPecdone> for bool {
    #[inline(always)]
    fn from(variant: SackctlAckoenOnPecdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKCTL_ACKOEN_ON_PECDONE` reader - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the received PEC byte."]
pub type SackctlAckoenOnPecdoneR = crate::BitReader<SackctlAckoenOnPecdone>;
impl SackctlAckoenOnPecdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SackctlAckoenOnPecdone {
        match self.bits {
            false => SackctlAckoenOnPecdone::SackctlAckoenOnPecdoneDisable,
            true => SackctlAckoenOnPecdone::SackctlAckoenOnPecdoneEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_on_pecdone_disable(&self) -> bool {
        *self == SackctlAckoenOnPecdone::SackctlAckoenOnPecdoneDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sackctl_ackoen_on_pecdone_enable(&self) -> bool {
        *self == SackctlAckoenOnPecdone::SackctlAckoenOnPecdoneEnable
    }
}
#[doc = "Field `SACKCTL_ACKOEN_ON_PECDONE` writer - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the received PEC byte."]
pub type SackctlAckoenOnPecdoneW<'a, REG> = crate::BitWriter<'a, REG, SackctlAckoenOnPecdone>;
impl<'a, REG> SackctlAckoenOnPecdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecdone_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoenOnPecdone::SackctlAckoenOnPecdoneDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecdone_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SackctlAckoenOnPecdone::SackctlAckoenOnPecdoneEnable)
    }
}
impl R {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    pub fn sackctl_ackoen(&self) -> SackctlAckoenR {
        SackctlAckoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value Note: for General Call this bit will be ignored if set to NACK and slave continues to receive data."]
    #[inline(always)]
    pub fn sackctl_ackoval(&self) -> SackctlAckovalR {
        SackctlAckovalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set this bit will automatically turn on the Slave ACKOEN field following a Start Condition."]
    #[inline(always)]
    pub fn sackctl_ackoen_on_start(&self) -> SackctlAckoenOnStartR {
        SackctlAckoenOnStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing SLAVE_SACKCTL."]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecnext(&self) -> SackctlAckoenOnPecnextR {
        SackctlAckoenOnPecnextR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the received PEC byte."]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecdone(&self) -> SackctlAckoenOnPecdoneR {
        SackctlAckoenOnPecdoneR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    pub fn sackctl_ackoen(&mut self) -> SackctlAckoenW<SackctlSpec> {
        SackctlAckoenW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value Note: for General Call this bit will be ignored if set to NACK and slave continues to receive data."]
    #[inline(always)]
    pub fn sackctl_ackoval(&mut self) -> SackctlAckovalW<SackctlSpec> {
        SackctlAckovalW::new(self, 1)
    }
    #[doc = "Bit 2 - When set this bit will automatically turn on the Slave ACKOEN field following a Start Condition."]
    #[inline(always)]
    pub fn sackctl_ackoen_on_start(&mut self) -> SackctlAckoenOnStartW<SackctlSpec> {
        SackctlAckoenOnStartW::new(self, 2)
    }
    #[doc = "Bit 3 - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the byte received just prior to the PEC byte. Note that when ACKOEN is set the PEC byte will not automatically be ACKed/NACKed by the State Machine and FW must perform this function by writing SLAVE_SACKCTL."]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecnext(&mut self) -> SackctlAckoenOnPecnextW<SackctlSpec> {
        SackctlAckoenOnPecnextW::new(self, 3)
    }
    #[doc = "Bit 4 - When set this bit will automatically turn on the Slave ACKOEN field following the ACK/NACK of the received PEC byte."]
    #[inline(always)]
    pub fn sackctl_ackoen_on_pecdone(&mut self) -> SackctlAckoenOnPecdoneW<SackctlSpec> {
        SackctlAckoenOnPecdoneW::new(self, 4)
    }
}
#[doc = "I2C Slave ACK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sackctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sackctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SackctlSpec;
impl crate::RegisterSpec for SackctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sackctl::R`](R) reader structure"]
impl crate::Readable for SackctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sackctl::W`](W) writer structure"]
impl crate::Writable for SackctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SACKCTL to value 0"]
impl crate::Resettable for SackctlSpec {
    const RESET_VALUE: u32 = 0;
}
