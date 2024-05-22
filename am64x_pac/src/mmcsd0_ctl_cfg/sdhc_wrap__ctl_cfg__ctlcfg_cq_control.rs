#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_control` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqControlSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_control` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqControlSpec>;
#[doc = "Field `HALT_BIT` reader - 0:0\\]
Halt Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command \\[CMDQ_TASK_MGMT\\]
When software writes 1, CQE shall complete the ongo-ing task if such a task is in progress. Once the task is completed and CQE is in idle state,CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state \\[i.e. resume CQE activity\\], soft-ware shall clear this bit \\[write 0\\]. Writing 0 when thevalue is already 0 shall have no effect."]
pub type HaltBitR = crate::BitReader;
#[doc = "Field `HALT_BIT` writer - 0:0\\]
Halt Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command \\[CMDQ_TASK_MGMT\\]
When software writes 1, CQE shall complete the ongo-ing task if such a task is in progress. Once the task is completed and CQE is in idle state,CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state \\[i.e. resume CQE activity\\], soft-ware shall clear this bit \\[write 0\\]. Writing 0 when thevalue is already 0 shall have no effect."]
pub type HaltBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_ALL_TASKS` reader - 8:8\\]
Clear All Tasks Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state \\[i.e.Halt bit is 1\\]. When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit.Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is softwares responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
pub type ClearAllTasksR = crate::BitReader;
#[doc = "Field `CLEAR_ALL_TASKS` writer - 8:8\\]
Clear All Tasks Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state \\[i.e.Halt bit is 1\\]. When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit.Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is softwares responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
pub type ClearAllTasksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command \\[CMDQ_TASK_MGMT\\]
When software writes 1, CQE shall complete the ongo-ing task if such a task is in progress. Once the task is completed and CQE is in idle state,CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state \\[i.e. resume CQE activity\\], soft-ware shall clear this bit \\[write 0\\]. Writing 0 when thevalue is already 0 shall have no effect."]
    #[inline(always)]
    pub fn halt_bit(&self) -> HaltBitR {
        HaltBitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear All Tasks Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state \\[i.e.Halt bit is 1\\]. When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit.Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is softwares responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
    #[inline(always)]
    pub fn clear_all_tasks(&self) -> ClearAllTasksR {
        ClearAllTasksR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt Host software shall write 1 to the bit when it wants to acquire software control over the eMMC bus and disable CQE from issuing commands on the bus. For example, issuing a Discard Task command \\[CMDQ_TASK_MGMT\\]
When software writes 1, CQE shall complete the ongo-ing task if such a task is in progress. Once the task is completed and CQE is in idle state,CQE shall not issue new commands and shall indicate so to software by setting this bit to 1. Software may poll on this bit until it is set to 1, and may only then send commands on the eMMC bus. In order to exit halt state \\[i.e. resume CQE activity\\], soft-ware shall clear this bit \\[write 0\\]. Writing 0 when thevalue is already 0 shall have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn halt_bit(&mut self) -> HaltBitW<SdhcWrap_CtlCfg_CtlcfgCqControlSpec> {
        HaltBitW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear All Tasks Software shall write 1 this bit when it wants to clear all the tasks sent to the device. This bit can only be written when CQE is in halt state \\[i.e.Halt bit is 1\\]. When software writes 1, the value of the register is updated to 1, and CQE shall reset CQTDBR register and all other context information for all unfinished tasks. Then CQE will clear this bit.Software should poll on this bit until it is set to back 0 and may then resume normal operation, by clearing the Halt bit. CQE does not communicate to the device that the tasks were cleared. It is softwares responsibility to order the device to discard the tasks in its queue using CMDQ_TASK_MGMT command. Writing 0 to this register shall have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn clear_all_tasks(&mut self) -> ClearAllTasksW<SdhcWrap_CtlCfg_CtlcfgCqControlSpec> {
        ClearAllTasksW::new(self, 8)
    }
}
#[doc = "This register controls CQE behavior affecting the general operation of command queueing 293 module or operation of multiple tasks in the same time.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqControlSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_control::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_control::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_control to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqControlSpec {
    const RESET_VALUE: u32 = 0;
}
