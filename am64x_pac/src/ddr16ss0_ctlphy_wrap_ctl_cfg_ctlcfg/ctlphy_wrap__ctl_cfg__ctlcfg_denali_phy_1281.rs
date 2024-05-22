#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1281` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1281` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec>;
#[doc = "Field `PHY_FREQ_SEL_FROM_REGIF` reader - 0:0\\]
Indicates which source is used to select the frequency copy. When set to 1, the frequency select source is given by parameter PHY_FREQ_SEL from register I/F. When cleared to 0, the frequency select source is the PHY input signal dfi_frequency"]
pub type PhyFreqSelFromRegifR = crate::BitReader;
#[doc = "Field `PHY_FREQ_SEL_FROM_REGIF` writer - 0:0\\]
Indicates which source is used to select the frequency copy. When set to 1, the frequency select source is given by parameter PHY_FREQ_SEL from register I/F. When cleared to 0, the frequency select source is the PHY input signal dfi_frequency"]
pub type PhyFreqSelFromRegifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_FREQ_SEL_MULTICAST_EN` reader - 8:8\\]
When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
pub type PhyFreqSelMulticastEnR = crate::BitReader;
#[doc = "Field `PHY_FREQ_SEL_MULTICAST_EN` writer - 8:8\\]
When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
pub type PhyFreqSelMulticastEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_FREQ_SEL_INDEX` reader - 17:16\\]
Selects which frequency set to update when PHY_FREQ_SEL_MULTICAST_EN is not set."]
pub type PhyFreqSelIndexR = crate::FieldReader;
#[doc = "Field `PHY_FREQ_SEL_INDEX` writer - 17:16\\]
Selects which frequency set to update when PHY_FREQ_SEL_MULTICAST_EN is not set."]
pub type PhyFreqSelIndexW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_SW_GRP0_SHIFT_0` reader - 28:24\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp0Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP0_SHIFT_0` writer - 28:24\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp0Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates which source is used to select the frequency copy. When set to 1, the frequency select source is given by parameter PHY_FREQ_SEL from register I/F. When cleared to 0, the frequency select source is the PHY input signal dfi_frequency"]
    #[inline(always)]
    pub fn phy_freq_sel_from_regif(&self) -> PhyFreqSelFromRegifR {
        PhyFreqSelFromRegifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_freq_sel_multicast_en(&self) -> PhyFreqSelMulticastEnR {
        PhyFreqSelMulticastEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Selects which frequency set to update when PHY_FREQ_SEL_MULTICAST_EN is not set."]
    #[inline(always)]
    pub fn phy_freq_sel_index(&self) -> PhyFreqSelIndexR {
        PhyFreqSelIndexR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp0_shift_0(&self) -> PhySwGrp0Shift0R {
        PhySwGrp0Shift0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates which source is used to select the frequency copy. When set to 1, the frequency select source is given by parameter PHY_FREQ_SEL from register I/F. When cleared to 0, the frequency select source is the PHY input signal dfi_frequency"]
    #[inline(always)]
    #[must_use]
    pub fn phy_freq_sel_from_regif(
        &mut self,
    ) -> PhyFreqSelFromRegifW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec> {
        PhyFreqSelFromRegifW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_freq_sel_multicast_en(
        &mut self,
    ) -> PhyFreqSelMulticastEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec> {
        PhyFreqSelMulticastEnW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Selects which frequency set to update when PHY_FREQ_SEL_MULTICAST_EN is not set."]
    #[inline(always)]
    #[must_use]
    pub fn phy_freq_sel_index(
        &mut self,
    ) -> PhyFreqSelIndexW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec> {
        PhyFreqSelIndexW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp0_shift_0(
        &mut self,
    ) -> PhySwGrp0Shift0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec> {
        PhySwGrp0Shift0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1281\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1281::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1281::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1281::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1281::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1281 to value 0x0100"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1281Spec {
    const RESET_VALUE: u32 = 0x0100;
}
