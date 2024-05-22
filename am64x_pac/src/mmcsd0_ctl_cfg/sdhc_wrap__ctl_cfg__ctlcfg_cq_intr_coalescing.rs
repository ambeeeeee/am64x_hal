#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_coalescing` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_coalescing` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec>;
#[doc = "Field `TIMEOUT_VAL` reader - 6:0\\]
Interrupt Coalescing Timeout Value \\[ICTOVAL\\]: Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timers unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register.The minimum value is 01h \\[1024 clock periods\\]
and the maximum value is 7Fh \\[127*1024 clock periods\\]. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us NOTE: When ICTOVAL is 0, the timer is not running, and timer-based interrupts are not generated. In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
pub type TimeoutValR = crate::FieldReader;
#[doc = "Field `TIMEOUT_VAL` writer - 6:0\\]
Interrupt Coalescing Timeout Value \\[ICTOVAL\\]: Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timers unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register.The minimum value is 01h \\[1024 clock periods\\]
and the maximum value is 7Fh \\[127*1024 clock periods\\]. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us NOTE: When ICTOVAL is 0, the timer is not running, and timer-based interrupts are not generated. In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
pub type TimeoutValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTR_THRESHOLD` reader - 12:8\\]
Interrupt Coalescing Counter Threshold \\[ICCTH\\]: Software uses this field to configure the number of task completions \\[only tasks with INT=0 in the Task Descriptor\\]
which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated.In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
pub type CtrThresholdR = crate::FieldReader;
#[doc = "Field `CTR_THRESHOLD` writer - 12:8\\]
Interrupt Coalescing Counter Threshold \\[ICCTH\\]: Software uses this field to configure the number of task completions \\[only tasks with INT=0 in the Task Descriptor\\]
which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated.In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
pub type CtrThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IC_STATUS` reader - 20:20\\]
This bit indicates to software whether any tasks \\[with INT=0\\]
have completed and counted towards interrupt coalescing \\[i.e., ICSB is set if and only if IC counter > 0\\]. Bit Value Description 1 = At least one task completion has been counted \\[IC counter >0\\]
0 = No task completions have occurred since last counter reset \\[IC counter =0\\]"]
pub type IcStatusR = crate::BitReader;
#[doc = "Field `IC_STATUS` writer - 20:20\\]
This bit indicates to software whether any tasks \\[with INT=0\\]
have completed and counted towards interrupt coalescing \\[i.e., ICSB is set if and only if IC counter > 0\\]. Bit Value Description 1 = At least one task completion has been counted \\[IC counter >0\\]
0 = No task completions have occurred since last counter reset \\[IC counter =0\\]"]
pub type IcStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQINTCOALESC_ENABLE` reader - 31:31\\]
When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
pub type CqintcoalescEnableR = crate::BitReader;
#[doc = "Field `CQINTCOALESC_ENABLE` writer - 31:31\\]
When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
pub type CqintcoalescEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Interrupt Coalescing Timeout Value \\[ICTOVAL\\]: Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timers unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register.The minimum value is 01h \\[1024 clock periods\\]
and the maximum value is 7Fh \\[127*1024 clock periods\\]. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us NOTE: When ICTOVAL is 0, the timer is not running, and timer-based interrupts are not generated. In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn timeout_val(&self) -> TimeoutValR {
        TimeoutValR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Coalescing Counter Threshold \\[ICCTH\\]: Software uses this field to configure the number of task completions \\[only tasks with INT=0 in the Task Descriptor\\]
which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated.In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
    #[inline(always)]
    pub fn ctr_threshold(&self) -> CtrThresholdR {
        CtrThresholdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
This bit indicates to software whether any tasks \\[with INT=0\\]
have completed and counted towards interrupt coalescing \\[i.e., ICSB is set if and only if IC counter > 0\\]. Bit Value Description 1 = At least one task completion has been counted \\[IC counter >0\\]
0 = No task completions have occurred since last counter reset \\[IC counter =0\\]"]
    #[inline(always)]
    pub fn ic_status(&self) -> IcStatusR {
        IcStatusR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
    #[inline(always)]
    pub fn cqintcoalesc_enable(&self) -> CqintcoalescEnableR {
        CqintcoalescEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Interrupt Coalescing Timeout Value \\[ICTOVAL\\]: Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when a data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field it generates an interrupt and stops. The timers unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register.The minimum value is 01h \\[1024 clock periods\\]
and the maximum value is 7Fh \\[127*1024 clock periods\\]. For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency \\[period = 52.08 ns\\]. If the setting in ICTOVAL is 10h, the calculated polling period is 16*1024*52.08 ns= 853.33 us NOTE: When ICTOVAL is 0, the timer is not running, and timer-based interrupts are not generated. In order to write to this field, the ICTOVALWEN bit must be set at the same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_val(&mut self) -> TimeoutValW<SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec> {
        TimeoutValW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Coalescing Counter Threshold \\[ICCTH\\]: Software uses this field to configure the number of task completions \\[only tasks with INT=0 in the Task Descriptor\\]
which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in ICCTH. The maximum allowed value is 31 NOTE : When ICCTH is 0, task completions are not counted, and counting-based interrupts are not generated.In order to write to this field, the ICCTHWEN bit must be set at the same write operation."]
    #[inline(always)]
    #[must_use]
    pub fn ctr_threshold(&mut self) -> CtrThresholdW<SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec> {
        CtrThresholdW::new(self, 8)
    }
    #[doc = "Bit 20 - 20:20\\]
This bit indicates to software whether any tasks \\[with INT=0\\]
have completed and counted towards interrupt coalescing \\[i.e., ICSB is set if and only if IC counter > 0\\]. Bit Value Description 1 = At least one task completion has been counted \\[IC counter >0\\]
0 = No task completions have occurred since last counter reset \\[IC counter =0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ic_status(&mut self) -> IcStatusW<SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec> {
        IcStatusW::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
When set to 0 by software, command responses are neither counted nor timed. Interrupts are still triggered by completion of tasks with INT=1 in the Task Descriptor. When set to 1, the interrupt coalescing mechanism is enabled and coalesced interrupts are generated."]
    #[inline(always)]
    #[must_use]
    pub fn cqintcoalesc_enable(
        &mut self,
    ) -> CqintcoalescEnableW<SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec> {
        CqintcoalescEnableW::new(self, 31)
    }
}
#[doc = "This register controls the interrupt coalescing feature.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_coalescing to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec {
    const RESET_VALUE: u32 = 0;
}
