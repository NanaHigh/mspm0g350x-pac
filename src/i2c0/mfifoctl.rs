#[doc = "Register `MFIFOCTL` reader"]
pub type R = crate::R<MfifoctlSpec>;
#[doc = "Register `MFIFOCTL` writer"]
pub type W = crate::W<MfifoctlSpec>;
#[doc = "TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MfifoctlTxtrig {
    #[doc = "4: LEVEL_4"]
    MfifoctlTxtrigLevel4 = 4,
    #[doc = "5: LEVEL_5"]
    MfifoctlTxtrigLevel5 = 5,
    #[doc = "6: LEVEL_6"]
    MfifoctlTxtrigLevel6 = 6,
    #[doc = "7: LEVEL_7"]
    MfifoctlTxtrigLevel7 = 7,
}
impl From<MfifoctlTxtrig> for u8 {
    #[inline(always)]
    fn from(variant: MfifoctlTxtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MfifoctlTxtrig {
    type Ux = u8;
}
impl crate::IsEnum for MfifoctlTxtrig {}
#[doc = "Field `MFIFOCTL_TXTRIG` reader - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
pub type MfifoctlTxtrigR = crate::FieldReader<MfifoctlTxtrig>;
impl MfifoctlTxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MfifoctlTxtrig> {
        match self.bits {
            4 => Some(MfifoctlTxtrig::MfifoctlTxtrigLevel4),
            5 => Some(MfifoctlTxtrig::MfifoctlTxtrigLevel5),
            6 => Some(MfifoctlTxtrig::MfifoctlTxtrigLevel6),
            7 => Some(MfifoctlTxtrig::MfifoctlTxtrigLevel7),
            _ => None,
        }
    }
    #[doc = "LEVEL_4"]
    #[inline(always)]
    pub fn is_mfifoctl_txtrig_level_4(&self) -> bool {
        *self == MfifoctlTxtrig::MfifoctlTxtrigLevel4
    }
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn is_mfifoctl_txtrig_level_5(&self) -> bool {
        *self == MfifoctlTxtrig::MfifoctlTxtrigLevel5
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn is_mfifoctl_txtrig_level_6(&self) -> bool {
        *self == MfifoctlTxtrig::MfifoctlTxtrigLevel6
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn is_mfifoctl_txtrig_level_7(&self) -> bool {
        *self == MfifoctlTxtrig::MfifoctlTxtrigLevel7
    }
}
#[doc = "Field `MFIFOCTL_TXTRIG` writer - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
pub type MfifoctlTxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 3, MfifoctlTxtrig>;
impl<'a, REG> MfifoctlTxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LEVEL_4"]
    #[inline(always)]
    pub fn mfifoctl_txtrig_level_4(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlTxtrig::MfifoctlTxtrigLevel4)
    }
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn mfifoctl_txtrig_level_5(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlTxtrig::MfifoctlTxtrigLevel5)
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn mfifoctl_txtrig_level_6(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlTxtrig::MfifoctlTxtrigLevel6)
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn mfifoctl_txtrig_level_7(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlTxtrig::MfifoctlTxtrigLevel7)
    }
}
#[doc = "TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfifoctlTxflush {
    #[doc = "0: NOFLUSH"]
    MfifoctlTxflushNoflush = 0,
    #[doc = "1: FLUSH"]
    MfifoctlTxflushFlush = 1,
}
impl From<MfifoctlTxflush> for bool {
    #[inline(always)]
    fn from(variant: MfifoctlTxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFIFOCTL_TXFLUSH` reader - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
pub type MfifoctlTxflushR = crate::BitReader<MfifoctlTxflush>;
impl MfifoctlTxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MfifoctlTxflush {
        match self.bits {
            false => MfifoctlTxflush::MfifoctlTxflushNoflush,
            true => MfifoctlTxflush::MfifoctlTxflushFlush,
        }
    }
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn is_mfifoctl_txflush_noflush(&self) -> bool {
        *self == MfifoctlTxflush::MfifoctlTxflushNoflush
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn is_mfifoctl_txflush_flush(&self) -> bool {
        *self == MfifoctlTxflush::MfifoctlTxflushFlush
    }
}
#[doc = "Field `MFIFOCTL_TXFLUSH` writer - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
pub type MfifoctlTxflushW<'a, REG> = crate::BitWriter<'a, REG, MfifoctlTxflush>;
impl<'a, REG> MfifoctlTxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn mfifoctl_txflush_noflush(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlTxflush::MfifoctlTxflushNoflush)
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn mfifoctl_txflush_flush(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlTxflush::MfifoctlTxflushFlush)
    }
}
#[doc = "RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MfifoctlRxtrig {
    #[doc = "4: LEVEL_5"]
    MfifoctlRxtrigLevel5 = 4,
    #[doc = "5: LEVEL_6"]
    MfifoctlRxtrigLevel6 = 5,
    #[doc = "6: LEVEL_7"]
    MfifoctlRxtrigLevel7 = 6,
    #[doc = "7: LEVEL_8"]
    MfifoctlRxtrigLevel8 = 7,
}
impl From<MfifoctlRxtrig> for u8 {
    #[inline(always)]
    fn from(variant: MfifoctlRxtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MfifoctlRxtrig {
    type Ux = u8;
}
impl crate::IsEnum for MfifoctlRxtrig {}
#[doc = "Field `MFIFOCTL_RXTRIG` reader - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
pub type MfifoctlRxtrigR = crate::FieldReader<MfifoctlRxtrig>;
impl MfifoctlRxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MfifoctlRxtrig> {
        match self.bits {
            4 => Some(MfifoctlRxtrig::MfifoctlRxtrigLevel5),
            5 => Some(MfifoctlRxtrig::MfifoctlRxtrigLevel6),
            6 => Some(MfifoctlRxtrig::MfifoctlRxtrigLevel7),
            7 => Some(MfifoctlRxtrig::MfifoctlRxtrigLevel8),
            _ => None,
        }
    }
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn is_mfifoctl_rxtrig_level_5(&self) -> bool {
        *self == MfifoctlRxtrig::MfifoctlRxtrigLevel5
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn is_mfifoctl_rxtrig_level_6(&self) -> bool {
        *self == MfifoctlRxtrig::MfifoctlRxtrigLevel6
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn is_mfifoctl_rxtrig_level_7(&self) -> bool {
        *self == MfifoctlRxtrig::MfifoctlRxtrigLevel7
    }
    #[doc = "LEVEL_8"]
    #[inline(always)]
    pub fn is_mfifoctl_rxtrig_level_8(&self) -> bool {
        *self == MfifoctlRxtrig::MfifoctlRxtrigLevel8
    }
}
#[doc = "Field `MFIFOCTL_RXTRIG` writer - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
pub type MfifoctlRxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 3, MfifoctlRxtrig>;
impl<'a, REG> MfifoctlRxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn mfifoctl_rxtrig_level_5(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlRxtrig::MfifoctlRxtrigLevel5)
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn mfifoctl_rxtrig_level_6(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlRxtrig::MfifoctlRxtrigLevel6)
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn mfifoctl_rxtrig_level_7(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlRxtrig::MfifoctlRxtrigLevel7)
    }
    #[doc = "LEVEL_8"]
    #[inline(always)]
    pub fn mfifoctl_rxtrig_level_8(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlRxtrig::MfifoctlRxtrigLevel8)
    }
}
#[doc = "RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfifoctlRxflush {
    #[doc = "0: NOFLUSH"]
    MfifoctlRxflushNoflush = 0,
    #[doc = "1: FLUSH"]
    MfifoctlRxflushFlush = 1,
}
impl From<MfifoctlRxflush> for bool {
    #[inline(always)]
    fn from(variant: MfifoctlRxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFIFOCTL_RXFLUSH` reader - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
pub type MfifoctlRxflushR = crate::BitReader<MfifoctlRxflush>;
impl MfifoctlRxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MfifoctlRxflush {
        match self.bits {
            false => MfifoctlRxflush::MfifoctlRxflushNoflush,
            true => MfifoctlRxflush::MfifoctlRxflushFlush,
        }
    }
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn is_mfifoctl_rxflush_noflush(&self) -> bool {
        *self == MfifoctlRxflush::MfifoctlRxflushNoflush
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn is_mfifoctl_rxflush_flush(&self) -> bool {
        *self == MfifoctlRxflush::MfifoctlRxflushFlush
    }
}
#[doc = "Field `MFIFOCTL_RXFLUSH` writer - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
pub type MfifoctlRxflushW<'a, REG> = crate::BitWriter<'a, REG, MfifoctlRxflush>;
impl<'a, REG> MfifoctlRxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn mfifoctl_rxflush_noflush(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlRxflush::MfifoctlRxflushNoflush)
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn mfifoctl_rxflush_flush(self) -> &'a mut crate::W<REG> {
        self.variant(MfifoctlRxflush::MfifoctlRxflushFlush)
    }
}
impl R {
    #[doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
    #[inline(always)]
    pub fn mfifoctl_txtrig(&self) -> MfifoctlTxtrigR {
        MfifoctlTxtrigR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn mfifoctl_txflush(&self) -> MfifoctlTxflushR {
        MfifoctlTxflushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
    #[inline(always)]
    pub fn mfifoctl_rxtrig(&self) -> MfifoctlRxtrigR {
        MfifoctlRxtrigR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn mfifoctl_rxflush(&self) -> MfifoctlRxflushR {
        MfifoctlRxflushR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
    #[inline(always)]
    pub fn mfifoctl_txtrig(&mut self) -> MfifoctlTxtrigW<MfifoctlSpec> {
        MfifoctlTxtrigW::new(self, 0)
    }
    #[doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn mfifoctl_txflush(&mut self) -> MfifoctlTxflushW<MfifoctlSpec> {
        MfifoctlTxflushW::new(self, 7)
    }
    #[doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
    #[inline(always)]
    pub fn mfifoctl_rxtrig(&mut self) -> MfifoctlRxtrigW<MfifoctlSpec> {
        MfifoctlRxtrigW::new(self, 8)
    }
    #[doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn mfifoctl_rxflush(&mut self) -> MfifoctlRxflushW<MfifoctlSpec> {
        MfifoctlRxflushW::new(self, 15)
    }
}
#[doc = "I2C Master FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfifoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MfifoctlSpec;
impl crate::RegisterSpec for MfifoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfifoctl::R`](R) reader structure"]
impl crate::Readable for MfifoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mfifoctl::W`](W) writer structure"]
impl crate::Writable for MfifoctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MFIFOCTL to value 0"]
impl crate::Resettable for MfifoctlSpec {
    const RESET_VALUE: u32 = 0;
}
