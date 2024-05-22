#[doc = "Register `APBADDR_ETM_CPU1_TRCVISSCTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcvissctlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCVISSCTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcvissctlrSpec>;
#[doc = "Field `START` reader - 15:0\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of starting trace. Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n is not selected as a start resource. 1 The single address comparator n is selected as a start resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type StartR = crate::FieldReader<u16>;
#[doc = "Field `START` writer - 15:0\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of starting trace. Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n is not selected as a start resource. 1 The single address comparator n is selected as a start resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STOP` reader - 31:16\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of stopping trace. Each bit represents a single address comparator, so bit\\[m\\]
controls the selection of single address comparator m-16. If bit\\[m\\]
is: 0 The single address comparator m-16 is not selected as a stop resource. 1 The single address comparator m-16 is selected as a stop resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type StopR = crate::FieldReader<u16>;
#[doc = "Field `STOP` writer - 31:16\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of stopping trace. Each bit represents a single address comparator, so bit\\[m\\]
controls the selection of single address comparator m-16. If bit\\[m\\]
is: 0 The single address comparator m-16 is not selected as a stop resource. 1 The single address comparator m-16 is selected as a stop resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of starting trace. Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n is not selected as a start resource. 1 The single address comparator n is selected as a start resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of stopping trace. Each bit represents a single address comparator, so bit\\[m\\]
controls the selection of single address comparator m-16. If bit\\[m\\]
is: 0 The single address comparator m-16 is not selected as a stop resource. 1 The single address comparator m-16 is selected as a stop resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of starting trace. Each bit represents a single address comparator, so bit\\[n\\]
controls the selection of single address comparator n. If bit\\[n\\]
is: 0 The single address comparator n is not selected as a start resource. 1 The single address comparator n is selected as a start resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<ApbaddrEtmCpu1TrcvissctlrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Selects which single address comparators are in use with ViewInst start-stop control, for the purpose of stopping trace. Each bit represents a single address comparator, so bit\\[m\\]
controls the selection of single address comparator m-16. If bit\\[m\\]
is: 0 The single address comparator m-16 is not selected as a stop resource. 1 The single address comparator m-16 is selected as a stop resource. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of 2 x TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<ApbaddrEtmCpu1TrcvissctlrSpec> {
        StopW::new(self, 16)
    }
}
#[doc = "ViewInst Start-Stop Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcvissctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcvissctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcvissctlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcvissctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcvissctlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcvissctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcvissctlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcvissctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCVISSCTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcvissctlrSpec {
    const RESET_VALUE: u32 = 0;
}
