#[doc = "Register `MMR__VBUSP__CFG2_CTRL` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg2CtrlSpec>;
#[doc = "Register `MMR__VBUSP__CFG2_CTRL` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg2CtrlSpec>;
#[doc = "Field `CONT` reader - 4:4\\]
Temp-Monitor control: ADC Continuous mode. Setting this mode enables the VTM to continuously monitor the sensor automatically. Each sample period the sensor will be accessed and the results captured. Reset value is POR only."]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - 4:4\\]
Temp-Monitor control: ADC Continuous mode. Setting this mode enables the VTM to continuously monitor the sensor automatically. Each sample period the sensor will be accessed and the results captured. Reset value is POR only."]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC` reader - 5:5\\]
Temp-Monitor control: ADC Start of Conversion. A transition from 0 to 1 starts a new ADC conversion cycle. The bit with automatically clear when the conversion has completed. This mode is not valid when already in continuous mode. Reset value is POR only."]
pub type SocR = crate::BitReader;
#[doc = "Field `SOC` writer - 5:5\\]
Temp-Monitor control: ADC Start of Conversion. A transition from 0 to 1 starts a new ADC conversion cycle. The bit with automatically clear when the conversion has completed. This mode is not valid when already in continuous mode. Reset value is POR only."]
pub type SocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRZ` reader - 6:6\\]
Temp-Monitor control: 0 = Reset all Temp-monitor digital outputs. 1 = Allow operation of sensor. Reset value is POR only."]
pub type ClrzR = crate::BitReader;
#[doc = "Field `CLRZ` writer - 6:6\\]
Temp-Monitor control: 0 = Reset all Temp-monitor digital outputs. 1 = Allow operation of sensor. Reset value is POR only."]
pub type ClrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXT_OUTRG_EN` reader - 11:11\\]
Enable out-of-range event. This bit enables generation of the alert in case the given temperature sensors generates a temp code above a programmed max. 0 = Don't generate alert. 1 = Generate alert. Reset value is POR only."]
pub type MaxtOutrgEnR = crate::BitReader;
#[doc = "Field `MAXT_OUTRG_EN` writer - 11:11\\]
Enable out-of-range event. This bit enables generation of the alert in case the given temperature sensors generates a temp code above a programmed max. 0 = Don't generate alert. 1 = Generate alert. Reset value is POR only."]
pub type MaxtOutrgEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Temp-Monitor control: ADC Continuous mode. Setting this mode enables the VTM to continuously monitor the sensor automatically. Each sample period the sensor will be accessed and the results captured. Reset value is POR only."]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Temp-Monitor control: ADC Start of Conversion. A transition from 0 to 1 starts a new ADC conversion cycle. The bit with automatically clear when the conversion has completed. This mode is not valid when already in continuous mode. Reset value is POR only."]
    #[inline(always)]
    pub fn soc(&self) -> SocR {
        SocR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Temp-Monitor control: 0 = Reset all Temp-monitor digital outputs. 1 = Allow operation of sensor. Reset value is POR only."]
    #[inline(always)]
    pub fn clrz(&self) -> ClrzR {
        ClrzR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable out-of-range event. This bit enables generation of the alert in case the given temperature sensors generates a temp code above a programmed max. 0 = Don't generate alert. 1 = Generate alert. Reset value is POR only."]
    #[inline(always)]
    pub fn maxt_outrg_en(&self) -> MaxtOutrgEnR {
        MaxtOutrgEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Temp-Monitor control: ADC Continuous mode. Setting this mode enables the VTM to continuously monitor the sensor automatically. Each sample period the sensor will be accessed and the results captured. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<Mmr_Vbusp_Cfg2CtrlSpec> {
        ContW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Temp-Monitor control: ADC Start of Conversion. A transition from 0 to 1 starts a new ADC conversion cycle. The bit with automatically clear when the conversion has completed. This mode is not valid when already in continuous mode. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn soc(&mut self) -> SocW<Mmr_Vbusp_Cfg2CtrlSpec> {
        SocW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Temp-Monitor control: 0 = Reset all Temp-monitor digital outputs. 1 = Allow operation of sensor. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn clrz(&mut self) -> ClrzW<Mmr_Vbusp_Cfg2CtrlSpec> {
        ClrzW::new(self, 6)
    }
    #[doc = "Bit 11 - 11:11\\]
Enable out-of-range event. This bit enables generation of the alert in case the given temperature sensors generates a temp code above a programmed max. 0 = Don't generate alert. 1 = Generate alert. Reset value is POR only."]
    #[inline(always)]
    #[must_use]
    pub fn maxt_outrg_en(&mut self) -> MaxtOutrgEnW<Mmr_Vbusp_Cfg2CtrlSpec> {
        MaxtOutrgEnW::new(self, 11)
    }
}
#[doc = "Temperature Sensor Band-gap control register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg2CtrlSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg2_ctrl::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg2_ctrl::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg2CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG2_CTRL to value 0x40"]
impl crate::Resettable for Mmr_Vbusp_Cfg2CtrlSpec {
    const RESET_VALUE: u32 = 0x40;
}
