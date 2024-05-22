#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_11` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_11` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec>;
#[doc = "Field `PI_INIT_WORK_FREQ` reader - 4:0\\]
Indicates the initial work frequency after initialization and initial leveling sequence."]
pub type PiInitWorkFreqR = crate::FieldReader;
#[doc = "Field `PI_INIT_WORK_FREQ` writer - 4:0\\]
Indicates the initial work frequency after initialization and initial leveling sequence."]
pub type PiInitWorkFreqW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_INIT_DFS_CALVL_ONLY` reader - 8:8\\]
Enables frequency training for CA leveling only. Other trainings are not performed."]
pub type PiInitDfsCalvlOnlyR = crate::BitReader;
#[doc = "Field `PI_INIT_DFS_CALVL_ONLY` writer - 8:8\\]
Enables frequency training for CA leveling only. Other trainings are not performed."]
pub type PiInitDfsCalvlOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Indicates the initial work frequency after initialization and initial leveling sequence."]
    #[inline(always)]
    pub fn pi_init_work_freq(&self) -> PiInitWorkFreqR {
        PiInitWorkFreqR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables frequency training for CA leveling only. Other trainings are not performed."]
    #[inline(always)]
    pub fn pi_init_dfs_calvl_only(&self) -> PiInitDfsCalvlOnlyR {
        PiInitDfsCalvlOnlyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Indicates the initial work frequency after initialization and initial leveling sequence."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_work_freq(&mut self) -> PiInitWorkFreqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec> {
        PiInitWorkFreqW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables frequency training for CA leveling only. Other trainings are not performed."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_dfs_calvl_only(
        &mut self,
    ) -> PiInitDfsCalvlOnlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec> {
        PiInitDfsCalvlOnlyW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_11::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_11::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_11 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi11Spec {
    const RESET_VALUE: u32 = 0;
}
