#[doc = "Register `INTAGGR_INTR_ENABLE_CLR` reader"]
pub type R = crate::R<IntaggrIntrEnableClrSpec>;
#[doc = "Register `INTAGGR_INTR_ENABLE_CLR` writer"]
pub type W = crate::W<IntaggrIntrEnableClrSpec>;
#[doc = "Field `INTR_ENABLE` reader - 63:0\\]
Interrupt enable clear value. On writes, set bits will cause corresponding bits in the internal interrupt enable register to be cleared. Reads will reflect back the current status of the internal interrupt enable register."]
pub type IntrEnableR = crate::FieldReader<u64>;
#[doc = "Field `INTR_ENABLE` writer - 63:0\\]
Interrupt enable clear value. On writes, set bits will cause corresponding bits in the internal interrupt enable register to be cleared. Reads will reflect back the current status of the internal interrupt enable register."]
pub type IntrEnableW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - 63:0\\]
Interrupt enable clear value. On writes, set bits will cause corresponding bits in the internal interrupt enable register to be cleared. Reads will reflect back the current status of the internal interrupt enable register."]
    #[inline(always)]
    pub fn intr_enable(&self) -> IntrEnableR {
        IntrEnableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - 63:0\\]
Interrupt enable clear value. On writes, set bits will cause corresponding bits in the internal interrupt enable register to be cleared. Reads will reflect back the current status of the internal interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn intr_enable(&mut self) -> IntrEnableW<IntaggrIntrEnableClrSpec> {
        IntrEnableW::new(self, 0)
    }
}
#[doc = "The Interrupt Enable Clear register is written by software to disable (i.e. mask) specified bits to disallow their current status from be considered in the generation of the corresponding level sensitive virtual interrupt output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_enable_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_enable_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrIntrEnableClrSpec;
impl crate::RegisterSpec for IntaggrIntrEnableClrSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_intr_enable_clr::R`](R) reader structure"]
impl crate::Readable for IntaggrIntrEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_intr_enable_clr::W`](W) writer structure"]
impl crate::Writable for IntaggrIntrEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_INTR_ENABLE_CLR to value 0"]
impl crate::Resettable for IntaggrIntrEnableClrSpec {
    const RESET_VALUE: u64 = 0;
}
