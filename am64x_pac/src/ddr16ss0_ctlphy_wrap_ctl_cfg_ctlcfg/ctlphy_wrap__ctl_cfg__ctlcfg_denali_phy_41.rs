#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_41` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_41` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec>;
#[doc = "Field `PHY_USER_PATT4_0` reader - 15:0\\]
User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
pub type PhyUserPatt4_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_USER_PATT4_0` writer - 15:0\\]
User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
pub type PhyUserPatt4_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_NTP_MULT_TRAIN_0` reader - 16:16\\]
Control for single pass only No-Topology training for slice 0."]
pub type PhyNtpMultTrain0R = crate::BitReader;
#[doc = "Field `PHY_NTP_MULT_TRAIN_0` writer - 16:16\\]
Control for single pass only No-Topology training for slice 0."]
pub type PhyNtpMultTrain0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
    #[inline(always)]
    pub fn phy_user_patt4_0(&self) -> PhyUserPatt4_0R {
        PhyUserPatt4_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Control for single pass only No-Topology training for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_mult_train_0(&self) -> PhyNtpMultTrain0R {
        PhyNtpMultTrain0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
User-defined pattern to be used during write data leveling for slice 0. This register holds the DM bit for the 15 to 0 DQ written/read from device."]
    #[inline(always)]
    #[must_use]
    pub fn phy_user_patt4_0(&mut self) -> PhyUserPatt4_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec> {
        PhyUserPatt4_0W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Control for single pass only No-Topology training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_mult_train_0(
        &mut self,
    ) -> PhyNtpMultTrain0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec> {
        PhyNtpMultTrain0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_41::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_41::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_41::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_41::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_41 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy41Spec {
    const RESET_VALUE: u32 = 0;
}
