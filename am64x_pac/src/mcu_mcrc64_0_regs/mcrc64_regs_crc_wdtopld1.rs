#[doc = "Register `MCRC64_REGS_CRC_WDTOPLD1` reader"]
pub type R = crate::R<Mcrc64RegsCrcWdtopld1Spec>;
#[doc = "Register `MCRC64_REGS_CRC_WDTOPLD1` writer"]
pub type W = crate::W<Mcrc64RegsCrcWdtopld1Spec>;
#[doc = "Field `CRC_WDTOPLD1` reader - 23:0\\]
Channel 1 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
pub type CrcWdtopld1R = crate::FieldReader<u32>;
#[doc = "Field `CRC_WDTOPLD1` writer - 23:0\\]
Channel 1 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
pub type CrcWdtopld1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 1 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
    #[inline(always)]
    pub fn crc_wdtopld1(&self) -> CrcWdtopld1R {
        CrcWdtopld1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 1 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
    #[inline(always)]
    #[must_use]
    pub fn crc_wdtopld1(&mut self) -> CrcWdtopld1W<Mcrc64RegsCrcWdtopld1Spec> {
        CrcWdtopld1W::new(self, 0)
    }
}
#[doc = "CRC channel 1 Watchdog Timeout Preload Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_wdtopld1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_wdtopld1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcWdtopld1Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcWdtopld1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_wdtopld1::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcWdtopld1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_wdtopld1::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcWdtopld1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_WDTOPLD1 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcWdtopld1Spec {
    const RESET_VALUE: u32 = 0;
}
