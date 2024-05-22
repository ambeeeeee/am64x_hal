#[doc = "Register `APBADDR_DBG_CPU0_DBGDTRTX_EL0` reader"]
pub type R = crate::R<ApbaddrDbgCpu0DbgdtrtxEl0Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_DBGDTRTX_EL0` writer"]
pub type W = crate::W<ApbaddrDbgCpu0DbgdtrtxEl0Spec>;
#[doc = "Field `DBGDTRTX_EL0` reader - 31:0\\]
Return DTRTX. Reads of this register return the value in DTRTX and clear TXfull to 0.Writes of this register update the value in DTRTX and do not change TXfull."]
pub type DbgdtrtxEl0R = crate::FieldReader<u32>;
#[doc = "Field `DBGDTRTX_EL0` writer - 31:0\\]
Return DTRTX. Reads of this register return the value in DTRTX and clear TXfull to 0.Writes of this register update the value in DTRTX and do not change TXfull."]
pub type DbgdtrtxEl0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Return DTRTX. Reads of this register return the value in DTRTX and clear TXfull to 0.Writes of this register update the value in DTRTX and do not change TXfull."]
    #[inline(always)]
    pub fn dbgdtrtx_el0(&self) -> DbgdtrtxEl0R {
        DbgdtrtxEl0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Return DTRTX. Reads of this register return the value in DTRTX and clear TXfull to 0.Writes of this register update the value in DTRTX and do not change TXfull."]
    #[inline(always)]
    #[must_use]
    pub fn dbgdtrtx_el0(&mut self) -> DbgdtrtxEl0W<ApbaddrDbgCpu0DbgdtrtxEl0Spec> {
        DbgdtrtxEl0W::new(self, 0)
    }
}
#[doc = "Debug Data Transfer Register Transmit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgdtrtx_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgdtrtx_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0DbgdtrtxEl0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0DbgdtrtxEl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_dbgdtrtx_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0DbgdtrtxEl0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_dbgdtrtx_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0DbgdtrtxEl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_DBGDTRTX_EL0 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0DbgdtrtxEl0Spec {
    const RESET_VALUE: u32 = 0;
}
