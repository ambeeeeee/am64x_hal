#[doc = "Register `CFG_pll8_CAL_CTRL` reader"]
pub type R = crate::R<CfgPll8CalCtrlSpec>;
#[doc = "Register `CFG_pll8_CAL_CTRL` writer"]
pub type W = crate::W<CfgPll8CalCtrlSpec>;
#[doc = "Field `CAL_IN` reader - 11:0\\]
Calibration input When cal_byp is 1'b0, this represents the initial condition for calibration. When cal_byp is 1'b1, this is the override value for calibration. Value is a signed integer with positive values delaying the faster path reset and negative values delaying the slower path reset."]
pub type CalInR = crate::FieldReader<u16>;
#[doc = "Field `CAL_IN` writer - 11:0\\]
Calibration input When cal_byp is 1'b0, this represents the initial condition for calibration. When cal_byp is 1'b1, this is the override value for calibration. Value is a signed integer with positive values delaying the faster path reset and negative values delaying the slower path reset."]
pub type CalInW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CAL_BYP` reader - 15:15\\]
Calibration bypass 1'b0 - Use the calibration output to set the phase correction 1'b1 - Use the cal_in input value to set the phase correction"]
pub type CalBypR = crate::BitReader;
#[doc = "Field `CAL_BYP` writer - 15:15\\]
Calibration bypass 1'b0 - Use the calibration output to set the phase correction 1'b1 - Use the cal_in input value to set the phase correction"]
pub type CalBypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_CNT` reader - 18:16\\]
Calibration loop programmable counter. Selects the number of PFD edges to wait after each calibration step defined by 2**cal_cnt"]
pub type CalCntR = crate::FieldReader;
#[doc = "Field `CAL_CNT` writer - 18:16\\]
Calibration loop programmable counter. Selects the number of PFD edges to wait after each calibration step defined by 2**cal_cnt"]
pub type CalCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST_CAL` reader - 20:20\\]
Fast calibration enabled 1'b0 - Normal operation 1'b1 - Used for initial calibration if initial value is not already known"]
pub type FastCalR = crate::BitReader;
#[doc = "Field `FAST_CAL` writer - 20:20\\]
Fast calibration enabled 1'b0 - Normal operation 1'b1 - Used for initial calibration if initial value is not already known"]
pub type FastCalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_EN` reader - 31:31\\]
Calibration enable to actively adjust for input skew 1'b0 - Disabled. Static phase offset determined by analog matching only 1'b1 - Enabled. Static phase offset adjusted by phase sensing at input"]
pub type CalEnR = crate::BitReader;
#[doc = "Field `CAL_EN` writer - 31:31\\]
Calibration enable to actively adjust for input skew 1'b0 - Disabled. Static phase offset determined by analog matching only 1'b1 - Enabled. Static phase offset adjusted by phase sensing at input"]
pub type CalEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Calibration input When cal_byp is 1'b0, this represents the initial condition for calibration. When cal_byp is 1'b1, this is the override value for calibration. Value is a signed integer with positive values delaying the faster path reset and negative values delaying the slower path reset."]
    #[inline(always)]
    pub fn cal_in(&self) -> CalInR {
        CalInR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Calibration bypass 1'b0 - Use the calibration output to set the phase correction 1'b1 - Use the cal_in input value to set the phase correction"]
    #[inline(always)]
    pub fn cal_byp(&self) -> CalBypR {
        CalBypR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Calibration loop programmable counter. Selects the number of PFD edges to wait after each calibration step defined by 2**cal_cnt"]
    #[inline(always)]
    pub fn cal_cnt(&self) -> CalCntR {
        CalCntR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Fast calibration enabled 1'b0 - Normal operation 1'b1 - Used for initial calibration if initial value is not already known"]
    #[inline(always)]
    pub fn fast_cal(&self) -> FastCalR {
        FastCalR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Calibration enable to actively adjust for input skew 1'b0 - Disabled. Static phase offset determined by analog matching only 1'b1 - Enabled. Static phase offset adjusted by phase sensing at input"]
    #[inline(always)]
    pub fn cal_en(&self) -> CalEnR {
        CalEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Calibration input When cal_byp is 1'b0, this represents the initial condition for calibration. When cal_byp is 1'b1, this is the override value for calibration. Value is a signed integer with positive values delaying the faster path reset and negative values delaying the slower path reset."]
    #[inline(always)]
    #[must_use]
    pub fn cal_in(&mut self) -> CalInW<CfgPll8CalCtrlSpec> {
        CalInW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Calibration bypass 1'b0 - Use the calibration output to set the phase correction 1'b1 - Use the cal_in input value to set the phase correction"]
    #[inline(always)]
    #[must_use]
    pub fn cal_byp(&mut self) -> CalBypW<CfgPll8CalCtrlSpec> {
        CalBypW::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Calibration loop programmable counter. Selects the number of PFD edges to wait after each calibration step defined by 2**cal_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn cal_cnt(&mut self) -> CalCntW<CfgPll8CalCtrlSpec> {
        CalCntW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Fast calibration enabled 1'b0 - Normal operation 1'b1 - Used for initial calibration if initial value is not already known"]
    #[inline(always)]
    #[must_use]
    pub fn fast_cal(&mut self) -> FastCalW<CfgPll8CalCtrlSpec> {
        FastCalW::new(self, 20)
    }
    #[doc = "Bit 31 - 31:31\\]
Calibration enable to actively adjust for input skew 1'b0 - Disabled. Static phase offset determined by analog matching only 1'b1 - Enabled. Static phase offset adjusted by phase sensing at input"]
    #[inline(always)]
    #[must_use]
    pub fn cal_en(&mut self) -> CalEnW<CfgPll8CalCtrlSpec> {
        CalEnW::new(self, 31)
    }
}
#[doc = "CFG_pll8_CAL_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_cal_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_cal_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll8CalCtrlSpec;
impl crate::RegisterSpec for CfgPll8CalCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll8_cal_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgPll8CalCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll8_cal_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgPll8CalCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll8_CAL_CTRL to value 0x0002_0000"]
impl crate::Resettable for CfgPll8CalCtrlSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
