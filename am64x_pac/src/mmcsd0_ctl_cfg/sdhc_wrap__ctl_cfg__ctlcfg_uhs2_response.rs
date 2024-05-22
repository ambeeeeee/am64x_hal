#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_response` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_response` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec>;
#[doc = "Field `RESP_PKT_BYTE` reader - 7:0\\]
Host Controller saves received UHS-II RES Packet image to this register except the response of an abort command."]
pub type RespPktByteR = crate::FieldReader;
#[doc = "Field `RESP_PKT_BYTE` writer - 7:0\\]
Host Controller saves received UHS-II RES Packet image to this register except the response of an abort command."]
pub type RespPktByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Host Controller saves received UHS-II RES Packet image to this register except the response of an abort command."]
    #[inline(always)]
    pub fn resp_pkt_byte(&self) -> RespPktByteR {
        RespPktByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Host Controller saves received UHS-II RES Packet image to this register except the response of an abort command."]
    #[inline(always)]
    #[must_use]
    pub fn resp_pkt_byte(&mut self) -> RespPktByteW<SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec> {
        RespPktByteW::new(self, 0)
    }
}
#[doc = "This register is used to store received UHS-II RES Packet image\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_response to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec {
    const RESET_VALUE: u8 = 0;
}
