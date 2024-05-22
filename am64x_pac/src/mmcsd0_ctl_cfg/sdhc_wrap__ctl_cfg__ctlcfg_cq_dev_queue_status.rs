#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_queue_status` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_queue_status` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec>;
#[doc = "Field `CQDQ_STS` reader - 31:0\\]
Every time the Host controller receives a queue status register \\[QSR\\]
from the device, it updates this register with the response of status command, i.e. the devices queue status."]
pub type CqdqStsR = crate::FieldReader<u32>;
#[doc = "Field `CQDQ_STS` writer - 31:0\\]
Every time the Host controller receives a queue status register \\[QSR\\]
from the device, it updates this register with the response of status command, i.e. the devices queue status."]
pub type CqdqStsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Every time the Host controller receives a queue status register \\[QSR\\]
from the device, it updates this register with the response of status command, i.e. the devices queue status."]
    #[inline(always)]
    pub fn cqdq_sts(&self) -> CqdqStsR {
        CqdqStsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Every time the Host controller receives a queue status register \\[QSR\\]
from the device, it updates this register with the response of status command, i.e. the devices queue status."]
    #[inline(always)]
    #[must_use]
    pub fn cqdq_sts(&mut self) -> CqdqStsW<SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec> {
        CqdqStsW::new(self, 0)
    }
}
#[doc = "This register stores the most recent value of the device s queue status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_queue_status to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec {
    const RESET_VALUE: u32 = 0;
}
