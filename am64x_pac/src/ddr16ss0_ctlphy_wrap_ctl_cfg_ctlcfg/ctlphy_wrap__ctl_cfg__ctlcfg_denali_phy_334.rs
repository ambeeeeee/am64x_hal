#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_334` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_334` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec>;
#[doc = "Field `PHY_NTP_WDQ_BIT_EN_1` reader - 7:0\\]
Enable Bit for WR DQ during No-Topology training for slice 1."]
pub type PhyNtpWdqBitEn1R = crate::FieldReader;
#[doc = "Field `PHY_NTP_WDQ_BIT_EN_1` writer - 7:0\\]
Enable Bit for WR DQ during No-Topology training for slice 1."]
pub type PhyNtpWdqBitEn1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_DVW_MIN_1` reader - 17:8\\]
Minimum data valid window across DQs and ranks for slice 1."]
pub type PhyWdqlvlDvwMin1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DVW_MIN_1` writer - 17:8\\]
Minimum data valid window across DQs and ranks for slice 1."]
pub type PhyWdqlvlDvwMin1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_SW_WDQLVL_DVW_MIN_EN_1` reader - 24:24\\]
SW override to enable use of PHY_WDQLVL_DVW_MIN for slice 1."]
pub type PhySwWdqlvlDvwMinEn1R = crate::BitReader;
#[doc = "Field `PHY_SW_WDQLVL_DVW_MIN_EN_1` writer - 24:24\\]
SW override to enable use of PHY_WDQLVL_DVW_MIN for slice 1."]
pub type PhySwWdqlvlDvwMinEn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enable Bit for WR DQ during No-Topology training for slice 1."]
    #[inline(always)]
    pub fn phy_ntp_wdq_bit_en_1(&self) -> PhyNtpWdqBitEn1R {
        PhyNtpWdqBitEn1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Minimum data valid window across DQs and ranks for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dvw_min_1(&self) -> PhyWdqlvlDvwMin1R {
        PhyWdqlvlDvwMin1R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
SW override to enable use of PHY_WDQLVL_DVW_MIN for slice 1."]
    #[inline(always)]
    pub fn phy_sw_wdqlvl_dvw_min_en_1(&self) -> PhySwWdqlvlDvwMinEn1R {
        PhySwWdqlvlDvwMinEn1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enable Bit for WR DQ during No-Topology training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_wdq_bit_en_1(
        &mut self,
    ) -> PhyNtpWdqBitEn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec> {
        PhyNtpWdqBitEn1W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Minimum data valid window across DQs and ranks for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dvw_min_1(
        &mut self,
    ) -> PhyWdqlvlDvwMin1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec> {
        PhyWdqlvlDvwMin1W::new(self, 8)
    }
    #[doc = "Bit 24 - 24:24\\]
SW override to enable use of PHY_WDQLVL_DVW_MIN for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wdqlvl_dvw_min_en_1(
        &mut self,
    ) -> PhySwWdqlvlDvwMinEn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec> {
        PhySwWdqlvlDvwMinEn1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_334\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_334::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_334::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_334::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_334::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_334 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy334Spec {
    const RESET_VALUE: u32 = 0;
}
