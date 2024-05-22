#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_515` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_515` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_0` reader - 10:0\\]
Observation register containing master delay results for address slice 0. READ-ONLY"]
pub type PhyAdrMasterDlyLockObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_0` writer - 10:0\\]
Observation register containing master delay results for address slice 0. READ-ONLY"]
pub type PhyAdrMasterDlyLockObs0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_BASE_SLV_DLY_ENC_OBS_0` reader - 22:16\\]
Observation register containing base slave delay for address slice 0. READ-ONLY"]
pub type PhyAdrBaseSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_BASE_SLV_DLY_ENC_OBS_0` writer - 22:16\\]
Observation register containing base slave delay for address slice 0. READ-ONLY"]
pub type PhyAdrBaseSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_ADR_ADDER_SLV_DLY_ENC_OBS_0` reader - 31:24\\]
Observation register containing addr slave delay for address slice 0. READ-ONLY"]
pub type PhyAdrAdderSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_ADDER_SLV_DLY_ENC_OBS_0` writer - 31:24\\]
Observation register containing addr slave delay for address slice 0. READ-ONLY"]
pub type PhyAdrAdderSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing master delay results for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_0(&self) -> PhyAdrMasterDlyLockObs0R {
        PhyAdrMasterDlyLockObs0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Observation register containing base slave delay for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_base_slv_dly_enc_obs_0(&self) -> PhyAdrBaseSlvDlyEncObs0R {
        PhyAdrBaseSlvDlyEncObs0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing addr slave delay for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_adder_slv_dly_enc_obs_0(&self) -> PhyAdrAdderSlvDlyEncObs0R {
        PhyAdrAdderSlvDlyEncObs0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing master delay results for address slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_dly_lock_obs_0(
        &mut self,
    ) -> PhyAdrMasterDlyLockObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec> {
        PhyAdrMasterDlyLockObs0W::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Observation register containing base slave delay for address slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_base_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyAdrBaseSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec> {
        PhyAdrBaseSlvDlyEncObs0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing addr slave delay for address slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_adder_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyAdrAdderSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec> {
        PhyAdrAdderSlvDlyEncObs0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_515\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_515::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_515::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_515::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_515::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_515 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy515Spec {
    const RESET_VALUE: u32 = 0;
}
