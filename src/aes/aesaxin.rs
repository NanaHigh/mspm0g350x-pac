#[doc = "Register `AESAXIN` writer"]
pub type W = crate::W<AesaxinSpec>;
#[doc = "AES data in byte n when AESAXIN is written as word. AES next data in byte when AESAXIN is written as byte. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxinXin0x {
    #[doc = "0: MINNUM"]
    AesaxinXin0xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxinXin0xMaxnum = 255,
}
impl From<AesaxinXin0x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxinXin0x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxinXin0x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxinXin0x {}
#[doc = "Field `AESAXIN_XIN0X` writer - AES data in byte n when AESAXIN is written as word. AES next data in byte when AESAXIN is written as byte. Do not mix word and byte access. Always reads as zero."]
pub type AesaxinXin0xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxinXin0x>;
impl<'a, REG> AesaxinXin0xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxin_xin0x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin0x::AesaxinXin0xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxin_xin0x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin0x::AesaxinXin0xMaxnum)
    }
}
#[doc = "AES data in byte n+1 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxinXin1x {
    #[doc = "0: MINNUM"]
    AesaxinXin1xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxinXin1xMaxnum = 255,
}
impl From<AesaxinXin1x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxinXin1x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxinXin1x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxinXin1x {}
#[doc = "Field `AESAXIN_XIN1X` writer - AES data in byte n+1 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesaxinXin1xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxinXin1x>;
impl<'a, REG> AesaxinXin1xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxin_xin1x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin1x::AesaxinXin1xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxin_xin1x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin1x::AesaxinXin1xMaxnum)
    }
}
#[doc = "AES data in byte n+2 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxinXin2x {
    #[doc = "0: MINNUM"]
    AesaxinXin2xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxinXin2xMaxnum = 255,
}
impl From<AesaxinXin2x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxinXin2x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxinXin2x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxinXin2x {}
#[doc = "Field `AESAXIN_XIN2X` writer - AES data in byte n+2 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesaxinXin2xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxinXin2x>;
impl<'a, REG> AesaxinXin2xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxin_xin2x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin2x::AesaxinXin2xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxin_xin2x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin2x::AesaxinXin2xMaxnum)
    }
}
#[doc = "AES data in byte n+3 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxinXin3x {
    #[doc = "0: MINNUM"]
    AesaxinXin3xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxinXin3xMaxnum = 255,
}
impl From<AesaxinXin3x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxinXin3x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxinXin3x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxinXin3x {}
#[doc = "Field `AESAXIN_XIN3X` writer - AES data in byte n+3 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesaxinXin3xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxinXin3x>;
impl<'a, REG> AesaxinXin3xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxin_xin3x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin3x::AesaxinXin3xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxin_xin3x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxinXin3x::AesaxinXin3xMaxnum)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXIN is written as word. AES next data in byte when AESAXIN is written as byte. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxin_xin0x(&mut self) -> AesaxinXin0xW<AesaxinSpec> {
        AesaxinXin0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxin_xin1x(&mut self) -> AesaxinXin1xW<AesaxinSpec> {
        AesaxinXin1xW::new(self, 8)
    }
    #[doc = "Bits 16:23 - AES data in byte n+2 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxin_xin2x(&mut self) -> AesaxinXin2xW<AesaxinSpec> {
        AesaxinXin2xW::new(self, 16)
    }
    #[doc = "Bits 24:31 - AES data in byte n+3 when AESAXIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxin_xin3x(&mut self) -> AesaxinXin3xW<AesaxinSpec> {
        AesaxinXin3xW::new(self, 24)
    }
}
#[doc = "aes accelerator xored data in register (no trigger)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesaxinSpec;
impl crate::RegisterSpec for AesaxinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesaxin::W`](W) writer structure"]
impl crate::Writable for AesaxinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESAXIN to value 0"]
impl crate::Resettable for AesaxinSpec {
    const RESET_VALUE: u32 = 0;
}
