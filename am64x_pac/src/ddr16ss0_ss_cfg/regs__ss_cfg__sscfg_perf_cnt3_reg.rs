#[doc = "Register `REGS__SS_CFG__SSCFG_PERF_CNT3_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPerfCnt3RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PERF_CNT3_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPerfCnt3RegSpec>;
#[doc = "Field `CNT3` reader - 31:0\\]
Soft 32-bit counter that can be configured as specified in the Performance Counter Select Register."]
pub type Cnt3R = crate::FieldReader<u32>;
#[doc = "Field `CNT3` writer - 31:0\\]
Soft 32-bit counter that can be configured as specified in the Performance Counter Select Register."]
pub type Cnt3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Soft 32-bit counter that can be configured as specified in the Performance Counter Select Register."]
    #[inline(always)]
    pub fn cnt3(&self) -> Cnt3R {
        Cnt3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Soft 32-bit counter that can be configured as specified in the Performance Counter Select Register."]
    #[inline(always)]
    #[must_use]
    pub fn cnt3(&mut self) -> Cnt3W<Regs_SsCfg_SscfgPerfCnt3RegSpec> {
        Cnt3W::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_perf_cnt3_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_perf_cnt3_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPerfCnt3RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPerfCnt3RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_perf_cnt3_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPerfCnt3RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_perf_cnt3_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPerfCnt3RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PERF_CNT3_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPerfCnt3RegSpec {
    const RESET_VALUE: u32 = 0;
}
