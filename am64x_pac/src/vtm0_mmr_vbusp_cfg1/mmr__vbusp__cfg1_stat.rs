#[doc = "Register `MMR__VBUSP__CFG1_STAT` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg1StatSpec>;
#[doc = "Register `MMR__VBUSP__CFG1_STAT` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg1StatSpec>;
#[doc = "Field `DATA_OUT` reader - 9:0\\]
Data_out signal value from sensor: Temperature data from the ADC in monitor. Valid after VTM_TMPSENS\\[a\\]_STAT.eoc_fc_update = 1. This value will be latched in this VTM register every time monitor output data_valid transitions from 0 to 1. Reset value is POR only."]
pub type DataOutR = crate::FieldReader<u16>;
#[doc = "Field `DATA_OUT` writer - 9:0\\]
Data_out signal value from sensor: Temperature data from the ADC in monitor. Valid after VTM_TMPSENS\\[a\\]_STAT.eoc_fc_update = 1. This value will be latched in this VTM register every time monitor output data_valid transitions from 0 to 1. Reset value is POR only."]
pub type DataOutW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DATA_VALID` reader - 10:10\\]
Data_valid signal value from sensor: ADC End of Conversion. End of conversion indicated by 0 to 1 transition. When high data_out(9:0) out of the temp-monitor is valid. This field doesn't reflect the instantaneous output from the temp-monitor. This field gets latched/updated in this VTM register when the data_valid value from temp-monitor toggles. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz."]
pub type DataValidR = crate::BitReader;
#[doc = "Field `DATA_VALID` writer - 10:10\\]
Data_valid signal value from sensor: ADC End of Conversion. End of conversion indicated by 0 to 1 transition. When high data_out(9:0) out of the temp-monitor is valid. This field doesn't reflect the instantaneous output from the temp-monitor. This field gets latched/updated in this VTM register when the data_valid value from temp-monitor toggles. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz."]
pub type DataValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC_FC_UPDATE` reader - 11:11\\]
First time end of conversion. This field is reset to 0 every time VTM.por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. This bit will be set to 1 after the first time after reset release that data_valid transitions from 0 to 1, and remain at 1 until next time either of por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz, or continuous mode deassertion."]
pub type EocFcUpdateR = crate::BitReader;
#[doc = "Field `EOC_FC_UPDATE` writer - 11:11\\]
First time end of conversion. This field is reset to 0 every time VTM.por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. This bit will be set to 1 after the first time after reset release that data_valid transitions from 0 to 1, and remain at 1 until next time either of por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz, or continuous mode deassertion."]
pub type EocFcUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GT_TH1_ALERT` reader - 12:12\\]
This field reflects the status of the gt_th1_alert comparator result. The control MMR field gt_th1_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
pub type GtTh1AlertR = crate::BitReader;
#[doc = "Field `GT_TH1_ALERT` writer - 12:12\\]
This field reflects the status of the gt_th1_alert comparator result. The control MMR field gt_th1_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
pub type GtTh1AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GT_TH2_ALERT` reader - 13:13\\]
This field reflects the status of the gt_th2_alert comparator result. The control MMR field gt_th2_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
pub type GtTh2AlertR = crate::BitReader;
#[doc = "Field `GT_TH2_ALERT` writer - 13:13\\]
This field reflects the status of the gt_th2_alert comparator result. The control MMR field gt_th2_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
pub type GtTh2AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LT_TH0_ALERT` reader - 14:14\\]
This field reflects the status of the lt_th0_alert comparator result. The control MMR field lt_th0_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
pub type LtTh0AlertR = crate::BitReader;
#[doc = "Field `LT_TH0_ALERT` writer - 14:14\\]
This field reflects the status of the lt_th0_alert comparator result. The control MMR field lt_th0_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
pub type LtTh0AlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXT_OUTRG_ALERT` reader - 15:15\\]
This bit will be driven to a level 1 for a given temperature monitor if it has its corresponding bit maxt_outrg_en = 1, and the temperature reading is reporting to be outside the max temperature supported, temp > programmed value. The level of this signal is a reflection, with some clock delays, of the temperature code reading. This is NOT an sticky bit. Reset value is POR only."]
pub type MaxtOutrgAlertR = crate::BitReader;
#[doc = "Field `MAXT_OUTRG_ALERT` writer - 15:15\\]
This bit will be driven to a level 1 for a given temperature monitor if it has its corresponding bit maxt_outrg_en = 1, and the temperature reading is reporting to be outside the max temperature supported, temp > programmed value. The level of this signal is a reflection, with some clock delays, of the temperature code reading. This is NOT an sticky bit. Reset value is POR only."]
pub type MaxtOutrgAlertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VD_MAP` reader - 19:16\\]
Indicates the core voltage domain placement of the temp sensor. Device specific field. This field indicates in which core voltage domain, cVD, has been physically placed the temp-monitor. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff at POR, d_VTM_TMPSENS\\[a\\]_STAT_vd_map_ipcfg."]
pub type VdMapR = crate::FieldReader;
#[doc = "Field `VD_MAP` writer - 19:16\\]
Indicates the core voltage domain placement of the temp sensor. Device specific field. This field indicates in which core voltage domain, cVD, has been physically placed the temp-monitor. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff at POR, d_VTM_TMPSENS\\[a\\]_STAT_vd_map_ipcfg."]
pub type VdMapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Data_out signal value from sensor: Temperature data from the ADC in monitor. Valid after VTM_TMPSENS\\[a\\]_STAT.eoc_fc_update = 1. This value will be latched in this VTM register every time monitor output data_valid transitions from 0 to 1. Reset value is POR only."]
    #[inline(always)]
    pub fn data_out(&self) -> DataOutR {
        DataOutR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - 10:10\\]
Data_valid signal value from sensor: ADC End of Conversion. End of conversion indicated by 0 to 1 transition. When high data_out(9:0) out of the temp-monitor is valid. This field doesn't reflect the instantaneous output from the temp-monitor. This field gets latched/updated in this VTM register when the data_valid value from temp-monitor toggles. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz."]
    #[inline(always)]
    pub fn data_valid(&self) -> DataValidR {
        DataValidR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
First time end of conversion. This field is reset to 0 every time VTM.por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. This bit will be set to 1 after the first time after reset release that data_valid transitions from 0 to 1, and remain at 1 until next time either of por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz, or continuous mode deassertion."]
    #[inline(always)]
    pub fn eoc_fc_update(&self) -> EocFcUpdateR {
        EocFcUpdateR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
This field reflects the status of the gt_th1_alert comparator result. The control MMR field gt_th1_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
    #[inline(always)]
    pub fn gt_th1_alert(&self) -> GtTh1AlertR {
        GtTh1AlertR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
This field reflects the status of the gt_th2_alert comparator result. The control MMR field gt_th2_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
    #[inline(always)]
    pub fn gt_th2_alert(&self) -> GtTh2AlertR {
        GtTh2AlertR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
This field reflects the status of the lt_th0_alert comparator result. The control MMR field lt_th0_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
    #[inline(always)]
    pub fn lt_th0_alert(&self) -> LtTh0AlertR {
        LtTh0AlertR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit will be driven to a level 1 for a given temperature monitor if it has its corresponding bit maxt_outrg_en = 1, and the temperature reading is reporting to be outside the max temperature supported, temp > programmed value. The level of this signal is a reflection, with some clock delays, of the temperature code reading. This is NOT an sticky bit. Reset value is POR only."]
    #[inline(always)]
    pub fn maxt_outrg_alert(&self) -> MaxtOutrgAlertR {
        MaxtOutrgAlertR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the core voltage domain placement of the temp sensor. Device specific field. This field indicates in which core voltage domain, cVD, has been physically placed the temp-monitor. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff at POR, d_VTM_TMPSENS\\[a\\]_STAT_vd_map_ipcfg."]
    #[inline(always)]
    pub fn vd_map(&self) -> VdMapR {
        VdMapR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Data_out signal value from sensor: Temperature data from the ADC in monitor. Valid after VTM_TMPSENS\\[a\\]_STAT.eoc_fc_update = 1. This value will be latched in this VTM register every time monitor output data_valid transitions from 0 to 1. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn data_out(&mut self) -> DataOutW<Mmr_Vbusp_Cfg1StatSpec> {
        DataOutW::new(self, 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Data_valid signal value from sensor: ADC End of Conversion. End of conversion indicated by 0 to 1 transition. When high data_out(9:0) out of the temp-monitor is valid. This field doesn't reflect the instantaneous output from the temp-monitor. This field gets latched/updated in this VTM register when the data_valid value from temp-monitor toggles. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz."]
    #[inline(always)]
    #[must_use]
    pub fn data_valid(&mut self) -> DataValidW<Mmr_Vbusp_Cfg1StatSpec> {
        DataValidW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
First time end of conversion. This field is reset to 0 every time VTM.por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. This bit will be set to 1 after the first time after reset release that data_valid transitions from 0 to 1, and remain at 1 until next time either of por_rst_n or VTM_TMPSENS\\[a\\]_CTRL.clrz are active, or when continuous mode is deasserted. Reset value is at POR or VTM_TMPSENS\\[a\\]_CTRL.clrz, or continuous mode deassertion."]
    #[inline(always)]
    #[must_use]
    pub fn eoc_fc_update(&mut self) -> EocFcUpdateW<Mmr_Vbusp_Cfg1StatSpec> {
        EocFcUpdateW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
This field reflects the status of the gt_th1_alert comparator result. The control MMR field gt_th1_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
    #[inline(always)]
    #[must_use]
    pub fn gt_th1_alert(&mut self) -> GtTh1AlertW<Mmr_Vbusp_Cfg1StatSpec> {
        GtTh1AlertW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
This field reflects the status of the gt_th2_alert comparator result. The control MMR field gt_th2_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
    #[inline(always)]
    #[must_use]
    pub fn gt_th2_alert(&mut self) -> GtTh2AlertW<Mmr_Vbusp_Cfg1StatSpec> {
        GtTh2AlertW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
This field reflects the status of the lt_th0_alert comparator result. The control MMR field lt_th0_en = 1 is required for this field to become 1. Reset value is at POR or clrz."]
    #[inline(always)]
    #[must_use]
    pub fn lt_th0_alert(&mut self) -> LtTh0AlertW<Mmr_Vbusp_Cfg1StatSpec> {
        LtTh0AlertW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit will be driven to a level 1 for a given temperature monitor if it has its corresponding bit maxt_outrg_en = 1, and the temperature reading is reporting to be outside the max temperature supported, temp > programmed value. The level of this signal is a reflection, with some clock delays, of the temperature code reading. This is NOT an sticky bit. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn maxt_outrg_alert(&mut self) -> MaxtOutrgAlertW<Mmr_Vbusp_Cfg1StatSpec> {
        MaxtOutrgAlertW::new(self, 15)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the core voltage domain placement of the temp sensor. Device specific field. This field indicates in which core voltage domain, cVD, has been physically placed the temp-monitor. Valid values: 0x0 to 0xE where: 0x0 = VD_RTC, not present is some SOCs, 0x1 = VD_WKUP, 0x2 = VD_MCU, 0x3 = VD_CORE, not present is some SOCs, 0x4-0xE = Mapping varies between SOCs, 0xF = not implemented. Reset value is a VTM tieoff at POR, d_VTM_TMPSENS\\[a\\]_STAT_vd_map_ipcfg."]
    #[inline(always)]
    #[must_use]
    pub fn vd_map(&mut self) -> VdMapW<Mmr_Vbusp_Cfg1StatSpec> {
        VdMapW::new(self, 16)
    }
}
#[doc = "Temperature Sensor Band-gap Status register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg1_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg1_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg1StatSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg1StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg1_stat::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg1StatSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg1_stat::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg1StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG1_STAT to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg1StatSpec {
    const RESET_VALUE: u32 = 0;
}
