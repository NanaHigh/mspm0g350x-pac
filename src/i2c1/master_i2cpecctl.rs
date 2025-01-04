#[doc = "Register `MASTER_I2CPECCTL` reader"]
pub type R = crate::R<MasterI2cpecctlSpec>;
#[doc = "Register `MASTER_I2CPECCTL` writer"]
pub type W = crate::W<MasterI2cpecctlSpec>;
#[doc = "Field `MASTER_I2CPECCTL_PECCNT` reader - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Master use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Slave Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the MASTER_I2CPECCTL Register will clear the current PEC Byte Count in the Master State Machine."]
pub type MasterI2cpecctlPeccntR = crate::FieldReader<u16>;
#[doc = "Field `MASTER_I2CPECCTL_PECCNT` writer - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Master use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Slave Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the MASTER_I2CPECCTL Register will clear the current PEC Byte Count in the Master State Machine."]
pub type MasterI2cpecctlPeccntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasterI2cpecctlPecen {
    #[doc = "0: DISABLE"]
    MasterI2cpecctlPecenDisable = 0,
    #[doc = "1: ENABLE"]
    MasterI2cpecctlPecenEnable = 1,
}
impl From<MasterI2cpecctlPecen> for bool {
    #[inline(always)]
    fn from(variant: MasterI2cpecctlPecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER_I2CPECCTL_PECEN` reader - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
pub type MasterI2cpecctlPecenR = crate::BitReader<MasterI2cpecctlPecen>;
impl MasterI2cpecctlPecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MasterI2cpecctlPecen {
        match self.bits {
            false => MasterI2cpecctlPecen::MasterI2cpecctlPecenDisable,
            true => MasterI2cpecctlPecen::MasterI2cpecctlPecenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_master_i2cpecctl_pecen_disable(&self) -> bool {
        *self == MasterI2cpecctlPecen::MasterI2cpecctlPecenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_master_i2cpecctl_pecen_enable(&self) -> bool {
        *self == MasterI2cpecctlPecen::MasterI2cpecctlPecenEnable
    }
}
#[doc = "Field `MASTER_I2CPECCTL_PECEN` writer - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
pub type MasterI2cpecctlPecenW<'a, REG> = crate::BitWriter<'a, REG, MasterI2cpecctlPecen>;
impl<'a, REG> MasterI2cpecctlPecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn master_i2cpecctl_pecen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MasterI2cpecctlPecen::MasterI2cpecctlPecenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn master_i2cpecctl_pecen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MasterI2cpecctlPecen::MasterI2cpecctlPecenEnable)
    }
}
impl R {
    #[doc = "Bits 0:8 - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Master use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Slave Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the MASTER_I2CPECCTL Register will clear the current PEC Byte Count in the Master State Machine."]
    #[inline(always)]
    pub fn master_i2cpecctl_peccnt(&self) -> MasterI2cpecctlPeccntR {
        MasterI2cpecctlPeccntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
    #[inline(always)]
    pub fn master_i2cpecctl_pecen(&self) -> MasterI2cpecctlPecenR {
        MasterI2cpecctlPecenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - PEC Count When this field is non zero, the number of I2C bytes are counted (Note that although the PEC is calculated on the I2C address it is not counted at a byte). When the byte count = PECCNT and the state machine is transmitting, the contents of the LSFR is loaded into the shift register instead of the byte received from the Tx FIFO. When the state machine is receiving, after the last bit of this byte is received the LSFR is checked and if it is non-zero, a PEC RX Error interrupt is generated. The I2C packet must be padded to include the PEC byte for both transmit and receive. In transmit mode the FIFO must be loaded with a dummy PEC byte. In receive mode the PEC byte will be passed to the Rx FIFO. In the normal Master use case, FW would set PECEN=1 and PECCNT=SMB packet length (Not including Slave Address byte, but including the PEC byte). FW would then configure DMA to allow the packet to complete unassisted and write MCTR to initiate the transaction. Note that when the byte count = PEC CNT, the byte count is reset to 0 and multiple PEC calculation can automatically occur within a single I2C transaction. Note that any write to the MASTER_I2CPECCTL Register will clear the current PEC Byte Count in the Master State Machine."]
    #[inline(always)]
    pub fn master_i2cpecctl_peccnt(&mut self) -> MasterI2cpecctlPeccntW<MasterI2cpecctlSpec> {
        MasterI2cpecctlPeccntW::new(self, 0)
    }
    #[doc = "Bit 12 - PEC Enable This bit enables the SMB Packet Error Checking (PEC). When enabled the PEC is calculated on all bits accept the Start, Stop, Ack and Nack. The PEC LSFR and the Byte Counter is set to 0 when the State Machine is in the IDLE state, which occur following a Stop or when a timeout occurs. The Counter is also set to 0 after the PEC byte is sent or received. Note that the NACK is automatically send following a PEC byte that results in a PEC error. The PEC Polynomial is x^8 + x^2 + x^1 + 1."]
    #[inline(always)]
    pub fn master_i2cpecctl_pecen(&mut self) -> MasterI2cpecctlPecenW<MasterI2cpecctlSpec> {
        MasterI2cpecctlPecenW::new(self, 12)
    }
}
#[doc = "I2C master PEC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_i2cpecctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_i2cpecctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterI2cpecctlSpec;
impl crate::RegisterSpec for MasterI2cpecctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master_i2cpecctl::R`](R) reader structure"]
impl crate::Readable for MasterI2cpecctlSpec {}
#[doc = "`write(|w| ..)` method takes [`master_i2cpecctl::W`](W) writer structure"]
impl crate::Writable for MasterI2cpecctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASTER_I2CPECCTL to value 0"]
impl crate::Resettable for MasterI2cpecctlSpec {
    const RESET_VALUE: u32 = 0;
}
