#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec>;
#[doc = "Field `HALT_COMPLETE` reader - 0:0\\]
Halt Complete Interrupt \\[HAC\\]
This status bit is asserted \\[if CQISTE.HAC=1\\]
when halt bit in CQCTL register transitions from 0 to 1 indicating that host controller has completed its current ongoing task and has entered halt state."]
pub type HaltCompleteR = crate::BitReader;
#[doc = "Field `HALT_COMPLETE` writer - 0:0\\]
Halt Complete Interrupt \\[HAC\\]
This status bit is asserted \\[if CQISTE.HAC=1\\]
when halt bit in CQCTL register transitions from 0 to 1 indicating that host controller has completed its current ongoing task and has entered halt state."]
pub type HaltCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_COMPLETE` reader - 1:1\\]
Task Complete Interrupt \\[TCC\\]
This status bit is asserted \\[if CQISTE.TCC=1\\]
when atleast one of the following two conditions are met: \\[1\\]
A task is completed and the INT bit is set in its Task Descriptor \\[2\\]
Interrupt caused by Interrupt Coalescing logic \\[see Section C.4.9\\]"]
pub type TaskCompleteR = crate::BitReader;
#[doc = "Field `TASK_COMPLETE` writer - 1:1\\]
Task Complete Interrupt \\[TCC\\]
This status bit is asserted \\[if CQISTE.TCC=1\\]
when atleast one of the following two conditions are met: \\[1\\]
A task is completed and the INT bit is set in its Task Descriptor \\[2\\]
Interrupt caused by Interrupt Coalescing logic \\[see Section C.4.9\\]"]
pub type TaskCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR_DET` reader - 2:2\\]
Response Error Detected Interrupt \\[RED\\]
This status bit is asserted \\[if CQISTE.RED=1\\]
when a response is received with an error bit set in the device status field. The contents of the device status field are listed in Section 6.13.Software uses CQRMEM register to configure which device status bit fields may trigger an interrupt, and which are masked."]
pub type RespErrDetR = crate::BitReader;
#[doc = "Field `RESP_ERR_DET` writer - 2:2\\]
Response Error Detected Interrupt \\[RED\\]
This status bit is asserted \\[if CQISTE.RED=1\\]
when a response is received with an error bit set in the device status field. The contents of the device status field are listed in Section 6.13.Software uses CQRMEM register to configure which device status bit fields may trigger an interrupt, and which are masked."]
pub type RespErrDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_CLEARED` reader - 3:3\\]
Task Cleared \\[TCL\\]
This status bit is asserted \\[if CQISTE.TCL=1\\]
when a task clear operation is completed by CQE. The com-pleted task clear operation is either an individual task clear \\[CQTCLR\\]
or clearing of all tasks \\[CQCTL\\]."]
pub type TaskClearedR = crate::BitReader;
#[doc = "Field `TASK_CLEARED` writer - 3:3\\]
Task Cleared \\[TCL\\]
This status bit is asserted \\[if CQISTE.TCL=1\\]
when a task clear operation is completed by CQE. The com-pleted task clear operation is either an individual task clear \\[CQTCLR\\]
or clearing of all tasks \\[CQCTL\\]."]
pub type TaskClearedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_ERROR` reader - 4:4\\]
Task Error Interrupt \\[TERR\\]
This bit is asserted when task error is detected due to invalid task descriptor"]
pub type TaskErrorR = crate::BitReader;
#[doc = "Field `TASK_ERROR` writer - 4:4\\]
Task Error Interrupt \\[TERR\\]
This bit is asserted when task error is detected due to invalid task descriptor"]
pub type TaskErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt Complete Interrupt \\[HAC\\]
This status bit is asserted \\[if CQISTE.HAC=1\\]
when halt bit in CQCTL register transitions from 0 to 1 indicating that host controller has completed its current ongoing task and has entered halt state."]
    #[inline(always)]
    pub fn halt_complete(&self) -> HaltCompleteR {
        HaltCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Task Complete Interrupt \\[TCC\\]
This status bit is asserted \\[if CQISTE.TCC=1\\]
when atleast one of the following two conditions are met: \\[1\\]
A task is completed and the INT bit is set in its Task Descriptor \\[2\\]
Interrupt caused by Interrupt Coalescing logic \\[see Section C.4.9\\]"]
    #[inline(always)]
    pub fn task_complete(&self) -> TaskCompleteR {
        TaskCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Response Error Detected Interrupt \\[RED\\]
This status bit is asserted \\[if CQISTE.RED=1\\]
when a response is received with an error bit set in the device status field. The contents of the device status field are listed in Section 6.13.Software uses CQRMEM register to configure which device status bit fields may trigger an interrupt, and which are masked."]
    #[inline(always)]
    pub fn resp_err_det(&self) -> RespErrDetR {
        RespErrDetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Task Cleared \\[TCL\\]
This status bit is asserted \\[if CQISTE.TCL=1\\]
when a task clear operation is completed by CQE. The com-pleted task clear operation is either an individual task clear \\[CQTCLR\\]
or clearing of all tasks \\[CQCTL\\]."]
    #[inline(always)]
    pub fn task_cleared(&self) -> TaskClearedR {
        TaskClearedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Task Error Interrupt \\[TERR\\]
This bit is asserted when task error is detected due to invalid task descriptor"]
    #[inline(always)]
    pub fn task_error(&self) -> TaskErrorR {
        TaskErrorR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt Complete Interrupt \\[HAC\\]
This status bit is asserted \\[if CQISTE.HAC=1\\]
when halt bit in CQCTL register transitions from 0 to 1 indicating that host controller has completed its current ongoing task and has entered halt state."]
    #[inline(always)]
    #[must_use]
    pub fn halt_complete(&mut self) -> HaltCompleteW<SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec> {
        HaltCompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Task Complete Interrupt \\[TCC\\]
This status bit is asserted \\[if CQISTE.TCC=1\\]
when atleast one of the following two conditions are met: \\[1\\]
A task is completed and the INT bit is set in its Task Descriptor \\[2\\]
Interrupt caused by Interrupt Coalescing logic \\[see Section C.4.9\\]"]
    #[inline(always)]
    #[must_use]
    pub fn task_complete(&mut self) -> TaskCompleteW<SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec> {
        TaskCompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Response Error Detected Interrupt \\[RED\\]
This status bit is asserted \\[if CQISTE.RED=1\\]
when a response is received with an error bit set in the device status field. The contents of the device status field are listed in Section 6.13.Software uses CQRMEM register to configure which device status bit fields may trigger an interrupt, and which are masked."]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_det(&mut self) -> RespErrDetW<SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec> {
        RespErrDetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Task Cleared \\[TCL\\]
This status bit is asserted \\[if CQISTE.TCL=1\\]
when a task clear operation is completed by CQE. The com-pleted task clear operation is either an individual task clear \\[CQTCLR\\]
or clearing of all tasks \\[CQCTL\\]."]
    #[inline(always)]
    #[must_use]
    pub fn task_cleared(&mut self) -> TaskClearedW<SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec> {
        TaskClearedW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Task Error Interrupt \\[TERR\\]
This bit is asserted when task error is detected due to invalid task descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn task_error(&mut self) -> TaskErrorW<SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec> {
        TaskErrorW::new(self, 4)
    }
}
#[doc = "This register indicates pending interrupts that require service. Each bit in this registers is asserted 296 in response a specific event, only if the respective bit is set in CQ ISTE register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec {
    const RESET_VALUE: u32 = 0;
}
