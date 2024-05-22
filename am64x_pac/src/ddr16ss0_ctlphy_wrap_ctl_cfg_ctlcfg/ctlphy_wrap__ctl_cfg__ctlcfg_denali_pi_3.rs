#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_3` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_3` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec>;
#[doc = "Field `PI_ID` reader - 15:0\\]
Holds the PI ID number. This is a Cadence DDR PHY IP identifier. It is set to 0x1387. READ-ONLY"]
pub type PiIdR = crate::FieldReader<u16>;
#[doc = "Field `PI_ID` writer - 15:0\\]
Holds the PI ID number. This is a Cadence DDR PHY IP identifier. It is set to 0x1387. READ-ONLY"]
pub type PiIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_RELEASE_DFI` reader - 16:16\\]
This is a status whether PI has release DFI. READ-ONLY."]
pub type PiReleaseDfiR = crate::BitReader;
#[doc = "Field `PI_RELEASE_DFI` writer - 16:16\\]
This is a status whether PI has release DFI. READ-ONLY."]
pub type PiReleaseDfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_NORMAL_LVL_SEQ` reader - 24:24\\]
Enable the PI to finish all the pending leveling before releasing the DFI bus."]
pub type PiNormalLvlSeqR = crate::BitReader;
#[doc = "Field `PI_NORMAL_LVL_SEQ` writer - 24:24\\]
Enable the PI to finish all the pending leveling before releasing the DFI bus."]
pub type PiNormalLvlSeqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Holds the PI ID number. This is a Cadence DDR PHY IP identifier. It is set to 0x1387. READ-ONLY"]
    #[inline(always)]
    pub fn pi_id(&self) -> PiIdR {
        PiIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
This is a status whether PI has release DFI. READ-ONLY."]
    #[inline(always)]
    pub fn pi_release_dfi(&self) -> PiReleaseDfiR {
        PiReleaseDfiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable the PI to finish all the pending leveling before releasing the DFI bus."]
    #[inline(always)]
    pub fn pi_normal_lvl_seq(&self) -> PiNormalLvlSeqR {
        PiNormalLvlSeqR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Holds the PI ID number. This is a Cadence DDR PHY IP identifier. It is set to 0x1387. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_id(&mut self) -> PiIdW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec> {
        PiIdW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This is a status whether PI has release DFI. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_release_dfi(&mut self) -> PiReleaseDfiW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec> {
        PiReleaseDfiW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable the PI to finish all the pending leveling before releasing the DFI bus."]
    #[inline(always)]
    #[must_use]
    pub fn pi_normal_lvl_seq(&mut self) -> PiNormalLvlSeqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec> {
        PiNormalLvlSeqW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_3::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_3::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_3 to value 0x0001_4999"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi3Spec {
    const RESET_VALUE: u32 = 0x0001_4999;
}
