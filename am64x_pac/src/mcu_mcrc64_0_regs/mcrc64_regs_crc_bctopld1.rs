#[doc = "Register `MCRC64_REGS_CRC_BCTOPLD1` reader"]
pub type R = crate::R<Mcrc64RegsCrcBctopld1Spec>;
#[doc = "Register `MCRC64_REGS_CRC_BCTOPLD1` writer"]
pub type W = crate::W<Mcrc64RegsCrcBctopld1Spec>;
#[doc = "Field `CRC_BCTOPLD1` reader - 23:0\\]
Channel 1 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
pub type CrcBctopld1R = crate::FieldReader<u32>;
#[doc = "Field `CRC_BCTOPLD1` writer - 23:0\\]
Channel 1 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
pub type CrcBctopld1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 1 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
    #[inline(always)]
    pub fn crc_bctopld1(&self) -> CrcBctopld1R {
        CrcBctopld1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 1 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn crc_bctopld1(&mut self) -> CrcBctopld1W<Mcrc64RegsCrcBctopld1Spec> {
        CrcBctopld1W::new(self, 0)
    }
}
#[doc = "CRC channel 1 Block Complete Timeout Preload Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_bctopld1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_bctopld1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcBctopld1Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcBctopld1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_bctopld1::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcBctopld1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_bctopld1::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcBctopld1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_BCTOPLD1 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcBctopld1Spec {
    const RESET_VALUE: u32 = 0;
}
