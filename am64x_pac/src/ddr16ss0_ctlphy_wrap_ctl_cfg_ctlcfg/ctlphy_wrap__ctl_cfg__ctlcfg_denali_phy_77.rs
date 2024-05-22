#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_77` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_77` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec>;
#[doc = "Field `PHY_NTP_WDQ_START_0` reader - 10:0\\]
Starting WR DQ slave delay in No-Topology training for slice 0."]
pub type PhyNtpWdqStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_NTP_WDQ_START_0` writer - 10:0\\]
Starting WR DQ slave delay in No-Topology training for slice 0."]
pub type PhyNtpWdqStart0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_NTP_WDQ_STOP_0` reader - 26:16\\]
End of WR DQ slave delay in No-Topology training for slice 0."]
pub type PhyNtpWdqStop0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_NTP_WDQ_STOP_0` writer - 26:16\\]
End of WR DQ slave delay in No-Topology training for slice 0."]
pub type PhyNtpWdqStop0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Starting WR DQ slave delay in No-Topology training for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_wdq_start_0(&self) -> PhyNtpWdqStart0R {
        PhyNtpWdqStart0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
End of WR DQ slave delay in No-Topology training for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_wdq_stop_0(&self) -> PhyNtpWdqStop0R {
        PhyNtpWdqStop0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Starting WR DQ slave delay in No-Topology training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_wdq_start_0(
        &mut self,
    ) -> PhyNtpWdqStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec> {
        PhyNtpWdqStart0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
End of WR DQ slave delay in No-Topology training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_wdq_stop_0(
        &mut self,
    ) -> PhyNtpWdqStop0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec> {
        PhyNtpWdqStop0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_77::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_77::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_77::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_77::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_77 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy77Spec {
    const RESET_VALUE: u32 = 0;
}
