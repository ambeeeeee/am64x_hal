#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_91` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_91` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec>;
#[doc = "Field `PI_BIST_FAIL_ADDR_1` reader - 0:0\\]
The burst aligned address of BIST error. READ-ONLY"]
pub type PiBistFailAddr1R = crate::BitReader;
#[doc = "Field `PI_BIST_FAIL_ADDR_1` writer - 0:0\\]
The burst aligned address of BIST error. READ-ONLY"]
pub type PiBistFailAddr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_BSTLEN` reader - 13:8\\]
Encoded burst length sent to DRAMs during initialization."]
pub type PiBstlenR = crate::FieldReader;
#[doc = "Field `PI_BSTLEN` writer - 13:8\\]
Encoded burst length sent to DRAMs during initialization."]
pub type PiBstlenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_LONG_COUNT_MASK` reader - 20:16\\]
Reduces the length of the long counter from 1024 cycles."]
pub type PiLongCountMaskR = crate::FieldReader;
#[doc = "Field `PI_LONG_COUNT_MASK` writer - 20:16\\]
Reduces the length of the long counter from 1024 cycles."]
pub type PiLongCountMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_CMD_SWAP_EN` reader - 24:24\\]
Command pin swap function enable"]
pub type PiCmdSwapEnR = crate::BitReader;
#[doc = "Field `PI_CMD_SWAP_EN` writer - 24:24\\]
Command pin swap function enable"]
pub type PiCmdSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The burst aligned address of BIST error. READ-ONLY"]
    #[inline(always)]
    pub fn pi_bist_fail_addr_1(&self) -> PiBistFailAddr1R {
        PiBistFailAddr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Encoded burst length sent to DRAMs during initialization."]
    #[inline(always)]
    pub fn pi_bstlen(&self) -> PiBstlenR {
        PiBstlenR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Reduces the length of the long counter from 1024 cycles."]
    #[inline(always)]
    pub fn pi_long_count_mask(&self) -> PiLongCountMaskR {
        PiLongCountMaskR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Command pin swap function enable"]
    #[inline(always)]
    pub fn pi_cmd_swap_en(&self) -> PiCmdSwapEnR {
        PiCmdSwapEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The burst aligned address of BIST error. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_fail_addr_1(
        &mut self,
    ) -> PiBistFailAddr1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec> {
        PiBistFailAddr1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Encoded burst length sent to DRAMs during initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bstlen(&mut self) -> PiBstlenW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec> {
        PiBstlenW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Reduces the length of the long counter from 1024 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_long_count_mask(
        &mut self,
    ) -> PiLongCountMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec> {
        PiLongCountMaskW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Command pin swap function enable"]
    #[inline(always)]
    #[must_use]
    pub fn pi_cmd_swap_en(&mut self) -> PiCmdSwapEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec> {
        PiCmdSwapEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_91::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_91::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_91::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_91::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_91 to value 0x0200"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi91Spec {
    const RESET_VALUE: u32 = 0x0200;
}
