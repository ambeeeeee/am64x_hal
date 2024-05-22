#[doc = "Register `APBADDR_ETM_CPU1_TRCSSCCR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcssccr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCSSCCR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcssccr0Spec>;
#[doc = "Field `SAC` reader - 15:0\\]
Selects one or more single address comparators for single-shot control.Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n, is not selected for single-shot control. 1 The single address comparator n, is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type SacR = crate::FieldReader<u16>;
#[doc = "Field `SAC` writer - 15:0\\]
Selects one or more single address comparators for single-shot control.Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n, is not selected for single-shot control. 1 The single address comparator n, is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type SacW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ARC` reader - 23:16\\]
Selects one or more address range comparators for single-shot control.Each bit represents an address range comparator pair, so bit\\[n-16\\]
controls the selection of address range comparator pair n-16. If bit\\[n-16\\]
is: 0 The address range comparator pair n-16 is not selected for single-shot control. 1 The address range comparator pair n-16 is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type ArcR = crate::FieldReader;
#[doc = "Field `ARC` writer - 23:16\\]
Selects one or more address range comparators for single-shot control.Each bit represents an address range comparator pair, so bit\\[n-16\\]
controls the selection of address range comparator pair n-16. If bit\\[n-16\\]
is: 0 The address range comparator pair n-16 is not selected for single-shot control. 1 The address range comparator pair n-16 is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type ArcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RST` reader - 24:24\\]
Controls whether the single-shot comparator resource is reset when it fires. 0 When the single-shot comparator resource fires, it is not reset. 1 When the single-shot comparator resource fires, it is reset. This enables the single-shot comparator resource to fire multiple times."]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - 24:24\\]
Controls whether the single-shot comparator resource is reset when it fires. 0 When the single-shot comparator resource fires, it is not reset. 1 When the single-shot comparator resource fires, it is reset. This enables the single-shot comparator resource to fire multiple times."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCSSCCR0_31_25` reader - 31:25\\]
Reserved, RES0."]
pub type Res0Trcssccr0_31_25R = crate::FieldReader;
#[doc = "Field `RES0_TRCSSCCR0_31_25` writer - 31:25\\]
Reserved, RES0."]
pub type Res0Trcssccr0_31_25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Selects one or more single address comparators for single-shot control.Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n, is not selected for single-shot control. 1 The single address comparator n, is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn sac(&self) -> SacR {
        SacR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Selects one or more address range comparators for single-shot control.Each bit represents an address range comparator pair, so bit\\[n-16\\]
controls the selection of address range comparator pair n-16. If bit\\[n-16\\]
is: 0 The address range comparator pair n-16 is not selected for single-shot control. 1 The address range comparator pair n-16 is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn arc(&self) -> ArcR {
        ArcR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls whether the single-shot comparator resource is reset when it fires. 0 When the single-shot comparator resource fires, it is not reset. 1 When the single-shot comparator resource fires, it is reset. This enables the single-shot comparator resource to fire multiple times."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcssccr0_31_25(&self) -> Res0Trcssccr0_31_25R {
        Res0Trcssccr0_31_25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Selects one or more single address comparators for single-shot control.Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n, is not selected for single-shot control. 1 The single address comparator n, is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn sac(&mut self) -> SacW<ApbaddrEtmCpu1Trcssccr0Spec> {
        SacW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Selects one or more address range comparators for single-shot control.Each bit represents an address range comparator pair, so bit\\[n-16\\]
controls the selection of address range comparator pair n-16. If bit\\[n-16\\]
is: 0 The address range comparator pair n-16 is not selected for single-shot control. 1 The address range comparator pair n-16 is selected for single-shot control. The width of this field is IMPLEMENTATION DEFINED. The field contains a number of implemented bits equal to TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn arc(&mut self) -> ArcW<ApbaddrEtmCpu1Trcssccr0Spec> {
        ArcW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls whether the single-shot comparator resource is reset when it fires. 0 When the single-shot comparator resource fires, it is not reset. 1 When the single-shot comparator resource fires, it is reset. This enables the single-shot comparator resource to fire multiple times."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<ApbaddrEtmCpu1Trcssccr0Spec> {
        RstW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcssccr0_31_25(&mut self) -> Res0Trcssccr0_31_25W<ApbaddrEtmCpu1Trcssccr0Spec> {
        Res0Trcssccr0_31_25W::new(self, 25)
    }
}
#[doc = "Single-Shot Comparator Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcssccr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcssccr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcssccr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcssccr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcssccr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcssccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcssccr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcssccr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCSSCCR0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trcssccr0Spec {
    const RESET_VALUE: u32 = 0;
}
