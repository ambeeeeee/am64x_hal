#[doc = "Register `ADCREGS_SEQUENCER_STAT` reader"]
pub type R = crate::R<AdcregsSequencerStatSpec>;
#[doc = "Register `ADCREGS_SEQUENCER_STAT` writer"]
pub type W = crate::W<AdcregsSequencerStatSpec>;
#[doc = "Field `STEP_IDLE` reader - 4:0\\]
10000 = idle, 000000 -> 01111 corresponds to step 1 -> step 16"]
pub type StepIdleR = crate::FieldReader;
#[doc = "Field `STEP_IDLE` writer - 4:0\\]
10000 = idle, 000000 -> 01111 corresponds to step 1 -> step 16"]
pub type StepIdleW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FSM_BUSY` reader - 5:5\\]
status of fsm, 1= conversion in progress"]
pub type FsmBusyR = crate::BitReader;
#[doc = "Field `FSM_BUSY` writer - 5:5\\]
status of fsm, 1= conversion in progress"]
pub type FsmBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_INIT_DONE` reader - 6:6\\]
status of ram initialization, 1= ram initialization to 0 after reset is done."]
pub type MemInitDoneR = crate::BitReader;
#[doc = "Field `MEM_INIT_DONE` writer - 6:6\\]
status of ram initialization, 1= ram initialization to 0 after reset is done."]
pub type MemInitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPADC_BUSY` reader - 8:8\\]
Monitor the AFE internal calibration, busy bit"]
pub type GpadcBusyR = crate::BitReader;
#[doc = "Field `GPADC_BUSY` writer - 8:8\\]
Monitor the AFE internal calibration, busy bit"]
pub type GpadcBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
10000 = idle, 000000 -> 01111 corresponds to step 1 -> step 16"]
    #[inline(always)]
    pub fn step_idle(&self) -> StepIdleR {
        StepIdleR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
status of fsm, 1= conversion in progress"]
    #[inline(always)]
    pub fn fsm_busy(&self) -> FsmBusyR {
        FsmBusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status of ram initialization, 1= ram initialization to 0 after reset is done."]
    #[inline(always)]
    pub fn mem_init_done(&self) -> MemInitDoneR {
        MemInitDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Monitor the AFE internal calibration, busy bit"]
    #[inline(always)]
    pub fn gpadc_busy(&self) -> GpadcBusyR {
        GpadcBusyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
10000 = idle, 000000 -> 01111 corresponds to step 1 -> step 16"]
    #[inline(always)]
    #[must_use]
    pub fn step_idle(&mut self) -> StepIdleW<AdcregsSequencerStatSpec> {
        StepIdleW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
status of fsm, 1= conversion in progress"]
    #[inline(always)]
    #[must_use]
    pub fn fsm_busy(&mut self) -> FsmBusyW<AdcregsSequencerStatSpec> {
        FsmBusyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
status of ram initialization, 1= ram initialization to 0 after reset is done."]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_done(&mut self) -> MemInitDoneW<AdcregsSequencerStatSpec> {
        MemInitDoneW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Monitor the AFE internal calibration, busy bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_busy(&mut self) -> GpadcBusyW<AdcregsSequencerStatSpec> {
        GpadcBusyW::new(self, 8)
    }
}
#[doc = "SW can read this register to find out the currently scheduled step id being converted on the ADC port. If you want to turn the controller off and then back on, the step_id bit should be checked and compared to IDLE before enabling the ADC12_SS module again. Also, before enabling the controller again, the user should wait for the FSM bit 5 to read idl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_sequencer_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_sequencer_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsSequencerStatSpec;
impl crate::RegisterSpec for AdcregsSequencerStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_sequencer_stat::R`](R) reader structure"]
impl crate::Readable for AdcregsSequencerStatSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_sequencer_stat::W`](W) writer structure"]
impl crate::Writable for AdcregsSequencerStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_SEQUENCER_STAT to value 0x20"]
impl crate::Resettable for AdcregsSequencerStatSpec {
    const RESET_VALUE: u32 = 0x20;
}
