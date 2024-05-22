#[doc = "Register `APBADDR_ETM_CPU0_TRCCNTVR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trccntvr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCCNTVR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trccntvr0Spec>;
#[doc = "Field `VALUE_N` reader - 15:0\\]
Contains the count value of counter &lt;n>."]
pub type ValueNR = crate::FieldReader<u16>;
#[doc = "Field `VALUE_N` writer - 15:0\\]
Contains the count value of counter &lt;n>."]
pub type ValueNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RES0_TRCCNTVR0_31_16` reader - 31:16\\]
Reserved, RES0."]
pub type Res0Trccntvr0_31_16R = crate::FieldReader<u16>;
#[doc = "Field `RES0_TRCCNTVR0_31_16` writer - 31:16\\]
Reserved, RES0."]
pub type Res0Trccntvr0_31_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Contains the count value of counter &lt;n>."]
    #[inline(always)]
    pub fn value_n(&self) -> ValueNR {
        ValueNR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trccntvr0_31_16(&self) -> Res0Trccntvr0_31_16R {
        Res0Trccntvr0_31_16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Contains the count value of counter &lt;n>."]
    #[inline(always)]
    #[must_use]
    pub fn value_n(&mut self) -> ValueNW<ApbaddrEtmCpu0Trccntvr0Spec> {
        ValueNW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trccntvr0_31_16(&mut self) -> Res0Trccntvr0_31_16W<ApbaddrEtmCpu0Trccntvr0Spec> {
        Res0Trccntvr0_31_16W::new(self, 16)
    }
}
#[doc = "Counter Value Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntvr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntvr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trccntvr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trccntvr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trccntvr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trccntvr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trccntvr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trccntvr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCCNTVR0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trccntvr0Spec {
    const RESET_VALUE: u32 = 0;
}
