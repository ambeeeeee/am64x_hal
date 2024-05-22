#[doc = "Register `ADCREGS_STEPCONFIG_4` reader"]
pub type R = crate::R<AdcregsStepconfig4Spec>;
#[doc = "Register `ADCREGS_STEPCONFIG_4` writer"]
pub type W = crate::W<AdcregsStepconfig4Spec>;
#[doc = "Field `MODE` reader - 1:0\\]
00 SW enabled one shot 01 SW enabled continuous 10 HW synchronized one-shot 11 HW synchronized continuous"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - 1:0\\]
00 SW enabled one shot 01 SW enabled continuous 10 HW synchronized one-shot 11 HW synchronized continuous"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AVERAGING` reader - 4:2\\]
000 -> no average 001 -> 2 samples average 010 -> 4 samples average 011 -> 8 samples average 100 -> 16 samples average"]
pub type AveragingR = crate::FieldReader;
#[doc = "Field `AVERAGING` writer - 4:2\\]
000 -> no average 001 -> 2 samples average 010 -> 4 samples average 011 -> 8 samples average 100 -> 16 samples average"]
pub type AveragingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_INM_SWM` reader - 18:15\\]
SEL_INM pins, negative differential"]
pub type SelInmSwmR = crate::FieldReader;
#[doc = "Field `SEL_INM_SWM` writer - 18:15\\]
SEL_INM pins, negative differential"]
pub type SelInmSwmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL_INP_SWC` reader - 22:19\\]
SEL_INP pins SW configuration"]
pub type SelInpSwcR = crate::FieldReader;
#[doc = "Field `SEL_INP_SWC` writer - 22:19\\]
SEL_INP pins SW configuration"]
pub type SelInpSwcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIFF_CNTRL` reader - 25:25\\]
Differential control. Single ended when = 0, differential input when = 1"]
pub type DiffCntrlR = crate::BitReader;
#[doc = "Field `DIFF_CNTRL` writer - 25:25\\]
Differential control. Single ended when = 0, differential input when = 1"]
pub type DiffCntrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOSEL` reader - 26:26\\]
Sampled data will be stored in FIFO0 when = 0, FIFO1 when = 1"]
pub type FifoselR = crate::BitReader;
#[doc = "Field `FIFOSEL` writer - 26:26\\]
Sampled data will be stored in FIFO0 when = 0, FIFO1 when = 1"]
pub type FifoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGECHECK` reader - 27:27\\]
0 = no range sel 1 = compare ADC data with range"]
pub type RangecheckR = crate::BitReader;
#[doc = "Field `RANGECHECK` writer - 27:27\\]
0 = no range sel 1 = compare ADC data with range"]
pub type RangecheckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
00 SW enabled one shot 01 SW enabled continuous 10 HW synchronized one-shot 11 HW synchronized continuous"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\]
000 -> no average 001 -> 2 samples average 010 -> 4 samples average 011 -> 8 samples average 100 -> 16 samples average"]
    #[inline(always)]
    pub fn averaging(&self) -> AveragingR {
        AveragingR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 15:18 - 18:15\\]
SEL_INM pins, negative differential"]
    #[inline(always)]
    pub fn sel_inm_swm(&self) -> SelInmSwmR {
        SelInmSwmR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - 22:19\\]
SEL_INP pins SW configuration"]
    #[inline(always)]
    pub fn sel_inp_swc(&self) -> SelInpSwcR {
        SelInpSwcR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Differential control. Single ended when = 0, differential input when = 1"]
    #[inline(always)]
    pub fn diff_cntrl(&self) -> DiffCntrlR {
        DiffCntrlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Sampled data will be stored in FIFO0 when = 0, FIFO1 when = 1"]
    #[inline(always)]
    pub fn fifosel(&self) -> FifoselR {
        FifoselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0 = no range sel 1 = compare ADC data with range"]
    #[inline(always)]
    pub fn rangecheck(&self) -> RangecheckR {
        RangecheckR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
00 SW enabled one shot 01 SW enabled continuous 10 HW synchronized one-shot 11 HW synchronized continuous"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<AdcregsStepconfig4Spec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
000 -> no average 001 -> 2 samples average 010 -> 4 samples average 011 -> 8 samples average 100 -> 16 samples average"]
    #[inline(always)]
    #[must_use]
    pub fn averaging(&mut self) -> AveragingW<AdcregsStepconfig4Spec> {
        AveragingW::new(self, 2)
    }
    #[doc = "Bits 15:18 - 18:15\\]
SEL_INM pins, negative differential"]
    #[inline(always)]
    #[must_use]
    pub fn sel_inm_swm(&mut self) -> SelInmSwmW<AdcregsStepconfig4Spec> {
        SelInmSwmW::new(self, 15)
    }
    #[doc = "Bits 19:22 - 22:19\\]
SEL_INP pins SW configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sel_inp_swc(&mut self) -> SelInpSwcW<AdcregsStepconfig4Spec> {
        SelInpSwcW::new(self, 19)
    }
    #[doc = "Bit 25 - 25:25\\]
Differential control. Single ended when = 0, differential input when = 1"]
    #[inline(always)]
    #[must_use]
    pub fn diff_cntrl(&mut self) -> DiffCntrlW<AdcregsStepconfig4Spec> {
        DiffCntrlW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Sampled data will be stored in FIFO0 when = 0, FIFO1 when = 1"]
    #[inline(always)]
    #[must_use]
    pub fn fifosel(&mut self) -> FifoselW<AdcregsStepconfig4Spec> {
        FifoselW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
0 = no range sel 1 = compare ADC data with range"]
    #[inline(always)]
    #[must_use]
    pub fn rangecheck(&mut self) -> RangecheckW<AdcregsStepconfig4Spec> {
        RangecheckW::new(self, 27)
    }
}
#[doc = "The user should write to this register the values of the input pins send to the AFE during step 1. the StepConfig WriteEnable bit should be set to 1 before a write can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepconfig_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepconfig_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsStepconfig4Spec;
impl crate::RegisterSpec for AdcregsStepconfig4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_stepconfig_4::R`](R) reader structure"]
impl crate::Readable for AdcregsStepconfig4Spec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_stepconfig_4::W`](W) writer structure"]
impl crate::Writable for AdcregsStepconfig4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_STEPCONFIG_4 to value 0x0004_0000"]
impl crate::Resettable for AdcregsStepconfig4Spec {
    const RESET_VALUE: u32 = 0x0004_0000;
}
