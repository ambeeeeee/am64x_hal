#[doc = "Register `CFG0_RST_CTRL` reader"]
pub type R = crate::R<Cfg0RstCtrlSpec>;
#[doc = "Register `CFG0_RST_CTRL` writer"]
pub type W = crate::W<Cfg0RstCtrlSpec>;
#[doc = "Field `RST_CTRL_SW_MAIN_WARMRST` reader - 3:0\\]
This is a fault tolerant bitfield."]
pub type RstCtrlSwMainWarmrstR = crate::FieldReader;
#[doc = "Field `RST_CTRL_SW_MAIN_WARMRST` writer - 3:0\\]
This is a fault tolerant bitfield."]
pub type RstCtrlSwMainWarmrstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RST_CTRL_SW_MAIN_POR` reader - 7:4\\]
This is a fault tolerant bitfield."]
pub type RstCtrlSwMainPorR = crate::FieldReader;
#[doc = "Field `RST_CTRL_SW_MAIN_POR` writer - 7:4\\]
This is a fault tolerant bitfield."]
pub type RstCtrlSwMainPorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RST_CTRL_SW_MCU_WARMRST` reader - 11:8\\]
This is a fault tolerant bitfield."]
pub type RstCtrlSwMcuWarmrstR = crate::FieldReader;
#[doc = "Field `RST_CTRL_SW_MCU_WARMRST` writer - 11:8\\]
This is a fault tolerant bitfield."]
pub type RstCtrlSwMcuWarmrstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RST_CTRL_DMSC_COLD_RESET_EN_Z` reader - 16:16\\]
Deactivate Reset of MCU by DMSC"]
pub type RstCtrlDmscColdResetEnZR = crate::BitReader;
#[doc = "Field `RST_CTRL_DMSC_COLD_RESET_EN_Z` writer - 16:16\\]
Deactivate Reset of MCU by DMSC"]
pub type RstCtrlDmscColdResetEnZW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CTRL_MCU_ESM_ERROR_RST_EN_Z` reader - 17:17\\]
Deactivate Reset of MCU by ESM"]
pub type RstCtrlMcuEsmErrorRstEnZR = crate::BitReader;
#[doc = "Field `RST_CTRL_MCU_ESM_ERROR_RST_EN_Z` writer - 17:17\\]
Deactivate Reset of MCU by ESM"]
pub type RstCtrlMcuEsmErrorRstEnZW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CTRL_MCU_RESET_ISO_DONE_Z` reader - 18:18\\]
MCU can set this bit to block warm reset in the main domain which is useful when the MCU may be accessing"]
pub type RstCtrlMcuResetIsoDoneZR = crate::BitReader;
#[doc = "Field `RST_CTRL_MCU_RESET_ISO_DONE_Z` writer - 18:18\\]
MCU can set this bit to block warm reset in the main domain which is useful when the MCU may be accessing"]
pub type RstCtrlMcuResetIsoDoneZW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This is a fault tolerant bitfield."]
    #[inline(always)]
    pub fn rst_ctrl_sw_main_warmrst(&self) -> RstCtrlSwMainWarmrstR {
        RstCtrlSwMainWarmrstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This is a fault tolerant bitfield."]
    #[inline(always)]
    pub fn rst_ctrl_sw_main_por(&self) -> RstCtrlSwMainPorR {
        RstCtrlSwMainPorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This is a fault tolerant bitfield."]
    #[inline(always)]
    pub fn rst_ctrl_sw_mcu_warmrst(&self) -> RstCtrlSwMcuWarmrstR {
        RstCtrlSwMcuWarmrstR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Deactivate Reset of MCU by DMSC"]
    #[inline(always)]
    pub fn rst_ctrl_dmsc_cold_reset_en_z(&self) -> RstCtrlDmscColdResetEnZR {
        RstCtrlDmscColdResetEnZR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Deactivate Reset of MCU by ESM"]
    #[inline(always)]
    pub fn rst_ctrl_mcu_esm_error_rst_en_z(&self) -> RstCtrlMcuEsmErrorRstEnZR {
        RstCtrlMcuEsmErrorRstEnZR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
MCU can set this bit to block warm reset in the main domain which is useful when the MCU may be accessing"]
    #[inline(always)]
    pub fn rst_ctrl_mcu_reset_iso_done_z(&self) -> RstCtrlMcuResetIsoDoneZR {
        RstCtrlMcuResetIsoDoneZR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This is a fault tolerant bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_sw_main_warmrst(&mut self) -> RstCtrlSwMainWarmrstW<Cfg0RstCtrlSpec> {
        RstCtrlSwMainWarmrstW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This is a fault tolerant bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_sw_main_por(&mut self) -> RstCtrlSwMainPorW<Cfg0RstCtrlSpec> {
        RstCtrlSwMainPorW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This is a fault tolerant bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_sw_mcu_warmrst(&mut self) -> RstCtrlSwMcuWarmrstW<Cfg0RstCtrlSpec> {
        RstCtrlSwMcuWarmrstW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Deactivate Reset of MCU by DMSC"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_dmsc_cold_reset_en_z(&mut self) -> RstCtrlDmscColdResetEnZW<Cfg0RstCtrlSpec> {
        RstCtrlDmscColdResetEnZW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Deactivate Reset of MCU by ESM"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_mcu_esm_error_rst_en_z(
        &mut self,
    ) -> RstCtrlMcuEsmErrorRstEnZW<Cfg0RstCtrlSpec> {
        RstCtrlMcuEsmErrorRstEnZW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
MCU can set this bit to block warm reset in the main domain which is useful when the MCU may be accessing"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ctrl_mcu_reset_iso_done_z(&mut self) -> RstCtrlMcuResetIsoDoneZW<Cfg0RstCtrlSpec> {
        RstCtrlMcuResetIsoDoneZW::new(self, 18)
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
#[doc = "`reset()` method sets CFG0_RST_CTRL to value 0x0002_1555"]
impl crate::Resettable for Cfg0RstCtrlSpec {
    const RESET_VALUE: u32 = 0x0002_1555;
}
