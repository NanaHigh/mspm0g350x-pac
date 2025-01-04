#[doc = "Register `AESADIN` writer"]
pub type W = crate::W<AesadinSpec>;
#[doc = "AES data in byte n when AESADIN is written as word. AES next data in byte when AESADIN is written as byte. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadinDin0x {
    #[doc = "0: MINNUM"]
    AesadinDin0xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadinDin0xMaxnum = 255,
}
impl From<AesadinDin0x> for u8 {
    #[inline(always)]
    fn from(variant: AesadinDin0x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadinDin0x {
    type Ux = u8;
}
impl crate::IsEnum for AesadinDin0x {}
#[doc = "Field `AESADIN_DIN0X` writer - AES data in byte n when AESADIN is written as word. AES next data in byte when AESADIN is written as byte. Do not mix word and byte access. Always reads as zero."]
pub type AesadinDin0xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesadinDin0x>;
impl<'a, REG> AesadinDin0xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesadin_din0x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin0x::AesadinDin0xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesadin_din0x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin0x::AesadinDin0xMaxnum)
    }
}
#[doc = "AES data in byte n+1 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadinDin1x {
    #[doc = "0: MINNUM"]
    AesadinDin1xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadinDin1xMaxnum = 255,
}
impl From<AesadinDin1x> for u8 {
    #[inline(always)]
    fn from(variant: AesadinDin1x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadinDin1x {
    type Ux = u8;
}
impl crate::IsEnum for AesadinDin1x {}
#[doc = "Field `AESADIN_DIN1X` writer - AES data in byte n+1 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesadinDin1xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesadinDin1x>;
impl<'a, REG> AesadinDin1xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesadin_din1x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin1x::AesadinDin1xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesadin_din1x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin1x::AesadinDin1xMaxnum)
    }
}
#[doc = "AES data in byte n+2 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadinDin2x {
    #[doc = "0: MINNUM"]
    AesadinDin2xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadinDin2xMaxnum = 255,
}
impl From<AesadinDin2x> for u8 {
    #[inline(always)]
    fn from(variant: AesadinDin2x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadinDin2x {
    type Ux = u8;
}
impl crate::IsEnum for AesadinDin2x {}
#[doc = "Field `AESADIN_DIN2X` writer - AES data in byte n+2 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesadinDin2xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesadinDin2x>;
impl<'a, REG> AesadinDin2xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesadin_din2x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin2x::AesadinDin2xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesadin_din2x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin2x::AesadinDin2xMaxnum)
    }
}
#[doc = "AES data in byte n+3 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadinDin3x {
    #[doc = "0: MINNUM"]
    AesadinDin3xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadinDin3xMaxnum = 255,
}
impl From<AesadinDin3x> for u8 {
    #[inline(always)]
    fn from(variant: AesadinDin3x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadinDin3x {
    type Ux = u8;
}
impl crate::IsEnum for AesadinDin3x {}
#[doc = "Field `AESADIN_DIN3X` writer - AES data in byte n+3 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesadinDin3xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesadinDin3x>;
impl<'a, REG> AesadinDin3xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesadin_din3x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin3x::AesadinDin3xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesadin_din3x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesadinDin3x::AesadinDin3xMaxnum)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESADIN is written as word. AES next data in byte when AESADIN is written as byte. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesadin_din0x(&mut self) -> AesadinDin0xW<AesadinSpec> {
        AesadinDin0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesadin_din1x(&mut self) -> AesadinDin1xW<AesadinSpec> {
        AesadinDin1xW::new(self, 8)
    }
    #[doc = "Bits 16:23 - AES data in byte n+2 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesadin_din2x(&mut self) -> AesadinDin2xW<AesadinSpec> {
        AesadinDin2xW::new(self, 16)
    }
    #[doc = "Bits 24:31 - AES data in byte n+3 when AESADIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesadin_din3x(&mut self) -> AesadinDin3xW<AesadinSpec> {
        AesadinDin3xW::new(self, 24)
    }
}
#[doc = "aes accelerator data in register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadinSpec;
impl crate::RegisterSpec for AesadinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesadin::W`](W) writer structure"]
impl crate::Writable for AesadinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESADIN to value 0"]
impl crate::Resettable for AesadinSpec {
    const RESET_VALUE: u32 = 0;
}
