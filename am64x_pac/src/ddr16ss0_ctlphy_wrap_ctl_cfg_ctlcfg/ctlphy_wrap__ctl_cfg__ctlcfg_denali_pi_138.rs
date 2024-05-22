#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_138` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_138` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec>;
#[doc = "Field `PI_DLL_RST` reader - 0:0\\]
Enables use of the DLL reset \\[dll_rst_n\\]."]
pub type PiDllRstR = crate::BitReader;
#[doc = "Field `PI_DLL_RST` writer - 0:0\\]
Enables use of the DLL reset \\[dll_rst_n\\]."]
pub type PiDllRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DRAM_INIT_EN` reader - 8:8\\]
Control for the initialization of DRAM by the PI."]
pub type PiDramInitEnR = crate::BitReader;
#[doc = "Field `PI_DRAM_INIT_EN` writer - 8:8\\]
Control for the initialization of DRAM by the PI."]
pub type PiDramInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DLL_RST_DELAY` reader - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held."]
pub type PiDllRstDelayR = crate::FieldReader<u16>;
#[doc = "Field `PI_DLL_RST_DELAY` writer - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held."]
pub type PiDllRstDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables use of the DLL reset \\[dll_rst_n\\]."]
    #[inline(always)]
    pub fn pi_dll_rst(&self) -> PiDllRstR {
        PiDllRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control for the initialization of DRAM by the PI."]
    #[inline(always)]
    pub fn pi_dram_init_en(&self) -> PiDramInitEnR {
        PiDramInitEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held."]
    #[inline(always)]
    pub fn pi_dll_rst_delay(&self) -> PiDllRstDelayR {
        PiDllRstDelayR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables use of the DLL reset \\[dll_rst_n\\]."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dll_rst(&mut self) -> PiDllRstW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec> {
        PiDllRstW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Control for the initialization of DRAM by the PI."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dram_init_en(&mut self) -> PiDramInitEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec> {
        PiDramInitEnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Minimum cycles required for DLL reset signal dll_rst_n to be held."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dll_rst_delay(&mut self) -> PiDllRstDelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec> {
        PiDllRstDelayW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_138\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_138::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_138::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_138::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_138::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_138 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi138Spec {
    const RESET_VALUE: u32 = 0;
}
