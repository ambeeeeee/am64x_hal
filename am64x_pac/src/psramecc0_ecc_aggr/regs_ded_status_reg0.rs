#[doc = "Register `REGS_ded_status_reg0` reader"]
pub type R = crate::R<RegsDedStatusReg0Spec>;
#[doc = "Register `REGS_ded_status_reg0` writer"]
pub type W = crate::W<RegsDedStatusReg0Spec>;
#[doc = "Field `SRAM_PEND` reader - 0:0\\]
Interrupt Pending Status for sram_pend"]
pub type SramPendR = crate::BitReader;
#[doc = "Field `SRAM_PEND` writer - 0:0\\]
Interrupt Pending Status for sram_pend"]
pub type SramPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for sram_pend"]
    #[inline(always)]
    pub fn sram_pend(&self) -> SramPendR {
        SramPendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for sram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pend(&mut self) -> SramPendW<RegsDedStatusReg0Spec> {
        SramPendW::new(self, 0)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsDedStatusReg0Spec;
impl crate::RegisterSpec for RegsDedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for RegsDedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`regs_ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for RegsDedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_ded_status_reg0 to value 0"]
impl crate::Resettable for RegsDedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
