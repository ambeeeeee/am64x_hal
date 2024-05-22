#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_response` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgResponseSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_response` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgResponseSpec>;
#[doc = "Field `CMD_RESP` reader - 15:0\\]
R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type CmdRespR = crate::FieldReader<u16>;
#[doc = "Field `CMD_RESP` writer - 15:0\\]
R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
pub type CmdRespW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    pub fn cmd_resp(&self) -> CmdRespR {
        CmdRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
R\\[\\]
refers to a bit range within the response data as transmitted on the SD Bus, REP\\[\\]
refers to a bit range within the Response register."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_resp(&mut self) -> CmdRespW<SdhcWrap_CtlCfg_CtlcfgResponseSpec> {
        CmdRespW::new(self, 0)
    }
}
#[doc = "This register is used to store responses from SD Cards\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_response::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_response::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgResponseSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgResponseSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_response::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgResponseSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_response::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgResponseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_response to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgResponseSpec {
    const RESET_VALUE: u16 = 0;
}
