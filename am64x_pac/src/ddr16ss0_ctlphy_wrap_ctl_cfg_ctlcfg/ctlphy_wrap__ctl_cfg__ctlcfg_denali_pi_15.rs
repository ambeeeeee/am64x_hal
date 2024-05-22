#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_15` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_15` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec>;
#[doc = "Field `PI_TMPRR` reader - 3:0\\]
DRAM tMPRR value in memory clock cycles."]
pub type PiTmprrR = crate::FieldReader;
#[doc = "Field `PI_TMPRR` writer - 3:0\\]
DRAM tMPRR value in memory clock cycles."]
pub type PiTmprrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_VRCG_EN` reader - 10:8\\]
Whether enable VRCG mode in two cases: bit0 - when DFS. bit1-when setting DQ Vref. bit2-when setting CBT."]
pub type PiVrcgEnR = crate::FieldReader;
#[doc = "Field `PI_VRCG_EN` writer - 10:8\\]
Whether enable VRCG mode in two cases: bit0 - when DFS. bit1-when setting DQ Vref. bit2-when setting CBT."]
pub type PiVrcgEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_MCAREF_FORWARD_ONLY` reader - 16:16\\]
Controls the generation of AREF from the PI module or forward the MC received value."]
pub type PiMcarefForwardOnlyR = crate::BitReader;
#[doc = "Field `PI_MCAREF_FORWARD_ONLY` writer - 16:16\\]
Controls the generation of AREF from the PI module or forward the MC received value."]
pub type PiMcarefForwardOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
DRAM tMPRR value in memory clock cycles."]
    #[inline(always)]
    pub fn pi_tmprr(&self) -> PiTmprrR {
        PiTmprrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Whether enable VRCG mode in two cases: bit0 - when DFS. bit1-when setting DQ Vref. bit2-when setting CBT."]
    #[inline(always)]
    pub fn pi_vrcg_en(&self) -> PiVrcgEnR {
        PiVrcgEnR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Controls the generation of AREF from the PI module or forward the MC received value."]
    #[inline(always)]
    pub fn pi_mcaref_forward_only(&self) -> PiMcarefForwardOnlyR {
        PiMcarefForwardOnlyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
DRAM tMPRR value in memory clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmprr(&mut self) -> PiTmprrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec> {
        PiTmprrW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Whether enable VRCG mode in two cases: bit0 - when DFS. bit1-when setting DQ Vref. bit2-when setting CBT."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vrcg_en(&mut self) -> PiVrcgEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec> {
        PiVrcgEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Controls the generation of AREF from the PI module or forward the MC received value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mcaref_forward_only(
        &mut self,
    ) -> PiMcarefForwardOnlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec> {
        PiMcarefForwardOnlyW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_15::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_15::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_15 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi15Spec {
    const RESET_VALUE: u32 = 0;
}
