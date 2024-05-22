#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR1` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcidr1Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR1` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcidr1Spec>;
#[doc = "Field `REVISION` reader - 3:0\\]
Returns an IMPLEMENTATION DEFINED value that identifies the revision of the trace registers and the OS Save and Restore registers.ARM recommends:That the initial implementation sets REVISION==0x0 and the field then increments for any subsequent implementations. However, it is acceptable to omit some values or use another scheme to identify the revision number.That TRCPIDR2.REVISION==TRCIDR1.REVISION. However, in situations where it is difficult to align these fields, such as with a metal layer fix, then it is acceptable to change the REVISION fields independently."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - 3:0\\]
Returns an IMPLEMENTATION DEFINED value that identifies the revision of the trace registers and the OS Save and Restore registers.ARM recommends:That the initial implementation sets REVISION==0x0 and the field then increments for any subsequent implementations. However, it is acceptable to omit some values or use another scheme to identify the revision number.That TRCPIDR2.REVISION==TRCIDR1.REVISION. However, in situations where it is difficult to align these fields, such as with a metal layer fix, then it is acceptable to change the REVISION fields independently."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCARCHMIN` reader - 7:4\\]
Indicates the minor version of the ETM architecture. The permitted value is: 0 ETMv4 minor version 0. All other values are reserved."]
pub type TrcarchminR = crate::FieldReader;
#[doc = "Field `TRCARCHMIN` writer - 7:4\\]
Indicates the minor version of the ETM architecture. The permitted value is: 0 ETMv4 minor version 0. All other values are reserved."]
pub type TrcarchminW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCARCHMAJ` reader - 11:8\\]
Indicates the major version of the ETM architecture. The permitted value is: 100 ETMv4. All other values are reserved."]
pub type TrcarchmajR = crate::FieldReader;
#[doc = "Field `TRCARCHMAJ` writer - 11:8\\]
Indicates the major version of the ETM architecture. The permitted value is: 100 ETMv4. All other values are reserved."]
pub type TrcarchmajW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES1_TRCIDR1_15_12` reader - 15:12\\]
Reserved, RES1."]
pub type Res1Trcidr1_15_12R = crate::FieldReader;
#[doc = "Field `RES1_TRCIDR1_15_12` writer - 15:12\\]
Reserved, RES1."]
pub type Res1Trcidr1_15_12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCIDR1_23_16` reader - 23:16\\]
Reserved, RES0."]
pub type Res0Trcidr1_23_16R = crate::FieldReader;
#[doc = "Field `RES0_TRCIDR1_23_16` writer - 23:16\\]
Reserved, RES0."]
pub type Res0Trcidr1_23_16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DESIGNER` reader - 31:24\\]
Indicates which company designed the trace unit. The permitted values are: 01000001 ARM Limited. 01000100 Digital Equipment Corporation. 01001101 Motorola, Freescale Semiconductor Inc. 01010001 QUALCOMM Inc. 01010110 Marvell Semiconductor Inc. 01101001 Intel Corporation. All other values are reserved."]
pub type DesignerR = crate::FieldReader;
#[doc = "Field `DESIGNER` writer - 31:24\\]
Indicates which company designed the trace unit. The permitted values are: 01000001 ARM Limited. 01000100 Digital Equipment Corporation. 01001101 Motorola, Freescale Semiconductor Inc. 01010001 QUALCOMM Inc. 01010110 Marvell Semiconductor Inc. 01101001 Intel Corporation. All other values are reserved."]
pub type DesignerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Returns an IMPLEMENTATION DEFINED value that identifies the revision of the trace registers and the OS Save and Restore registers.ARM recommends:That the initial implementation sets REVISION==0x0 and the field then increments for any subsequent implementations. However, it is acceptable to omit some values or use another scheme to identify the revision number.That TRCPIDR2.REVISION==TRCIDR1.REVISION. However, in situations where it is difficult to align these fields, such as with a metal layer fix, then it is acceptable to change the REVISION fields independently."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the minor version of the ETM architecture. The permitted value is: 0 ETMv4 minor version 0. All other values are reserved."]
    #[inline(always)]
    pub fn trcarchmin(&self) -> TrcarchminR {
        TrcarchminR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the major version of the ETM architecture. The permitted value is: 100 ETMv4. All other values are reserved."]
    #[inline(always)]
    pub fn trcarchmaj(&self) -> TrcarchmajR {
        TrcarchmajR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES1."]
    #[inline(always)]
    pub fn res1_trcidr1_15_12(&self) -> Res1Trcidr1_15_12R {
        Res1Trcidr1_15_12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr1_23_16(&self) -> Res0Trcidr1_23_16R {
        Res0Trcidr1_23_16R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Indicates which company designed the trace unit. The permitted values are: 01000001 ARM Limited. 01000100 Digital Equipment Corporation. 01001101 Motorola, Freescale Semiconductor Inc. 01010001 QUALCOMM Inc. 01010110 Marvell Semiconductor Inc. 01101001 Intel Corporation. All other values are reserved."]
    #[inline(always)]
    pub fn designer(&self) -> DesignerR {
        DesignerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Returns an IMPLEMENTATION DEFINED value that identifies the revision of the trace registers and the OS Save and Restore registers.ARM recommends:That the initial implementation sets REVISION==0x0 and the field then increments for any subsequent implementations. However, it is acceptable to omit some values or use another scheme to identify the revision number.That TRCPIDR2.REVISION==TRCIDR1.REVISION. However, in situations where it is difficult to align these fields, such as with a metal layer fix, then it is acceptable to change the REVISION fields independently."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<ApbaddrEtmCpu1Trcidr1Spec> {
        RevisionW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the minor version of the ETM architecture. The permitted value is: 0 ETMv4 minor version 0. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn trcarchmin(&mut self) -> TrcarchminW<ApbaddrEtmCpu1Trcidr1Spec> {
        TrcarchminW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the major version of the ETM architecture. The permitted value is: 100 ETMv4. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn trcarchmaj(&mut self) -> TrcarchmajW<ApbaddrEtmCpu1Trcidr1Spec> {
        TrcarchmajW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES1."]
    #[inline(always)]
    #[must_use]
    pub fn res1_trcidr1_15_12(&mut self) -> Res1Trcidr1_15_12W<ApbaddrEtmCpu1Trcidr1Spec> {
        Res1Trcidr1_15_12W::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr1_23_16(&mut self) -> Res0Trcidr1_23_16W<ApbaddrEtmCpu1Trcidr1Spec> {
        Res0Trcidr1_23_16W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Indicates which company designed the trace unit. The permitted values are: 01000001 ARM Limited. 01000100 Digital Equipment Corporation. 01001101 Motorola, Freescale Semiconductor Inc. 01010001 QUALCOMM Inc. 01010110 Marvell Semiconductor Inc. 01101001 Intel Corporation. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn designer(&mut self) -> DesignerW<ApbaddrEtmCpu1Trcidr1Spec> {
        DesignerW::new(self, 24)
    }
}
#[doc = "ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcidr1Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcidr1::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcidr1::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIDR1 to value 0x6501_5404"]
impl crate::Resettable for ApbaddrEtmCpu1Trcidr1Spec {
    const RESET_VALUE: u32 = 0x6501_5404;
}
