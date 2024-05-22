#[doc = "Register `BCDMA_GCFG_PERF_CTRL` reader"]
pub type R = crate::R<BcdmaGcfgPerfCtrlSpec>;
#[doc = "Register `BCDMA_GCFG_PERF_CTRL` writer"]
pub type W = crate::W<BcdmaGcfgPerfCtrlSpec>;
#[doc = "Field `TIMEOUT_CNT` reader - 15:0\\]
This feature is not currently supported."]
pub type TimeoutCntR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT_CNT` writer - 15:0\\]
This feature is not currently supported."]
pub type TimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This feature is not currently supported."]
    #[inline(always)]
    pub fn timeout_cnt(&self) -> TimeoutCntR {
        TimeoutCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This feature is not currently supported."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_cnt(&mut self) -> TimeoutCntW<BcdmaGcfgPerfCtrlSpec> {
        TimeoutCntW::new(self, 0)
    }
}
#[doc = "The performance control register contains fields which can be used to adjust the performance of the BCDMA in the system.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_perf_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_perf_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgPerfCtrlSpec;
impl crate::RegisterSpec for BcdmaGcfgPerfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_perf_ctrl::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgPerfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_perf_ctrl::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgPerfCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_PERF_CTRL to value 0x64"]
impl crate::Resettable for BcdmaGcfgPerfCtrlSpec {
    const RESET_VALUE: u32 = 0x64;
}
