#[doc = "Register `CFG0_PRG_PP_1_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PrgPp1CtrlProxySpec>;
#[doc = "Register `CFG0_PRG_PP_1_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PrgPp1CtrlProxySpec>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_EN_PROXY` reader - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddrCoreEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_EN_PROXY` writer - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddrCoreEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_EN_PROXY` reader - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu1p8EnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_EN_PROXY` writer - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu1p8EnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_EN_PROXY` reader - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu3p3EnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_EN_PROXY` writer - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMcu3p3EnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_EN_PROXY` reader - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_EN_PROXY` writer - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_EN_PROXY` reader - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain1p8EnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_EN_PROXY` writer - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain1p8EnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_EN_PROXY` reader - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain3p3EnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_EN_PROXY` writer - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddshvMain3p3EnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_EN_PROXY` reader - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddsDdrioEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_EN_PROXY` writer - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
pub type PrgPp1CtrlPokVddsDdrioEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_OV_SEL_PROXY` reader - 8:8\\]
POK_VDDR_CORE Mode:"]
pub type PrgPp1CtrlPokVddrCoreOvSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDR_CORE_OV_SEL_PROXY` writer - 8:8\\]
POK_VDDR_CORE Mode:"]
pub type PrgPp1CtrlPokVddrCoreOvSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_OV_SEL_PROXY` reader - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMcu1p8OvSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_1P8_OV_SEL_PROXY` writer - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMcu1p8OvSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_OV_SEL_PROXY` reader - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMcu3p3OvSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MCU_3P3_OV_SEL_PROXY` writer - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMcu3p3OvSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_OV_SEL_PROXY` reader - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralOvSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VMON_CAP_MCU_GENERAL_OV_SEL_PROXY` writer - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
pub type PrgPp1CtrlPokVmonCapMcuGeneralOvSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_OV_SEL_PROXY` reader - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMain1p8OvSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_1P8_OV_SEL_PROXY` writer - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
pub type PrgPp1CtrlPokVddshvMain1p8OvSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_OV_SEL_PROXY` reader - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMain3p3OvSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDSHV_MAIN_3P3_OV_SEL_PROXY` writer - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
pub type PrgPp1CtrlPokVddshvMain3p3OvSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_OV_SEL_PROXY` reader - 14:14\\]
POK_VDDS_DDRIO mode:"]
pub type PrgPp1CtrlPokVddsDdrioOvSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_VDDS_DDRIO_OV_SEL_PROXY` writer - 14:14\\]
POK_VDDS_DDRIO mode:"]
pub type PrgPp1CtrlPokVddsDdrioOvSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_POK_EN_SEL_PROXY` reader - 15:15\\]
Select POK active source"]
pub type PrgPp1CtrlPokEnSelProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_EN_SEL_PROXY` writer - 15:15\\]
Select POK active source"]
pub type PrgPp1CtrlPokEnSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_PP_1_CTRL_DEGLITCH_SEL_PROXY` reader - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
pub type PrgPp1CtrlDeglitchSelProxyR = crate::FieldReader;
#[doc = "Field `PRG_PP_1_CTRL_DEGLITCH_SEL_PROXY` writer - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
pub type PrgPp1CtrlDeglitchSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRG_PP_1_CTRL_POK_PP_EN_PROXY` reader - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
pub type PrgPp1CtrlPokPpEnProxyR = crate::BitReader;
#[doc = "Field `PRG_PP_1_CTRL_POK_PP_EN_PROXY` writer - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
pub type PrgPp1CtrlPokPpEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddr_core_en_proxy(&self) -> PrgPp1CtrlPokVddrCoreEnProxyR {
        PrgPp1CtrlPokVddrCoreEnProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_en_proxy(&self) -> PrgPp1CtrlPokVddshvMcu1p8EnProxyR {
        PrgPp1CtrlPokVddshvMcu1p8EnProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_en_proxy(&self) -> PrgPp1CtrlPokVddshvMcu3p3EnProxyR {
        PrgPp1CtrlPokVddshvMcu3p3EnProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_en_proxy(
        &self,
    ) -> PrgPp1CtrlPokVmonCapMcuGeneralEnProxyR {
        PrgPp1CtrlPokVmonCapMcuGeneralEnProxyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_en_proxy(&self) -> PrgPp1CtrlPokVddshvMain1p8EnProxyR {
        PrgPp1CtrlPokVddshvMain1p8EnProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_en_proxy(&self) -> PrgPp1CtrlPokVddshvMain3p3EnProxyR {
        PrgPp1CtrlPokVddshvMain3p3EnProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_en_proxy(&self) -> PrgPp1CtrlPokVddsDdrioEnProxyR {
        PrgPp1CtrlPokVddsDdrioEnProxyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
POK_VDDR_CORE Mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddr_core_ov_sel_proxy(&self) -> PrgPp1CtrlPokVddrCoreOvSelProxyR {
        PrgPp1CtrlPokVddrCoreOvSelProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_ov_sel_proxy(
        &self,
    ) -> PrgPp1CtrlPokVddshvMcu1p8OvSelProxyR {
        PrgPp1CtrlPokVddshvMcu1p8OvSelProxyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_ov_sel_proxy(
        &self,
    ) -> PrgPp1CtrlPokVddshvMcu3p3OvSelProxyR {
        PrgPp1CtrlPokVddshvMcu3p3OvSelProxyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_ov_sel_proxy(
        &self,
    ) -> PrgPp1CtrlPokVmonCapMcuGeneralOvSelProxyR {
        PrgPp1CtrlPokVmonCapMcuGeneralOvSelProxyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_ov_sel_proxy(
        &self,
    ) -> PrgPp1CtrlPokVddshvMain1p8OvSelProxyR {
        PrgPp1CtrlPokVddshvMain1p8OvSelProxyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_ov_sel_proxy(
        &self,
    ) -> PrgPp1CtrlPokVddshvMain3p3OvSelProxyR {
        PrgPp1CtrlPokVddshvMain3p3OvSelProxyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
POK_VDDS_DDRIO mode:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_ov_sel_proxy(&self) -> PrgPp1CtrlPokVddsDdrioOvSelProxyR {
        PrgPp1CtrlPokVddsDdrioOvSelProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_en_sel_proxy(&self) -> PrgPp1CtrlPokEnSelProxyR {
        PrgPp1CtrlPokEnSelProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_deglitch_sel_proxy(&self) -> PrgPp1CtrlDeglitchSelProxyR {
        PrgPp1CtrlDeglitchSelProxyR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
    #[inline(always)]
    pub fn prg_pp_1_ctrl_pok_pp_en_proxy(&self) -> PrgPp1CtrlPokPpEnProxyR {
        PrgPp1CtrlPokPpEnProxyR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Active POK_VDDR_CORE (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddr_core_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddrCoreEnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddrCoreEnProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Active POK_VDDSHV_MCU_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu1p8EnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMcu1p8EnProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Active POK_VDDSHV_MCU_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu3p3EnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMcu3p3EnProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Active POK_VMON_CAP_MCU_GENERAL (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVmonCapMcuGeneralEnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVmonCapMcuGeneralEnProxyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Active POK_VDDSHV_MAIN_1P8 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain1p8EnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMain1p8EnProxyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Active POK_VDDSHV_MAIN_3P3 (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain3p3EnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMain3p3EnProxyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Active POK_VDDS_DDRIO (if pok_en_sel = 1):"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddsDdrioEnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddsDdrioEnProxyW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
POK_VDDR_CORE Mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddr_core_ov_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddrCoreOvSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddrCoreOvSelProxyW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
POK_VDDSHV_MCU_1P8 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_1p8_ov_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu1p8OvSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMcu1p8OvSelProxyW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
POK_VDDSHV_MCU_3P3 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_mcu_3p3_ov_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMcu3p3OvSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMcu3p3OvSelProxyW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
POK_VMON_CAP_MCU_GENERAL mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vmon_cap_mcu_general_ov_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVmonCapMcuGeneralOvSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVmonCapMcuGeneralOvSelProxyW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
POK_VDDSHV_MAIN_1P8 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_1p8_ov_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain1p8OvSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMain1p8OvSelProxyW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
POK_VDDSHV_MAIN_3P3 mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vddshv_main_3p3_ov_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddshvMain3p3OvSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddshvMain3p3OvSelProxyW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
POK_VDDS_DDRIO mode:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_vdds_ddrio_ov_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokVddsDdrioOvSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokVddsDdrioOvSelProxyW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Select POK active source"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_en_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokEnSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokEnSelProxyW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Deglitch period for PRG_PP1 POKs:"]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_deglitch_sel_proxy(
        &mut self,
    ) -> PrgPp1CtrlDeglitchSelProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlDeglitchSelProxyW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
POK ping-pong active. When set, activesautomatic switching between undervoltage andovervoltage detection on PRG_PP1 (POK_VDDR_CORE, POK_VDDSHV_MCU_1P8, POK_VDDSHV_MCU_3P3, POK_VMON_CAP_MCU_GENERAL, POK_VDDSHV_MAIN_1P8, POK_VDDSHV_MAIN_3P3, and POK_VDDS_DDRIO) POKs. This bit has no effect if the POK's ov_sel bit = 1."]
    #[inline(always)]
    #[must_use]
    pub fn prg_pp_1_ctrl_pok_pp_en_proxy(
        &mut self,
    ) -> PrgPp1CtrlPokPpEnProxyW<Cfg0PrgPp1CtrlProxySpec> {
        PrgPp1CtrlPokPpEnProxyW::new(self, 19)
    }
}
#[doc = "CFG0_PRG_PP_1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_1_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_1_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PrgPp1CtrlProxySpec;
impl crate::RegisterSpec for Cfg0PrgPp1CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_prg_pp_1_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PrgPp1CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_prg_pp_1_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PrgPp1CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PRG_PP_1_CTRL_PROXY to value 0x0002_007f"]
impl crate::Resettable for Cfg0PrgPp1CtrlProxySpec {
    const RESET_VALUE: u32 = 0x0002_007f;
}
