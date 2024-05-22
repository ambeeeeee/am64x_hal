#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_error_task_id` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_error_task_id` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec>;
#[doc = "Field `TERR_ID` reader - 4:0\\]
Task Error ID"]
pub type TerrIdR = crate::FieldReader;
#[doc = "Field `TERR_ID` writer - 4:0\\]
Task Error ID"]
pub type TerrIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Task Error ID"]
    #[inline(always)]
    pub fn terr_id(&self) -> TerrIdR {
        TerrIdR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Task Error ID"]
    #[inline(always)]
    #[must_use]
    pub fn terr_id(&mut self) -> TerrIdW<SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec> {
        TerrIdW::new(self, 0)
    }
}
#[doc = "CQ Error Task ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_error_task_id to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec {
    const RESET_VALUE: u32 = 0;
}
