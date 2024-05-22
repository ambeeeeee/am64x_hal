#[doc = "Register `MCRC64_REGS_RAW_DATAREGH1` reader"]
pub type R = crate::R<Mcrc64RegsRawDataregh1Spec>;
#[doc = "Register `MCRC64_REGS_RAW_DATAREGH1` writer"]
pub type W = crate::W<Mcrc64RegsRawDataregh1Spec>;
#[doc = "Field `RAW_DATA1_47_32` reader - 31:0\\]
Channel 1 Raw Data High Register. This register contains bit 63:32 of the uncompressed raw data."]
pub type RawData1_47_32R = crate::FieldReader<u32>;
#[doc = "Field `RAW_DATA1_47_32` writer - 31:0\\]
Channel 1 Raw Data High Register. This register contains bit 63:32 of the uncompressed raw data."]
pub type RawData1_47_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 Raw Data High Register. This register contains bit 63:32 of the uncompressed raw data."]
    #[inline(always)]
    pub fn raw_data1_47_32(&self) -> RawData1_47_32R {
        RawData1_47_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 Raw Data High Register. This register contains bit 63:32 of the uncompressed raw data."]
    #[inline(always)]
    #[must_use]
    pub fn raw_data1_47_32(&mut self) -> RawData1_47_32W<Mcrc64RegsRawDataregh1Spec> {
        RawData1_47_32W::new(self, 0)
    }
}
#[doc = "Channel 1 Raw Data High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregh1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregh1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsRawDataregh1Spec;
impl crate::RegisterSpec for Mcrc64RegsRawDataregh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_raw_dataregh1::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsRawDataregh1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_raw_dataregh1::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsRawDataregh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_RAW_DATAREGH1 to value 0"]
impl crate::Resettable for Mcrc64RegsRawDataregh1Spec {
    const RESET_VALUE: u32 = 0;
}
