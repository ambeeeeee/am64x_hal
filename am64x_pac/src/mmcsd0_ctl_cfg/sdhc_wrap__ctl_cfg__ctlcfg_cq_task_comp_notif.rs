#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_comp_notif` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_comp_notif` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec>;
#[doc = "Field `CQTCN_VAL` reader - 31:0\\]
CQE shall set bit n of this register \\[at the same time it clears bit n of CQTDBR\\]
when a task execution is com-pleted \\[with success or error\\]. When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the rele-vant bit fields by writing 1 to the corresponding bits."]
pub type CqtcnValR = crate::FieldReader<u32>;
#[doc = "Field `CQTCN_VAL` writer - 31:0\\]
CQE shall set bit n of this register \\[at the same time it clears bit n of CQTDBR\\]
when a task execution is com-pleted \\[with success or error\\]. When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the rele-vant bit fields by writing 1 to the corresponding bits."]
pub type CqtcnValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CQE shall set bit n of this register \\[at the same time it clears bit n of CQTDBR\\]
when a task execution is com-pleted \\[with success or error\\]. When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the rele-vant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    pub fn cqtcn_val(&self) -> CqtcnValR {
        CqtcnValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CQE shall set bit n of this register \\[at the same time it clears bit n of CQTDBR\\]
when a task execution is com-pleted \\[with success or error\\]. When receiving interrupt for task completion, software may read this register to know which tasks have finished. After reading this register, software may clear the rele-vant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    #[must_use]
    pub fn cqtcn_val(&mut self) -> CqtcnValW<SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec> {
        CqtcnValW::new(self, 0)
    }
}
#[doc = "This register is used by CQE to notify software about completed tasks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_comp_notif to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec {
    const RESET_VALUE: u32 = 0;
}
