#[doc = "Register `MCRC64_REGS_CRC_WDTOPLD3` reader"]
pub type R = crate::R<Mcrc64RegsCrcWdtopld3Spec>;
#[doc = "Register `MCRC64_REGS_CRC_WDTOPLD3` writer"]
pub type W = crate::W<Mcrc64RegsCrcWdtopld3Spec>;
#[doc = "Field `CRC_WDTOPLD3` reader - 23:0\\]
Channel 3 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
pub type CrcWdtopld3R = crate::FieldReader<u32>;
#[doc = "Field `CRC_WDTOPLD3` writer - 23:0\\]
Channel 3 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
pub type CrcWdtopld3W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 3 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
    #[inline(always)]
    pub fn crc_wdtopld3(&self) -> CrcWdtopld3R {
        CrcWdtopld3R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 3 Watchdog Timeout Counter Preload Register. This register contains the number of clock cycles within which the DMA must transfer the next block of data patterns."]
    #[inline(always)]
    #[must_use]
    pub fn crc_wdtopld3(&mut self) -> CrcWdtopld3W<Mcrc64RegsCrcWdtopld3Spec> {
        CrcWdtopld3W::new(self, 0)
    }
}
#[doc = "CRC channel 3 Watchdog Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_wdtopld3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_wdtopld3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcWdtopld3Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcWdtopld3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_wdtopld3::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcWdtopld3Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_wdtopld3::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcWdtopld3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_WDTOPLD3 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcWdtopld3Spec {
    const RESET_VALUE: u32 = 0;
}
