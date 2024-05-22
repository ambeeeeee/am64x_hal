#[doc = "Register `CFG0_PRG_PP_0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PrgPp0CtrlProxySpec>;
#[doc = "Register `CFG0_PRG_PP_0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PrgPp0CtrlProxySpec>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_UV_EN_PROXY` reader - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuUvEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_UV_EN_PROXY` writer - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuUvEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_OV_EN_PROXY` reader - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuOvEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_OV_EN_PROXY` writer - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuOvEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_UV_EN_PROXY` reader - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuUvEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_UV_EN_PROXY` writer - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuUvEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_OV_EN_PROXY` reader - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuOvEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_OV_EN_PROXY` writer - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuOvEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_PMIC_IN_UV_EN_PROXY` reader - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaPmicInUvEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_PMIC_IN_UV_EN_PROXY` writer - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaPmicInUvEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_EN_SEL_PROXY` reader - 15:15\\]
Select POK active source"]
pub type PrgPp0CtrlPokEnSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_EN_SEL_PROXY` writer - 15:15\\]
Select POK active source"]
pub type PrgPp0CtrlPokEnSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_DEGLITCH_SEL_PROXY` reader - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
pub type PrgPp0CtrlDeglitchSelProxyR = crate::FieldReader;
#[doc = "Field `PRG_PP_0_CTRL_DEGLITCH_SEL_PROXY` writer - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
pub type PrgPp0CtrlDeglitchSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_uv_en_proxy(&self) -> PrgPp0CtrlPokVddaMcuUvEnProxyR {
        PrgPp0CtrlPokVddaMcuUvEnProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_ov_en_proxy(&self) -> PrgPp0CtrlPokVddaMcuOvEnProxyR {
        PrgPp0CtrlPokVddaMcuOvEnProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_uv_en_proxy(&self) -> PrgPp0CtrlPokVddMcuUvEnProxyR {
        PrgPp0CtrlPokVddMcuUvEnProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_ov_en_proxy(&self) -> PrgPp0CtrlPokVddMcuOvEnProxyR {
        PrgPp0CtrlPokVddMcuOvEnProxyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdda_pmic_in_uv_en_proxy(&self) -> PrgPp0CtrlPokVddaPmicInUvEnProxyR {
        PrgPp0CtrlPokVddaPmicInUvEnProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_en_sel_proxy(&self) -> PrgPp0CtrlPokEnSelProxyR {
        PrgPp0CtrlPokEnSelProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_deglitch_sel_proxy(&self) -> PrgPp0CtrlDeglitchSelProxyR {
        PrgPp0CtrlDeglitchSelProxyR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_uv_en_proxy(
        &mut self,
    ) -> PrgPp0CtrlPokVddaMcuUvEnProxyW<Cfg0PrgPp0CtrlProxySpec> {
        PrgPp0CtrlPokVddaMcuUvEnProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_ov_en_proxy(
        &mut self,
    ) -> PrgPp0CtrlPokVddaMcuOvEnProxyW<Cfg0PrgPp0CtrlProxySpec> {
        PrgPp0CtrlPokVddaMcuOvEnProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_uv_en_proxy(
        &mut self,
    ) -> PrgPp0CtrlPokVddMcuUvEnProxyW<Cfg0PrgPp0CtrlProxySpec> {
        PrgPp0CtrlPokVddMcuUvEnProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_ov_en_proxy(
        &mut self,
    ) -> PrgPp0CtrlPokVddMcuOvEnProxyW<Cfg0PrgPp0CtrlProxySpec> {
        PrgPp0CtrlPokVddMcuOvEnProxyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdda_pmic_in_uv_en_proxy(
        &mut self,
    ) -> PrgPp0CtrlPokVddaPmicInUvEnProxyW<Cfg0PrgPp0CtrlProxySpec> {
        PrgPp0CtrlPokVddaPmicInUvEnProxyW::new(self, 4)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_en_sel_proxy(
        &mut self,
    ) -> PrgPp0CtrlPokEnSelProxyW<Cfg0PrgPp0CtrlProxySpec> {
        PrgPp0CtrlPokEnSelProxyW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_deglitch_sel_proxy(
        &mut self,
    ) -> PrgPp0CtrlDeglitchSelProxyW<Cfg0PrgPp0CtrlProxySpec> {
        PrgPp0CtrlDeglitchSelProxyW::new(self, 16)
    }
}
#[doc = "CFG0_PRG_PP_0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PrgPp0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0PrgPp0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_prg_pp_0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PrgPp0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_prg_pp_0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PrgPp0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PRG_PP_0_CTRL_PROXY to value 0x0002_001f"]
impl crate::Resettable for Cfg0PrgPp0CtrlProxySpec {
    const RESET_VALUE: u32 = 0x0002_001f;
}
