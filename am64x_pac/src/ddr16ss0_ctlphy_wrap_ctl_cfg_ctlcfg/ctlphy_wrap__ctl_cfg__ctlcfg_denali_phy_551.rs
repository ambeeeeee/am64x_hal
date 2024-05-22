#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_551` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_551` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_0` reader - 10:0\\]
Start value for master delay line locking algorithm for address slice 0."]
pub type PhyAdrMasterDelayStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_0` writer - 10:0\\]
Start value for master delay line locking algorithm for address slice 0."]
pub type PhyAdrMasterDelayStart0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_0` reader - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 0."]
pub type PhyAdrMasterDelayStep0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_0` writer - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 0."]
pub type PhyAdrMasterDelayStep0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_0` reader - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 0. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_0` writer - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 0. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Start value for master delay line locking algorithm for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_master_delay_start_0(&self) -> PhyAdrMasterDelayStart0R {
        PhyAdrMasterDelayStart0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_master_delay_step_0(&self) -> PhyAdrMasterDelayStep0R {
        PhyAdrMasterDelayStep0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 0. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_adr_master_delay_wait_0(&self) -> PhyAdrMasterDelayWait0R {
        PhyAdrMasterDelayWait0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Start value for master delay line locking algorithm for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_start_0(
        &mut self,
    ) -> PhyAdrMasterDelayStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec> {
        PhyAdrMasterDelayStart0W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_step_0(
        &mut self,
    ) -> PhyAdrMasterDelayStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec> {
        PhyAdrMasterDelayStep0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 0. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_wait_0(
        &mut self,
    ) -> PhyAdrMasterDelayWait0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec> {
        PhyAdrMasterDelayWait0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_551\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_551::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_551::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_551::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_551::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_551 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy551Spec {
    const RESET_VALUE: u32 = 0;
}
