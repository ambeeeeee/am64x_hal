#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_clear` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_clear` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec>;
#[doc = "Field `CQTCLR` reader - 31:0\\]
Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued.This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit.When software writes 1 to a bit in this register,CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields \\[sets a value of 0\\]
in CQTCLR and in CQTDBR once clear operation is complete.Software should poll on the CQTCLR until it is cleared to verify clear operation was complete.Writing to this register only clears the task in the CQE and does not have impact on the device. In order to dis-card the task in the device, host software shall send CMDQ_TASK _MGMT while CQE is still in Halt state.Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register.Writing 0 to a register bit shall have no impact."]
pub type CqtclrR = crate::FieldReader<u32>;
#[doc = "Field `CQTCLR` writer - 31:0\\]
Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued.This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit.When software writes 1 to a bit in this register,CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields \\[sets a value of 0\\]
in CQTCLR and in CQTDBR once clear operation is complete.Software should poll on the CQTCLR until it is cleared to verify clear operation was complete.Writing to this register only clears the task in the CQE and does not have impact on the device. In order to dis-card the task in the device, host software shall send CMDQ_TASK _MGMT while CQE is still in Halt state.Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register.Writing 0 to a register bit shall have no impact."]
pub type CqtclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued.This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit.When software writes 1 to a bit in this register,CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields \\[sets a value of 0\\]
in CQTCLR and in CQTDBR once clear operation is complete.Software should poll on the CQTCLR until it is cleared to verify clear operation was complete.Writing to this register only clears the task in the CQE and does not have impact on the device. In order to dis-card the task in the device, host software shall send CMDQ_TASK _MGMT while CQE is still in Halt state.Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register.Writing 0 to a register bit shall have no impact."]
    #[inline(always)]
    pub fn cqtclr(&self) -> CqtclrR {
        CqtclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Writing 1 to bit n of this register orders CQE to clear a task which software has previously issued.This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit.When software writes 1 to a bit in this register,CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields \\[sets a value of 0\\]
in CQTCLR and in CQTDBR once clear operation is complete.Software should poll on the CQTCLR until it is cleared to verify clear operation was complete.Writing to this register only clears the task in the CQE and does not have impact on the device. In order to dis-card the task in the device, host software shall send CMDQ_TASK _MGMT while CQE is still in Halt state.Host driver is not allowed to use this register to clear multiple tasks at the same time. Clearing multiple tasks can be done using CQCTL register.Writing 0 to a register bit shall have no impact."]
    #[inline(always)]
    #[must_use]
    pub fn cqtclr(&mut self) -> CqtclrW<SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec> {
        CqtclrW::new(self, 0)
    }
}
#[doc = "This register is used for removing an outstanding task in the CQE. 327. The register should be used only when CQE is in Halt state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_clear to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec {
    const RESET_VALUE: u32 = 0;
}
