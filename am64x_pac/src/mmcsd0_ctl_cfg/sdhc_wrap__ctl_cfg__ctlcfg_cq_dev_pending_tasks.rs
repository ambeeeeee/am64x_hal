#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_pending_tasks` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_pending_tasks` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec>;
#[doc = "Field `CQDP_TSKS` reader - 31:0\\]
Bit n of this register is set if and only if QUEUED_TASK_PARAMS \\[CMD44\\]
and QUEUED_TASK_ADDRESS \\[CMD45\\]
were sent for this specific task and if this task hasnt been executed yet.CQE shall set this bit after receiving a successful response for CMD45. CQE shall clear this bit after the task has completed execution.Software needs to read this register in the task-discard procedure, when the controller is halted, to determine if the task is queued in the device. If the task is queued,the driver sends a CMDQ_TASK_MGMT \\[CMD48\\]
to the device ordering it to discard the task. Then software clears the task in the CQE. Only then the software orders CQE to resume its operation using CQCTL register."]
pub type CqdpTsksR = crate::FieldReader<u32>;
#[doc = "Field `CQDP_TSKS` writer - 31:0\\]
Bit n of this register is set if and only if QUEUED_TASK_PARAMS \\[CMD44\\]
and QUEUED_TASK_ADDRESS \\[CMD45\\]
were sent for this specific task and if this task hasnt been executed yet.CQE shall set this bit after receiving a successful response for CMD45. CQE shall clear this bit after the task has completed execution.Software needs to read this register in the task-discard procedure, when the controller is halted, to determine if the task is queued in the device. If the task is queued,the driver sends a CMDQ_TASK_MGMT \\[CMD48\\]
to the device ordering it to discard the task. Then software clears the task in the CQE. Only then the software orders CQE to resume its operation using CQCTL register."]
pub type CqdpTsksW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bit n of this register is set if and only if QUEUED_TASK_PARAMS \\[CMD44\\]
and QUEUED_TASK_ADDRESS \\[CMD45\\]
were sent for this specific task and if this task hasnt been executed yet.CQE shall set this bit after receiving a successful response for CMD45. CQE shall clear this bit after the task has completed execution.Software needs to read this register in the task-discard procedure, when the controller is halted, to determine if the task is queued in the device. If the task is queued,the driver sends a CMDQ_TASK_MGMT \\[CMD48\\]
to the device ordering it to discard the task. Then software clears the task in the CQE. Only then the software orders CQE to resume its operation using CQCTL register."]
    #[inline(always)]
    pub fn cqdp_tsks(&self) -> CqdpTsksR {
        CqdpTsksR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bit n of this register is set if and only if QUEUED_TASK_PARAMS \\[CMD44\\]
and QUEUED_TASK_ADDRESS \\[CMD45\\]
were sent for this specific task and if this task hasnt been executed yet.CQE shall set this bit after receiving a successful response for CMD45. CQE shall clear this bit after the task has completed execution.Software needs to read this register in the task-discard procedure, when the controller is halted, to determine if the task is queued in the device. If the task is queued,the driver sends a CMDQ_TASK_MGMT \\[CMD48\\]
to the device ordering it to discard the task. Then software clears the task in the CQE. Only then the software orders CQE to resume its operation using CQCTL register."]
    #[inline(always)]
    #[must_use]
    pub fn cqdp_tsks(&mut self) -> CqdpTsksW<SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec> {
        CqdpTsksW::new(self, 0)
    }
}
#[doc = "This register indicates to software which tasks are queued in the device, awaiting execution.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_pending_tasks to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec {
    const RESET_VALUE: u32 = 0;
}
