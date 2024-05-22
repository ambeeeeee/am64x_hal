#[doc = "Register `APBADDR_ETM_CPU1_TRCCNTRLDVR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trccntrldvr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCCNTRLDVR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trccntrldvr0Spec>;
#[doc = "Field `VALUE_N` reader - 15:0\\]
Contains the reload value for counter &lt;n>. When a reload event occurs for counter &lt;n> then the trace unit copies the VALUE&lt;n> field into counter &lt;n>."]
pub type ValueNR = crate::FieldReader<u16>;
#[doc = "Field `VALUE_N` writer - 15:0\\]
Contains the reload value for counter &lt;n>. When a reload event occurs for counter &lt;n> then the trace unit copies the VALUE&lt;n> field into counter &lt;n>."]
pub type ValueNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RES0_TRCCNTRLDVR0_31_16` reader - 31:16\\]
Reserved, RES0."]
pub type Res0Trccntrldvr0_31_16R = crate::FieldReader<u16>;
#[doc = "Field `RES0_TRCCNTRLDVR0_31_16` writer - 31:16\\]
Reserved, RES0."]
pub type Res0Trccntrldvr0_31_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Contains the reload value for counter &lt;n>. When a reload event occurs for counter &lt;n> then the trace unit copies the VALUE&lt;n> field into counter &lt;n>."]
    #[inline(always)]
    pub fn value_n(&self) -> ValueNR {
        ValueNR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trccntrldvr0_31_16(&self) -> Res0Trccntrldvr0_31_16R {
        Res0Trccntrldvr0_31_16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Contains the reload value for counter &lt;n>. When a reload event occurs for counter &lt;n> then the trace unit copies the VALUE&lt;n> field into counter &lt;n>."]
    #[inline(always)]
    #[must_use]
    pub fn value_n(&mut self) -> ValueNW<ApbaddrEtmCpu1Trccntrldvr0Spec> {
        ValueNW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trccntrldvr0_31_16(
        &mut self,
    ) -> Res0Trccntrldvr0_31_16W<ApbaddrEtmCpu1Trccntrldvr0Spec> {
        Res0Trccntrldvr0_31_16W::new(self, 16)
    }
}
#[doc = "Counter Reload Value Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccntrldvr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccntrldvr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trccntrldvr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trccntrldvr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trccntrldvr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trccntrldvr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trccntrldvr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trccntrldvr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCCNTRLDVR0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trccntrldvr0Spec {
    const RESET_VALUE: u32 = 0;
}
