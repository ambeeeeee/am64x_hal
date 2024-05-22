#[doc = "Register `MCRC64_REGS_CRC_INT_OFFSET_REG` reader"]
pub type R = crate::R<Mcrc64RegsCrcIntOffsetRegSpec>;
#[doc = "Register `MCRC64_REGS_CRC_INT_OFFSET_REG` writer"]
pub type W = crate::W<Mcrc64RegsCrcIntOffsetRegSpec>;
#[doc = "Field `CRC` reader - 7:0\\]
Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register automatically clears the respective interrupt flag."]
pub type CrcR = crate::FieldReader;
#[doc = "Field `CRC` writer - 7:0\\]
Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register automatically clears the respective interrupt flag."]
pub type CrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register automatically clears the respective interrupt flag."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register automatically clears the respective interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<Mcrc64RegsCrcIntOffsetRegSpec> {
        CrcW::new(self, 0)
    }
}
#[doc = "CRC Interrupt Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_int_offset_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_int_offset_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcIntOffsetRegSpec;
impl crate::RegisterSpec for Mcrc64RegsCrcIntOffsetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_int_offset_reg::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcIntOffsetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_int_offset_reg::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcIntOffsetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_INT_OFFSET_REG to value 0"]
impl crate::Resettable for Mcrc64RegsCrcIntOffsetRegSpec {
    const RESET_VALUE: u32 = 0;
}
