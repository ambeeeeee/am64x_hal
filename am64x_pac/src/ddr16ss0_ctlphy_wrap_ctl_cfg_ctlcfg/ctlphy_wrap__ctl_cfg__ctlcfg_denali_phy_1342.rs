#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1342` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1342` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec>;
#[doc = "Field `PHY_CAL_PU_FINE_ADJ_0` reader - 7:0\\]
defines adjustment for PU code in pad calibration process"]
pub type PhyCalPuFineAdj0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_PU_FINE_ADJ_0` writer - 7:0\\]
defines adjustment for PU code in pad calibration process"]
pub type PhyCalPuFineAdj0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_CAL_PD_FINE_ADJ_0` reader - 15:8\\]
defines adjustment for PD code in pad calibration process"]
pub type PhyCalPdFineAdj0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_PD_FINE_ADJ_0` writer - 15:8\\]
defines adjustment for PD code in pad calibration process"]
pub type PhyCalPdFineAdj0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_CAL_RCV_FINE_ADJ_0` reader - 23:16\\]
defines adjustment for RCV code in pad calibration process"]
pub type PhyCalRcvFineAdj0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RCV_FINE_ADJ_0` writer - 23:16\\]
defines adjustment for RCV code in pad calibration process"]
pub type PhyCalRcvFineAdj0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_CAL_DBG_CFG_0` reader - 24:24\\]
defines debug configuration in pad calibration process"]
pub type PhyCalDbgCfg0R = crate::BitReader;
#[doc = "Field `PHY_CAL_DBG_CFG_0` writer - 24:24\\]
defines debug configuration in pad calibration process"]
pub type PhyCalDbgCfg0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
defines adjustment for PU code in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_pu_fine_adj_0(&self) -> PhyCalPuFineAdj0R {
        PhyCalPuFineAdj0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
defines adjustment for PD code in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_pd_fine_adj_0(&self) -> PhyCalPdFineAdj0R {
        PhyCalPdFineAdj0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
defines adjustment for RCV code in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_rcv_fine_adj_0(&self) -> PhyCalRcvFineAdj0R {
        PhyCalRcvFineAdj0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
defines debug configuration in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_dbg_cfg_0(&self) -> PhyCalDbgCfg0R {
        PhyCalDbgCfg0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
defines adjustment for PU code in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_pu_fine_adj_0(
        &mut self,
    ) -> PhyCalPuFineAdj0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec> {
        PhyCalPuFineAdj0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
defines adjustment for PD code in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_pd_fine_adj_0(
        &mut self,
    ) -> PhyCalPdFineAdj0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec> {
        PhyCalPdFineAdj0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
defines adjustment for RCV code in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_rcv_fine_adj_0(
        &mut self,
    ) -> PhyCalRcvFineAdj0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec> {
        PhyCalRcvFineAdj0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
defines debug configuration in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_dbg_cfg_0(
        &mut self,
    ) -> PhyCalDbgCfg0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec> {
        PhyCalDbgCfg0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1342\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1342::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1342::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1342::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1342::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1342 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1342Spec {
    const RESET_VALUE: u32 = 0;
}
