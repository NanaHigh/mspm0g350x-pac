#[doc = "Register `AESASTAT` reader"]
pub type R = crate::R<AesastatSpec>;
#[doc = "Register `AESASTAT` writer"]
pub type W = crate::W<AesastatSpec>;
#[doc = "AES accelerator module busy; encryption, decryption, or key generation in progress. 0 = Not busy 1 = Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesastatBusy {
    #[doc = "0: IDLE"]
    AesastatBusyIdle = 0,
    #[doc = "1: BUSY"]
    AesastatBusyBusy = 1,
}
impl From<AesastatBusy> for bool {
    #[inline(always)]
    fn from(variant: AesastatBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESASTAT_BUSY` reader - AES accelerator module busy; encryption, decryption, or key generation in progress. 0 = Not busy 1 = Busy"]
pub type AesastatBusyR = crate::BitReader<AesastatBusy>;
impl AesastatBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesastatBusy {
        match self.bits {
            false => AesastatBusy::AesastatBusyIdle,
            true => AesastatBusy::AesastatBusyBusy,
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_aesastat_busy_idle(&self) -> bool {
        *self == AesastatBusy::AesastatBusyIdle
    }
    #[doc = "BUSY"]
    #[inline(always)]
    pub fn is_aesastat_busy_busy(&self) -> bool {
        *self == AesastatBusy::AesastatBusyBusy
    }
}
#[doc = "All bytes written to AESAKEY. This bit can be modified by software but it must not be reset by software (10) if AESCMEN=1. Changing its state by software also resets the AESKEYCNTx bits. AESKEYWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, and the start to (over)write a new key. Because it is reset when AESOPx is changed it can be set by software again to indicate that the loaded key is still valid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesastatKeywr {
    #[doc = "0: NALL"]
    AesastatKeywrNall = 0,
    #[doc = "1: ALL"]
    AesastatKeywrAll = 1,
}
impl From<AesastatKeywr> for bool {
    #[inline(always)]
    fn from(variant: AesastatKeywr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESASTAT_KEYWR` reader - All bytes written to AESAKEY. This bit can be modified by software but it must not be reset by software (10) if AESCMEN=1. Changing its state by software also resets the AESKEYCNTx bits. AESKEYWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, and the start to (over)write a new key. Because it is reset when AESOPx is changed it can be set by software again to indicate that the loaded key is still valid."]
pub type AesastatKeywrR = crate::BitReader<AesastatKeywr>;
impl AesastatKeywrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesastatKeywr {
        match self.bits {
            false => AesastatKeywr::AesastatKeywrNall,
            true => AesastatKeywr::AesastatKeywrAll,
        }
    }
    #[doc = "NALL"]
    #[inline(always)]
    pub fn is_aesastat_keywr_nall(&self) -> bool {
        *self == AesastatKeywr::AesastatKeywrNall
    }
    #[doc = "ALL"]
    #[inline(always)]
    pub fn is_aesastat_keywr_all(&self) -> bool {
        *self == AesastatKeywr::AesastatKeywrAll
    }
}
#[doc = "Field `AESASTAT_KEYWR` writer - All bytes written to AESAKEY. This bit can be modified by software but it must not be reset by software (10) if AESCMEN=1. Changing its state by software also resets the AESKEYCNTx bits. AESKEYWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, and the start to (over)write a new key. Because it is reset when AESOPx is changed it can be set by software again to indicate that the loaded key is still valid."]
pub type AesastatKeywrW<'a, REG> = crate::BitWriter<'a, REG, AesastatKeywr>;
impl<'a, REG> AesastatKeywrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NALL"]
    #[inline(always)]
    pub fn aesastat_keywr_nall(self) -> &'a mut crate::W<REG> {
        self.variant(AesastatKeywr::AesastatKeywrNall)
    }
    #[doc = "ALL"]
    #[inline(always)]
    pub fn aesastat_keywr_all(self) -> &'a mut crate::W<REG> {
        self.variant(AesastatKeywr::AesastatKeywrAll)
    }
}
#[doc = "All 16 bytes written to AESADIN, AESAXDIN or AESAXIN. Changing its state by software also resets the AESDINCNTx bits. AESDINWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, the start to (over)write the data, and when the AES accelerator is busy. Because it is reset when AESOPx or AESKLx is changed it can be set by software again to indicate that the current data is still valid. 0 = Not all bytes written 1 = All bytes written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesastatDinwr {
    #[doc = "0: NALL"]
    AesastatDinwrNall = 0,
    #[doc = "1: ALL"]
    AesastatDinwrAll = 1,
}
impl From<AesastatDinwr> for bool {
    #[inline(always)]
    fn from(variant: AesastatDinwr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESASTAT_DINWR` reader - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN. Changing its state by software also resets the AESDINCNTx bits. AESDINWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, the start to (over)write the data, and when the AES accelerator is busy. Because it is reset when AESOPx or AESKLx is changed it can be set by software again to indicate that the current data is still valid. 0 = Not all bytes written 1 = All bytes written"]
pub type AesastatDinwrR = crate::BitReader<AesastatDinwr>;
impl AesastatDinwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesastatDinwr {
        match self.bits {
            false => AesastatDinwr::AesastatDinwrNall,
            true => AesastatDinwr::AesastatDinwrAll,
        }
    }
    #[doc = "NALL"]
    #[inline(always)]
    pub fn is_aesastat_dinwr_nall(&self) -> bool {
        *self == AesastatDinwr::AesastatDinwrNall
    }
    #[doc = "ALL"]
    #[inline(always)]
    pub fn is_aesastat_dinwr_all(&self) -> bool {
        *self == AesastatDinwr::AesastatDinwrAll
    }
}
#[doc = "Field `AESASTAT_DINWR` writer - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN. Changing its state by software also resets the AESDINCNTx bits. AESDINWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, the start to (over)write the data, and when the AES accelerator is busy. Because it is reset when AESOPx or AESKLx is changed it can be set by software again to indicate that the current data is still valid. 0 = Not all bytes written 1 = All bytes written"]
pub type AesastatDinwrW<'a, REG> = crate::BitWriter<'a, REG, AesastatDinwr>;
impl<'a, REG> AesastatDinwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NALL"]
    #[inline(always)]
    pub fn aesastat_dinwr_nall(self) -> &'a mut crate::W<REG> {
        self.variant(AesastatDinwr::AesastatDinwrNall)
    }
    #[doc = "ALL"]
    #[inline(always)]
    pub fn aesastat_dinwr_all(self) -> &'a mut crate::W<REG> {
        self.variant(AesastatDinwr::AesastatDinwrAll)
    }
}
#[doc = "All 16 bytes read from AESADOUT. AESDOUTRD is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, when the AES accelerator is busy, and when the output data is read again. 0 = Not all bytes read 1 = All bytes read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesastatDoutrd {
    #[doc = "0: NALL"]
    AesastatDoutrdNall = 0,
    #[doc = "1: ALL"]
    AesastatDoutrdAll = 1,
}
impl From<AesastatDoutrd> for bool {
    #[inline(always)]
    fn from(variant: AesastatDoutrd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESASTAT_DOUTRD` reader - All 16 bytes read from AESADOUT. AESDOUTRD is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, when the AES accelerator is busy, and when the output data is read again. 0 = Not all bytes read 1 = All bytes read"]
pub type AesastatDoutrdR = crate::BitReader<AesastatDoutrd>;
impl AesastatDoutrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesastatDoutrd {
        match self.bits {
            false => AesastatDoutrd::AesastatDoutrdNall,
            true => AesastatDoutrd::AesastatDoutrdAll,
        }
    }
    #[doc = "NALL"]
    #[inline(always)]
    pub fn is_aesastat_doutrd_nall(&self) -> bool {
        *self == AesastatDoutrd::AesastatDoutrdNall
    }
    #[doc = "ALL"]
    #[inline(always)]
    pub fn is_aesastat_doutrd_all(&self) -> bool {
        *self == AesastatDoutrd::AesastatDoutrdAll
    }
}
#[doc = "Bytes written to AESAKEY when AESKLx = 00, half-words written to AESAKEY if AESKLx = b10. Reset when AESKEYWR is reset. If AESKEYCNTx = 0 and AESKEYWR = 0, no bytes were written. If AESKEYCNTx = 0 and AESKEYWR = 1, all bytes were written.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesastatKeycntx {
    #[doc = "0: MINNUM"]
    AesastatKeycntxMinnum = 0,
    #[doc = "15: MAXNUM"]
    AesastatKeycntxMaxnum = 15,
}
impl From<AesastatKeycntx> for u8 {
    #[inline(always)]
    fn from(variant: AesastatKeycntx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesastatKeycntx {
    type Ux = u8;
}
impl crate::IsEnum for AesastatKeycntx {}
#[doc = "Field `AESASTAT_KEYCNTX` reader - Bytes written to AESAKEY when AESKLx = 00, half-words written to AESAKEY if AESKLx = b10. Reset when AESKEYWR is reset. If AESKEYCNTx = 0 and AESKEYWR = 0, no bytes were written. If AESKEYCNTx = 0 and AESKEYWR = 1, all bytes were written."]
pub type AesastatKeycntxR = crate::FieldReader<AesastatKeycntx>;
impl AesastatKeycntxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesastatKeycntx> {
        match self.bits {
            0 => Some(AesastatKeycntx::AesastatKeycntxMinnum),
            15 => Some(AesastatKeycntx::AesastatKeycntxMaxnum),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesastat_keycntx_minnum(&self) -> bool {
        *self == AesastatKeycntx::AesastatKeycntxMinnum
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn is_aesastat_keycntx_maxnum(&self) -> bool {
        *self == AesastatKeycntx::AesastatKeycntxMaxnum
    }
}
#[doc = "Bytes written to AESADIN, AESAXDIN or AESAXIN. Reset when AESDINWR is reset. If AESDINCNTx = 0 and AESDINWR = 0, no bytes were written. If AESDINCNTx = 0 and AESDINWR = 1, all bytes were written.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesastatDincntx {
    #[doc = "0: MINNUM"]
    AesastatDincntxMinnum = 0,
    #[doc = "15: MAXNUM"]
    AesastatDincntxMaxnum = 15,
}
impl From<AesastatDincntx> for u8 {
    #[inline(always)]
    fn from(variant: AesastatDincntx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesastatDincntx {
    type Ux = u8;
}
impl crate::IsEnum for AesastatDincntx {}
#[doc = "Field `AESASTAT_DINCNTX` reader - Bytes written to AESADIN, AESAXDIN or AESAXIN. Reset when AESDINWR is reset. If AESDINCNTx = 0 and AESDINWR = 0, no bytes were written. If AESDINCNTx = 0 and AESDINWR = 1, all bytes were written."]
pub type AesastatDincntxR = crate::FieldReader<AesastatDincntx>;
impl AesastatDincntxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesastatDincntx> {
        match self.bits {
            0 => Some(AesastatDincntx::AesastatDincntxMinnum),
            15 => Some(AesastatDincntx::AesastatDincntxMaxnum),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesastat_dincntx_minnum(&self) -> bool {
        *self == AesastatDincntx::AesastatDincntxMinnum
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn is_aesastat_dincntx_maxnum(&self) -> bool {
        *self == AesastatDincntx::AesastatDincntxMaxnum
    }
}
#[doc = "Bytes read from AESADOUT. Reset when AESDOUTRD is reset. If AESDOUTCNTx = 0 and AESDOUTRD = 0, no bytes were read. If AESDOUTCNTx = 0 and AESDOUTRD = 1, all bytes were read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesastatDoutcntx {
    #[doc = "0: MINNUM"]
    AesastatDoutcntxMinnum = 0,
    #[doc = "15: MAXNUM"]
    AesastatDoutcntxMaxnum = 15,
}
impl From<AesastatDoutcntx> for u8 {
    #[inline(always)]
    fn from(variant: AesastatDoutcntx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesastatDoutcntx {
    type Ux = u8;
}
impl crate::IsEnum for AesastatDoutcntx {}
#[doc = "Field `AESASTAT_DOUTCNTX` reader - Bytes read from AESADOUT. Reset when AESDOUTRD is reset. If AESDOUTCNTx = 0 and AESDOUTRD = 0, no bytes were read. If AESDOUTCNTx = 0 and AESDOUTRD = 1, all bytes were read."]
pub type AesastatDoutcntxR = crate::FieldReader<AesastatDoutcntx>;
impl AesastatDoutcntxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesastatDoutcntx> {
        match self.bits {
            0 => Some(AesastatDoutcntx::AesastatDoutcntxMinnum),
            15 => Some(AesastatDoutcntx::AesastatDoutcntxMaxnum),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesastat_doutcntx_minnum(&self) -> bool {
        *self == AesastatDoutcntx::AesastatDoutcntxMinnum
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn is_aesastat_doutcntx_maxnum(&self) -> bool {
        *self == AesastatDoutcntx::AesastatDoutcntxMaxnum
    }
}
impl R {
    #[doc = "Bit 0 - AES accelerator module busy; encryption, decryption, or key generation in progress. 0 = Not busy 1 = Busy"]
    #[inline(always)]
    pub fn aesastat_busy(&self) -> AesastatBusyR {
        AesastatBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - All bytes written to AESAKEY. This bit can be modified by software but it must not be reset by software (10) if AESCMEN=1. Changing its state by software also resets the AESKEYCNTx bits. AESKEYWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, and the start to (over)write a new key. Because it is reset when AESOPx is changed it can be set by software again to indicate that the loaded key is still valid."]
    #[inline(always)]
    pub fn aesastat_keywr(&self) -> AesastatKeywrR {
        AesastatKeywrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN. Changing its state by software also resets the AESDINCNTx bits. AESDINWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, the start to (over)write the data, and when the AES accelerator is busy. Because it is reset when AESOPx or AESKLx is changed it can be set by software again to indicate that the current data is still valid. 0 = Not all bytes written 1 = All bytes written"]
    #[inline(always)]
    pub fn aesastat_dinwr(&self) -> AesastatDinwrR {
        AesastatDinwrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - All 16 bytes read from AESADOUT. AESDOUTRD is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, when the AES accelerator is busy, and when the output data is read again. 0 = Not all bytes read 1 = All bytes read"]
    #[inline(always)]
    pub fn aesastat_doutrd(&self) -> AesastatDoutrdR {
        AesastatDoutrdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bytes written to AESAKEY when AESKLx = 00, half-words written to AESAKEY if AESKLx = b10. Reset when AESKEYWR is reset. If AESKEYCNTx = 0 and AESKEYWR = 0, no bytes were written. If AESKEYCNTx = 0 and AESKEYWR = 1, all bytes were written."]
    #[inline(always)]
    pub fn aesastat_keycntx(&self) -> AesastatKeycntxR {
        AesastatKeycntxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Bytes written to AESADIN, AESAXDIN or AESAXIN. Reset when AESDINWR is reset. If AESDINCNTx = 0 and AESDINWR = 0, no bytes were written. If AESDINCNTx = 0 and AESDINWR = 1, all bytes were written."]
    #[inline(always)]
    pub fn aesastat_dincntx(&self) -> AesastatDincntxR {
        AesastatDincntxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Bytes read from AESADOUT. Reset when AESDOUTRD is reset. If AESDOUTCNTx = 0 and AESDOUTRD = 0, no bytes were read. If AESDOUTCNTx = 0 and AESDOUTRD = 1, all bytes were read."]
    #[inline(always)]
    pub fn aesastat_doutcntx(&self) -> AesastatDoutcntxR {
        AesastatDoutcntxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - All bytes written to AESAKEY. This bit can be modified by software but it must not be reset by software (10) if AESCMEN=1. Changing its state by software also resets the AESKEYCNTx bits. AESKEYWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, and the start to (over)write a new key. Because it is reset when AESOPx is changed it can be set by software again to indicate that the loaded key is still valid."]
    #[inline(always)]
    pub fn aesastat_keywr(&mut self) -> AesastatKeywrW<AesastatSpec> {
        AesastatKeywrW::new(self, 1)
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN. Changing its state by software also resets the AESDINCNTx bits. AESDINWR is reset by PUC, AESSWRST, an error condition, changing AESOPx, changing AESKLx, the start to (over)write the data, and when the AES accelerator is busy. Because it is reset when AESOPx or AESKLx is changed it can be set by software again to indicate that the current data is still valid. 0 = Not all bytes written 1 = All bytes written"]
    #[inline(always)]
    pub fn aesastat_dinwr(&mut self) -> AesastatDinwrW<AesastatSpec> {
        AesastatDinwrW::new(self, 2)
    }
}
#[doc = "aes accelerator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesastat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesastat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesastatSpec;
impl crate::RegisterSpec for AesastatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesastat::R`](R) reader structure"]
impl crate::Readable for AesastatSpec {}
#[doc = "`write(|w| ..)` method takes [`aesastat::W`](W) writer structure"]
impl crate::Writable for AesastatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESASTAT to value 0"]
impl crate::Resettable for AesastatSpec {
    const RESET_VALUE: u32 = 0;
}
