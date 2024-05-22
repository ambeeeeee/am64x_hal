#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR2` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcidr2Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR2` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcidr2Spec>;
#[doc = "Field `IASIZE` reader - 4:0\\]
Indicates the instruction address size. The permitted values are: 00100 Maximum of 32-bit address size. 01000 Maximum of 64-bit address size. All other values are reserved."]
pub type IasizeR = crate::FieldReader;
#[doc = "Field `IASIZE` writer - 4:0\\]
Indicates the instruction address size. The permitted values are: 00100 Maximum of 32-bit address size. 01000 Maximum of 64-bit address size. All other values are reserved."]
pub type IasizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CIDSIZE` reader - 9:5\\]
Indicates the Context ID size. The permitted values are: 00000 Context ID tracing is not supported. 00100 Maximum of 32-bit Context ID size, so TRCCONFIGR.CID is supported. All other values are reserved."]
pub type CidsizeR = crate::FieldReader;
#[doc = "Field `CIDSIZE` writer - 9:5\\]
Indicates the Context ID size. The permitted values are: 00000 Context ID tracing is not supported. 00100 Maximum of 32-bit Context ID size, so TRCCONFIGR.CID is supported. All other values are reserved."]
pub type CidsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VMIDSIZE` reader - 14:10\\]
Indicates the VMID size. The permitted values are: 00000 VMID tracing is not supported. 00001 Maximum of 8-bit VMID size, so TRCCONFIGR.VMID is supported. All other values are reserved."]
pub type VmidsizeR = crate::FieldReader;
#[doc = "Field `VMIDSIZE` writer - 14:10\\]
Indicates the VMID size. The permitted values are: 00000 VMID tracing is not supported. 00001 Maximum of 8-bit VMID size, so TRCCONFIGR.VMID is supported. All other values are reserved."]
pub type VmidsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DASIZE` reader - 19:15\\]
Indicates the data address size in bytes. The permitted values are: 00000 Data address tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data address size. 01000 Maximum of 64-bit data address size. All other values are reserved."]
pub type DasizeR = crate::FieldReader;
#[doc = "Field `DASIZE` writer - 19:15\\]
Indicates the data address size in bytes. The permitted values are: 00000 Data address tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data address size. 01000 Maximum of 64-bit data address size. All other values are reserved."]
pub type DasizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DVSIZE` reader - 24:20\\]
Indicates the data value size in bytes. The permitted values are: 00000 Data value tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data value size. 01000 Maximum of 64-bit data value size. All other values are reserved."]
pub type DvsizeR = crate::FieldReader;
#[doc = "Field `DVSIZE` writer - 24:20\\]
Indicates the data value size in bytes. The permitted values are: 00000 Data value tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data value size. 01000 Maximum of 64-bit data value size. All other values are reserved."]
pub type DvsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCSIZE` reader - 28:25\\]
Indicates the size of the cycle counter in bits minus 12. 0000 The cycle counter is 12 bits in length. 0001 The cycle counter is 13 bits in length. and so on up to 0b1000, indicating the cycle counter is 20 bits in length.All other values are reserved.If cycle counting is not implemented, as indicated by TRCIDR0.TRCCCI, this field is 0b0000."]
pub type CcsizeR = crate::FieldReader;
#[doc = "Field `CCSIZE` writer - 28:25\\]
Indicates the size of the cycle counter in bits minus 12. 0000 The cycle counter is 12 bits in length. 0001 The cycle counter is 13 bits in length. and so on up to 0b1000, indicating the cycle counter is 20 bits in length.All other values are reserved.If cycle counting is not implemented, as indicated by TRCIDR0.TRCCCI, this field is 0b0000."]
pub type CcsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCIDR2_31_29` reader - 31:29\\]
Reserved, RES0."]
pub type Res0Trcidr2_31_29R = crate::FieldReader;
#[doc = "Field `RES0_TRCIDR2_31_29` writer - 31:29\\]
Reserved, RES0."]
pub type Res0Trcidr2_31_29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Indicates the instruction address size. The permitted values are: 00100 Maximum of 32-bit address size. 01000 Maximum of 64-bit address size. All other values are reserved."]
    #[inline(always)]
    pub fn iasize(&self) -> IasizeR {
        IasizeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Indicates the Context ID size. The permitted values are: 00000 Context ID tracing is not supported. 00100 Maximum of 32-bit Context ID size, so TRCCONFIGR.CID is supported. All other values are reserved."]
    #[inline(always)]
    pub fn cidsize(&self) -> CidsizeR {
        CidsizeR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Indicates the VMID size. The permitted values are: 00000 VMID tracing is not supported. 00001 Maximum of 8-bit VMID size, so TRCCONFIGR.VMID is supported. All other values are reserved."]
    #[inline(always)]
    pub fn vmidsize(&self) -> VmidsizeR {
        VmidsizeR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Indicates the data address size in bytes. The permitted values are: 00000 Data address tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data address size. 01000 Maximum of 64-bit data address size. All other values are reserved."]
    #[inline(always)]
    pub fn dasize(&self) -> DasizeR {
        DasizeR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 24:20\\]
Indicates the data value size in bytes. The permitted values are: 00000 Data value tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data value size. 01000 Maximum of 64-bit data value size. All other values are reserved."]
    #[inline(always)]
    pub fn dvsize(&self) -> DvsizeR {
        DvsizeR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Indicates the size of the cycle counter in bits minus 12. 0000 The cycle counter is 12 bits in length. 0001 The cycle counter is 13 bits in length. and so on up to 0b1000, indicating the cycle counter is 20 bits in length.All other values are reserved.If cycle counting is not implemented, as indicated by TRCIDR0.TRCCCI, this field is 0b0000."]
    #[inline(always)]
    pub fn ccsize(&self) -> CcsizeR {
        CcsizeR::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr2_31_29(&self) -> Res0Trcidr2_31_29R {
        Res0Trcidr2_31_29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Indicates the instruction address size. The permitted values are: 00100 Maximum of 32-bit address size. 01000 Maximum of 64-bit address size. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn iasize(&mut self) -> IasizeW<ApbaddrEtmCpu1Trcidr2Spec> {
        IasizeW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Indicates the Context ID size. The permitted values are: 00000 Context ID tracing is not supported. 00100 Maximum of 32-bit Context ID size, so TRCCONFIGR.CID is supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn cidsize(&mut self) -> CidsizeW<ApbaddrEtmCpu1Trcidr2Spec> {
        CidsizeW::new(self, 5)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Indicates the VMID size. The permitted values are: 00000 VMID tracing is not supported. 00001 Maximum of 8-bit VMID size, so TRCCONFIGR.VMID is supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn vmidsize(&mut self) -> VmidsizeW<ApbaddrEtmCpu1Trcidr2Spec> {
        VmidsizeW::new(self, 10)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Indicates the data address size in bytes. The permitted values are: 00000 Data address tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data address size. 01000 Maximum of 64-bit data address size. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn dasize(&mut self) -> DasizeW<ApbaddrEtmCpu1Trcidr2Spec> {
        DasizeW::new(self, 15)
    }
    #[doc = "Bits 20:24 - 24:20\\]
Indicates the data value size in bytes. The permitted values are: 00000 Data value tracing is not supported. Therefore, an implementation must also set TRCIDR0.TRCDATA==0b00. 00100 Maximum of 32-bit data value size. 01000 Maximum of 64-bit data value size. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn dvsize(&mut self) -> DvsizeW<ApbaddrEtmCpu1Trcidr2Spec> {
        DvsizeW::new(self, 20)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Indicates the size of the cycle counter in bits minus 12. 0000 The cycle counter is 12 bits in length. 0001 The cycle counter is 13 bits in length. and so on up to 0b1000, indicating the cycle counter is 20 bits in length.All other values are reserved.If cycle counting is not implemented, as indicated by TRCIDR0.TRCCCI, this field is 0b0000."]
    #[inline(always)]
    #[must_use]
    pub fn ccsize(&mut self) -> CcsizeW<ApbaddrEtmCpu1Trcidr2Spec> {
        CcsizeW::new(self, 25)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr2_31_29(&mut self) -> Res0Trcidr2_31_29W<ApbaddrEtmCpu1Trcidr2Spec> {
        Res0Trcidr2_31_29W::new(self, 29)
    }
}
#[doc = "ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcidr2Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcidr2::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcidr2::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIDR2 to value 0x0488"]
impl crate::Resettable for ApbaddrEtmCpu1Trcidr2Spec {
    const RESET_VALUE: u32 = 0x0488;
}
