#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1064` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1064` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_HALF_MEASURE_2` reader - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 2"]
pub type PhyAdrMasterDelayHalfMeasure2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_HALF_MEASURE_2` writer - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 2"]
pub type PhyAdrMasterDelayHalfMeasure2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_2` reader - 17:8\\]
Sets the software override data valid window size during CA training for address slice 2."]
pub type PhyAdrSwCalvlDvwMin2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_2` writer - 17:8\\]
Sets the software override data valid window size during CA training for address slice 2."]
pub type PhyAdrSwCalvlDvwMin2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_EN_2` reader - 24:24\\]
Enables the software override data valid window size during CA training for address slice 2."]
pub type PhyAdrSwCalvlDvwMinEn2R = crate::BitReader;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_EN_2` writer - 24:24\\]
Enables the software override data valid window size during CA training for address slice 2."]
pub type PhyAdrSwCalvlDvwMinEn2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 2"]
    #[inline(always)]
    pub fn phy_adr_master_delay_half_measure_2(&self) -> PhyAdrMasterDelayHalfMeasure2R {
        PhyAdrMasterDelayHalfMeasure2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Sets the software override data valid window size during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_sw_calvl_dvw_min_2(&self) -> PhyAdrSwCalvlDvwMin2R {
        PhyAdrSwCalvlDvwMin2R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the software override data valid window size during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_sw_calvl_dvw_min_en_2(&self) -> PhyAdrSwCalvlDvwMinEn2R {
        PhyAdrSwCalvlDvwMinEn2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 2"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_half_measure_2(
        &mut self,
    ) -> PhyAdrMasterDelayHalfMeasure2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec> {
        PhyAdrMasterDelayHalfMeasure2W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Sets the software override data valid window size during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_calvl_dvw_min_2(
        &mut self,
    ) -> PhyAdrSwCalvlDvwMin2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec> {
        PhyAdrSwCalvlDvwMin2W::new(self, 8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the software override data valid window size during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_calvl_dvw_min_en_2(
        &mut self,
    ) -> PhyAdrSwCalvlDvwMinEn2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec> {
        PhyAdrSwCalvlDvwMinEn2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1064\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1064::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1064::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1064::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1064::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1064 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1064Spec {
    const RESET_VALUE: u32 = 0;
}
