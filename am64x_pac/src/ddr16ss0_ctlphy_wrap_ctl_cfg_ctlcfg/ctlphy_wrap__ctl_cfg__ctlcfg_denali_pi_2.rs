#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_2` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_2` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec>;
#[doc = "Field `PI_VERSION_1` reader - 31:0\\]
Holds the PI version number. This is a unique number for each PHY IP delivery. This will help in identifying different version of the PHY IP. READ-ONLY"]
pub type PiVersion1R = crate::FieldReader<u32>;
#[doc = "Field `PI_VERSION_1` writer - 31:0\\]
Holds the PI version number. This is a unique number for each PHY IP delivery. This will help in identifying different version of the PHY IP. READ-ONLY"]
pub type PiVersion1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Holds the PI version number. This is a unique number for each PHY IP delivery. This will help in identifying different version of the PHY IP. READ-ONLY"]
    #[inline(always)]
    pub fn pi_version_1(&self) -> PiVersion1R {
        PiVersion1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Holds the PI version number. This is a unique number for each PHY IP delivery. This will help in identifying different version of the PHY IP. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_version_1(&mut self) -> PiVersion1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec> {
        PiVersion1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_2::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_2::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_2 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi2Spec {
    const RESET_VALUE: u32 = 0;
}
