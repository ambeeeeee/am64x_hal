#[doc = "Register `REGS_sec_status_reg0` reader"]
pub type R = crate::R<RegsSecStatusReg0Spec>;
#[doc = "Register `REGS_sec_status_reg0` writer"]
pub type W = crate::W<RegsSecStatusReg0Spec>;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__TIMER_FSM_RAMECC_PEND` reader - 0:0\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_TimerFsmRameccPendR = crate::BitReader;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__TIMER_FSM_RAMECC_PEND` writer - 0:0\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_TimerFsmRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__REPROG_FSM_RAMECC_PEND` reader - 1:1\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_ReprogFsmRameccPendR = crate::BitReader;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__REPROG_FSM_RAMECC_PEND` writer - 1:1\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
pub type ItimerMgr1024Main0_ReprogFsmRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__EFIFO_RAMECC_PEND` reader - 2:2\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
pub type ItimerMgr1024Main0_EfifoRameccPendR = crate::BitReader;
#[doc = "Field `ITIMER_MGR1024_MAIN_0__EFIFO_RAMECC_PEND` writer - 2:2\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
pub type ItimerMgr1024Main0_EfifoRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
    #[inline(always)]
    pub fn itimer_mgr1024_main_0__timer_fsm_ramecc_pend(
        &self,
    ) -> ItimerMgr1024Main0_TimerFsmRameccPendR {
        ItimerMgr1024Main0_TimerFsmRameccPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
    #[inline(always)]
    pub fn itimer_mgr1024_main_0__reprog_fsm_ramecc_pend(
        &self,
    ) -> ItimerMgr1024Main0_ReprogFsmRameccPendR {
        ItimerMgr1024Main0_ReprogFsmRameccPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
    #[inline(always)]
    pub fn itimer_mgr1024_main_0__efifo_ramecc_pend(&self) -> ItimerMgr1024Main0_EfifoRameccPendR {
        ItimerMgr1024Main0_EfifoRameccPendR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__timer_fsm_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn itimer_mgr1024_main_0__timer_fsm_ramecc_pend(
        &mut self,
    ) -> ItimerMgr1024Main0_TimerFsmRameccPendW<RegsSecStatusReg0Spec> {
        ItimerMgr1024Main0_TimerFsmRameccPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__reprog_fsm_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn itimer_mgr1024_main_0__reprog_fsm_ramecc_pend(
        &mut self,
    ) -> ItimerMgr1024Main0_ReprogFsmRameccPendW<RegsSecStatusReg0Spec> {
        ItimerMgr1024Main0_ReprogFsmRameccPendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for Itimer_mgr1024_main_0__efifo_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn itimer_mgr1024_main_0__efifo_ramecc_pend(
        &mut self,
    ) -> ItimerMgr1024Main0_EfifoRameccPendW<RegsSecStatusReg0Spec> {
        ItimerMgr1024Main0_EfifoRameccPendW::new(self, 2)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_sec_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_sec_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsSecStatusReg0Spec;
impl crate::RegisterSpec for RegsSecStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_sec_status_reg0::R`](R) reader structure"]
impl crate::Readable for RegsSecStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`regs_sec_status_reg0::W`](W) writer structure"]
impl crate::Writable for RegsSecStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_sec_status_reg0 to value 0"]
impl crate::Resettable for RegsSecStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
