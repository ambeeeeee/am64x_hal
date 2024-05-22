#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_index` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_index` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec>;
#[doc = "Field `LAST_CRI` reader - 5:0\\]
This field stores the index of the last received command response. CQE shall update the value every time a com-mand response is received."]
pub type LastCriR = crate::FieldReader;
#[doc = "Field `LAST_CRI` writer - 5:0\\]
This field stores the index of the last received command response. CQE shall update the value every time a com-mand response is received."]
pub type LastCriW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
This field stores the index of the last received command response. CQE shall update the value every time a com-mand response is received."]
    #[inline(always)]
    pub fn last_cri(&self) -> LastCriR {
        LastCriR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
This field stores the index of the last received command response. CQE shall update the value every time a com-mand response is received."]
    #[inline(always)]
    #[must_use]
    pub fn last_cri(&mut self) -> LastCriW<SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec> {
        LastCriW::new(self, 0)
    }
}
#[doc = "This register stores the index of the last received command response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_index to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec {
    const RESET_VALUE: u32 = 0;
}
