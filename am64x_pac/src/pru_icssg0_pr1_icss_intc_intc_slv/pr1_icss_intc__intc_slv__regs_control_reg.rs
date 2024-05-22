#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsControlRegSpec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsControlRegSpec>;
#[doc = "Field `WAKEUP_MODE` reader - 1:1\\]
Wakeup mode enable"]
pub type WakeupModeR = crate::BitReader;
#[doc = "Field `WAKEUP_MODE` writer - 1:1\\]
Wakeup mode enable"]
pub type WakeupModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEST_MODE` reader - 3:2\\]
Nesting Mode"]
pub type NestModeR = crate::FieldReader;
#[doc = "Field `NEST_MODE` writer - 3:2\\]
Nesting Mode"]
pub type NestModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRIORITY_HOLD_MODE` reader - 4:4\\]
Priority Holding Mode"]
pub type PriorityHoldModeR = crate::BitReader;
#[doc = "Field `PRIORITY_HOLD_MODE` writer - 4:4\\]
Priority Holding Mode"]
pub type PriorityHoldModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Wakeup mode enable"]
    #[inline(always)]
    pub fn wakeup_mode(&self) -> WakeupModeR {
        WakeupModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Nesting Mode"]
    #[inline(always)]
    pub fn nest_mode(&self) -> NestModeR {
        NestModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Priority Holding Mode"]
    #[inline(always)]
    pub fn priority_hold_mode(&self) -> PriorityHoldModeR {
        PriorityHoldModeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Wakeup mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_mode(&mut self) -> WakeupModeW<Pr1IcssIntc_IntcSlv_RegsControlRegSpec> {
        WakeupModeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Nesting Mode"]
    #[inline(always)]
    #[must_use]
    pub fn nest_mode(&mut self) -> NestModeW<Pr1IcssIntc_IntcSlv_RegsControlRegSpec> {
        NestModeW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Priority Holding Mode"]
    #[inline(always)]
    #[must_use]
    pub fn priority_hold_mode(
        &mut self,
    ) -> PriorityHoldModeW<Pr1IcssIntc_IntcSlv_RegsControlRegSpec> {
        PriorityHoldModeW::new(self, 4)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_control_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_control_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsControlRegSpec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_control_reg::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_control_reg::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CONTROL_REG to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
