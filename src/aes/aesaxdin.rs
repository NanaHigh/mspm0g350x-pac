#[doc = "Register `AESAXDIN` writer"]
pub type W = crate::W<AesaxdinSpec>;
#[doc = "AES data in byte n when AESAXDIN is written as word. AES next data in byte when AESAXDIN is written as byte. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxdinXdin0x {
    #[doc = "0: MINNUM"]
    AesaxdinXdin0xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxdinXdin0xMaxnum = 255,
}
impl From<AesaxdinXdin0x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxdinXdin0x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxdinXdin0x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxdinXdin0x {}
#[doc = "Field `AESAXDIN_XDIN0X` writer - AES data in byte n when AESAXDIN is written as word. AES next data in byte when AESAXDIN is written as byte. Do not mix word and byte access. Always reads as zero."]
pub type AesaxdinXdin0xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxdinXdin0x>;
impl<'a, REG> AesaxdinXdin0xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin0x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin0x::AesaxdinXdin0xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin0x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin0x::AesaxdinXdin0xMaxnum)
    }
}
#[doc = "AES data in byte n+1 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxdinXdin1x {
    #[doc = "0: MINNUM"]
    AesaxdinXdin1xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxdinXdin1xMaxnum = 255,
}
impl From<AesaxdinXdin1x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxdinXdin1x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxdinXdin1x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxdinXdin1x {}
#[doc = "Field `AESAXDIN_XDIN1X` writer - AES data in byte n+1 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesaxdinXdin1xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxdinXdin1x>;
impl<'a, REG> AesaxdinXdin1xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin1x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin1x::AesaxdinXdin1xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin1x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin1x::AesaxdinXdin1xMaxnum)
    }
}
#[doc = "AES data in byte n+2 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxdinXdin2x {
    #[doc = "0: MINNUM"]
    AesaxdinXdin2xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxdinXdin2xMaxnum = 255,
}
impl From<AesaxdinXdin2x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxdinXdin2x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxdinXdin2x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxdinXdin2x {}
#[doc = "Field `AESAXDIN_XDIN2X` writer - AES data in byte n+2 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesaxdinXdin2xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxdinXdin2x>;
impl<'a, REG> AesaxdinXdin2xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin2x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin2x::AesaxdinXdin2xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin2x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin2x::AesaxdinXdin2xMaxnum)
    }
}
#[doc = "AES data in byte n+3 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesaxdinXdin3x {
    #[doc = "0: MINNUM"]
    AesaxdinXdin3xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesaxdinXdin3xMaxnum = 255,
}
impl From<AesaxdinXdin3x> for u8 {
    #[inline(always)]
    fn from(variant: AesaxdinXdin3x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesaxdinXdin3x {
    type Ux = u8;
}
impl crate::IsEnum for AesaxdinXdin3x {}
#[doc = "Field `AESAXDIN_XDIN3X` writer - AES data in byte n+3 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
pub type AesaxdinXdin3xW<'a, REG> = crate::FieldWriter<'a, REG, 8, AesaxdinXdin3x>;
impl<'a, REG> AesaxdinXdin3xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin3x_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin3x::AesaxdinXdin3xMinnum)
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn aesaxdin_xdin3x_maxnum(self) -> &'a mut crate::W<REG> {
        self.variant(AesaxdinXdin3x::AesaxdinXdin3xMaxnum)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXDIN is written as word. AES next data in byte when AESAXDIN is written as byte. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxdin_xdin0x(&mut self) -> AesaxdinXdin0xW<AesaxdinSpec> {
        AesaxdinXdin0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxdin_xdin1x(&mut self) -> AesaxdinXdin1xW<AesaxdinSpec> {
        AesaxdinXdin1xW::new(self, 8)
    }
    #[doc = "Bits 16:23 - AES data in byte n+2 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxdin_xdin2x(&mut self) -> AesaxdinXdin2xW<AesaxdinSpec> {
        AesaxdinXdin2xW::new(self, 16)
    }
    #[doc = "Bits 24:31 - AES data in byte n+3 when AESAXDIN is written as word. Do not use these bits for byte access. Do not mix word and byte access. Always reads as zero."]
    #[inline(always)]
    pub fn aesaxdin_xdin3x(&mut self) -> AesaxdinXdin3xW<AesaxdinSpec> {
        AesaxdinXdin3xW::new(self, 24)
    }
}
#[doc = "aes accelerator xored data in register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxdin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesaxdinSpec;
impl crate::RegisterSpec for AesaxdinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aesaxdin::W`](W) writer structure"]
impl crate::Writable for AesaxdinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESAXDIN to value 0"]
impl crate::Resettable for AesaxdinSpec {
    const RESET_VALUE: u32 = 0;
}
