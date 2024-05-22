#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_135` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_135` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec>;
#[doc = "Field `PI_NO_MRW_BT_INIT` reader - 0:0\\]
Disable MRW commands before training during initialization. Set to 1 to disable."]
pub type PiNoMrwBtInitR = crate::BitReader;
#[doc = "Field `PI_NO_MRW_BT_INIT` writer - 0:0\\]
Disable MRW commands before training during initialization. Set to 1 to disable."]
pub type PiNoMrwBtInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_NO_MRW_INIT` reader - 8:8\\]
Disable MRW commands after training during initialization. Set to 1 to disable."]
pub type PiNoMrwInitR = crate::BitReader;
#[doc = "Field `PI_NO_MRW_INIT` writer - 8:8\\]
Disable MRW commands after training during initialization. Set to 1 to disable."]
pub type PiNoMrwInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_NO_PHY_IND_TRAIN_INIT` reader - 16:16\\]
Disable PHY Independent Training during initialization. Set to 1 to disable."]
pub type PiNoPhyIndTrainInitR = crate::BitReader;
#[doc = "Field `PI_NO_PHY_IND_TRAIN_INIT` writer - 16:16\\]
Disable PHY Independent Training during initialization. Set to 1 to disable."]
pub type PiNoPhyIndTrainInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_NO_AUTO_MRR_INIT` reader - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
pub type PiNoAutoMrrInitR = crate::BitReader;
#[doc = "Field `PI_NO_AUTO_MRR_INIT` writer - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
pub type PiNoAutoMrrInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disable MRW commands before training during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_no_mrw_bt_init(&self) -> PiNoMrwBtInitR {
        PiNoMrwBtInitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable MRW commands after training during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_no_mrw_init(&self) -> PiNoMrwInitR {
        PiNoMrwInitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Disable PHY Independent Training during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_no_phy_ind_train_init(&self) -> PiNoPhyIndTrainInitR {
        PiNoPhyIndTrainInitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_no_auto_mrr_init(&self) -> PiNoAutoMrrInitR {
        PiNoAutoMrrInitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disable MRW commands before training during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_no_mrw_bt_init(&mut self) -> PiNoMrwBtInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec> {
        PiNoMrwBtInitW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable MRW commands after training during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_no_mrw_init(&mut self) -> PiNoMrwInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec> {
        PiNoMrwInitW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Disable PHY Independent Training during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_no_phy_ind_train_init(
        &mut self,
    ) -> PiNoPhyIndTrainInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec> {
        PiNoPhyIndTrainInitW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_no_auto_mrr_init(
        &mut self,
    ) -> PiNoAutoMrrInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec> {
        PiNoAutoMrrInitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_135::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_135::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_135::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_135::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_135 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi135Spec {
    const RESET_VALUE: u32 = 0;
}
