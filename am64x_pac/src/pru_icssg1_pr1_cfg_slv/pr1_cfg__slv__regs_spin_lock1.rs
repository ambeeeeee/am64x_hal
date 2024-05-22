#[doc = "Register `PR1_CFG__SLV__REGS_spin_lock1` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsSpinLock1Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_spin_lock1` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsSpinLock1Spec>;
#[doc = "Field `MMR_OWN_REQ_STATUS_1` reader - 0:0\\]
Spin Lock Status"]
pub type MmrOwnReqStatus1R = crate::BitReader;
#[doc = "Field `MMR_OWN_REQ_STATUS_1` writer - 0:0\\]
Spin Lock Status"]
pub type MmrOwnReqStatus1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMR_OWN_REQ_CLR_1` reader - 1:1\\]
Spin Lock Status Clear"]
pub type MmrOwnReqClr1R = crate::BitReader;
#[doc = "Field `MMR_OWN_REQ_CLR_1` writer - 1:1\\]
Spin Lock Status Clear"]
pub type MmrOwnReqClr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMR_OWN_REQ_VECTOR_1` reader - 13:8\\]
Spin Lock flag Vector"]
pub type MmrOwnReqVector1R = crate::FieldReader;
#[doc = "Field `MMR_OWN_REQ_VECTOR_1` writer - 13:8\\]
Spin Lock flag Vector"]
pub type MmrOwnReqVector1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Spin Lock Status"]
    #[inline(always)]
    pub fn mmr_own_req_status_1(&self) -> MmrOwnReqStatus1R {
        MmrOwnReqStatus1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Spin Lock Status Clear"]
    #[inline(always)]
    pub fn mmr_own_req_clr_1(&self) -> MmrOwnReqClr1R {
        MmrOwnReqClr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Spin Lock flag Vector"]
    #[inline(always)]
    pub fn mmr_own_req_vector_1(&self) -> MmrOwnReqVector1R {
        MmrOwnReqVector1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Spin Lock Status"]
    #[inline(always)]
    #[must_use]
    pub fn mmr_own_req_status_1(&mut self) -> MmrOwnReqStatus1W<Pr1Cfg_Slv_RegsSpinLock1Spec> {
        MmrOwnReqStatus1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Spin Lock Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mmr_own_req_clr_1(&mut self) -> MmrOwnReqClr1W<Pr1Cfg_Slv_RegsSpinLock1Spec> {
        MmrOwnReqClr1W::new(self, 1)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Spin Lock flag Vector"]
    #[inline(always)]
    #[must_use]
    pub fn mmr_own_req_vector_1(&mut self) -> MmrOwnReqVector1W<Pr1Cfg_Slv_RegsSpinLock1Spec> {
        MmrOwnReqVector1W::new(self, 8)
    }
}
#[doc = "PR1_CFG__SLV__REGS_spin_lock1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_spin_lock1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_spin_lock1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsSpinLock1Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsSpinLock1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_spin_lock1::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsSpinLock1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_spin_lock1::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsSpinLock1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_spin_lock1 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsSpinLock1Spec {
    const RESET_VALUE: u32 = 0;
}
