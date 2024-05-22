#[doc = "Register `CFG0_PRG_PP_1_CTRL` reader"]
pub type R = crate::R<Cfg0PrgPp1CtrlSpec>;
#[doc = "Register `CFG0_PRG_PP_1_CTRL` writer"]
pub type W = crate::W<Cfg0PrgPp1CtrlSpec>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_EN` reader - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddrCoreEnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_EN` writer - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddrCoreEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_EN` reader - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu1p8EnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_EN` writer - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu1p8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_EN` reader - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu3p3EnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_EN` writer - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu3p3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_EN` reader - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralEnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_EN` writer - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_EN` reader - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain1p8EnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_EN` writer - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain1p8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_EN` reader - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain3p3EnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_EN` writer - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain3p3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_EN` reader - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddsDdrioEnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_EN` writer - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddsDdrioEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_OV_SEL` reader - 8:8\\]
POK_VDDR_CORE Mode:"]
pub type PrgPp1CtrlPokVddrCoreOvSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_OV_SEL` writer - 8:8\\]
POK_VDDR_CORE Mode:"]
pub type PrgPp1CtrlPokVddrCoreOvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_OV_SEL` reader - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMcu1p8OvSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_OV_SEL` writer - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMcu1p8OvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_OV_SEL` reader - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMcu3p3OvSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_OV_SEL` writer - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMcu3p3OvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_OV_SEL` reader - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralOvSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_OV_SEL` writer - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralOvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_OV_SEL` reader - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMain1p8OvSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_OV_SEL` writer - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMain1p8OvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_OV_SEL` reader - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMain3p3OvSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_OV_SEL` writer - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMain3p3OvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_OV_SEL` reader - 14:14\\]
POK_VDDS_DDRIO mode:"]
pub type PrgPp1CtrlPokVddsDdrioOvSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_OV_SEL` writer - 14:14\\]
POK_VDDS_DDRIO mode:"]
pub type PrgPp1CtrlPokVddsDdrioOvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_EN_SEL` reader - 15:15\\]
Select POK active source"]
pub type PrgPp1CtrlPokEnSelR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_EN_SEL` writer - 15:15\\]
Select POK active source"]
pub type PrgPp1CtrlPokEnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_DEGLITCH_SEL` reader - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
pub type PrgPp1CtrlDeglitchSelR = crate::FieldReader;
#[doc = "Field `PRG_PP_1_CTRL_DEGLITCH_SEL` writer - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
pub type PrgPp1CtrlDeglitchSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRG_PP_1_CTRL_POK_PP_EN` reader - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
pub type PrgPp1CtrlPokPpEnR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_PP_EN` writer - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
pub type PrgPp1CtrlPokPpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddr_core_en(&self) -> PrgPp1CtrlPokVddrCoreEnR {
        PrgPp1CtrlPokVddrCoreEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_en(&self) -> PrgPp1CtrlPokVddshvMcu1p8EnR {
        PrgPp1CtrlPokVddshvMcu1p8EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_en(&self) -> PrgPp1CtrlPokVddshvMcu3p3EnR {
        PrgPp1CtrlPokVddshvMcu3p3EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_en(&self) -> PrgPp1CtrlPokVmonCapMcuGeneralEnR {
        PrgPp1CtrlPokVmonCapMcuGeneralEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_en(&self) -> PrgPp1CtrlPokVddshvMain1p8EnR {
        PrgPp1CtrlPokVddshvMain1p8EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_en(&self) -> PrgPp1CtrlPokVddshvMain3p3EnR {
        PrgPp1CtrlPokVddshvMain3p3EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_en(&self) -> PrgPp1CtrlPokVddsDdrioEnR {
        PrgPp1CtrlPokVddsDdrioEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
POK_VDDR_CORE Mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddr_core_ov_sel(&self) -> PrgPp1CtrlPokVddrCoreOvSelR {
        PrgPp1CtrlPokVddrCoreOvSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_ov_sel(&self) -> PrgPp1CtrlPokVddshvMcu1p8OvSelR {
        PrgPp1CtrlPokVddshvMcu1p8OvSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_ov_sel(&self) -> PrgPp1CtrlPokVddshvMcu3p3OvSelR {
        PrgPp1CtrlPokVddshvMcu3p3OvSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_ov_sel(
        &self,
    ) -> PrgPp1CtrlPokVmonCapMcuGeneralOvSelR {
        PrgPp1CtrlPokVmonCapMcuGeneralOvSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_ov_sel(&self) -> PrgPp1CtrlPokVddshvMain1p8OvSelR {
        PrgPp1CtrlPokVddshvMain1p8OvSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_ov_sel(&self) -> PrgPp1CtrlPokVddshvMain3p3OvSelR {
        PrgPp1CtrlPokVddshvMain3p3OvSelR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
POK_VDDS_DDRIO mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_ov_sel(&self) -> PrgPp1CtrlPokVddsDdrioOvSelR {
        PrgPp1CtrlPokVddsDdrioOvSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_en_sel(&self) -> PrgPp1CtrlPokEnSelR {
        PrgPp1CtrlPokEnSelR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_deglitch_sel(&self) -> PrgPp1CtrlDeglitchSelR {
        PrgPp1CtrlDeglitchSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_pp_en(&self) -> PrgPp1CtrlPokPpEnR {
        PrgPp1CtrlPokPpEnR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddr_core_en(
        &mut self,
    ) -> PrgPp1CtrlPokVddrCoreEnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddrCoreEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_en(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu1p8EnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMcu1p8EnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_en(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu3p3EnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMcu3p3EnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_en(
        &mut self,
    ) -> PrgPp1CtrlPokVmonCapMcuGeneralEnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVmonCapMcuGeneralEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_en(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain1p8EnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMain1p8EnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_en(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain3p3EnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMain3p3EnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_en(
        &mut self,
    ) -> PrgPp1CtrlPokVddsDdrioEnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddsDdrioEnW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
POK_VDDR_CORE Mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddr_core_ov_sel(
        &mut self,
    ) -> PrgPp1CtrlPokVddrCoreOvSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddrCoreOvSelW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_ov_sel(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu1p8OvSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMcu1p8OvSelW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_ov_sel(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu3p3OvSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMcu3p3OvSelW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_ov_sel(
        &mut self,
    ) -> PrgPp1CtrlPokVmonCapMcuGeneralOvSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVmonCapMcuGeneralOvSelW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_ov_sel(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain1p8OvSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMain1p8OvSelW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_ov_sel(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain3p3OvSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddshvMain3p3OvSelW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
POK_VDDS_DDRIO mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_ov_sel(
        &mut self,
    ) -> PrgPp1CtrlPokVddsDdrioOvSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokVddsDdrioOvSelW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_en_sel(&mut self) -> PrgPp1CtrlPokEnSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokEnSelW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_deglitch_sel(&mut self) -> PrgPp1CtrlDeglitchSelW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlDeglitchSelW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_pp_en(&mut self) -> PrgPp1CtrlPokPpEnW<Cfg0PrgPp1CtrlSpec> {
        PrgPp1CtrlPokPpEnW::new(self, 19)
    }
}
#[doc = "CFG0_PRG_PP_1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PrgPp1CtrlSpec;
impl crate::RegisterSpec for Cfg0PrgPp1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_prg_pp_1_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0PrgPp1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_prg_pp_1_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0PrgPp1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PRG_PP_1_CTRL to value 0x0002_007f"]
impl crate::Resettable for Cfg0PrgPp1CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_007f;
}
