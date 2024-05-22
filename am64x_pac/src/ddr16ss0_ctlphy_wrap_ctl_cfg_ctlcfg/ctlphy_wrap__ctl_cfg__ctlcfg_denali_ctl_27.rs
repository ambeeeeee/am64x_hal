#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_27` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_27` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec>;
#[doc = "Field `DQS_OSC_TST` reader - 0:0\\]
Enable DQS Oscillator TEST mode."]
pub type DqsOscTstR = crate::BitReader;
#[doc = "Field `DQS_OSC_TST` writer - 0:0\\]
Enable DQS Oscillator TEST mode."]
pub type DqsOscTstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS_OSC_MPC_CMD` reader - 31:8\\]
Set MPC encoding for DQS Oscillator TEST mode."]
pub type DqsOscMpcCmdR = crate::FieldReader<u32>;
#[doc = "Field `DQS_OSC_MPC_CMD` writer - 31:8\\]
Set MPC encoding for DQS Oscillator TEST mode."]
pub type DqsOscMpcCmdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable DQS Oscillator TEST mode."]
    #[inline(always)]
    pub fn dqs_osc_tst(&self) -> DqsOscTstR {
        DqsOscTstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Set MPC encoding for DQS Oscillator TEST mode."]
    #[inline(always)]
    pub fn dqs_osc_mpc_cmd(&self) -> DqsOscMpcCmdR {
        DqsOscMpcCmdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable DQS Oscillator TEST mode."]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_tst(&mut self) -> DqsOscTstW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec> {
        DqsOscTstW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Set MPC encoding for DQS Oscillator TEST mode."]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_mpc_cmd(&mut self) -> DqsOscMpcCmdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec> {
        DqsOscMpcCmdW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_27::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_27::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_27 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl27Spec {
    const RESET_VALUE: u32 = 0;
}
