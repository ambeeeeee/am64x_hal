#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_379` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_379` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_1` reader - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 1."]
pub type PhyWdqlvlDqdmSlvDlyStart1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_1` writer - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 1."]
pub type PhyWdqlvlDqdmSlvDlyStart1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_NTP_WRLAT_START_1` reader - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 1."]
pub type PhyNtpWrlatStart1R = crate::FieldReader;
#[doc = "Field `PHY_NTP_WRLAT_START_1` writer - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 1."]
pub type PhyNtpWrlatStart1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_NTP_PASS_1` reader - 24:24\\]
Indicates if No-topology training found a passing result for slice 1."]
pub type PhyNtpPass1R = crate::BitReader;
#[doc = "Field `PHY_NTP_PASS_1` writer - 24:24\\]
Indicates if No-topology training found a passing result for slice 1."]
pub type PhyNtpPass1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_1(&self) -> PhyWdqlvlDqdmSlvDlyStart1R {
        PhyWdqlvlDqdmSlvDlyStart1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 1."]
    #[inline(always)]
    pub fn phy_ntp_wrlat_start_1(&self) -> PhyNtpWrlatStart1R {
        PhyNtpWrlatStart1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates if No-topology training found a passing result for slice 1."]
    #[inline(always)]
    pub fn phy_ntp_pass_1(&self) -> PhyNtpPass1R {
        PhyNtpPass1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_1(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyStart1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec> {
        PhyWdqlvlDqdmSlvDlyStart1W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_wrlat_start_1(
        &mut self,
    ) -> PhyNtpWrlatStart1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec> {
        PhyNtpWrlatStart1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates if No-topology training found a passing result for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_pass_1(&mut self) -> PhyNtpPass1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec> {
        PhyNtpPass1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_379\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_379::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_379::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_379::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_379::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_379 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy379Spec {
    const RESET_VALUE: u32 = 0;
}
