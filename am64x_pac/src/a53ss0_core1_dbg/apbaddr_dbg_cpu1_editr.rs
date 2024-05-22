#[doc = "Register `APBADDR_DBG_CPU1_EDITR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EditrSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDITR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EditrSpec>;
#[doc = "Field `EDITR` reader - 31:0\\]
Used in Debug state for passing instructions to the processor for execution"]
pub type EditrR = crate::FieldReader<u32>;
#[doc = "Field `EDITR` writer - 31:0\\]
Used in Debug state for passing instructions to the processor for execution"]
pub type EditrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Used in Debug state for passing instructions to the processor for execution"]
    #[inline(always)]
    pub fn editr(&self) -> EditrR {
        EditrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Used in Debug state for passing instructions to the processor for execution"]
    #[inline(always)]
    #[must_use]
    pub fn editr(&mut self) -> EditrW<ApbaddrDbgCpu1EditrSpec> {
        EditrW::new(self, 0)
    }
}
#[doc = "External Debug Instruction Transfer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_editr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_editr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EditrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EditrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_editr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EditrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_editr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EditrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDITR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1EditrSpec {
    const RESET_VALUE: u32 = 0;
}
