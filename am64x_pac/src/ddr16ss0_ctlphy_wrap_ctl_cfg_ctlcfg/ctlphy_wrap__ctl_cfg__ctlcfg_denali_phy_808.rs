#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_808` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_808` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_HALF_MEASURE_1` reader - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 1"]
pub type PhyAdrMasterDelayHalfMeasure1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_HALF_MEASURE_1` writer - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 1"]
pub type PhyAdrMasterDelayHalfMeasure1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_1` reader - 17:8\\]
Sets the software override data valid window size during CA training for address slice 1."]
pub type PhyAdrSwCalvlDvwMin1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_1` writer - 17:8\\]
Sets the software override data valid window size during CA training for address slice 1."]
pub type PhyAdrSwCalvlDvwMin1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_EN_1` reader - 24:24\\]
Enables the software override data valid window size during CA training for address slice 1."]
pub type PhyAdrSwCalvlDvwMinEn1R = crate::BitReader;
#[doc = "Field `PHY_ADR_SW_CALVL_DVW_MIN_EN_1` writer - 24:24\\]
Enables the software override data valid window size during CA training for address slice 1."]
pub type PhyAdrSwCalvlDvwMinEn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 1"]
    #[inline(always)]
    pub fn phy_adr_master_delay_half_measure_1(&self) -> PhyAdrMasterDelayHalfMeasure1R {
        PhyAdrMasterDelayHalfMeasure1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Sets the software override data valid window size during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_sw_calvl_dvw_min_1(&self) -> PhyAdrSwCalvlDvwMin1R {
        PhyAdrSwCalvlDvwMin1R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the software override data valid window size during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_sw_calvl_dvw_min_en_1(&self) -> PhyAdrSwCalvlDvwMinEn1R {
        PhyAdrSwCalvlDvwMinEn1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the number of delay line elements to be considered in determing whether to lock to a half clock cycle for the master in address slice 1"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_half_measure_1(
        &mut self,
    ) -> PhyAdrMasterDelayHalfMeasure1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec> {
        PhyAdrMasterDelayHalfMeasure1W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Sets the software override data valid window size during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_calvl_dvw_min_1(
        &mut self,
    ) -> PhyAdrSwCalvlDvwMin1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec> {
        PhyAdrSwCalvlDvwMin1W::new(self, 8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the software override data valid window size during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_calvl_dvw_min_en_1(
        &mut self,
    ) -> PhyAdrSwCalvlDvwMinEn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec> {
        PhyAdrSwCalvlDvwMinEn1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_808\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_808::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_808::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_808::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_808::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_808 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy808Spec {
    const RESET_VALUE: u32 = 0;
}
