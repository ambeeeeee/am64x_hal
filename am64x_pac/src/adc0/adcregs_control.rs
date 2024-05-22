#[doc = "Register `ADCREGS_CONTROL` reader"]
pub type R = crate::R<AdcregsControlSpec>;
#[doc = "Register `ADCREGS_CONTROL` writer"]
pub type W = crate::W<AdcregsControlSpec>;
#[doc = "Field `MODULE_ENABLE` reader - 0:0\\]
ADC12_SS module enable bit. After programming all the configuration and step enable registers, write a 1 to this bit to start conversion. Writing a 0 will disable the module after the current conversion. Before turning on again, the ADC_SEQUENCER_STATUS register show read back STEP_ID=Idle and FSM_BUSY=0. Enabling the controller will be held off until mem_init_done =1."]
pub type ModuleEnableR = crate::BitReader;
#[doc = "Field `MODULE_ENABLE` writer - 0:0\\]
ADC12_SS module enable bit. After programming all the configuration and step enable registers, write a 1 to this bit to start conversion. Writing a 0 will disable the module after the current conversion. Before turning on again, the ADC_SEQUENCER_STATUS register show read back STEP_ID=Idle and FSM_BUSY=0. Enabling the controller will be held off until mem_init_done =1."]
pub type ModuleEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_ID_EN` reader - 1:1\\]
writing 1 will store the stepid number with the captured adc data in the fifo"]
pub type StepIdEnR = crate::BitReader;
#[doc = "Field `STEP_ID_EN` writer - 1:1\\]
writing 1 will store the stepid number with the captured adc data in the fifo"]
pub type StepIdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS_SEL` reader - 3:3\\]
AFE select bias control"]
pub type BiasSelR = crate::BitReader;
#[doc = "Field `BIAS_SEL` writer - 3:3\\]
AFE select bias control"]
pub type BiasSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD` reader - 4:4\\]
AFE powered down"]
pub type PdR = crate::BitReader;
#[doc = "Field `PD` writer - 4:4\\]
AFE powered down"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_MAP` reader - 8:8\\]
1 = hw events enabled"]
pub type HwMapR = crate::BitReader;
#[doc = "Field `HW_MAP` writer - 8:8\\]
1 = hw events enabled"]
pub type HwMapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_PREEMPT` reader - 9:9\\]
1 steps are preempted"]
pub type HwPreemptR = crate::BitReader;
#[doc = "Field `HW_PREEMPT` writer - 9:9\\]
1 steps are preempted"]
pub type HwPreemptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_MID_EN` reader - 10:10\\]
Functional safety debug mode. enable fixed reference to ADC for testing"]
pub type HiMidEnR = crate::BitReader;
#[doc = "Field `HI_MID_EN` writer - 10:10\\]
Functional safety debug mode. enable fixed reference to ADC for testing"]
pub type HiMidEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_MID_SEL` reader - 11:11\\]
Functional safety debug mode. =1 choose ADCREFP, =0 VMID reference input to ADC"]
pub type HiMidSelR = crate::BitReader;
#[doc = "Field `HI_MID_SEL` writer - 11:11\\]
Functional safety debug mode. =1 choose ADCREFP, =0 VMID reference input to ADC"]
pub type HiMidSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ADC12_SS module enable bit. After programming all the configuration and step enable registers, write a 1 to this bit to start conversion. Writing a 0 will disable the module after the current conversion. Before turning on again, the ADC_SEQUENCER_STATUS register show read back STEP_ID=Idle and FSM_BUSY=0. Enabling the controller will be held off until mem_init_done =1."]
    #[inline(always)]
    pub fn module_enable(&self) -> ModuleEnableR {
        ModuleEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
writing 1 will store the stepid number with the captured adc data in the fifo"]
    #[inline(always)]
    pub fn step_id_en(&self) -> StepIdEnR {
        StepIdEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AFE select bias control"]
    #[inline(always)]
    pub fn bias_sel(&self) -> BiasSelR {
        BiasSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AFE powered down"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
1 = hw events enabled"]
    #[inline(always)]
    pub fn hw_map(&self) -> HwMapR {
        HwMapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
1 steps are preempted"]
    #[inline(always)]
    pub fn hw_preempt(&self) -> HwPreemptR {
        HwPreemptR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Functional safety debug mode. enable fixed reference to ADC for testing"]
    #[inline(always)]
    pub fn hi_mid_en(&self) -> HiMidEnR {
        HiMidEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Functional safety debug mode. =1 choose ADCREFP, =0 VMID reference input to ADC"]
    #[inline(always)]
    pub fn hi_mid_sel(&self) -> HiMidSelR {
        HiMidSelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ADC12_SS module enable bit. After programming all the configuration and step enable registers, write a 1 to this bit to start conversion. Writing a 0 will disable the module after the current conversion. Before turning on again, the ADC_SEQUENCER_STATUS register show read back STEP_ID=Idle and FSM_BUSY=0. Enabling the controller will be held off until mem_init_done =1."]
    #[inline(always)]
    #[must_use]
    pub fn module_enable(&mut self) -> ModuleEnableW<AdcregsControlSpec> {
        ModuleEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
writing 1 will store the stepid number with the captured adc data in the fifo"]
    #[inline(always)]
    #[must_use]
    pub fn step_id_en(&mut self) -> StepIdEnW<AdcregsControlSpec> {
        StepIdEnW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
AFE select bias control"]
    #[inline(always)]
    #[must_use]
    pub fn bias_sel(&mut self) -> BiasSelW<AdcregsControlSpec> {
        BiasSelW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
AFE powered down"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<AdcregsControlSpec> {
        PdW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
1 = hw events enabled"]
    #[inline(always)]
    #[must_use]
    pub fn hw_map(&mut self) -> HwMapW<AdcregsControlSpec> {
        HwMapW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
1 steps are preempted"]
    #[inline(always)]
    #[must_use]
    pub fn hw_preempt(&mut self) -> HwPreemptW<AdcregsControlSpec> {
        HwPreemptW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Functional safety debug mode. enable fixed reference to ADC for testing"]
    #[inline(always)]
    #[must_use]
    pub fn hi_mid_en(&mut self) -> HiMidEnW<AdcregsControlSpec> {
        HiMidEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Functional safety debug mode. =1 choose ADCREFP, =0 VMID reference input to ADC"]
    #[inline(always)]
    #[must_use]
    pub fn hi_mid_sel(&mut self) -> HiMidSelW<AdcregsControlSpec> {
        HiMidSelW::new(self, 11)
    }
}
#[doc = "Controls various parameters of the cotroller state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsControlSpec;
impl crate::RegisterSpec for AdcregsControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_control::R`](R) reader structure"]
impl crate::Readable for AdcregsControlSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_control::W`](W) writer structure"]
impl crate::Writable for AdcregsControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_CONTROL to value 0x10"]
impl crate::Resettable for AdcregsControlSpec {
    const RESET_VALUE: u32 = 0x10;
}
