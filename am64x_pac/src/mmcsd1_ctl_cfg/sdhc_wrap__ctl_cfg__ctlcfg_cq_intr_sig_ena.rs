#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sig_ena` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sig_ena` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec>;
#[doc = "Field `HALT_COMPLETE` reader - 0:0\\]
Halt Complete Signal Enable \\[HAC\\]
When set and CQIS.HAC is asserted, the CQE shall generate an interrupt"]
pub type HaltCompleteR = crate::BitReader;
#[doc = "Field `HALT_COMPLETE` writer - 0:0\\]
Halt Complete Signal Enable \\[HAC\\]
When set and CQIS.HAC is asserted, the CQE shall generate an interrupt"]
pub type HaltCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_COMPLETE` reader - 1:1\\]
Task Complete Signal Enable \\[TCC\\]
When set and CQIS.TCC is asserted, the CQE shall generate an interrupt"]
pub type TaskCompleteR = crate::BitReader;
#[doc = "Field `TASK_COMPLETE` writer - 1:1\\]
Task Complete Signal Enable \\[TCC\\]
When set and CQIS.TCC is asserted, the CQE shall generate an interrupt"]
pub type TaskCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR_DET` reader - 2:2\\]
Response Error Detected Signal Enable \\[TCC\\]
When set and CQIS.RED is asserted, the CQE shall generate an interrupt"]
pub type RespErrDetR = crate::BitReader;
#[doc = "Field `RESP_ERR_DET` writer - 2:2\\]
Response Error Detected Signal Enable \\[TCC\\]
When set and CQIS.RED is asserted, the CQE shall generate an interrupt"]
pub type RespErrDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_CLEARED` reader - 3:3\\]
Task Cleared Signal Enable \\[TCL\\]
When set and CQIS.TCL is asserted, the CQE shall generate an interrupt"]
pub type TaskClearedR = crate::BitReader;
#[doc = "Field `TASK_CLEARED` writer - 3:3\\]
Task Cleared Signal Enable \\[TCL\\]
When set and CQIS.TCL is asserted, the CQE shall generate an interrupt"]
pub type TaskClearedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_ERROR` reader - 4:4\\]
Task Error Interrupt Signal Enable \\[TERR\\]
When set and CQIS.TERR is asserted, the CQE shall generate an interrupt"]
pub type TaskErrorR = crate::BitReader;
#[doc = "Field `TASK_ERROR` writer - 4:4\\]
Task Error Interrupt Signal Enable \\[TERR\\]
When set and CQIS.TERR is asserted, the CQE shall generate an interrupt"]
pub type TaskErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Halt Complete Signal Enable \\[HAC\\]
When set and CQIS.HAC is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    pub fn halt_complete(&self) -> HaltCompleteR {
        HaltCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Task Complete Signal Enable \\[TCC\\]
When set and CQIS.TCC is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    pub fn task_complete(&self) -> TaskCompleteR {
        TaskCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Response Error Detected Signal Enable \\[TCC\\]
When set and CQIS.RED is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    pub fn resp_err_det(&self) -> RespErrDetR {
        RespErrDetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Task Cleared Signal Enable \\[TCL\\]
When set and CQIS.TCL is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    pub fn task_cleared(&self) -> TaskClearedR {
        TaskClearedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Task Error Interrupt Signal Enable \\[TERR\\]
When set and CQIS.TERR is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    pub fn task_error(&self) -> TaskErrorR {
        TaskErrorR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Halt Complete Signal Enable \\[HAC\\]
When set and CQIS.HAC is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn halt_complete(&mut self) -> HaltCompleteW<SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec> {
        HaltCompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Task Complete Signal Enable \\[TCC\\]
When set and CQIS.TCC is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn task_complete(&mut self) -> TaskCompleteW<SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec> {
        TaskCompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Response Error Detected Signal Enable \\[TCC\\]
When set and CQIS.RED is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_det(&mut self) -> RespErrDetW<SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec> {
        RespErrDetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Task Cleared Signal Enable \\[TCL\\]
When set and CQIS.TCL is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn task_cleared(&mut self) -> TaskClearedW<SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec> {
        TaskClearedW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Task Error Interrupt Signal Enable \\[TERR\\]
When set and CQIS.TERR is asserted, the CQE shall generate an interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn task_error(&mut self) -> TaskErrorW<SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec> {
        TaskErrorW::new(self, 4)
    }
}
#[doc = "This register enables and disables the generation of interrupts to host software. When a bit is set 304 ( 1 ) and the corresponding bit in CQIS is set, then an interrupt is gene -rated. Interrupt sources 305 that are disabled ( 0 ) are still indicated in the CQIS register. This register is bit-index matched 306 to CQIS register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sig_ena to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec {
    const RESET_VALUE: u32 = 0;
}
