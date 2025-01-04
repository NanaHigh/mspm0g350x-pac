#[doc = "Register `MCTR` reader"]
pub type R = crate::R<MctrSpec>;
#[doc = "Register `MCTR` writer"]
pub type W = crate::W<MctrSpec>;
#[doc = "I2C Master Enable and start transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MctrBurstrun {
    #[doc = "0: DISABLE"]
    MctrBurstrunDisable = 0,
    #[doc = "1: ENABLE"]
    MctrBurstrunEnable = 1,
}
impl From<MctrBurstrun> for bool {
    #[inline(always)]
    fn from(variant: MctrBurstrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTR_BURSTRUN` reader - I2C Master Enable and start transaction"]
pub type MctrBurstrunR = crate::BitReader<MctrBurstrun>;
impl MctrBurstrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MctrBurstrun {
        match self.bits {
            false => MctrBurstrun::MctrBurstrunDisable,
            true => MctrBurstrun::MctrBurstrunEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mctr_burstrun_disable(&self) -> bool {
        *self == MctrBurstrun::MctrBurstrunDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mctr_burstrun_enable(&self) -> bool {
        *self == MctrBurstrun::MctrBurstrunEnable
    }
}
#[doc = "Field `MCTR_BURSTRUN` writer - I2C Master Enable and start transaction"]
pub type MctrBurstrunW<'a, REG> = crate::BitWriter<'a, REG, MctrBurstrun>;
impl<'a, REG> MctrBurstrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mctr_burstrun_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrBurstrun::MctrBurstrunDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mctr_burstrun_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrBurstrun::MctrBurstrunEnable)
    }
}
#[doc = "Generate START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MctrStart {
    #[doc = "0: DISABLE"]
    MctrStartDisable = 0,
    #[doc = "1: ENABLE"]
    MctrStartEnable = 1,
}
impl From<MctrStart> for bool {
    #[inline(always)]
    fn from(variant: MctrStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTR_START` reader - Generate START"]
pub type MctrStartR = crate::BitReader<MctrStart>;
impl MctrStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MctrStart {
        match self.bits {
            false => MctrStart::MctrStartDisable,
            true => MctrStart::MctrStartEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mctr_start_disable(&self) -> bool {
        *self == MctrStart::MctrStartDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mctr_start_enable(&self) -> bool {
        *self == MctrStart::MctrStartEnable
    }
}
#[doc = "Field `MCTR_START` writer - Generate START"]
pub type MctrStartW<'a, REG> = crate::BitWriter<'a, REG, MctrStart>;
impl<'a, REG> MctrStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mctr_start_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrStart::MctrStartDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mctr_start_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrStart::MctrStartEnable)
    }
}
#[doc = "Generate STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MctrStop {
    #[doc = "0: DISABLE"]
    MctrStopDisable = 0,
    #[doc = "1: ENABLE"]
    MctrStopEnable = 1,
}
impl From<MctrStop> for bool {
    #[inline(always)]
    fn from(variant: MctrStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTR_STOP` reader - Generate STOP"]
pub type MctrStopR = crate::BitReader<MctrStop>;
impl MctrStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MctrStop {
        match self.bits {
            false => MctrStop::MctrStopDisable,
            true => MctrStop::MctrStopEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mctr_stop_disable(&self) -> bool {
        *self == MctrStop::MctrStopDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mctr_stop_enable(&self) -> bool {
        *self == MctrStop::MctrStopEnable
    }
}
#[doc = "Field `MCTR_STOP` writer - Generate STOP"]
pub type MctrStopW<'a, REG> = crate::BitWriter<'a, REG, MctrStop>;
impl<'a, REG> MctrStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mctr_stop_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrStop::MctrStopDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mctr_stop_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrStop::MctrStopEnable)
    }
}
#[doc = "Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK. See field decoding in Table: MCTR Field decoding.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MctrAck {
    #[doc = "0: DISABLE"]
    MctrAckDisable = 0,
    #[doc = "1: ENABLE"]
    MctrAckEnable = 1,
}
impl From<MctrAck> for bool {
    #[inline(always)]
    fn from(variant: MctrAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTR_ACK` reader - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK. See field decoding in Table: MCTR Field decoding."]
pub type MctrAckR = crate::BitReader<MctrAck>;
impl MctrAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MctrAck {
        match self.bits {
            false => MctrAck::MctrAckDisable,
            true => MctrAck::MctrAckEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mctr_ack_disable(&self) -> bool {
        *self == MctrAck::MctrAckDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mctr_ack_enable(&self) -> bool {
        *self == MctrAck::MctrAckEnable
    }
}
#[doc = "Field `MCTR_ACK` writer - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK. See field decoding in Table: MCTR Field decoding."]
pub type MctrAckW<'a, REG> = crate::BitWriter<'a, REG, MctrAck>;
impl<'a, REG> MctrAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mctr_ack_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrAck::MctrAckDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mctr_ack_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrAck::MctrAckEnable)
    }
}
#[doc = "Master ACK overrride Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MctrMackoen {
    #[doc = "0: DISABLE"]
    MctrMackoenDisable = 0,
    #[doc = "1: ENABLE"]
    MctrMackoenEnable = 1,
}
impl From<MctrMackoen> for bool {
    #[inline(always)]
    fn from(variant: MctrMackoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTR_MACKOEN` reader - Master ACK overrride Enable"]
pub type MctrMackoenR = crate::BitReader<MctrMackoen>;
impl MctrMackoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MctrMackoen {
        match self.bits {
            false => MctrMackoen::MctrMackoenDisable,
            true => MctrMackoen::MctrMackoenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mctr_mackoen_disable(&self) -> bool {
        *self == MctrMackoen::MctrMackoenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mctr_mackoen_enable(&self) -> bool {
        *self == MctrMackoen::MctrMackoenEnable
    }
}
#[doc = "Field `MCTR_MACKOEN` writer - Master ACK overrride Enable"]
pub type MctrMackoenW<'a, REG> = crate::BitWriter<'a, REG, MctrMackoen>;
impl<'a, REG> MctrMackoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mctr_mackoen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrMackoen::MctrMackoenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mctr_mackoen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrMackoen::MctrMackoenEnable)
    }
}
#[doc = "Read on TX Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MctrRdOnTxempty {
    #[doc = "0: DISABLE"]
    MctrRdOnTxemptyDisable = 0,
    #[doc = "1: ENABLE"]
    MctrRdOnTxemptyEnable = 1,
}
impl From<MctrRdOnTxempty> for bool {
    #[inline(always)]
    fn from(variant: MctrRdOnTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCTR_RD_ON_TXEMPTY` reader - Read on TX Empty"]
pub type MctrRdOnTxemptyR = crate::BitReader<MctrRdOnTxempty>;
impl MctrRdOnTxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MctrRdOnTxempty {
        match self.bits {
            false => MctrRdOnTxempty::MctrRdOnTxemptyDisable,
            true => MctrRdOnTxempty::MctrRdOnTxemptyEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mctr_rd_on_txempty_disable(&self) -> bool {
        *self == MctrRdOnTxempty::MctrRdOnTxemptyDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mctr_rd_on_txempty_enable(&self) -> bool {
        *self == MctrRdOnTxempty::MctrRdOnTxemptyEnable
    }
}
#[doc = "Field `MCTR_RD_ON_TXEMPTY` writer - Read on TX Empty"]
pub type MctrRdOnTxemptyW<'a, REG> = crate::BitWriter<'a, REG, MctrRdOnTxempty>;
impl<'a, REG> MctrRdOnTxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mctr_rd_on_txempty_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrRdOnTxempty::MctrRdOnTxemptyDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mctr_rd_on_txempty_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MctrRdOnTxempty::MctrRdOnTxemptyEnable)
    }
}
#[doc = "Field `MCTR_MBLEN` reader - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
pub type MctrMblenR = crate::FieldReader<u16>;
#[doc = "Field `MCTR_MBLEN` writer - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
pub type MctrMblenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - I2C Master Enable and start transaction"]
    #[inline(always)]
    pub fn mctr_burstrun(&self) -> MctrBurstrunR {
        MctrBurstrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn mctr_start(&self) -> MctrStartR {
        MctrStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn mctr_stop(&self) -> MctrStopR {
        MctrStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn mctr_ack(&self) -> MctrAckR {
        MctrAckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master ACK overrride Enable"]
    #[inline(always)]
    pub fn mctr_mackoen(&self) -> MctrMackoenR {
        MctrMackoenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read on TX Empty"]
    #[inline(always)]
    pub fn mctr_rd_on_txempty(&self) -> MctrRdOnTxemptyR {
        MctrRdOnTxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:27 - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
    #[inline(always)]
    pub fn mctr_mblen(&self) -> MctrMblenR {
        MctrMblenR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Master Enable and start transaction"]
    #[inline(always)]
    pub fn mctr_burstrun(&mut self) -> MctrBurstrunW<MctrSpec> {
        MctrBurstrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn mctr_start(&mut self) -> MctrStartW<MctrSpec> {
        MctrStartW::new(self, 1)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn mctr_stop(&mut self) -> MctrStopW<MctrSpec> {
        MctrStopW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable. Software needs to configure this bit to send the ACK or NACK. See field decoding in Table: MCTR Field decoding."]
    #[inline(always)]
    pub fn mctr_ack(&mut self) -> MctrAckW<MctrSpec> {
        MctrAckW::new(self, 3)
    }
    #[doc = "Bit 4 - Master ACK overrride Enable"]
    #[inline(always)]
    pub fn mctr_mackoen(&mut self) -> MctrMackoenW<MctrSpec> {
        MctrMackoenW::new(self, 4)
    }
    #[doc = "Bit 5 - Read on TX Empty"]
    #[inline(always)]
    pub fn mctr_rd_on_txempty(&mut self) -> MctrRdOnTxemptyW<MctrSpec> {
        MctrRdOnTxemptyW::new(self, 5)
    }
    #[doc = "Bits 16:27 - I2C transaction length This field contains the programmed length of bytes of the Transaction."]
    #[inline(always)]
    pub fn mctr_mblen(&mut self) -> MctrMblenW<MctrSpec> {
        MctrMblenW::new(self, 16)
    }
}
#[doc = "I2C Master Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctrSpec;
impl crate::RegisterSpec for MctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctr::R`](R) reader structure"]
impl crate::Readable for MctrSpec {}
#[doc = "`write(|w| ..)` method takes [`mctr::W`](W) writer structure"]
impl crate::Writable for MctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCTR to value 0"]
impl crate::Resettable for MctrSpec {
    const RESET_VALUE: u32 = 0;
}
