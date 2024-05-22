#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_422` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_422` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec>;
#[doc = "Field `TDFI_CTRLMSG_RESP_F0` reader - 6:0\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=0"]
pub type TdfiCtrlmsgRespF0R = crate::FieldReader;
#[doc = "Field `TDFI_CTRLMSG_RESP_F0` writer - 6:0\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=0"]
pub type TdfiCtrlmsgRespF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TDFI_CTRLMSG_RESP_F1` reader - 14:8\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=1"]
pub type TdfiCtrlmsgRespF1R = crate::FieldReader;
#[doc = "Field `TDFI_CTRLMSG_RESP_F1` writer - 14:8\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=1"]
pub type TdfiCtrlmsgRespF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TDFI_CTRLMSG_RESP_F2` reader - 22:16\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=2"]
pub type TdfiCtrlmsgRespF2R = crate::FieldReader;
#[doc = "Field `TDFI_CTRLMSG_RESP_F2` writer - 22:16\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=2"]
pub type TdfiCtrlmsgRespF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=0"]
    #[inline(always)]
    pub fn tdfi_ctrlmsg_resp_f0(&self) -> TdfiCtrlmsgRespF0R {
        TdfiCtrlmsgRespF0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=1"]
    #[inline(always)]
    pub fn tdfi_ctrlmsg_resp_f1(&self) -> TdfiCtrlmsgRespF1R {
        TdfiCtrlmsgRespF1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=2"]
    #[inline(always)]
    pub fn tdfi_ctrlmsg_resp_f2(&self) -> TdfiCtrlmsgRespF2R {
        TdfiCtrlmsgRespF2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlmsg_resp_f0(
        &mut self,
    ) -> TdfiCtrlmsgRespF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec> {
        TdfiCtrlmsgRespF0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlmsg_resp_f1(
        &mut self,
    ) -> TdfiCtrlmsgRespF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec> {
        TdfiCtrlmsgRespF1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Defines the DFI tCTRLMSG_RESP timing parameter \\[in DFI clocks\\], the maximum number of DFI clocks allowed for dfi_ctrlmsg_ack to assert after dfi_ctrlmsg_req goes high. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlmsg_resp_f2(
        &mut self,
    ) -> TdfiCtrlmsgRespF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec> {
        TdfiCtrlmsgRespF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_422\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_422::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_422::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_422::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_422::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_422 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl422Spec {
    const RESET_VALUE: u32 = 0;
}
