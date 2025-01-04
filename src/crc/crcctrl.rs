#[doc = "Register `CRCCTRL` reader"]
pub type R = crate::R<CrcctrlSpec>;
#[doc = "Register `CRCCTRL` writer"]
pub type W = crate::W<CrcctrlSpec>;
#[doc = "This bit indicates which CRC calculation is performed by the generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcctrlPolysize {
    #[doc = "0: CRC32"]
    CrcctrlPolysizeCrc32 = 0,
    #[doc = "1: CRC16"]
    CrcctrlPolysizeCrc16 = 1,
}
impl From<CrcctrlPolysize> for bool {
    #[inline(always)]
    fn from(variant: CrcctrlPolysize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCCTRL_POLYSIZE` reader - This bit indicates which CRC calculation is performed by the generator."]
pub type CrcctrlPolysizeR = crate::BitReader<CrcctrlPolysize>;
impl CrcctrlPolysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcctrlPolysize {
        match self.bits {
            false => CrcctrlPolysize::CrcctrlPolysizeCrc32,
            true => CrcctrlPolysize::CrcctrlPolysizeCrc16,
        }
    }
    #[doc = "CRC32"]
    #[inline(always)]
    pub fn is_crcctrl_polysize_crc32(&self) -> bool {
        *self == CrcctrlPolysize::CrcctrlPolysizeCrc32
    }
    #[doc = "CRC16"]
    #[inline(always)]
    pub fn is_crcctrl_polysize_crc16(&self) -> bool {
        *self == CrcctrlPolysize::CrcctrlPolysizeCrc16
    }
}
#[doc = "Field `CRCCTRL_POLYSIZE` writer - This bit indicates which CRC calculation is performed by the generator."]
pub type CrcctrlPolysizeW<'a, REG> = crate::BitWriter<'a, REG, CrcctrlPolysize>;
impl<'a, REG> CrcctrlPolysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC32"]
    #[inline(always)]
    pub fn crcctrl_polysize_crc32(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlPolysize::CrcctrlPolysizeCrc32)
    }
    #[doc = "CRC16"]
    #[inline(always)]
    pub fn crcctrl_polysize_crc16(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlPolysize::CrcctrlPolysizeCrc16)
    }
}
#[doc = "CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcctrlBitreverse {
    #[doc = "0: NOT_REVERSED"]
    CrcctrlBitreverseNotReversed = 0,
    #[doc = "1: REVERSED"]
    CrcctrlBitreverseReversed = 1,
}
impl From<CrcctrlBitreverse> for bool {
    #[inline(always)]
    fn from(variant: CrcctrlBitreverse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCCTRL_BITREVERSE` reader - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
pub type CrcctrlBitreverseR = crate::BitReader<CrcctrlBitreverse>;
impl CrcctrlBitreverseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcctrlBitreverse {
        match self.bits {
            false => CrcctrlBitreverse::CrcctrlBitreverseNotReversed,
            true => CrcctrlBitreverse::CrcctrlBitreverseReversed,
        }
    }
    #[doc = "NOT_REVERSED"]
    #[inline(always)]
    pub fn is_crcctrl_bitreverse_not_reversed(&self) -> bool {
        *self == CrcctrlBitreverse::CrcctrlBitreverseNotReversed
    }
    #[doc = "REVERSED"]
    #[inline(always)]
    pub fn is_crcctrl_bitreverse_reversed(&self) -> bool {
        *self == CrcctrlBitreverse::CrcctrlBitreverseReversed
    }
}
#[doc = "Field `CRCCTRL_BITREVERSE` writer - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
pub type CrcctrlBitreverseW<'a, REG> = crate::BitWriter<'a, REG, CrcctrlBitreverse>;
impl<'a, REG> CrcctrlBitreverseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOT_REVERSED"]
    #[inline(always)]
    pub fn crcctrl_bitreverse_not_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlBitreverse::CrcctrlBitreverseNotReversed)
    }
    #[doc = "REVERSED"]
    #[inline(always)]
    pub fn crcctrl_bitreverse_reversed(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlBitreverse::CrcctrlBitreverseReversed)
    }
}
#[doc = "CRC Endian. This bit indicates the byte order within a word or half word of input data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcctrlInputEndianness {
    #[doc = "0: LITTLE_ENDIAN"]
    CrcctrlInputEndiannessLittleEndian = 0,
    #[doc = "1: BIG_ENDIAN"]
    CrcctrlInputEndiannessBigEndian = 1,
}
impl From<CrcctrlInputEndianness> for bool {
    #[inline(always)]
    fn from(variant: CrcctrlInputEndianness) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCCTRL_INPUT_ENDIANNESS` reader - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
pub type CrcctrlInputEndiannessR = crate::BitReader<CrcctrlInputEndianness>;
impl CrcctrlInputEndiannessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcctrlInputEndianness {
        match self.bits {
            false => CrcctrlInputEndianness::CrcctrlInputEndiannessLittleEndian,
            true => CrcctrlInputEndianness::CrcctrlInputEndiannessBigEndian,
        }
    }
    #[doc = "LITTLE_ENDIAN"]
    #[inline(always)]
    pub fn is_crcctrl_input_endianness_little_endian(&self) -> bool {
        *self == CrcctrlInputEndianness::CrcctrlInputEndiannessLittleEndian
    }
    #[doc = "BIG_ENDIAN"]
    #[inline(always)]
    pub fn is_crcctrl_input_endianness_big_endian(&self) -> bool {
        *self == CrcctrlInputEndianness::CrcctrlInputEndiannessBigEndian
    }
}
#[doc = "Field `CRCCTRL_INPUT_ENDIANNESS` writer - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
pub type CrcctrlInputEndiannessW<'a, REG> = crate::BitWriter<'a, REG, CrcctrlInputEndianness>;
impl<'a, REG> CrcctrlInputEndiannessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LITTLE_ENDIAN"]
    #[inline(always)]
    pub fn crcctrl_input_endianness_little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlInputEndianness::CrcctrlInputEndiannessLittleEndian)
    }
    #[doc = "BIG_ENDIAN"]
    #[inline(always)]
    pub fn crcctrl_input_endianness_big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlInputEndianness::CrcctrlInputEndiannessBigEndian)
    }
}
#[doc = "CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcctrlOutputByteswap {
    #[doc = "0: DISABLE"]
    CrcctrlOutputByteswapDisable = 0,
    #[doc = "1: ENABLE"]
    CrcctrlOutputByteswapEnable = 1,
}
impl From<CrcctrlOutputByteswap> for bool {
    #[inline(always)]
    fn from(variant: CrcctrlOutputByteswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCCTRL_OUTPUT_BYTESWAP` reader - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
pub type CrcctrlOutputByteswapR = crate::BitReader<CrcctrlOutputByteswap>;
impl CrcctrlOutputByteswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcctrlOutputByteswap {
        match self.bits {
            false => CrcctrlOutputByteswap::CrcctrlOutputByteswapDisable,
            true => CrcctrlOutputByteswap::CrcctrlOutputByteswapEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_crcctrl_output_byteswap_disable(&self) -> bool {
        *self == CrcctrlOutputByteswap::CrcctrlOutputByteswapDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_crcctrl_output_byteswap_enable(&self) -> bool {
        *self == CrcctrlOutputByteswap::CrcctrlOutputByteswapEnable
    }
}
#[doc = "Field `CRCCTRL_OUTPUT_BYTESWAP` writer - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
pub type CrcctrlOutputByteswapW<'a, REG> = crate::BitWriter<'a, REG, CrcctrlOutputByteswap>;
impl<'a, REG> CrcctrlOutputByteswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn crcctrl_output_byteswap_disable(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlOutputByteswap::CrcctrlOutputByteswapDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn crcctrl_output_byteswap_enable(self) -> &'a mut crate::W<REG> {
        self.variant(CrcctrlOutputByteswap::CrcctrlOutputByteswapEnable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit indicates which CRC calculation is performed by the generator."]
    #[inline(always)]
    pub fn crcctrl_polysize(&self) -> CrcctrlPolysizeR {
        CrcctrlPolysizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
    #[inline(always)]
    pub fn crcctrl_bitreverse(&self) -> CrcctrlBitreverseR {
        CrcctrlBitreverseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
    #[inline(always)]
    pub fn crcctrl_input_endianness(&self) -> CrcctrlInputEndiannessR {
        CrcctrlInputEndiannessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
    #[inline(always)]
    pub fn crcctrl_output_byteswap(&self) -> CrcctrlOutputByteswapR {
        CrcctrlOutputByteswapR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates which CRC calculation is performed by the generator."]
    #[inline(always)]
    pub fn crcctrl_polysize(&mut self) -> CrcctrlPolysizeW<CrcctrlSpec> {
        CrcctrlPolysizeW::new(self, 0)
    }
    #[doc = "Bit 1 - CRC Bit Input and output Reverse. This bit indictes that the bit order of each input byte used for the CRC calculation is reversed before it is passed to the generator, and that the bit order of the calculated CRC is be reversed when read from CRC_RESULT."]
    #[inline(always)]
    pub fn crcctrl_bitreverse(&mut self) -> CrcctrlBitreverseW<CrcctrlSpec> {
        CrcctrlBitreverseW::new(self, 1)
    }
    #[doc = "Bit 2 - CRC Endian. This bit indicates the byte order within a word or half word of input data."]
    #[inline(always)]
    pub fn crcctrl_input_endianness(&mut self) -> CrcctrlInputEndiannessW<CrcctrlSpec> {
        CrcctrlInputEndiannessW::new(self, 2)
    }
    #[doc = "Bit 4 - CRC Output Byteswap Enable. This bit controls whether the output is byte-swapped upon a read of the CRCOUT register. If CRCOUT is accessed as a half-word, and the OUTPUT_BYTESWAP is set to to 1, then the two bytes in the 16-bit access are swapped and returned. B1 is returned as B0 B0 is returned as B1 If CRCOUT is accessed as a word, and the OUTPUT_BYTESWAP is set to 1, then the four bytes in the 32-bit read are swapped. B3 is returned as B0 B2 is returned as B1 B1 is returned as B2 B0 is returned as B3 Note that if the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP enabled, then the output is: MSB LSB 0x0 0x0 B0 B1 If the CRC POLYSIZE is 16-bit and a 32-bit read of CRCOUT is performed with OUTPUT_BYTESWAP disabled, then the output is: MSB LSB 0x0 0x0 B1 B0"]
    #[inline(always)]
    pub fn crcctrl_output_byteswap(&mut self) -> CrcctrlOutputByteswapW<CrcctrlSpec> {
        CrcctrlOutputByteswapW::new(self, 4)
    }
}
#[doc = "CRC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcctrlSpec;
impl crate::RegisterSpec for CrcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcctrl::R`](R) reader structure"]
impl crate::Readable for CrcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crcctrl::W`](W) writer structure"]
impl crate::Writable for CrcctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CrcctrlSpec {
    const RESET_VALUE: u32 = 0;
}
