#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1063` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1063` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_2` reader - 10:0\\]
Start value for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStart2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_START_2` writer - 10:0\\]
Start value for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStart2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_2` reader - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStep2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_STEP_2` writer - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 2."]
pub type PhyAdrMasterDelayStep2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_2` reader - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 2. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DELAY_WAIT_2` writer - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 2. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
pub type PhyAdrMasterDelayWait2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Start value for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_master_delay_start_2(&self) -> PhyAdrMasterDelayStart2R {
        PhyAdrMasterDelayStart2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_master_delay_step_2(&self) -> PhyAdrMasterDelayStep2R {
        PhyAdrMasterDelayStep2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 2. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    pub fn phy_adr_master_delay_wait_2(&self) -> PhyAdrMasterDelayWait2R {
        PhyAdrMasterDelayWait2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Start value for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_start_2(
        &mut self,
    ) -> PhyAdrMasterDelayStart2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec> {
        PhyAdrMasterDelayStart2W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Incremental step size for master delay line locking algorithm for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_step_2(
        &mut self,
    ) -> PhyAdrMasterDelayStep2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec> {
        PhyAdrMasterDelayStep2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Wait cycles for master delay line locking algorithm for address slice 2. Bits \\[3:0\\]
is the cycle wait count after a calibration clock setting change. Bits \\[7:4\\]
is the cycle wait count after a master delay setting change."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_delay_wait_2(
        &mut self,
    ) -> PhyAdrMasterDelayWait2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec> {
        PhyAdrMasterDelayWait2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1063\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1063::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1063::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1063::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1063::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1063 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1063Spec {
    const RESET_VALUE: u32 = 0;
}
