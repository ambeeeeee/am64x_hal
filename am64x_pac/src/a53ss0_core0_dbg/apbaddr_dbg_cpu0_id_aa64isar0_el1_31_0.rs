#[doc = "Register `APBADDR_DBG_CPU0_ID_AA64ISAR0_EL1_31_0` reader"]
pub type R = crate::R<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_ID_AA64ISAR0_EL1_31_0` writer"]
pub type W = crate::W<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec>;
#[doc = "Field `RES0_ID_AA64ISAR0_EL1_31_0_3_0` reader - 3:0\\]
Reserved, RES0."]
pub type Res0IdAa64isar0El1_31_0_3_0R = crate::FieldReader;
#[doc = "Field `RES0_ID_AA64ISAR0_EL1_31_0_3_0` writer - 3:0\\]
Reserved, RES0."]
pub type Res0IdAa64isar0El1_31_0_3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AES` reader - 7:4\\]
AES instructions in AArch64. Possible values of this field are: 0000 No AES instructions implemented. 0001 AESE, AESD, AESMC, and AESIMC instructions implemented. 0010 As for 0b0001, plus PMULL/PMULL2 instructions operating on 64-bit data quantities."]
pub type AesR = crate::FieldReader;
#[doc = "Field `AES` writer - 7:4\\]
AES instructions in AArch64. Possible values of this field are: 0000 No AES instructions implemented. 0001 AESE, AESD, AESMC, and AESIMC instructions implemented. 0010 As for 0b0001, plus PMULL/PMULL2 instructions operating on 64-bit data quantities."]
pub type AesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SHA1` reader - 11:8\\]
SHA1 instructions in AArch64. Possible values of this field are: 0000 No SHA1 instructions implemented. 0001 SHA1C, SHA1P, SHA1M, SHA1H, SHA1SU0, and SHA1SU1 instructions implemented. All other values are reserved."]
pub type Sha1R = crate::FieldReader;
#[doc = "Field `SHA1` writer - 11:8\\]
SHA1 instructions in AArch64. Possible values of this field are: 0000 No SHA1 instructions implemented. 0001 SHA1C, SHA1P, SHA1M, SHA1H, SHA1SU0, and SHA1SU1 instructions implemented. All other values are reserved."]
pub type Sha1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SHA2` reader - 15:12\\]
SHA2 instructions in AArch64. Possible values of this field are: 0000 No SHA2 instructions implemented. 0001 SHA256H, SHA256H2, SHA256SU0, and SHA256SU1 instructions implemented. All other values are reserved."]
pub type Sha2R = crate::FieldReader;
#[doc = "Field `SHA2` writer - 15:12\\]
SHA2 instructions in AArch64. Possible values of this field are: 0000 No SHA2 instructions implemented. 0001 SHA256H, SHA256H2, SHA256SU0, and SHA256SU1 instructions implemented. All other values are reserved."]
pub type Sha2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRC32` reader - 19:16\\]
CRC32 instructions in AArch64. Possible values of this field are: 0000 No CRC32 instructions implemented. 0001 CRC32B, CRC32H, CRC32W, CRC32X, CRC32CB, CRC32CH, CRC32CW, and CRC32CX instructions implemented. All other values are reserved.This field must have the same value as ID_ISAR5.CRC32. The architecture requires that if CRC32 is supported in one Execution state, it must be supported in both Execution states."]
pub type Crc32R = crate::FieldReader;
#[doc = "Field `CRC32` writer - 19:16\\]
CRC32 instructions in AArch64. Possible values of this field are: 0000 No CRC32 instructions implemented. 0001 CRC32B, CRC32H, CRC32W, CRC32X, CRC32CB, CRC32CH, CRC32CW, and CRC32CX instructions implemented. All other values are reserved.This field must have the same value as ID_ISAR5.CRC32. The architecture requires that if CRC32 is supported in one Execution state, it must be supported in both Execution states."]
pub type Crc32W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_ID_AA64ISAR0_EL1_31_0_31_20` reader - 31:20\\]
Reserved, RES0."]
pub type Res0IdAa64isar0El1_31_0_31_20R = crate::FieldReader<u16>;
#[doc = "Field `RES0_ID_AA64ISAR0_EL1_31_0_31_20` writer - 31:20\\]
Reserved, RES0."]
pub type Res0IdAa64isar0El1_31_0_31_20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_id_aa64isar0_el1_31_0_3_0(&self) -> Res0IdAa64isar0El1_31_0_3_0R {
        Res0IdAa64isar0El1_31_0_3_0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
AES instructions in AArch64. Possible values of this field are: 0000 No AES instructions implemented. 0001 AESE, AESD, AESMC, and AESIMC instructions implemented. 0010 As for 0b0001, plus PMULL/PMULL2 instructions operating on 64-bit data quantities."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
SHA1 instructions in AArch64. Possible values of this field are: 0000 No SHA1 instructions implemented. 0001 SHA1C, SHA1P, SHA1M, SHA1H, SHA1SU0, and SHA1SU1 instructions implemented. All other values are reserved."]
    #[inline(always)]
    pub fn sha1(&self) -> Sha1R {
        Sha1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
SHA2 instructions in AArch64. Possible values of this field are: 0000 No SHA2 instructions implemented. 0001 SHA256H, SHA256H2, SHA256SU0, and SHA256SU1 instructions implemented. All other values are reserved."]
    #[inline(always)]
    pub fn sha2(&self) -> Sha2R {
        Sha2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
CRC32 instructions in AArch64. Possible values of this field are: 0000 No CRC32 instructions implemented. 0001 CRC32B, CRC32H, CRC32W, CRC32X, CRC32CB, CRC32CH, CRC32CW, and CRC32CX instructions implemented. All other values are reserved.This field must have the same value as ID_ISAR5.CRC32. The architecture requires that if CRC32 is supported in one Execution state, it must be supported in both Execution states."]
    #[inline(always)]
    pub fn crc32(&self) -> Crc32R {
        Crc32R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_id_aa64isar0_el1_31_0_31_20(&self) -> Res0IdAa64isar0El1_31_0_31_20R {
        Res0IdAa64isar0El1_31_0_31_20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_id_aa64isar0_el1_31_0_3_0(
        &mut self,
    ) -> Res0IdAa64isar0El1_31_0_3_0W<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec> {
        Res0IdAa64isar0El1_31_0_3_0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
AES instructions in AArch64. Possible values of this field are: 0000 No AES instructions implemented. 0001 AESE, AESD, AESMC, and AESIMC instructions implemented. 0010 As for 0b0001, plus PMULL/PMULL2 instructions operating on 64-bit data quantities."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AesW<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec> {
        AesW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
SHA1 instructions in AArch64. Possible values of this field are: 0000 No SHA1 instructions implemented. 0001 SHA1C, SHA1P, SHA1M, SHA1H, SHA1SU0, and SHA1SU1 instructions implemented. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sha1(&mut self) -> Sha1W<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec> {
        Sha1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
SHA2 instructions in AArch64. Possible values of this field are: 0000 No SHA2 instructions implemented. 0001 SHA256H, SHA256H2, SHA256SU0, and SHA256SU1 instructions implemented. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sha2(&mut self) -> Sha2W<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec> {
        Sha2W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
CRC32 instructions in AArch64. Possible values of this field are: 0000 No CRC32 instructions implemented. 0001 CRC32B, CRC32H, CRC32W, CRC32X, CRC32CB, CRC32CH, CRC32CW, and CRC32CX instructions implemented. All other values are reserved.This field must have the same value as ID_ISAR5.CRC32. The architecture requires that if CRC32 is supported in one Execution state, it must be supported in both Execution states."]
    #[inline(always)]
    #[must_use]
    pub fn crc32(&mut self) -> Crc32W<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec> {
        Crc32W::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_id_aa64isar0_el1_31_0_31_20(
        &mut self,
    ) -> Res0IdAa64isar0El1_31_0_31_20W<ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec> {
        Res0IdAa64isar0El1_31_0_31_20W::new(self, 20)
    }
}
#[doc = "Instruction Set Attribute Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_ID_AA64ISAR0_EL1_31_0 to value 0x0001_1120"]
impl crate::Resettable for ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec {
    const RESET_VALUE: u32 = 0x0001_1120;
}
