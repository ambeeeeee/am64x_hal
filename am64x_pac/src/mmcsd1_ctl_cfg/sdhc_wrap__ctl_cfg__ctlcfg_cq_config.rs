#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_config` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqConfigSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_config` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqConfigSpec>;
#[doc = "Field `CQ_ENABLE` reader - 0:0\\]
Command Queueing Enable Software shall write 1 this bit when in order to enable command queueing mode \\[i.e. enable CQE\\]. When this bit is 0, CQE is disabled and software controls the eMMC bus using the legacy eMMC host controller. Before software writes 1 to this bit, software shall verify that the eMMC host controller is in idle state and there are no commands or data transfers ongoing. When software wants to exit command queueing mode,it shall clear all previous tasks if such exist before setting this bit to 0."]
pub type CqEnableR = crate::BitReader;
#[doc = "Field `CQ_ENABLE` writer - 0:0\\]
Command Queueing Enable Software shall write 1 this bit when in order to enable command queueing mode \\[i.e. enable CQE\\]. When this bit is 0, CQE is disabled and software controls the eMMC bus using the legacy eMMC host controller. Before software writes 1 to this bit, software shall verify that the eMMC host controller is in idle state and there are no commands or data transfers ongoing. When software wants to exit command queueing mode,it shall clear all previous tasks if such exist before setting this bit to 0."]
pub type CqEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_DESC_SIZE` reader - 8:8\\]
Task Descriptor Size This bit indicates whether the task descriptor size is 128 bits or 64 bits as detailed in Data Structures section. This bit can only be configured when Command Queueing Enable bit is 0 \\[command queueing is disabled\\]
Bit Value Description 1 = Task descriptor size is 128 bits 0 = Task descriptor size is 64 bits"]
pub type TaskDescSizeR = crate::BitReader;
#[doc = "Field `TASK_DESC_SIZE` writer - 8:8\\]
Task Descriptor Size This bit indicates whether the task descriptor size is 128 bits or 64 bits as detailed in Data Structures section. This bit can only be configured when Command Queueing Enable bit is 0 \\[command queueing is disabled\\]
Bit Value Description 1 = Task descriptor size is 128 bits 0 = Task descriptor size is 64 bits"]
pub type TaskDescSizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMD_ENA` reader - 12:12\\]
Direct Command \\[DCMD\\]
Enable This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a Data Transfer Task Descriptor, or a Direct Command Task Descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Bit Value Description 1 = Task descriptor in slot #31 is a DCMD Task Descrip-tor 0 = Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
pub type DcmdEnaR = crate::BitReader;
#[doc = "Field `DCMD_ENA` writer - 12:12\\]
Direct Command \\[DCMD\\]
Enable This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a Data Transfer Task Descriptor, or a Direct Command Task Descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Bit Value Description 1 = Task descriptor in slot #31 is a DCMD Task Descrip-tor 0 = Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
pub type DcmdEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Command Queueing Enable Software shall write 1 this bit when in order to enable command queueing mode \\[i.e. enable CQE\\]. When this bit is 0, CQE is disabled and software controls the eMMC bus using the legacy eMMC host controller. Before software writes 1 to this bit, software shall verify that the eMMC host controller is in idle state and there are no commands or data transfers ongoing. When software wants to exit command queueing mode,it shall clear all previous tasks if such exist before setting this bit to 0."]
    #[inline(always)]
    pub fn cq_enable(&self) -> CqEnableR {
        CqEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Task Descriptor Size This bit indicates whether the task descriptor size is 128 bits or 64 bits as detailed in Data Structures section. This bit can only be configured when Command Queueing Enable bit is 0 \\[command queueing is disabled\\]
Bit Value Description 1 = Task descriptor size is 128 bits 0 = Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn task_desc_size(&self) -> TaskDescSizeR {
        TaskDescSizeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Direct Command \\[DCMD\\]
Enable This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a Data Transfer Task Descriptor, or a Direct Command Task Descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Bit Value Description 1 = Task descriptor in slot #31 is a DCMD Task Descrip-tor 0 = Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn dcmd_ena(&self) -> DcmdEnaR {
        DcmdEnaR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Command Queueing Enable Software shall write 1 this bit when in order to enable command queueing mode \\[i.e. enable CQE\\]. When this bit is 0, CQE is disabled and software controls the eMMC bus using the legacy eMMC host controller. Before software writes 1 to this bit, software shall verify that the eMMC host controller is in idle state and there are no commands or data transfers ongoing. When software wants to exit command queueing mode,it shall clear all previous tasks if such exist before setting this bit to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cq_enable(&mut self) -> CqEnableW<SdhcWrap_CtlCfg_CtlcfgCqConfigSpec> {
        CqEnableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Task Descriptor Size This bit indicates whether the task descriptor size is 128 bits or 64 bits as detailed in Data Structures section. This bit can only be configured when Command Queueing Enable bit is 0 \\[command queueing is disabled\\]
Bit Value Description 1 = Task descriptor size is 128 bits 0 = Task descriptor size is 64 bits"]
    #[inline(always)]
    #[must_use]
    pub fn task_desc_size(&mut self) -> TaskDescSizeW<SdhcWrap_CtlCfg_CtlcfgCqConfigSpec> {
        TaskDescSizeW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Direct Command \\[DCMD\\]
Enable This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a Data Transfer Task Descriptor, or a Direct Command Task Descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Bit Value Description 1 = Task descriptor in slot #31 is a DCMD Task Descrip-tor 0 = Task descriptor in slot #31 is a Data Transfer Task Descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn dcmd_ena(&mut self) -> DcmdEnaW<SdhcWrap_CtlCfg_CtlcfgCqConfigSpec> {
        DcmdEnaW::new(self, 12)
    }
}
#[doc = "This register controls CQE behavior affecting the general operation of command queueing 290 module or operation of multiple tasks in the same time.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqConfigSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_config::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_config::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_config to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqConfigSpec {
    const RESET_VALUE: u32 = 0;
}
