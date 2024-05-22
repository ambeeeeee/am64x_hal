#[doc = "Register `MEM_ELM_SYSSTATUS` reader"]
pub type R = crate::R<MemElmSysstatusSpec>;
#[doc = "Register `MEM_ELM_SYSSTATUS` writer"]
pub type W = crate::W<MemElmSysstatusSpec>;
#[doc = "Field `RESETDONE` reader - 0:0\\]
Internal Reset monitoring \\[OCP domain\\]
Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1"]
pub type ResetdoneR = crate::BitReader;
#[doc = "Field `RESETDONE` writer - 0:0\\]
Internal Reset monitoring \\[OCP domain\\]
Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1"]
pub type ResetdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal Reset monitoring \\[OCP domain\\]
Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1"]
    #[inline(always)]
    pub fn resetdone(&self) -> ResetdoneR {
        ResetdoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal Reset monitoring \\[OCP domain\\]
Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1"]
    #[inline(always)]
    #[must_use]
    pub fn resetdone(&mut self) -> ResetdoneW<MemElmSysstatusSpec> {
        ResetdoneW::new(self, 0)
    }
}
#[doc = "Internal Reset monitoring (OCP domain) Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_sysstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_sysstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSysstatusSpec;
impl crate::RegisterSpec for MemElmSysstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_sysstatus::R`](R) reader structure"]
impl crate::Readable for MemElmSysstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_sysstatus::W`](W) writer structure"]
impl crate::Writable for MemElmSysstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYSSTATUS to value 0"]
impl crate::Resettable for MemElmSysstatusSpec {
    const RESET_VALUE: u32 = 0;
}
