#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_123` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_123` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_0` reader - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_START_0` writer - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyStart0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_NTP_WRLAT_START_0` reader - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 0."]
pub type PhyNtpWrlatStart0R = crate::FieldReader;
#[doc = "Field `PHY_NTP_WRLAT_START_0` writer - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 0."]
pub type PhyNtpWrlatStart0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_NTP_PASS_0` reader - 24:24\\]
Indicates if No-topology training found a passing result for slice 0."]
pub type PhyNtpPass0R = crate::BitReader;
#[doc = "Field `PHY_NTP_PASS_0` writer - 24:24\\]
Indicates if No-topology training found a passing result for slice 0."]
pub type PhyNtpPass0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_0(&self) -> PhyWdqlvlDqdmSlvDlyStart0R {
        PhyWdqlvlDqdmSlvDlyStart0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_wrlat_start_0(&self) -> PhyNtpWrlatStart0R {
        PhyNtpWrlatStart0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates if No-topology training found a passing result for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_pass_0(&self) -> PhyNtpPass0R {
        PhyNtpPass0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Initial DQ/DM slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_start_0(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec> {
        PhyWdqlvlDqdmSlvDlyStart0W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Initial value for phy_write_path_lat_add for No-topology training and early threshold for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_wrlat_start_0(
        &mut self,
    ) -> PhyNtpWrlatStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec> {
        PhyNtpWrlatStart0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates if No-topology training found a passing result for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_pass_0(&mut self) -> PhyNtpPass0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec> {
        PhyNtpPass0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_123::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_123::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_123 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy123Spec {
    const RESET_VALUE: u32 = 0;
}
