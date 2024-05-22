#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_19` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_19` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec>;
#[doc = "Field `PI_SW_LEVELING_MODE` reader - 2:0\\]
Defines the leveling operation for software leveling. Set to 'b111 for DDR4 VREF training, set to b001 for write leveling, set to b010 for read data eye training, or set to b011 for read gate training, set to b100 for ca training, set to b101 for wdq training."]
pub type PiSwLevelingModeR = crate::FieldReader;
#[doc = "Field `PI_SW_LEVELING_MODE` writer - 2:0\\]
Defines the leveling operation for software leveling. Set to 'b111 for DDR4 VREF training, set to b001 for write leveling, set to b010 for read data eye training, or set to b011 for read gate training, set to b100 for ca training, set to b101 for wdq training."]
pub type PiSwLevelingModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_SWLVL_START` reader - 8:8\\]
User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger. WRITE-ONLY"]
pub type PiSwlvlStartR = crate::BitReader;
#[doc = "Field `PI_SWLVL_START` writer - 8:8\\]
User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger. WRITE-ONLY"]
pub type PiSwlvlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_EXIT` reader - 16:16\\]
User request to exit software leveling. Set to 1 to exit. WRITE-ONLY"]
pub type PiSwlvlExitR = crate::BitReader;
#[doc = "Field `PI_SWLVL_EXIT` writer - 16:16\\]
User request to exit software leveling. Set to 1 to exit. WRITE-ONLY"]
pub type PiSwlvlExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_WR_SLICE_0` reader - 24:24\\]
SW leveling write command in WDQ training. WRITE-ONLY"]
pub type PiSwlvlWrSlice0R = crate::BitReader;
#[doc = "Field `PI_SWLVL_WR_SLICE_0` writer - 24:24\\]
SW leveling write command in WDQ training. WRITE-ONLY"]
pub type PiSwlvlWrSlice0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the leveling operation for software leveling. Set to 'b111 for DDR4 VREF training, set to b001 for write leveling, set to b010 for read data eye training, or set to b011 for read gate training, set to b100 for ca training, set to b101 for wdq training."]
    #[inline(always)]
    pub fn pi_sw_leveling_mode(&self) -> PiSwLevelingModeR {
        PiSwLevelingModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_start(&self) -> PiSwlvlStartR {
        PiSwlvlStartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
User request to exit software leveling. Set to 1 to exit. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_exit(&self) -> PiSwlvlExitR {
        PiSwlvlExitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
SW leveling write command in WDQ training. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_swlvl_wr_slice_0(&self) -> PiSwlvlWrSlice0R {
        PiSwlvlWrSlice0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the leveling operation for software leveling. Set to 'b111 for DDR4 VREF training, set to b001 for write leveling, set to b010 for read data eye training, or set to b011 for read gate training, set to b100 for ca training, set to b101 for wdq training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_leveling_mode(
        &mut self,
    ) -> PiSwLevelingModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec> {
        PiSwLevelingModeW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_start(&mut self) -> PiSwlvlStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec> {
        PiSwlvlStartW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
User request to exit software leveling. Set to 1 to exit. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_exit(&mut self) -> PiSwlvlExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec> {
        PiSwlvlExitW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
SW leveling write command in WDQ training. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_wr_slice_0(
        &mut self,
    ) -> PiSwlvlWrSlice0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec> {
        PiSwlvlWrSlice0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_19::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_19::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_19 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi19Spec {
    const RESET_VALUE: u32 = 0;
}
