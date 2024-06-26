#[doc = "Register `MCRC64_REGS_CRC_BCTOPLD2` reader"]
pub type R = crate::R<Mcrc64RegsCrcBctopld2Spec>;
#[doc = "Register `MCRC64_REGS_CRC_BCTOPLD2` writer"]
pub type W = crate::W<Mcrc64RegsCrcBctopld2Spec>;
#[doc = "Field `CRC_BCTOPLD2` reader - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC of an entire block needs to complete before a timeout interrupt is generated."]
pub type CrcBctopld2R = crate::FieldReader<u32>;
#[doc = "Field `CRC_BCTOPLD2` writer - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC of an entire block needs to complete before a timeout interrupt is generated."]
pub type CrcBctopld2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC of an entire block needs to complete before a timeout interrupt is generated."]
    #[inline(always)]
    pub fn crc_bctopld2(&self) -> CrcBctopld2R {
        CrcBctopld2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Register. This register contains the number of clock cycles within which the CRC of an entire block needs to complete before a timeout interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn crc_bctopld2(&mut self) -> CrcBctopld2W<Mcrc64RegsCrcBctopld2Spec> {
        CrcBctopld2W::new(self, 0)
    }
}
#[doc = "CRC channel 2 Block Complete Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_bctopld2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_bctopld2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcBctopld2Spec;
impl crate::RegisterSpec for Mcrc64RegsCrcBctopld2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_bctopld2::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcBctopld2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_bctopld2::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcBctopld2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_BCTOPLD2 to value 0"]
impl crate::Resettable for Mcrc64RegsCrcBctopld2Spec {
    const RESET_VALUE: u32 = 0;
}
