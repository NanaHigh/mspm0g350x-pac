#[doc = "Register `AESADOUT` reader"]
pub type R = crate::R<AesadoutSpec>;
#[doc = "AES data out byte n when AESADOUT is read as word. AES next data out byte when AESADOUT is read as byte. Do not mix word and byte access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadoutDout0x {
    #[doc = "0: MINNUM"]
    AesadoutDout0xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadoutDout0xMaxnum = 255,
}
impl From<AesadoutDout0x> for u8 {
    #[inline(always)]
    fn from(variant: AesadoutDout0x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadoutDout0x {
    type Ux = u8;
}
impl crate::IsEnum for AesadoutDout0x {}
#[doc = "Field `AESADOUT_DOUT0X` reader - AES data out byte n when AESADOUT is read as word. AES next data out byte when AESADOUT is read as byte. Do not mix word and byte access."]
pub type AesadoutDout0xR = crate::FieldReader<AesadoutDout0x>;
impl AesadoutDout0xR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesadoutDout0x> {
        match self.bits {
            0 => Some(AesadoutDout0x::AesadoutDout0xMinnum),
            255 => Some(AesadoutDout0x::AesadoutDout0xMaxnum),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout0x_minnum(&self) -> bool {
        *self == AesadoutDout0x::AesadoutDout0xMinnum
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout0x_maxnum(&self) -> bool {
        *self == AesadoutDout0x::AesadoutDout0xMaxnum
    }
}
#[doc = "AES data out byte n+1 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadoutDout1x {
    #[doc = "0: MINNUM"]
    AesadoutDout1xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadoutDout1xMaxnum = 255,
}
impl From<AesadoutDout1x> for u8 {
    #[inline(always)]
    fn from(variant: AesadoutDout1x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadoutDout1x {
    type Ux = u8;
}
impl crate::IsEnum for AesadoutDout1x {}
#[doc = "Field `AESADOUT_DOUT1X` reader - AES data out byte n+1 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access."]
pub type AesadoutDout1xR = crate::FieldReader<AesadoutDout1x>;
impl AesadoutDout1xR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesadoutDout1x> {
        match self.bits {
            0 => Some(AesadoutDout1x::AesadoutDout1xMinnum),
            255 => Some(AesadoutDout1x::AesadoutDout1xMaxnum),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout1x_minnum(&self) -> bool {
        *self == AesadoutDout1x::AesadoutDout1xMinnum
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout1x_maxnum(&self) -> bool {
        *self == AesadoutDout1x::AesadoutDout1xMaxnum
    }
}
#[doc = "AES data out byte n+2 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadoutDout2x {
    #[doc = "0: MINNUM"]
    AesadoutDout2xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadoutDout2xMaxnum = 255,
}
impl From<AesadoutDout2x> for u8 {
    #[inline(always)]
    fn from(variant: AesadoutDout2x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadoutDout2x {
    type Ux = u8;
}
impl crate::IsEnum for AesadoutDout2x {}
#[doc = "Field `AESADOUT_DOUT2X` reader - AES data out byte n+2 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access."]
pub type AesadoutDout2xR = crate::FieldReader<AesadoutDout2x>;
impl AesadoutDout2xR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesadoutDout2x> {
        match self.bits {
            0 => Some(AesadoutDout2x::AesadoutDout2xMinnum),
            255 => Some(AesadoutDout2x::AesadoutDout2xMaxnum),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout2x_minnum(&self) -> bool {
        *self == AesadoutDout2x::AesadoutDout2xMinnum
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout2x_maxnum(&self) -> bool {
        *self == AesadoutDout2x::AesadoutDout2xMaxnum
    }
}
#[doc = "AES data out byte n+3 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AesadoutDout3x {
    #[doc = "0: MINNUM"]
    AesadoutDout3xMinnum = 0,
    #[doc = "255: MAXNUM"]
    AesadoutDout3xMaxnum = 255,
}
impl From<AesadoutDout3x> for u8 {
    #[inline(always)]
    fn from(variant: AesadoutDout3x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AesadoutDout3x {
    type Ux = u8;
}
impl crate::IsEnum for AesadoutDout3x {}
#[doc = "Field `AESADOUT_DOUT3X` reader - AES data out byte n+3 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access."]
pub type AesadoutDout3xR = crate::FieldReader<AesadoutDout3x>;
impl AesadoutDout3xR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AesadoutDout3x> {
        match self.bits {
            0 => Some(AesadoutDout3x::AesadoutDout3xMinnum),
            255 => Some(AesadoutDout3x::AesadoutDout3xMaxnum),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout3x_minnum(&self) -> bool {
        *self == AesadoutDout3x::AesadoutDout3xMinnum
    }
    #[doc = "MAXNUM"]
    #[inline(always)]
    pub fn is_aesadout_dout3x_maxnum(&self) -> bool {
        *self == AesadoutDout3x::AesadoutDout3xMaxnum
    }
}
impl R {
    #[doc = "Bits 0:7 - AES data out byte n when AESADOUT is read as word. AES next data out byte when AESADOUT is read as byte. Do not mix word and byte access."]
    #[inline(always)]
    pub fn aesadout_dout0x(&self) -> AesadoutDout0xR {
        AesadoutDout0xR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AES data out byte n+1 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access."]
    #[inline(always)]
    pub fn aesadout_dout1x(&self) -> AesadoutDout1xR {
        AesadoutDout1xR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AES data out byte n+2 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access."]
    #[inline(always)]
    pub fn aesadout_dout2x(&self) -> AesadoutDout2xR {
        AesadoutDout2xR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - AES data out byte n+3 when AESADOUT is read as word. Do not use these bits for byte access. Do not mix word and byte access."]
    #[inline(always)]
    pub fn aesadout_dout3x(&self) -> AesadoutDout3xR {
        AesadoutDout3xR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "aes accelerator data out register\n\nYou can [`read`](crate::Reg::read) this register and get [`aesadout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadoutSpec;
impl crate::RegisterSpec for AesadoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesadout::R`](R) reader structure"]
impl crate::Readable for AesadoutSpec {}
#[doc = "`reset()` method sets AESADOUT to value 0"]
impl crate::Resettable for AesadoutSpec {
    const RESET_VALUE: u32 = 0;
}
