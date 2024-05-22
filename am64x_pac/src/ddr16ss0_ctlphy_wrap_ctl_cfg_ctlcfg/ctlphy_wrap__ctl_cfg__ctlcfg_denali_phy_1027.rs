#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1027` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1027` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_2` reader - 10:0\\]
Observation register containing master delay results for address slice 2. READ-ONLY"]
pub type PhyAdrMasterDlyLockObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_2` writer - 10:0\\]
Observation register containing master delay results for address slice 2. READ-ONLY"]
pub type PhyAdrMasterDlyLockObs2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_BASE_SLV_DLY_ENC_OBS_2` reader - 22:16\\]
Observation register containing base slave delay for address slice 2. READ-ONLY"]
pub type PhyAdrBaseSlvDlyEncObs2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_BASE_SLV_DLY_ENC_OBS_2` writer - 22:16\\]
Observation register containing base slave delay for address slice 2. READ-ONLY"]
pub type PhyAdrBaseSlvDlyEncObs2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_ADR_ADDER_SLV_DLY_ENC_OBS_2` reader - 31:24\\]
Observation register containing addr slave delay for address slice 2. READ-ONLY"]
pub type PhyAdrAdderSlvDlyEncObs2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_ADDER_SLV_DLY_ENC_OBS_2` writer - 31:24\\]
Observation register containing addr slave delay for address slice 2. READ-ONLY"]
pub type PhyAdrAdderSlvDlyEncObs2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing master delay results for address slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_2(&self) -> PhyAdrMasterDlyLockObs2R {
        PhyAdrMasterDlyLockObs2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Observation register containing base slave delay for address slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_base_slv_dly_enc_obs_2(&self) -> PhyAdrBaseSlvDlyEncObs2R {
        PhyAdrBaseSlvDlyEncObs2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing addr slave delay for address slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_adder_slv_dly_enc_obs_2(&self) -> PhyAdrAdderSlvDlyEncObs2R {
        PhyAdrAdderSlvDlyEncObs2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing master delay results for address slice 2. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_dly_lock_obs_2(
        &mut self,
    ) -> PhyAdrMasterDlyLockObs2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec> {
        PhyAdrMasterDlyLockObs2W::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Observation register containing base slave delay for address slice 2. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_base_slv_dly_enc_obs_2(
        &mut self,
    ) -> PhyAdrBaseSlvDlyEncObs2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec> {
        PhyAdrBaseSlvDlyEncObs2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing addr slave delay for address slice 2. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_adder_slv_dly_enc_obs_2(
        &mut self,
    ) -> PhyAdrAdderSlvDlyEncObs2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec> {
        PhyAdrAdderSlvDlyEncObs2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1027\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1027::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1027::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1027::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1027::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1027 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1027Spec {
    const RESET_VALUE: u32 = 0;
}
