#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_124` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_124` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec>;
#[doc = "Field `PI_BIST_PAT_NUM` reader - 5:0\\]
Sets the max used pattern number of BIST from a total of 8 built-in patterns. Ex. set to 3, The BIST would use pattern 1, 2 and 3."]
pub type PiBistPatNumR = crate::FieldReader;
#[doc = "Field `PI_BIST_PAT_NUM` writer - 5:0\\]
Sets the max used pattern number of BIST from a total of 8 built-in patterns. Ex. set to 3, The BIST would use pattern 1, 2 and 3."]
pub type PiBistPatNumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Sets the max used pattern number of BIST from a total of 8 built-in patterns. Ex. set to 3, The BIST would use pattern 1, 2 and 3."]
    #[inline(always)]
    pub fn pi_bist_pat_num(&self) -> PiBistPatNumR {
        PiBistPatNumR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Sets the max used pattern number of BIST from a total of 8 built-in patterns. Ex. set to 3, The BIST would use pattern 1, 2 and 3."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_pat_num(&mut self) -> PiBistPatNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec> {
        PiBistPatNumW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_124::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_124::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_124::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_124::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_124 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi124Spec {
    const RESET_VALUE: u32 = 0;
}
