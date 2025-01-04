#[doc = "Register `SFIFOCTL` reader"]
pub type R = crate::R<SfifoctlSpec>;
#[doc = "Register `SFIFOCTL` writer"]
pub type W = crate::W<SfifoctlSpec>;
#[doc = "TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SfifoctlTxtrig {
    #[doc = "4: LEVEL_4"]
    SfifoctlTxtrigLevel4 = 4,
    #[doc = "5: LEVEL_5"]
    SfifoctlTxtrigLevel5 = 5,
    #[doc = "6: LEVEL_6"]
    SfifoctlTxtrigLevel6 = 6,
    #[doc = "7: LEVEL_7"]
    SfifoctlTxtrigLevel7 = 7,
}
impl From<SfifoctlTxtrig> for u8 {
    #[inline(always)]
    fn from(variant: SfifoctlTxtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SfifoctlTxtrig {
    type Ux = u8;
}
impl crate::IsEnum for SfifoctlTxtrig {}
#[doc = "Field `SFIFOCTL_TXTRIG` reader - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
pub type SfifoctlTxtrigR = crate::FieldReader<SfifoctlTxtrig>;
impl SfifoctlTxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SfifoctlTxtrig> {
        match self.bits {
            4 => Some(SfifoctlTxtrig::SfifoctlTxtrigLevel4),
            5 => Some(SfifoctlTxtrig::SfifoctlTxtrigLevel5),
            6 => Some(SfifoctlTxtrig::SfifoctlTxtrigLevel6),
            7 => Some(SfifoctlTxtrig::SfifoctlTxtrigLevel7),
            _ => None,
        }
    }
    #[doc = "LEVEL_4"]
    #[inline(always)]
    pub fn is_sfifoctl_txtrig_level_4(&self) -> bool {
        *self == SfifoctlTxtrig::SfifoctlTxtrigLevel4
    }
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn is_sfifoctl_txtrig_level_5(&self) -> bool {
        *self == SfifoctlTxtrig::SfifoctlTxtrigLevel5
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn is_sfifoctl_txtrig_level_6(&self) -> bool {
        *self == SfifoctlTxtrig::SfifoctlTxtrigLevel6
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn is_sfifoctl_txtrig_level_7(&self) -> bool {
        *self == SfifoctlTxtrig::SfifoctlTxtrigLevel7
    }
}
#[doc = "Field `SFIFOCTL_TXTRIG` writer - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
pub type SfifoctlTxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 3, SfifoctlTxtrig>;
impl<'a, REG> SfifoctlTxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LEVEL_4"]
    #[inline(always)]
    pub fn sfifoctl_txtrig_level_4(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlTxtrig::SfifoctlTxtrigLevel4)
    }
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn sfifoctl_txtrig_level_5(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlTxtrig::SfifoctlTxtrigLevel5)
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn sfifoctl_txtrig_level_6(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlTxtrig::SfifoctlTxtrigLevel6)
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn sfifoctl_txtrig_level_7(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlTxtrig::SfifoctlTxtrigLevel7)
    }
}
#[doc = "TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfifoctlTxflush {
    #[doc = "0: NOFLUSH"]
    SfifoctlTxflushNoflush = 0,
    #[doc = "1: FLUSH"]
    SfifoctlTxflushFlush = 1,
}
impl From<SfifoctlTxflush> for bool {
    #[inline(always)]
    fn from(variant: SfifoctlTxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFIFOCTL_TXFLUSH` reader - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
pub type SfifoctlTxflushR = crate::BitReader<SfifoctlTxflush>;
impl SfifoctlTxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SfifoctlTxflush {
        match self.bits {
            false => SfifoctlTxflush::SfifoctlTxflushNoflush,
            true => SfifoctlTxflush::SfifoctlTxflushFlush,
        }
    }
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn is_sfifoctl_txflush_noflush(&self) -> bool {
        *self == SfifoctlTxflush::SfifoctlTxflushNoflush
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn is_sfifoctl_txflush_flush(&self) -> bool {
        *self == SfifoctlTxflush::SfifoctlTxflushFlush
    }
}
#[doc = "Field `SFIFOCTL_TXFLUSH` writer - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
pub type SfifoctlTxflushW<'a, REG> = crate::BitWriter<'a, REG, SfifoctlTxflush>;
impl<'a, REG> SfifoctlTxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn sfifoctl_txflush_noflush(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlTxflush::SfifoctlTxflushNoflush)
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn sfifoctl_txflush_flush(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlTxflush::SfifoctlTxflushFlush)
    }
}
#[doc = "RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SfifoctlRxtrig {
    #[doc = "4: LEVEL_5"]
    SfifoctlRxtrigLevel5 = 4,
    #[doc = "5: LEVEL_6"]
    SfifoctlRxtrigLevel6 = 5,
    #[doc = "6: LEVEL_7"]
    SfifoctlRxtrigLevel7 = 6,
    #[doc = "7: LEVEL_8"]
    SfifoctlRxtrigLevel8 = 7,
}
impl From<SfifoctlRxtrig> for u8 {
    #[inline(always)]
    fn from(variant: SfifoctlRxtrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SfifoctlRxtrig {
    type Ux = u8;
}
impl crate::IsEnum for SfifoctlRxtrig {}
#[doc = "Field `SFIFOCTL_RXTRIG` reader - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
pub type SfifoctlRxtrigR = crate::FieldReader<SfifoctlRxtrig>;
impl SfifoctlRxtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SfifoctlRxtrig> {
        match self.bits {
            4 => Some(SfifoctlRxtrig::SfifoctlRxtrigLevel5),
            5 => Some(SfifoctlRxtrig::SfifoctlRxtrigLevel6),
            6 => Some(SfifoctlRxtrig::SfifoctlRxtrigLevel7),
            7 => Some(SfifoctlRxtrig::SfifoctlRxtrigLevel8),
            _ => None,
        }
    }
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn is_sfifoctl_rxtrig_level_5(&self) -> bool {
        *self == SfifoctlRxtrig::SfifoctlRxtrigLevel5
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn is_sfifoctl_rxtrig_level_6(&self) -> bool {
        *self == SfifoctlRxtrig::SfifoctlRxtrigLevel6
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn is_sfifoctl_rxtrig_level_7(&self) -> bool {
        *self == SfifoctlRxtrig::SfifoctlRxtrigLevel7
    }
    #[doc = "LEVEL_8"]
    #[inline(always)]
    pub fn is_sfifoctl_rxtrig_level_8(&self) -> bool {
        *self == SfifoctlRxtrig::SfifoctlRxtrigLevel8
    }
}
#[doc = "Field `SFIFOCTL_RXTRIG` writer - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
pub type SfifoctlRxtrigW<'a, REG> = crate::FieldWriter<'a, REG, 3, SfifoctlRxtrig>;
impl<'a, REG> SfifoctlRxtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LEVEL_5"]
    #[inline(always)]
    pub fn sfifoctl_rxtrig_level_5(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlRxtrig::SfifoctlRxtrigLevel5)
    }
    #[doc = "LEVEL_6"]
    #[inline(always)]
    pub fn sfifoctl_rxtrig_level_6(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlRxtrig::SfifoctlRxtrigLevel6)
    }
    #[doc = "LEVEL_7"]
    #[inline(always)]
    pub fn sfifoctl_rxtrig_level_7(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlRxtrig::SfifoctlRxtrigLevel7)
    }
    #[doc = "LEVEL_8"]
    #[inline(always)]
    pub fn sfifoctl_rxtrig_level_8(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlRxtrig::SfifoctlRxtrigLevel8)
    }
}
#[doc = "RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfifoctlRxflush {
    #[doc = "0: NOFLUSH"]
    SfifoctlRxflushNoflush = 0,
    #[doc = "1: FLUSH"]
    SfifoctlRxflushFlush = 1,
}
impl From<SfifoctlRxflush> for bool {
    #[inline(always)]
    fn from(variant: SfifoctlRxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFIFOCTL_RXFLUSH` reader - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
pub type SfifoctlRxflushR = crate::BitReader<SfifoctlRxflush>;
impl SfifoctlRxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SfifoctlRxflush {
        match self.bits {
            false => SfifoctlRxflush::SfifoctlRxflushNoflush,
            true => SfifoctlRxflush::SfifoctlRxflushFlush,
        }
    }
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn is_sfifoctl_rxflush_noflush(&self) -> bool {
        *self == SfifoctlRxflush::SfifoctlRxflushNoflush
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn is_sfifoctl_rxflush_flush(&self) -> bool {
        *self == SfifoctlRxflush::SfifoctlRxflushFlush
    }
}
#[doc = "Field `SFIFOCTL_RXFLUSH` writer - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
pub type SfifoctlRxflushW<'a, REG> = crate::BitWriter<'a, REG, SfifoctlRxflush>;
impl<'a, REG> SfifoctlRxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOFLUSH"]
    #[inline(always)]
    pub fn sfifoctl_rxflush_noflush(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlRxflush::SfifoctlRxflushNoflush)
    }
    #[doc = "FLUSH"]
    #[inline(always)]
    pub fn sfifoctl_rxflush_flush(self) -> &'a mut crate::W<REG> {
        self.variant(SfifoctlRxflush::SfifoctlRxflushFlush)
    }
}
impl R {
    #[doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
    #[inline(always)]
    pub fn sfifoctl_txtrig(&self) -> SfifoctlTxtrigR {
        SfifoctlTxtrigR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn sfifoctl_txflush(&self) -> SfifoctlTxflushR {
        SfifoctlTxflushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
    #[inline(always)]
    pub fn sfifoctl_rxtrig(&self) -> SfifoctlRxtrigR {
        SfifoctlRxtrigR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn sfifoctl_rxflush(&self) -> SfifoctlRxflushR {
        SfifoctlRxflushR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TX FIFO Trigger Indicates at what fill level in the TX FIFO a trigger will be generated."]
    #[inline(always)]
    pub fn sfifoctl_txtrig(&mut self) -> SfifoctlTxtrigW<SfifoctlSpec> {
        SfifoctlTxtrigW::new(self, 0)
    }
    #[doc = "Bit 7 - TX FIFO Flush Setting this bit will Flush the TX FIFO. Before reseting this bit to stop Flush the TXFIFOCNT should be checked to be 8 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn sfifoctl_txflush(&mut self) -> SfifoctlTxflushW<SfifoctlSpec> {
        SfifoctlTxflushW::new(self, 7)
    }
    #[doc = "Bits 8:10 - RX FIFO Trigger Indicates at what fill level in the RX FIFO a trigger will be generated. Note: Programming RXTRIG to 0x0 has no effect since no data is present to transfer out of RX FIFO."]
    #[inline(always)]
    pub fn sfifoctl_rxtrig(&mut self) -> SfifoctlRxtrigW<SfifoctlSpec> {
        SfifoctlRxtrigW::new(self, 8)
    }
    #[doc = "Bit 15 - RX FIFO Flush Setting this bit will Flush the RX FIFO. Before reseting this bit to stop Flush the RXFIFOCNT should be checked to be 0 and indicating that the Flush has completed."]
    #[inline(always)]
    pub fn sfifoctl_rxflush(&mut self) -> SfifoctlRxflushW<SfifoctlSpec> {
        SfifoctlRxflushW::new(self, 15)
    }
}
#[doc = "I2C Slave FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifoctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfifoctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfifoctlSpec;
impl crate::RegisterSpec for SfifoctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfifoctl::R`](R) reader structure"]
impl crate::Readable for SfifoctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sfifoctl::W`](W) writer structure"]
impl crate::Writable for SfifoctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SFIFOCTL to value 0"]
impl crate::Resettable for SfifoctlSpec {
    const RESET_VALUE: u32 = 0;
}
