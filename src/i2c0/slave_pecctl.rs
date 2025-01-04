#[doc = "Register `SLAVE_PECCTL` reader"]
pub type R = crate::R<SlavePecctlSpec>;
#[doc = "Register `SLAVE_PECCTL` writer"]
pub type W = crate::W<SlavePecctlSpec>;
#[doc = "Field `SLAVE_PECCTL_PECCNT` reader - When this field is non zero, the number of I2C data bytes are counted. When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Slave use case, FW would set PECEN=1 and PECCNT=0 and use the ACKOEN until the remaining SMB packet length is known. FW would then set the PECCNT to the remaining packet length (Including PEC bye). FW would then configure DMA to allow the packet to complete unassisted and exit NoAck mode. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction"]
pub type SlavePecctlPeccntR = crate::FieldReader<u16>;
#[doc = "Field `SLAVE_PECCTL_PECCNT` writer - When this field is non zero, the number of I2C data bytes are counted. When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Slave use case, FW would set PECEN=1 and PECCNT=0 and use the ACKOEN until the remaining SMB packet length is known. FW would then set the PECCNT to the remaining packet length (Including PEC bye). FW would then configure DMA to allow the packet to complete unassisted and exit NoAck mode. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction"]
pub type SlavePecctlPeccntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlavePecctlPecen {
    #[doc = "0: DISABLE"]
    SlavePecctlPecenDisable = 0,
    #[doc = "1: ENABLE"]
    SlavePecctlPecenEnable = 1,
}
impl From<SlavePecctlPecen> for bool {
    #[inline(always)]
    fn from(variant: SlavePecctlPecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE_PECCTL_PECEN` reader - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
pub type SlavePecctlPecenR = crate::BitReader<SlavePecctlPecen>;
impl SlavePecctlPecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlavePecctlPecen {
        match self.bits {
            false => SlavePecctlPecen::SlavePecctlPecenDisable,
            true => SlavePecctlPecen::SlavePecctlPecenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_slave_pecctl_pecen_disable(&self) -> bool {
        *self == SlavePecctlPecen::SlavePecctlPecenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_slave_pecctl_pecen_enable(&self) -> bool {
        *self == SlavePecctlPecen::SlavePecctlPecenEnable
    }
}
#[doc = "Field `SLAVE_PECCTL_PECEN` writer - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
pub type SlavePecctlPecenW<'a, REG> = crate::BitWriter<'a, REG, SlavePecctlPecen>;
impl<'a, REG> SlavePecctlPecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn slave_pecctl_pecen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SlavePecctlPecen::SlavePecctlPecenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn slave_pecctl_pecen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SlavePecctlPecen::SlavePecctlPecenEnable)
    }
}
impl R {
    #[doc = "Bits 0:8 - When this field is non zero, the number of I2C data bytes are counted. When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Slave use case, FW would set PECEN=1 and PECCNT=0 and use the ACKOEN until the remaining SMB packet length is known. FW would then set the PECCNT to the remaining packet length (Including PEC bye). FW would then configure DMA to allow the packet to complete unassisted and exit NoAck mode. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction"]
    #[inline(always)]
    pub fn slave_pecctl_peccnt(&self) -> SlavePecctlPeccntR {
        SlavePecctlPeccntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
    #[inline(always)]
    pub fn slave_pecctl_pecen(&self) -> SlavePecctlPecenR {
        SlavePecctlPecenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - When this field is non zero, the number of I2C data bytes are counted. When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Slave use case, FW would set PECEN=1 and PECCNT=0 and use the ACKOEN until the remaining SMB packet length is known. FW would then set the PECCNT to the remaining packet length (Including PEC bye). FW would then configure DMA to allow the packet to complete unassisted and exit NoAck mode. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction"]
    #[inline(always)]
    pub fn slave_pecctl_peccnt(&mut self) -> SlavePecctlPeccntW<SlavePecctlSpec> {
        SlavePecctlPeccntW::new(self, 0)
    }
    #[doc = "Bit 12 - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
    #[inline(always)]
    pub fn slave_pecctl_pecen(&mut self) -> SlavePecctlPecenW<SlavePecctlSpec> {
        SlavePecctlPecenW::new(self, 12)
    }
}
#[doc = "I2C Slave PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_pecctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_pecctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlavePecctlSpec;
impl crate::RegisterSpec for SlavePecctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_pecctl::R`](R) reader structure"]
impl crate::Readable for SlavePecctlSpec {}
#[doc = "`write(|w| ..)` method takes [`slave_pecctl::W`](W) writer structure"]
impl crate::Writable for SlavePecctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE_PECCTL to value 0"]
impl crate::Resettable for SlavePecctlSpec {
    const RESET_VALUE: u32 = 0;
}
