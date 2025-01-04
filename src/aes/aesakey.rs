#[doc = "Register `AESAKEY` writer"]
pub type W = crate::W<AesakeySpec>;
#[doc = "AES key byte n when AESAKEY is written as word. AES next key byte when AESAKEY is written as byte. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesakeyKey0x {
    #[doc = "0: MINNUM"]
    AesakeyKey0xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesakeyKey0xMaxnum = 255,
}
impl From<AesakeyKey0x> for u8 {
    #[inline(always)]
    fn from(variant: AesakeyKey0x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesakeyKey0x {
    type Ux = u8;
}
impl crate::IsEnum for AesakeyKey0x {}
#[doc = "Field `AESAKEY_KEY0X` writer - AES key byte n when AESAKEY is written as word. AES next key byte when AESAKEY is written as byte. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
pub type AesakeyKey0xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesakeyKey0x>;
impl<'a, REG> AesakeyKey0xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesakey_key0x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey0x::AesakeyKey0xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesakey_key0x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey0x::AesakeyKey0xMaxnum)
    }
}
#[doc = "AES key byte n+1 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesakeyKey1x {
    #[doc = "0: MINNUM"]
    AesakeyKey1xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesakeyKey1xMaxnum = 255,
}
impl From<AesakeyKey1x> for u8 {
    #[inline(always)]
    fn from(variant: AesakeyKey1x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesakeyKey1x {
    type Ux = u8;
}
impl crate::IsEnum for AesakeyKey1x {}
#[doc = "Field `AESAKEY_KEY1X` writer - AES key byte n+1 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
pub type AesakeyKey1xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesakeyKey1x>;
impl<'a, REG> AesakeyKey1xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesakey_key1x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey1x::AesakeyKey1xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesakey_key1x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey1x::AesakeyKey1xMaxnum)
    }
}
#[doc = "AES key byte n+2 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesakeyKey2x {
    #[doc = "0: MINNUM"]
    AesakeyKey2xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesakeyKey2xMaxnum = 255,
}
impl From<AesakeyKey2x> for u8 {
    #[inline(always)]
    fn from(variant: AesakeyKey2x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesakeyKey2x {
    type Ux = u8;
}
impl crate::IsEnum for AesakeyKey2x {}
#[doc = "Field `AESAKEY_KEY2X` writer - AES key byte n+2 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
pub type AesakeyKey2xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesakeyKey2x>;
impl<'a, REG> AesakeyKey2xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesakey_key2x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey2x::AesakeyKey2xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesakey_key2x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey2x::AesakeyKey2xMaxnum)
    }
}
#[doc = "AES key byte n+3 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesakeyKey3x {
    #[doc = "0: MINNUM"]
    AesakeyKey3xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesakeyKey3xMaxnum = 255,
}
impl From<AesakeyKey3x> for u8 {
    #[inline(always)]
    fn from(variant: AesakeyKey3x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesakeyKey3x {
    type Ux = u8;
}
impl crate::IsEnum for AesakeyKey3x {}
#[doc = "Field `AESAKEY_KEY3X` writer - AES key byte n+3 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
pub type AesakeyKey3xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesakeyKey3x>;
impl<'a, REG> AesakeyKey3xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesakey_key3x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey3x::AesakeyKey3xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesakey_key3x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesakeyKey3x::AesakeyKey3xMaxnum)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES key byte n when AESAKEY is written as word. AES next key byte when AESAKEY is written as byte. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
    #[inline(always)]
    pub fn aesakey_key0x(&mut self) -> AesakeyKey0xW<AesakeySpec> {
        AesakeyKey0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES key byte n+1 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
    #[inline(always)]
    pub fn aesakey_key1x(&mut self) -> AesakeyKey1xW<AesakeySpec> {
        AesakeyKey1xW::new(self, 8)
    }
    #[doc = "Bits 16:23 - AES key byte n+2 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
    #[inline(always)]
    pub fn aesakey_key2x(&mut self) -> AesakeyKey2xW<AesakeySpec> {
        AesakeyKey2xW::new(self, 16)
    }
    #[doc = "Bits 24:31 - AES key byte n+3 when AESAKEY is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero. The key is reset by PUC or by AESSWRST = 1."]
    #[inline(always)]
    pub fn aesakey_key3x(&mut self) -> AesakeyKey3xW<AesakeySpec> {
        AesakeyKey3xW::new(self, 24)
    }
}
#[doc = "aes accelerator key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesakey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesakeySpec;
impl crate::RegisterSpec for AesakeySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesakey::W`](W) writer structure"]
impl crate::Writable for AesakeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESAKEY to value 0"]
impl crate::Resettable for AesakeySpec {
    const RESET_VALUE: u32 = 0;
}
