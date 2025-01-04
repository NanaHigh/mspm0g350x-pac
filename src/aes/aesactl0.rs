#[doc = "Register `AESACTL0` reader"]
pub type R = crate::R<Aesactl0Spec>;
#[doc = "Register `AESACTL0` writer"]
pub type W = crate::W<Aesactl0Spec>;
#[doc = "AES operation. The AESOPx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = Encryption. 01b = Decryption. The provided key is the same key used for encryption. 10b = Generate first round key required for decryption. 11b = Decryption. The provided key is the first round key required for decryption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesactl0Opx {
    #[doc = "0: OP0"]
    Aesactl0OpxOp0 = 0,
    #[doc = "1: OP1"]
    Aesactl0OpxOp1 = 1,
    #[doc = "2: OP2"]
    Aesactl0OpxOp2 = 2,
    #[doc = "3: OP3"]
    Aesactl0OpxOp3 = 3,
}
impl From<Aesactl0Opx> for u8 {
    #[inline(always)]
    fn from(variant: Aesactl0Opx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesactl0Opx {
    type Ux = u8;
}
impl crate::IsEnum for Aesactl0Opx {}
#[doc = "Field `AESACTL0_OPX` reader - AES operation. The AESOPx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = Encryption. 01b = Decryption. The provided key is the same key used for encryption. 10b = Generate first round key required for decryption. 11b = Decryption. The provided key is the first round key required for decryption."]
pub type Aesactl0OpxR = crate::FieldReader<Aesactl0Opx>;
impl Aesactl0OpxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesactl0Opx {
        match self.bits {
            0 => Aesactl0Opx::Aesactl0OpxOp0,
            1 => Aesactl0Opx::Aesactl0OpxOp1,
            2 => Aesactl0Opx::Aesactl0OpxOp2,
            3 => Aesactl0Opx::Aesactl0OpxOp3,
            _ => unreachable!(),
        }
    }
    #[doc = "OP0"]
    #[inline(always)]
    pub fn is_aesactl0_opx_op0(&self) -> bool {
        *self == Aesactl0Opx::Aesactl0OpxOp0
    }
    #[doc = "OP1"]
    #[inline(always)]
    pub fn is_aesactl0_opx_op1(&self) -> bool {
        *self == Aesactl0Opx::Aesactl0OpxOp1
    }
    #[doc = "OP2"]
    #[inline(always)]
    pub fn is_aesactl0_opx_op2(&self) -> bool {
        *self == Aesactl0Opx::Aesactl0OpxOp2
    }
    #[doc = "OP3"]
    #[inline(always)]
    pub fn is_aesactl0_opx_op3(&self) -> bool {
        *self == Aesactl0Opx::Aesactl0OpxOp3
    }
}
#[doc = "Field `AESACTL0_OPX` writer - AES operation. The AESOPx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = Encryption. 01b = Decryption. The provided key is the same key used for encryption. 10b = Generate first round key required for decryption. 11b = Decryption. The provided key is the first round key required for decryption."]
pub type Aesactl0OpxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aesactl0Opx, crate::Safe>;
impl<'a, REG> Aesactl0OpxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OP0"]
    #[inline(always)]
    pub fn aesactl0_opx_op0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Opx::Aesactl0OpxOp0)
    }
    #[doc = "OP1"]
    #[inline(always)]
    pub fn aesactl0_opx_op1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Opx::Aesactl0OpxOp1)
    }
    #[doc = "OP2"]
    #[inline(always)]
    pub fn aesactl0_opx_op2(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Opx::Aesactl0OpxOp2)
    }
    #[doc = "OP3"]
    #[inline(always)]
    pub fn aesactl0_opx_op3(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Opx::Aesactl0OpxOp3)
    }
}
#[doc = "AES key length. These bits define which of the 1 AES standards is performed. The AESKLx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesactl0Klx {
    #[doc = "0: AES128"]
    Aesactl0KlxAes128 = 0,
    #[doc = "2: AES256"]
    Aesactl0KlxAes256 = 2,
}
impl From<Aesactl0Klx> for u8 {
    #[inline(always)]
    fn from(variant: Aesactl0Klx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesactl0Klx {
    type Ux = u8;
}
impl crate::IsEnum for Aesactl0Klx {}
#[doc = "Field `AESACTL0_KLX` reader - AES key length. These bits define which of the 1 AES standards is performed. The AESKLx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
pub type Aesactl0KlxR = crate::FieldReader<Aesactl0Klx>;
impl Aesactl0KlxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aesactl0Klx> {
        match self.bits {
            0 => Some(Aesactl0Klx::Aesactl0KlxAes128),
            2 => Some(Aesactl0Klx::Aesactl0KlxAes256),
            _ => None,
        }
    }
    #[doc = "AES128"]
    #[inline(always)]
    pub fn is_aesactl0_klx_aes128(&self) -> bool {
        *self == Aesactl0Klx::Aesactl0KlxAes128
    }
    #[doc = "AES256"]
    #[inline(always)]
    pub fn is_aesactl0_klx_aes256(&self) -> bool {
        *self == Aesactl0Klx::Aesactl0KlxAes256
    }
}
#[doc = "Field `AESACTL0_KLX` writer - AES key length. These bits define which of the 1 AES standards is performed. The AESKLx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
pub type Aesactl0KlxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aesactl0Klx>;
impl<'a, REG> Aesactl0KlxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES128"]
    #[inline(always)]
    pub fn aesactl0_klx_aes128(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Klx::Aesactl0KlxAes128)
    }
    #[doc = "AES256"]
    #[inline(always)]
    pub fn aesactl0_klx_aes256(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Klx::Aesactl0KlxAes256)
    }
}
#[doc = "AES cipher mode select. These bits are ignored for AESCMEN = 0. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = ECB 01b = CBC 10b = OFB 11b = CFB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesactl0Cmx {
    #[doc = "0: ECB"]
    Aesactl0CmxEcb = 0,
    #[doc = "1: CBC"]
    Aesactl0CmxCbc = 1,
    #[doc = "2: OFB"]
    Aesactl0CmxOfb = 2,
    #[doc = "3: CFB"]
    Aesactl0CmxCfb = 3,
}
impl From<Aesactl0Cmx> for u8 {
    #[inline(always)]
    fn from(variant: Aesactl0Cmx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesactl0Cmx {
    type Ux = u8;
}
impl crate::IsEnum for Aesactl0Cmx {}
#[doc = "Field `AESACTL0_CMX` reader - AES cipher mode select. These bits are ignored for AESCMEN = 0. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = ECB 01b = CBC 10b = OFB 11b = CFB"]
pub type Aesactl0CmxR = crate::FieldReader<Aesactl0Cmx>;
impl Aesactl0CmxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesactl0Cmx {
        match self.bits {
            0 => Aesactl0Cmx::Aesactl0CmxEcb,
            1 => Aesactl0Cmx::Aesactl0CmxCbc,
            2 => Aesactl0Cmx::Aesactl0CmxOfb,
            3 => Aesactl0Cmx::Aesactl0CmxCfb,
            _ => unreachable!(),
        }
    }
    #[doc = "ECB"]
    #[inline(always)]
    pub fn is_aesactl0_cmx_ecb(&self) -> bool {
        *self == Aesactl0Cmx::Aesactl0CmxEcb
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn is_aesactl0_cmx_cbc(&self) -> bool {
        *self == Aesactl0Cmx::Aesactl0CmxCbc
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn is_aesactl0_cmx_ofb(&self) -> bool {
        *self == Aesactl0Cmx::Aesactl0CmxOfb
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn is_aesactl0_cmx_cfb(&self) -> bool {
        *self == Aesactl0Cmx::Aesactl0CmxCfb
    }
}
#[doc = "Field `AESACTL0_CMX` writer - AES cipher mode select. These bits are ignored for AESCMEN = 0. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = ECB 01b = CBC 10b = OFB 11b = CFB"]
pub type Aesactl0CmxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aesactl0Cmx, crate::Safe>;
impl<'a, REG> Aesactl0CmxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB"]
    #[inline(always)]
    pub fn aesactl0_cmx_ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Cmx::Aesactl0CmxEcb)
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn aesactl0_cmx_cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Cmx::Aesactl0CmxCbc)
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn aesactl0_cmx_ofb(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Cmx::Aesactl0CmxOfb)
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn aesactl0_cmx_cfb(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Cmx::Aesactl0CmxCfb)
    }
}
#[doc = "AES software reset. Immediately resets the complete AES accelerator module even when busy except for the AESRDYIE, the AESKLx and the AESOPx bits. It also clears the (internal) state memory. The AESSWRST bit is automatically reset and is always read as zero. 0b = No reset 1b = Reset AES accelerator module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesactl0Swrst {
    #[doc = "0: NORST"]
    Aesactl0SwrstNorst = 0,
    #[doc = "1: RST"]
    Aesactl0SwrstRst = 1,
}
impl From<Aesactl0Swrst> for bool {
    #[inline(always)]
    fn from(variant: Aesactl0Swrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESACTL0_SWRST` reader - AES software reset. Immediately resets the complete AES accelerator module even when busy except for the AESRDYIE, the AESKLx and the AESOPx bits. It also clears the (internal) state memory. The AESSWRST bit is automatically reset and is always read as zero. 0b = No reset 1b = Reset AES accelerator module"]
pub type Aesactl0SwrstR = crate::BitReader<Aesactl0Swrst>;
impl Aesactl0SwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesactl0Swrst {
        match self.bits {
            false => Aesactl0Swrst::Aesactl0SwrstNorst,
            true => Aesactl0Swrst::Aesactl0SwrstRst,
        }
    }
    #[doc = "NORST"]
    #[inline(always)]
    pub fn is_aesactl0_swrst_norst(&self) -> bool {
        *self == Aesactl0Swrst::Aesactl0SwrstNorst
    }
    #[doc = "RST"]
    #[inline(always)]
    pub fn is_aesactl0_swrst_rst(&self) -> bool {
        *self == Aesactl0Swrst::Aesactl0SwrstRst
    }
}
#[doc = "Field `AESACTL0_SWRST` writer - AES software reset. Immediately resets the complete AES accelerator module even when busy except for the AESRDYIE, the AESKLx and the AESOPx bits. It also clears the (internal) state memory. The AESSWRST bit is automatically reset and is always read as zero. 0b = No reset 1b = Reset AES accelerator module"]
pub type Aesactl0SwrstW<'a, REG> = crate::BitWriter<'a, REG, Aesactl0Swrst>;
impl<'a, REG> Aesactl0SwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NORST"]
    #[inline(always)]
    pub fn aesactl0_swrst_norst(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Swrst::Aesactl0SwrstNorst)
    }
    #[doc = "RST"]
    #[inline(always)]
    pub fn aesactl0_swrst_rst(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Swrst::Aesactl0SwrstRst)
    }
}
#[doc = "AES error flag. AESAKEY or AESADIN were written while an AES operation was in progress. The bit must be cleared by software. 0b = No error 1b = Error occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesactl0Errfg {
    #[doc = "0: NOERR"]
    Aesactl0ErrfgNoerr = 0,
    #[doc = "1: ERR"]
    Aesactl0ErrfgErr = 1,
}
impl From<Aesactl0Errfg> for bool {
    #[inline(always)]
    fn from(variant: Aesactl0Errfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESACTL0_ERRFG` reader - AES error flag. AESAKEY or AESADIN were written while an AES operation was in progress. The bit must be cleared by software. 0b = No error 1b = Error occurred"]
pub type Aesactl0ErrfgR = crate::BitReader<Aesactl0Errfg>;
impl Aesactl0ErrfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesactl0Errfg {
        match self.bits {
            false => Aesactl0Errfg::Aesactl0ErrfgNoerr,
            true => Aesactl0Errfg::Aesactl0ErrfgErr,
        }
    }
    #[doc = "NOERR"]
    #[inline(always)]
    pub fn is_aesactl0_errfg_noerr(&self) -> bool {
        *self == Aesactl0Errfg::Aesactl0ErrfgNoerr
    }
    #[doc = "ERR"]
    #[inline(always)]
    pub fn is_aesactl0_errfg_err(&self) -> bool {
        *self == Aesactl0Errfg::Aesactl0ErrfgErr
    }
}
#[doc = "Field `AESACTL0_ERRFG` writer - AES error flag. AESAKEY or AESADIN were written while an AES operation was in progress. The bit must be cleared by software. 0b = No error 1b = Error occurred"]
pub type Aesactl0ErrfgW<'a, REG> = crate::BitWriter<'a, REG, Aesactl0Errfg>;
impl<'a, REG> Aesactl0ErrfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOERR"]
    #[inline(always)]
    pub fn aesactl0_errfg_noerr(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Errfg::Aesactl0ErrfgNoerr)
    }
    #[doc = "ERR"]
    #[inline(always)]
    pub fn aesactl0_errfg_err(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Errfg::Aesactl0ErrfgErr)
    }
}
#[doc = "AESCMEN enables the support of the cipher modes ECB, CBC, OFB and CFB together with the DMA. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 0 = No DMA triggers are generated. 1 = DMA cipher mode support operation is enabled and the corresponding DMA triggers are generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesactl0Cmen {
    #[doc = "0: DISABLE"]
    Aesactl0CmenDisable = 0,
    #[doc = "1: ENABLE"]
    Aesactl0CmenEnable = 1,
}
impl From<Aesactl0Cmen> for bool {
    #[inline(always)]
    fn from(variant: Aesactl0Cmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESACTL0_CMEN` reader - AESCMEN enables the support of the cipher modes ECB, CBC, OFB and CFB together with the DMA. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 0 = No DMA triggers are generated. 1 = DMA cipher mode support operation is enabled and the corresponding DMA triggers are generated."]
pub type Aesactl0CmenR = crate::BitReader<Aesactl0Cmen>;
impl Aesactl0CmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesactl0Cmen {
        match self.bits {
            false => Aesactl0Cmen::Aesactl0CmenDisable,
            true => Aesactl0Cmen::Aesactl0CmenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_aesactl0_cmen_disable(&self) -> bool {
        *self == Aesactl0Cmen::Aesactl0CmenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_aesactl0_cmen_enable(&self) -> bool {
        *self == Aesactl0Cmen::Aesactl0CmenEnable
    }
}
#[doc = "Field `AESACTL0_CMEN` writer - AESCMEN enables the support of the cipher modes ECB, CBC, OFB and CFB together with the DMA. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 0 = No DMA triggers are generated. 1 = DMA cipher mode support operation is enabled and the corresponding DMA triggers are generated."]
pub type Aesactl0CmenW<'a, REG> = crate::BitWriter<'a, REG, Aesactl0Cmen>;
impl<'a, REG> Aesactl0CmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn aesactl0_cmen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Cmen::Aesactl0CmenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn aesactl0_cmen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl0Cmen::Aesactl0CmenEnable)
    }
}
impl R {
    #[doc = "Bits 0:1 - AES operation. The AESOPx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = Encryption. 01b = Decryption. The provided key is the same key used for encryption. 10b = Generate first round key required for decryption. 11b = Decryption. The provided key is the first round key required for decryption."]
    #[inline(always)]
    pub fn aesactl0_opx(&self) -> Aesactl0OpxR {
        Aesactl0OpxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AES key length. These bits define which of the 1 AES standards is performed. The AESKLx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
    #[inline(always)]
    pub fn aesactl0_klx(&self) -> Aesactl0KlxR {
        Aesactl0KlxR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES cipher mode select. These bits are ignored for AESCMEN = 0. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = ECB 01b = CBC 10b = OFB 11b = CFB"]
    #[inline(always)]
    pub fn aesactl0_cmx(&self) -> Aesactl0CmxR {
        Aesactl0CmxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - AES software reset. Immediately resets the complete AES accelerator module even when busy except for the AESRDYIE, the AESKLx and the AESOPx bits. It also clears the (internal) state memory. The AESSWRST bit is automatically reset and is always read as zero. 0b = No reset 1b = Reset AES accelerator module"]
    #[inline(always)]
    pub fn aesactl0_swrst(&self) -> Aesactl0SwrstR {
        Aesactl0SwrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - AES error flag. AESAKEY or AESADIN were written while an AES operation was in progress. The bit must be cleared by software. 0b = No error 1b = Error occurred"]
    #[inline(always)]
    pub fn aesactl0_errfg(&self) -> Aesactl0ErrfgR {
        Aesactl0ErrfgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - AESCMEN enables the support of the cipher modes ECB, CBC, OFB and CFB together with the DMA. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 0 = No DMA triggers are generated. 1 = DMA cipher mode support operation is enabled and the corresponding DMA triggers are generated."]
    #[inline(always)]
    pub fn aesactl0_cmen(&self) -> Aesactl0CmenR {
        Aesactl0CmenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES operation. The AESOPx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = Encryption. 01b = Decryption. The provided key is the same key used for encryption. 10b = Generate first round key required for decryption. 11b = Decryption. The provided key is the first round key required for decryption."]
    #[inline(always)]
    pub fn aesactl0_opx(&mut self) -> Aesactl0OpxW<Aesactl0Spec> {
        Aesactl0OpxW::new(self, 0)
    }
    #[doc = "Bits 2:3 - AES key length. These bits define which of the 1 AES standards is performed. The AESKLx bits are not reset by AESSWRST = 1. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
    #[inline(always)]
    pub fn aesactl0_klx(&mut self) -> Aesactl0KlxW<Aesactl0Spec> {
        Aesactl0KlxW::new(self, 2)
    }
    #[doc = "Bits 5:6 - AES cipher mode select. These bits are ignored for AESCMEN = 0. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 00b = ECB 01b = CBC 10b = OFB 11b = CFB"]
    #[inline(always)]
    pub fn aesactl0_cmx(&mut self) -> Aesactl0CmxW<Aesactl0Spec> {
        Aesactl0CmxW::new(self, 5)
    }
    #[doc = "Bit 7 - AES software reset. Immediately resets the complete AES accelerator module even when busy except for the AESRDYIE, the AESKLx and the AESOPx bits. It also clears the (internal) state memory. The AESSWRST bit is automatically reset and is always read as zero. 0b = No reset 1b = Reset AES accelerator module"]
    #[inline(always)]
    pub fn aesactl0_swrst(&mut self) -> Aesactl0SwrstW<Aesactl0Spec> {
        Aesactl0SwrstW::new(self, 7)
    }
    #[doc = "Bit 11 - AES error flag. AESAKEY or AESADIN were written while an AES operation was in progress. The bit must be cleared by software. 0b = No error 1b = Error occurred"]
    #[inline(always)]
    pub fn aesactl0_errfg(&mut self) -> Aesactl0ErrfgW<Aesactl0Spec> {
        Aesactl0ErrfgW::new(self, 11)
    }
    #[doc = "Bit 15 - AESCMEN enables the support of the cipher modes ECB, CBC, OFB and CFB together with the DMA. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0. 0 = No DMA triggers are generated. 1 = DMA cipher mode support operation is enabled and the corresponding DMA triggers are generated."]
    #[inline(always)]
    pub fn aesactl0_cmen(&mut self) -> Aesactl0CmenW<Aesactl0Spec> {
        Aesactl0CmenW::new(self, 15)
    }
}
#[doc = "AES accelerator control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesactl0Spec;
impl crate::RegisterSpec for Aesactl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesactl0::R`](R) reader structure"]
impl crate::Readable for Aesactl0Spec {}
#[doc = "`write(|w| ..)` method takes [`aesactl0::W`](W) writer structure"]
impl crate::Writable for Aesactl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESACTL0 to value 0"]
impl crate::Resettable for Aesactl0Spec {
    const RESET_VALUE: u32 = 0;
}
