#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR4` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcidr4Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR4` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcidr4Spec>;
#[doc = "Field `NUMACPAIRS` reader - 3:0\\]
Indicates the number of address comparator pairs that are available for tracing. The permitted values are: 0000 No address comparator pairs are available. 0001 The implementation has one address comparator pair. 0010 The implementation has two address comparator pairs. and so on up to 0b1000, which indicates that the implementation has eight address comparator pairs.All other values are reserved."]
pub type NumacpairsR = crate::FieldReader;
#[doc = "Field `NUMACPAIRS` writer - 3:0\\]
Indicates the number of address comparator pairs that are available for tracing. The permitted values are: 0000 No address comparator pairs are available. 0001 The implementation has one address comparator pair. 0010 The implementation has two address comparator pairs. and so on up to 0b1000, which indicates that the implementation has eight address comparator pairs.All other values are reserved."]
pub type NumacpairsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUMDVC` reader - 7:4\\]
Indicates the number of data value comparators that are available for tracing. The permitted values are: 0000 No data value comparators are available. 0001 The implementation has one data value comparator. 0010 The implementation has two data value comparators. and so on up to 0b1000, which indicates that the implementation has eight data value comparators.All other values are reserved."]
pub type NumdvcR = crate::FieldReader;
#[doc = "Field `NUMDVC` writer - 7:4\\]
Indicates the number of data value comparators that are available for tracing. The permitted values are: 0000 No data value comparators are available. 0001 The implementation has one data value comparator. 0010 The implementation has two data value comparators. and so on up to 0b1000, which indicates that the implementation has eight data value comparators.All other values are reserved."]
pub type NumdvcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUPPDAC` reader - 8:8\\]
Indicates if the implementation can support data address comparisons: 0 The implementation does not support data address comparisons. 1 The implementation can support data address comparisons"]
pub type SuppdacR = crate::BitReader;
#[doc = "Field `SUPPDAC` writer - 8:8\\]
Indicates if the implementation can support data address comparisons: 0 The implementation does not support data address comparisons. 1 The implementation can support data address comparisons"]
pub type SuppdacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCIDR4_11_9` reader - 11:9\\]
Reserved, RES0."]
pub type Res0Trcidr4_11_9R = crate::FieldReader;
#[doc = "Field `RES0_TRCIDR4_11_9` writer - 11:9\\]
Reserved, RES0."]
pub type Res0Trcidr4_11_9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NUMPC` reader - 15:12\\]
Indicates the number of processor comparator inputs that are available for tracing. The permitted values are: 0000 No processor comparator inputs are available. 0001 The implementation has one processor comparator input. 0010 The implementation has two processor comparator inputs. and so on up to 0b1000, which indicates that the implementation has eight processor comparator inputs.All other values are reserved."]
pub type NumpcR = crate::FieldReader;
#[doc = "Field `NUMPC` writer - 15:12\\]
Indicates the number of processor comparator inputs that are available for tracing. The permitted values are: 0000 No processor comparator inputs are available. 0001 The implementation has one processor comparator input. 0010 The implementation has two processor comparator inputs. and so on up to 0b1000, which indicates that the implementation has eight processor comparator inputs.All other values are reserved."]
pub type NumpcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUMRSPAIR` reader - 19:16\\]
Indicates the number of resource selection pairs that are available for tracing. The permitted values are: 0000 The implementation has one resource selection pair. 0001 The implementation has two resource selection pairs. 0010 The implementation has three resource selection pairs. and so on up to 0b1111, which indicates that the implementation has 16 resource selection pairs.Implementations always have at least one resource selection pair so that they can support the FALSE and TRUE resource selectors, 0 and 1."]
pub type NumrspairR = crate::FieldReader;
#[doc = "Field `NUMRSPAIR` writer - 19:16\\]
Indicates the number of resource selection pairs that are available for tracing. The permitted values are: 0000 The implementation has one resource selection pair. 0001 The implementation has two resource selection pairs. 0010 The implementation has three resource selection pairs. and so on up to 0b1111, which indicates that the implementation has 16 resource selection pairs.Implementations always have at least one resource selection pair so that they can support the FALSE and TRUE resource selectors, 0 and 1."]
pub type NumrspairW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUMSSCC` reader - 23:20\\]
Indicates the number of single-shot comparator controls that are available for tracing. The permitted values are: 0000 No single-shot comparator controls are available. 0001 The implementation has one single-shot comparator control. 0010 The implementation has two single-shot comparator controls. and so on up to 0b1000, which indicates that the implementation has eight single-shot comparator controls.All other values are reserved."]
pub type NumssccR = crate::FieldReader;
#[doc = "Field `NUMSSCC` writer - 23:20\\]
Indicates the number of single-shot comparator controls that are available for tracing. The permitted values are: 0000 No single-shot comparator controls are available. 0001 The implementation has one single-shot comparator control. 0010 The implementation has two single-shot comparator controls. and so on up to 0b1000, which indicates that the implementation has eight single-shot comparator controls.All other values are reserved."]
pub type NumssccW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUMCIDC` reader - 27:24\\]
Indicates the number of Context ID comparators that are available for tracing. The permitted values are: 0000 No Context ID comparators are available. 0001 The implementation has one Context ID comparator. 0010 The implementation has two Context ID comparators. and so on up to 0b1000, which indicates that the implementation has eight Context ID comparators.All other values are reserved."]
pub type NumcidcR = crate::FieldReader;
#[doc = "Field `NUMCIDC` writer - 27:24\\]
Indicates the number of Context ID comparators that are available for tracing. The permitted values are: 0000 No Context ID comparators are available. 0001 The implementation has one Context ID comparator. 0010 The implementation has two Context ID comparators. and so on up to 0b1000, which indicates that the implementation has eight Context ID comparators.All other values are reserved."]
pub type NumcidcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUMVMIDC` reader - 31:28\\]
Indicates the number of VMID comparators that are available for tracing. The permitted values are: 0000 No VMID comparators are available. 0001 The implementation has one VMID comparator. 0010 The implementation has two VMID comparators. and so on up to 0b1000, which indicates that the implementation has eight VMID comparators.All other values are reserved."]
pub type NumvmidcR = crate::FieldReader;
#[doc = "Field `NUMVMIDC` writer - 31:28\\]
Indicates the number of VMID comparators that are available for tracing. The permitted values are: 0000 No VMID comparators are available. 0001 The implementation has one VMID comparator. 0010 The implementation has two VMID comparators. and so on up to 0b1000, which indicates that the implementation has eight VMID comparators.All other values are reserved."]
pub type NumvmidcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the number of address comparator pairs that are available for tracing. The permitted values are: 0000 No address comparator pairs are available. 0001 The implementation has one address comparator pair. 0010 The implementation has two address comparator pairs. and so on up to 0b1000, which indicates that the implementation has eight address comparator pairs.All other values are reserved."]
    #[inline(always)]
    pub fn numacpairs(&self) -> NumacpairsR {
        NumacpairsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the number of data value comparators that are available for tracing. The permitted values are: 0000 No data value comparators are available. 0001 The implementation has one data value comparator. 0010 The implementation has two data value comparators. and so on up to 0b1000, which indicates that the implementation has eight data value comparators.All other values are reserved."]
    #[inline(always)]
    pub fn numdvc(&self) -> NumdvcR {
        NumdvcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates if the implementation can support data address comparisons: 0 The implementation does not support data address comparisons. 1 The implementation can support data address comparisons"]
    #[inline(always)]
    pub fn suppdac(&self) -> SuppdacR {
        SuppdacR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr4_11_9(&self) -> Res0Trcidr4_11_9R {
        Res0Trcidr4_11_9R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates the number of processor comparator inputs that are available for tracing. The permitted values are: 0000 No processor comparator inputs are available. 0001 The implementation has one processor comparator input. 0010 The implementation has two processor comparator inputs. and so on up to 0b1000, which indicates that the implementation has eight processor comparator inputs.All other values are reserved."]
    #[inline(always)]
    pub fn numpc(&self) -> NumpcR {
        NumpcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the number of resource selection pairs that are available for tracing. The permitted values are: 0000 The implementation has one resource selection pair. 0001 The implementation has two resource selection pairs. 0010 The implementation has three resource selection pairs. and so on up to 0b1111, which indicates that the implementation has 16 resource selection pairs.Implementations always have at least one resource selection pair so that they can support the FALSE and TRUE resource selectors, 0 and 1."]
    #[inline(always)]
    pub fn numrspair(&self) -> NumrspairR {
        NumrspairR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the number of single-shot comparator controls that are available for tracing. The permitted values are: 0000 No single-shot comparator controls are available. 0001 The implementation has one single-shot comparator control. 0010 The implementation has two single-shot comparator controls. and so on up to 0b1000, which indicates that the implementation has eight single-shot comparator controls.All other values are reserved."]
    #[inline(always)]
    pub fn numsscc(&self) -> NumssccR {
        NumssccR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the number of Context ID comparators that are available for tracing. The permitted values are: 0000 No Context ID comparators are available. 0001 The implementation has one Context ID comparator. 0010 The implementation has two Context ID comparators. and so on up to 0b1000, which indicates that the implementation has eight Context ID comparators.All other values are reserved."]
    #[inline(always)]
    pub fn numcidc(&self) -> NumcidcR {
        NumcidcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the number of VMID comparators that are available for tracing. The permitted values are: 0000 No VMID comparators are available. 0001 The implementation has one VMID comparator. 0010 The implementation has two VMID comparators. and so on up to 0b1000, which indicates that the implementation has eight VMID comparators.All other values are reserved."]
    #[inline(always)]
    pub fn numvmidc(&self) -> NumvmidcR {
        NumvmidcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the number of address comparator pairs that are available for tracing. The permitted values are: 0000 No address comparator pairs are available. 0001 The implementation has one address comparator pair. 0010 The implementation has two address comparator pairs. and so on up to 0b1000, which indicates that the implementation has eight address comparator pairs.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numacpairs(&mut self) -> NumacpairsW<ApbaddrEtmCpu0Trcidr4Spec> {
        NumacpairsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the number of data value comparators that are available for tracing. The permitted values are: 0000 No data value comparators are available. 0001 The implementation has one data value comparator. 0010 The implementation has two data value comparators. and so on up to 0b1000, which indicates that the implementation has eight data value comparators.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numdvc(&mut self) -> NumdvcW<ApbaddrEtmCpu0Trcidr4Spec> {
        NumdvcW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates if the implementation can support data address comparisons: 0 The implementation does not support data address comparisons. 1 The implementation can support data address comparisons"]
    #[inline(always)]
    #[must_use]
    pub fn suppdac(&mut self) -> SuppdacW<ApbaddrEtmCpu0Trcidr4Spec> {
        SuppdacW::new(self, 8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr4_11_9(&mut self) -> Res0Trcidr4_11_9W<ApbaddrEtmCpu0Trcidr4Spec> {
        Res0Trcidr4_11_9W::new(self, 9)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates the number of processor comparator inputs that are available for tracing. The permitted values are: 0000 No processor comparator inputs are available. 0001 The implementation has one processor comparator input. 0010 The implementation has two processor comparator inputs. and so on up to 0b1000, which indicates that the implementation has eight processor comparator inputs.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numpc(&mut self) -> NumpcW<ApbaddrEtmCpu0Trcidr4Spec> {
        NumpcW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the number of resource selection pairs that are available for tracing. The permitted values are: 0000 The implementation has one resource selection pair. 0001 The implementation has two resource selection pairs. 0010 The implementation has three resource selection pairs. and so on up to 0b1111, which indicates that the implementation has 16 resource selection pairs.Implementations always have at least one resource selection pair so that they can support the FALSE and TRUE resource selectors, 0 and 1."]
    #[inline(always)]
    #[must_use]
    pub fn numrspair(&mut self) -> NumrspairW<ApbaddrEtmCpu0Trcidr4Spec> {
        NumrspairW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the number of single-shot comparator controls that are available for tracing. The permitted values are: 0000 No single-shot comparator controls are available. 0001 The implementation has one single-shot comparator control. 0010 The implementation has two single-shot comparator controls. and so on up to 0b1000, which indicates that the implementation has eight single-shot comparator controls.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numsscc(&mut self) -> NumssccW<ApbaddrEtmCpu0Trcidr4Spec> {
        NumssccW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the number of Context ID comparators that are available for tracing. The permitted values are: 0000 No Context ID comparators are available. 0001 The implementation has one Context ID comparator. 0010 The implementation has two Context ID comparators. and so on up to 0b1000, which indicates that the implementation has eight Context ID comparators.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numcidc(&mut self) -> NumcidcW<ApbaddrEtmCpu0Trcidr4Spec> {
        NumcidcW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the number of VMID comparators that are available for tracing. The permitted values are: 0000 No VMID comparators are available. 0001 The implementation has one VMID comparator. 0010 The implementation has two VMID comparators. and so on up to 0b1000, which indicates that the implementation has eight VMID comparators.All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numvmidc(&mut self) -> NumvmidcW<ApbaddrEtmCpu0Trcidr4Spec> {
        NumvmidcW::new(self, 28)
    }
}
#[doc = "ID Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcidr4Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcidr4::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcidr4Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcidr4::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcidr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCIDR4 to value 0x1117_0004"]
impl crate::Resettable for ApbaddrEtmCpu0Trcidr4Spec {
    const RESET_VALUE: u32 = 0x1117_0004;
}
