#[doc = "Register `CFG0_PRG_PP_0_CTRL` reader"]
pub type R = crate::R<Cfg0PrgPp0CtrlSpec>;
#[doc = "Register `CFG0_PRG_PP_0_CTRL` writer"]
pub type W = crate::W<Cfg0PrgPp0CtrlSpec>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_UV_EN` reader - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuUvEnR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_UV_EN` writer - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuUvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_OV_EN` reader - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuOvEnR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_MCU_OV_EN` writer - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddaMcuOvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_UV_EN` reader - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuUvEnR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_UV_EN` writer - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuUvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_OV_EN` reader - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuOvEnR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDD_MCU_OV_EN` writer - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
pub type PrgPp0CtrlPokVddMcuOvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_PMIC_IN_UV_EN` reader - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaPmicInUvEnR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_VDDA_PMIC_IN_UV_EN` writer - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
pub type PrgPp0CtrlPokVddaPmicInUvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_POK_EN_SEL` reader - 15:15\\]
Select POK active source"]
pub type PrgPp0CtrlPokEnSelR = crate::BitReader;
#[doc = "Field `PRG_PP_0_CTRL_POK_EN_SEL` writer - 15:15\\]
Select POK active source"]
pub type PrgPp0CtrlPokEnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_0_CTRL_DEGLITCH_SEL` reader - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
pub type PrgPp0CtrlDeglitchSelR = crate::FieldReader;
#[doc = "Field `PRG_PP_0_CTRL_DEGLITCH_SEL` writer - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
pub type PrgPp0CtrlDeglitchSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_uv_en(&self) -> PrgPp0CtrlPokVddaMcuUvEnR {
        PrgPp0CtrlPokVddaMcuUvEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_ov_en(&self) -> PrgPp0CtrlPokVddaMcuOvEnR {
        PrgPp0CtrlPokVddaMcuOvEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_uv_en(&self) -> PrgPp0CtrlPokVddMcuUvEnR {
        PrgPp0CtrlPokVddMcuUvEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_ov_en(&self) -> PrgPp0CtrlPokVddMcuOvEnR {
        PrgPp0CtrlPokVddMcuOvEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_vdda_pmic_in_uv_en(&self) -> PrgPp0CtrlPokVddaPmicInUvEnR {
        PrgPp0CtrlPokVddaPmicInUvEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_pok_en_sel(&self) -> PrgPp0CtrlPokEnSelR {
        PrgPp0CtrlPokEnSelR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
    #[inline(always)]
    pub fn prg_pp_0_ctrl_deglitch_sel(&self) -> PrgPp0CtrlDeglitchSelR {
        PrgPp0CtrlDeglitchSelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Active 1.8V VDDA_MCU undervoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_uv_en(
        &mut self,
    ) -> PrgPp0CtrlPokVddaMcuUvEnW<Cfg0PrgPp0CtrlSpec> {
        PrgPp0CtrlPokVddaMcuUvEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active 1.8V VDDA_MCU overvoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdda_mcu_ov_en(
        &mut self,
    ) -> PrgPp0CtrlPokVddaMcuOvEnW<Cfg0PrgPp0CtrlSpec> {
        PrgPp0CtrlPokVddaMcuOvEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Active VDD_MCU undervoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_uv_en(
        &mut self,
    ) -> PrgPp0CtrlPokVddMcuUvEnW<Cfg0PrgPp0CtrlSpec> {
        PrgPp0CtrlPokVddMcuUvEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Active VDD_MCU overvoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdd_mcu_ov_en(
        &mut self,
    ) -> PrgPp0CtrlPokVddMcuOvEnW<Cfg0PrgPp0CtrlSpec> {
        PrgPp0CtrlPokVddMcuOvEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Active VDDA_PMIC_IN undervoltage POK detection"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_vdda_pmic_in_uv_en(
        &mut self,
    ) -> PrgPp0CtrlPokVddaPmicInUvEnW<Cfg0PrgPp0CtrlSpec> {
        PrgPp0CtrlPokVddaPmicInUvEnW::new(self, 4)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_pok_en_sel(&mut self) -> PrgPp0CtrlPokEnSelW<Cfg0PrgPp0CtrlSpec> {
        PrgPp0CtrlPokEnSelW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP0 POKs:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_0_ctrl_deglitch_sel(&mut self) -> PrgPp0CtrlDeglitchSelW<Cfg0PrgPp0CtrlSpec> {
        PrgPp0CtrlDeglitchSelW::new(self, 16)
    }
}
#[doc = "CFG0_PRG_PP_0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PrgPp0CtrlSpec;
impl crate::RegisterSpec for Cfg0PrgPp0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_prg_pp_0_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0PrgPp0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_prg_pp_0_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0PrgPp0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PRG_PP_0_CTRL to value 0x0002_001f"]
impl crate::Resettable for Cfg0PrgPp0CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_001f;
}
