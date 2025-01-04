#[doc = "Register `AESACTL1` reader"]
pub type R = crate::R<Aesactl1Spec>;
#[doc = "Register `AESACTL1` writer"]
pub type W = crate::W<Aesactl1Spec>;
#[doc = "Cipher Block Counter. Number of blocks to be encrypted or decrypted with block cipher modes enabled (AESCMEN = 1). Ignored if AESCMEN = 0. The block counter decrements with each performed encryption or decryption. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesactl1Blkcntx {
    #[doc = "0: MINNUM"]
    Aesactl1BlkcntxMinnum = 0,
    #[doc = "255: ENABLE"]
    Aesactl1BlkcntxEnable = 255,
}
impl From<Aesactl1Blkcntx> for u8 {
    #[inline(always)]
    fn from(variant: Aesactl1Blkcntx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesactl1Blkcntx {
    type Ux = u8;
}
impl crate::IsEnum for Aesactl1Blkcntx {}
#[doc = "Field `AESACTL1_BLKCNTX` reader - Cipher Block Counter. Number of blocks to be encrypted or decrypted with block cipher modes enabled (AESCMEN = 1). Ignored if AESCMEN = 0. The block counter decrements with each performed encryption or decryption. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
pub type Aesactl1BlkcntxR = crate::FieldReader<Aesactl1Blkcntx>;
impl Aesactl1BlkcntxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aesactl1Blkcntx> {
        match self.bits {
            0 => Some(Aesactl1Blkcntx::Aesactl1BlkcntxMinnum),
            255 => Some(Aesactl1Blkcntx::Aesactl1BlkcntxEnable),
            _ => None,
        }
    }
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn is_aesactl1_blkcntx_minnum(&self) -> bool {
        *self == Aesactl1Blkcntx::Aesactl1BlkcntxMinnum
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_aesactl1_blkcntx_enable(&self) -> bool {
        *self == Aesactl1Blkcntx::Aesactl1BlkcntxEnable
    }
}
#[doc = "Field `AESACTL1_BLKCNTX` writer - Cipher Block Counter. Number of blocks to be encrypted or decrypted with block cipher modes enabled (AESCMEN = 1). Ignored if AESCMEN = 0. The block counter decrements with each performed encryption or decryption. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
pub type Aesactl1BlkcntxW<'a, REG> = crate::FieldWriter<'a, REG, 8, Aesactl1Blkcntx>;
impl<'a, REG> Aesactl1BlkcntxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINNUM"]
    #[inline(always)]
    pub fn aesactl1_blkcntx_minnum(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl1Blkcntx::Aesactl1BlkcntxMinnum)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn aesactl1_blkcntx_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Aesactl1Blkcntx::Aesactl1BlkcntxEnable)
    }
}
impl R {
    #[doc = "Bits 0:7 - Cipher Block Counter. Number of blocks to be encrypted or decrypted with block cipher modes enabled (AESCMEN = 1). Ignored if AESCMEN = 0. The block counter decrements with each performed encryption or decryption. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
    #[inline(always)]
    pub fn aesactl1_blkcntx(&self) -> Aesactl1BlkcntxR {
        Aesactl1BlkcntxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cipher Block Counter. Number of blocks to be encrypted or decrypted with block cipher modes enabled (AESCMEN = 1). Ignored if AESCMEN = 0. The block counter decrements with each performed encryption or decryption. Writes are ignored when AESCMEN = 1 and AESBLKCNTx &gt; 0."]
    #[inline(always)]
    pub fn aesactl1_blkcntx(&mut self) -> Aesactl1BlkcntxW<Aesactl1Spec> {
        Aesactl1BlkcntxW::new(self, 0)
    }
}
#[doc = "AES accelerator control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesactl1Spec;
impl crate::RegisterSpec for Aesactl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesactl1::R`](R) reader structure"]
impl crate::Readable for Aesactl1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesactl1::W`](W) writer structure"]
impl crate::Writable for Aesactl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESACTL1 to value 0"]
impl crate::Resettable for Aesactl1Spec {
    const RESET_VALUE: u32 = 0;
}
