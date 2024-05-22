#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_124` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_124` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_0` reader - 9:0\\]
Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
pub type PhyRdlvlRddqsDqSlvDlyStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_SLV_DLY_START_0` writer - 9:0\\]
Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
pub type PhyRdlvlRddqsDqSlvDlyStart0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_0(&self) -> PhyRdlvlRddqsDqSlvDlyStart0R {
        PhyRdlvlRddqsDqSlvDlyStart0R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Read leveling starting value for the DQS/DQ slave delay settings for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_slv_dly_start_0(
        &mut self,
    ) -> PhyRdlvlRddqsDqSlvDlyStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec> {
        PhyRdlvlRddqsDqSlvDlyStart0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_124::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_124::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_124::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_124::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_124 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy124Spec {
    const RESET_VALUE: u32 = 0;
}
