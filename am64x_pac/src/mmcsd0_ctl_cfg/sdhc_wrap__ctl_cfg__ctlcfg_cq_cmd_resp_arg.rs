#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_arg` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_arg` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec>;
#[doc = "Field `LAST_CRA` reader - 31:0\\]
This field stores the argument of the last received com-mand. CQE shall update the value every time a com-mand response is received."]
pub type LastCraR = crate::FieldReader<u32>;
#[doc = "Field `LAST_CRA` writer - 31:0\\]
This field stores the argument of the last received com-mand. CQE shall update the value every time a com-mand response is received."]
pub type LastCraW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field stores the argument of the last received com-mand. CQE shall update the value every time a com-mand response is received."]
    #[inline(always)]
    pub fn last_cra(&self) -> LastCraR {
        LastCraR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field stores the argument of the last received com-mand. CQE shall update the value every time a com-mand response is received."]
    #[inline(always)]
    #[must_use]
    pub fn last_cra(&mut self) -> LastCraW<SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec> {
        LastCraW::new(self, 0)
    }
}
#[doc = "This register stores the index of the last received command response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_arg to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec {
    const RESET_VALUE: u32 = 0;
}
