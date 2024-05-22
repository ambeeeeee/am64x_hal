#[doc = "Register `CFG0_RST_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0RstCtrlProxySpec>;
#[doc = "Register `CFG0_RST_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0RstCtrlProxySpec>;
#[doc = "Field `RST_CTRL_SW_MAIN_WARMRST_PROXY` reader - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainWarmrstProxyR = crate::FieldReader;
#[doc = "Field `RST_CTRL_SW_MAIN_WARMRST_PROXY` writer - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainWarmrstProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RST_CTRL_SW_MAIN_POR_PROXY` reader - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainPorProxyR = crate::FieldReader;
#[doc = "Field `RST_CTRL_SW_MAIN_POR_PROXY` writer - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainPorProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RST_CTRL_DMSC_COLD_RESET_EN_Z_PROXY` reader - 16:16\\]
Deactivate Reset of Main by DMSC"]
pub type RstCtrlDmscColdResetEnZProxyR = crate::BitReader;
#[doc = "Field `RST_CTRL_DMSC_COLD_RESET_EN_Z_PROXY` writer - 16:16\\]
Deactivate Reset of Main by DMSC"]
pub type RstCtrlDmscColdResetEnZProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CTRL_MAIN_ESM_ERROR_RST_EN_Z_PROXY` reader - 17:17\\]
Deactivate Reset of Main by ESM"]
pub type RstCtrlMainEsmErrorRstEnZProxyR = crate::BitReader;
#[doc = "Field `RST_CTRL_MAIN_ESM_ERROR_RST_EN_Z_PROXY` writer - 17:17\\]
Deactivate Reset of Main by ESM"]
pub type RstCtrlMainEsmErrorRstEnZProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CTRL_MAIN_RESET_ISO_DONE_Z_PROXY` reader - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
pub type RstCtrlMainResetIsoDoneZProxyR = crate::BitReader;
#[doc = "Field `RST_CTRL_MAIN_RESET_ISO_DONE_Z_PROXY` writer - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
pub type RstCtrlMainResetIsoDoneZProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    pub fn rst_ctrl_sw_main_warmrst_proxy(&self) -> RstCtrlSwMainWarmrstProxyR {
        RstCtrlSwMainWarmrstProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    pub fn rst_ctrl_sw_main_por_proxy(&self) -> RstCtrlSwMainPorProxyR {
        RstCtrlSwMainPorProxyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Deactivate Reset of Main by DMSC"]
    #[inline(always)]
    pub fn rst_ctrl_dmsc_cold_reset_en_z_proxy(&self) -> RstCtrlDmscColdResetEnZProxyR {
        RstCtrlDmscColdResetEnZProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Deactivate Reset of Main by ESM"]
    #[inline(always)]
    pub fn rst_ctrl_main_esm_error_rst_en_z_proxy(&self) -> RstCtrlMainEsmErrorRstEnZProxyR {
        RstCtrlMainEsmErrorRstEnZProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
    #[inline(always)]
    pub fn rst_ctrl_main_reset_iso_done_z_proxy(&self) -> RstCtrlMainResetIsoDoneZProxyR {
        RstCtrlMainResetIsoDoneZProxyR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_sw_main_warmrst_proxy(
        &mut self,
    ) -> RstCtrlSwMainWarmrstProxyW<Cfg0RstCtrlProxySpec> {
        RstCtrlSwMainWarmrstProxyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_sw_main_por_proxy(&mut self) -> RstCtrlSwMainPorProxyW<Cfg0RstCtrlProxySpec> {
        RstCtrlSwMainPorProxyW::new(self, 4)
    }
    #[doc = "Bit 16 - 16:16\\]
Deactivate Reset of Main by DMSC"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_dmsc_cold_reset_en_z_proxy(
        &mut self,
    ) -> RstCtrlDmscColdResetEnZProxyW<Cfg0RstCtrlProxySpec> {
        RstCtrlDmscColdResetEnZProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Deactivate Reset of Main by ESM"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_main_esm_error_rst_en_z_proxy(
        &mut self,
    ) -> RstCtrlMainEsmErrorRstEnZProxyW<Cfg0RstCtrlProxySpec> {
        RstCtrlMainEsmErrorRstEnZProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_main_reset_iso_done_z_proxy(
        &mut self,
    ) -> RstCtrlMainResetIsoDoneZProxyW<Cfg0RstCtrlProxySpec> {
        RstCtrlMainResetIsoDoneZProxyW::new(self, 18)
    }
}
#[doc = "CFG0_RST_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstCtrlProxySpec;
impl crate::RegisterSpec for Cfg0RstCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0RstCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0RstCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_CTRL_PROXY to value 0x0002_0155"]
impl crate::Resettable for Cfg0RstCtrlProxySpec {
    const RESET_VALUE: u32 = 0x0002_0155;
}
