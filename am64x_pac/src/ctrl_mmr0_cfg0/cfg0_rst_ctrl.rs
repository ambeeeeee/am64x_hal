#[doc = "Register `CFG0_RST_CTRL` reader"]
pub type R = crate::R<Cfg0RstCtrlSpec>;
#[doc = "Register `CFG0_RST_CTRL` writer"]
pub type W = crate::W<Cfg0RstCtrlSpec>;
#[doc = "Field `RST_CTRL_SW_MAIN_WARMRST` reader - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainWarmrstR = crate::FieldReader;
#[doc = "Field `RST_CTRL_SW_MAIN_WARMRST` writer - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainWarmrstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RST_CTRL_SW_MAIN_POR` reader - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainPorR = crate::FieldReader;
#[doc = "Field `RST_CTRL_SW_MAIN_POR` writer - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
pub type RstCtrlSwMainPorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RST_CTRL_DMSC_COLD_RESET_EN_Z` reader - 16:16\\]
Deactivate Reset of Main by DMSC"]
pub type RstCtrlDmscColdResetEnZR = crate::BitReader;
#[doc = "Field `RST_CTRL_DMSC_COLD_RESET_EN_Z` writer - 16:16\\]
Deactivate Reset of Main by DMSC"]
pub type RstCtrlDmscColdResetEnZW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CTRL_MAIN_ESM_ERROR_RST_EN_Z` reader - 17:17\\]
Deactivate Reset of Main by ESM"]
pub type RstCtrlMainEsmErrorRstEnZR = crate::BitReader;
#[doc = "Field `RST_CTRL_MAIN_ESM_ERROR_RST_EN_Z` writer - 17:17\\]
Deactivate Reset of Main by ESM"]
pub type RstCtrlMainEsmErrorRstEnZW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CTRL_MAIN_RESET_ISO_DONE_Z` reader - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
pub type RstCtrlMainResetIsoDoneZR = crate::BitReader;
#[doc = "Field `RST_CTRL_MAIN_RESET_ISO_DONE_Z` writer - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
pub type RstCtrlMainResetIsoDoneZW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    pub fn rst_ctrl_sw_main_warmrst(&self) -> RstCtrlSwMainWarmrstR {
        RstCtrlSwMainWarmrstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    pub fn rst_ctrl_sw_main_por(&self) -> RstCtrlSwMainPorR {
        RstCtrlSwMainPorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Deactivate Reset of Main by DMSC"]
    #[inline(always)]
    pub fn rst_ctrl_dmsc_cold_reset_en_z(&self) -> RstCtrlDmscColdResetEnZR {
        RstCtrlDmscColdResetEnZR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Deactivate Reset of Main by ESM"]
    #[inline(always)]
    pub fn rst_ctrl_main_esm_error_rst_en_z(&self) -> RstCtrlMainEsmErrorRstEnZR {
        RstCtrlMainEsmErrorRstEnZR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
    #[inline(always)]
    pub fn rst_ctrl_main_reset_iso_done_z(&self) -> RstCtrlMainResetIsoDoneZR {
        RstCtrlMainResetIsoDoneZR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Causes Main Domain Warm Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_sw_main_warmrst(&mut self) -> RstCtrlSwMainWarmrstW<Cfg0RstCtrlSpec> {
        RstCtrlSwMainWarmrstW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Causes Main Domain Power On Reset when set to 4'b0110, Bits will reset to 4'b1111"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_sw_main_por(&mut self) -> RstCtrlSwMainPorW<Cfg0RstCtrlSpec> {
        RstCtrlSwMainPorW::new(self, 4)
    }
    #[doc = "Bit 16 - 16:16\\]
Deactivate Reset of Main by DMSC"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_dmsc_cold_reset_en_z(&mut self) -> RstCtrlDmscColdResetEnZW<Cfg0RstCtrlSpec> {
        RstCtrlDmscColdResetEnZW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Deactivate Reset of Main by ESM"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_main_esm_error_rst_en_z(
        &mut self,
    ) -> RstCtrlMainEsmErrorRstEnZW<Cfg0RstCtrlSpec> {
        RstCtrlMainEsmErrorRstEnZW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Main Domain CPUs can set this bit to block warm reset in the main domain which is useful when the Main domain may be accessing"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_main_reset_iso_done_z(&mut self) -> RstCtrlMainResetIsoDoneZW<Cfg0RstCtrlSpec> {
        RstCtrlMainResetIsoDoneZW::new(self, 18)
    }
}
#[doc = "CFG0_RST_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstCtrlSpec;
impl crate::RegisterSpec for Cfg0RstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0RstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0RstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_CTRL to value 0x0002_0155"]
impl crate::Resettable for Cfg0RstCtrlSpec {
    const RESET_VALUE: u32 = 0x0002_0155;
}
