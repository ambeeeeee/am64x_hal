#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_26` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_26` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec>;
#[doc = "Field `PI_WRLVL_RESP_MASK` reader - 1:0\\]
Mask for the dfi_wrlvl_resp signal during write leveling."]
pub type PiWrlvlRespMaskR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_RESP_MASK` writer - 1:0\\]
Mask for the dfi_wrlvl_resp signal during write leveling."]
pub type PiWrlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WRLVL_ROTATE` reader - 8:8\\]
Enables rotational CS for counter triggered automatic write leveling. Set to 1, only one rank's write levling will process, the rank number is rotational for each time that write leveling been triggered by counter expiring. Set to 0 or not a short pattern leveling \\[indicated by dfi_lvl_periodic\\], the counter expired write leveling will process all the ranks."]
pub type PiWrlvlRotateR = crate::BitReader;
#[doc = "Field `PI_WRLVL_ROTATE` writer - 8:8\\]
Enables rotational CS for counter triggered automatic write leveling. Set to 1, only one rank's write levling will process, the rank number is rotational for each time that write leveling been triggered by counter expiring. Set to 0 or not a short pattern leveling \\[indicated by dfi_lvl_periodic\\], the counter expired write leveling will process all the ranks."]
pub type PiWrlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WRLVL_CS_MAP` reader - 17:16\\]
Defines the chip select map for write leveling operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
pub type PiWrlvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_CS_MAP` writer - 17:16\\]
Defines the chip select map for write leveling operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
pub type PiWrlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WRLVL_ON_MPD_EXIT` reader - 24:24\\]
Enables automatic write leveling on a maximum power down mode exit. Set to 1 to enable."]
pub type PiWrlvlOnMpdExitR = crate::BitReader;
#[doc = "Field `PI_WRLVL_ON_MPD_EXIT` writer - 24:24\\]
Enables automatic write leveling on a maximum power down mode exit. Set to 1 to enable."]
pub type PiWrlvlOnMpdExitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Mask for the dfi_wrlvl_resp signal during write leveling."]
    #[inline(always)]
    pub fn pi_wrlvl_resp_mask(&self) -> PiWrlvlRespMaskR {
        PiWrlvlRespMaskR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables rotational CS for counter triggered automatic write leveling. Set to 1, only one rank's write levling will process, the rank number is rotational for each time that write leveling been triggered by counter expiring. Set to 0 or not a short pattern leveling \\[indicated by dfi_lvl_periodic\\], the counter expired write leveling will process all the ranks."]
    #[inline(always)]
    pub fn pi_wrlvl_rotate(&self) -> PiWrlvlRotateR {
        PiWrlvlRotateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the chip select map for write leveling operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
    #[inline(always)]
    pub fn pi_wrlvl_cs_map(&self) -> PiWrlvlCsMapR {
        PiWrlvlCsMapR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables automatic write leveling on a maximum power down mode exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_on_mpd_exit(&self) -> PiWrlvlOnMpdExitR {
        PiWrlvlOnMpdExitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Mask for the dfi_wrlvl_resp signal during write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_resp_mask(
        &mut self,
    ) -> PiWrlvlRespMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec> {
        PiWrlvlRespMaskW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables rotational CS for counter triggered automatic write leveling. Set to 1, only one rank's write levling will process, the rank number is rotational for each time that write leveling been triggered by counter expiring. Set to 0 or not a short pattern leveling \\[indicated by dfi_lvl_periodic\\], the counter expired write leveling will process all the ranks."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_rotate(&mut self) -> PiWrlvlRotateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec> {
        PiWrlvlRotateW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the chip select map for write leveling operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_cs_map(&mut self) -> PiWrlvlCsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec> {
        PiWrlvlCsMapW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables automatic write leveling on a maximum power down mode exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_on_mpd_exit(
        &mut self,
    ) -> PiWrlvlOnMpdExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec> {
        PiWrlvlOnMpdExitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_26::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_26::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_26 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi26Spec {
    const RESET_VALUE: u32 = 0;
}
