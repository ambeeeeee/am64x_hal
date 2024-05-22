#[doc = "Register `REGS_ded_enable_clr_reg0` reader"]
pub type R = crate::R<RegsDedEnableClrReg0Spec>;
#[doc = "Register `REGS_ded_enable_clr_reg0` writer"]
pub type W = crate::W<RegsDedEnableClrReg0Spec>;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__TIMER_FSM_RAMECC_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_TimerFsmRameccEnableClrR = crate::BitReader;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__TIMER_FSM_RAMECC_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_TimerFsmRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__REPROG_FSM_RAMECC_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_ReprogFsmRameccEnableClrR = crate::BitReader;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__REPROG_FSM_RAMECC_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_ReprogFsmRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__EFIFO_RAMECC_ENABLE_CLR` reader - 2:2\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
pub type ItimerMgr1024Main0_EfifoRameccEnableClrR = crate::BitReader;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__EFIFO_RAMECC_ENABLE_CLR` writer - 2:2\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
pub type ItimerMgr1024Main0_EfifoRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
    #[inline(always)]
    pub fn itimer_mgr1024_main_0__timer_fsm_ramecc_enable_clr(
        &self,
    ) -> ItimerMgr1024Main0_TimerFsmRameccEnableClrR {
        ItimerMgr1024Main0_TimerFsmRameccEnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
    #[inline(always)]
    pub fn itimer_mgr1024_main_0__reprog_fsm_ramecc_enable_clr(
        &self,
    ) -> ItimerMgr1024Main0_ReprogFsmRameccEnableClrR {
        ItimerMgr1024Main0_ReprogFsmRameccEnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
    #[inline(always)]
    pub fn itimer_mgr1024_main_0__efifo_ramecc_enable_clr(
        &self,
    ) -> ItimerMgr1024Main0_EfifoRameccEnableClrR {
        ItimerMgr1024Main0_EfifoRameccEnableClrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn itimer_mgr1024_main_0__timer_fsm_ramecc_enable_clr(
        &mut self,
    ) -> ItimerMgr1024Main0_TimerFsmRameccEnableClrW<RegsDedEnableClrReg0Spec> {
        ItimerMgr1024Main0_TimerFsmRameccEnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn itimer_mgr1024_main_0__reprog_fsm_ramecc_enable_clr(
        &mut self,
    ) -> ItimerMgr1024Main0_ReprogFsmRameccEnableClrW<RegsDedEnableClrReg0Spec> {
        ItimerMgr1024Main0_ReprogFsmRameccEnableClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn itimer_mgr1024_main_0__efifo_ramecc_enable_clr(
        &mut self,
    ) -> ItimerMgr1024Main0_EfifoRameccEnableClrW<RegsDedEnableClrReg0Spec> {
        ItimerMgr1024Main0_EfifoRameccEnableClrW::new(self, 2)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsDedEnableClrReg0Spec;
impl crate::RegisterSpec for RegsDedEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_ded_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for RegsDedEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`regs_ded_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for RegsDedEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_ded_enable_clr_reg0 to value 0"]
impl crate::Resettable for RegsDedEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
