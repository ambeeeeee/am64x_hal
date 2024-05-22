#[doc = "Register `APBADDR_CTI_CPU1_CTIAUTHSTATUS` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtiauthstatusSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIAUTHSTATUS` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtiauthstatusSpec>;
#[doc = "Field `NSID` reader - 1:0\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSID."]
pub type NsidR = crate::FieldReader;
#[doc = "Field `NSID` writer - 1:0\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSID."]
pub type NsidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NSNID` reader - 3:2\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SNID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
pub type NsnidR = crate::FieldReader;
#[doc = "Field `NSNID` writer - 3:2\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SNID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
pub type NsnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_CTIAUTHSTATUS_31_4` reader - 31:4\\]
Reserved, RES0."]
pub type Res0Ctiauthstatus31_4R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTIAUTHSTATUS_31_4` writer - 31:4\\]
Reserved, RES0."]
pub type Res0Ctiauthstatus31_4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSID."]
    #[inline(always)]
    pub fn nsid(&self) -> NsidR {
        NsidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SNID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctiauthstatus_31_4(&self) -> Res0Ctiauthstatus31_4R {
        Res0Ctiauthstatus31_4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSID."]
    #[inline(always)]
    #[must_use]
    pub fn nsid(&mut self) -> NsidW<ApbaddrCtiCpu1CtiauthstatusSpec> {
        NsidW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
If EL3 is not implemented and the processor is Secure, holds the same value as DBGAUTHSTATUS_EL1.SNID.Otherwise, holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
    #[inline(always)]
    #[must_use]
    pub fn nsnid(&mut self) -> NsnidW<ApbaddrCtiCpu1CtiauthstatusSpec> {
        NsnidW::new(self, 2)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctiauthstatus_31_4(
        &mut self,
    ) -> Res0Ctiauthstatus31_4W<ApbaddrCtiCpu1CtiauthstatusSpec> {
        Res0Ctiauthstatus31_4W::new(self, 4)
    }
}
#[doc = "CTI Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiauthstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiauthstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtiauthstatusSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtiauthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctiauthstatus::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtiauthstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctiauthstatus::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtiauthstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIAUTHSTATUS to value 0x0a"]
impl crate::Resettable for ApbaddrCtiCpu1CtiauthstatusSpec {
    const RESET_VALUE: u32 = 0x0a;
}
