#[doc = "Register `PKTDMA_CRED_CRED` reader"]
pub type R = crate::R<PktdmaCredCredSpec>;
#[doc = "Register `PKTDMA_CRED_CRED` writer"]
pub type W = crate::W<PktdmaCredCredSpec>;
#[doc = "Field `PRIVID` reader - 23:16\\]
Privelege ID attribute"]
pub type PrividR = crate::FieldReader;
#[doc = "Field `PRIVID` writer - 23:16\\]
Privelege ID attribute"]
pub type PrividW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRIV` reader - 25:24\\]
Privelege attribute"]
pub type PrivR = crate::FieldReader;
#[doc = "Field `PRIV` writer - 25:24\\]
Privelege attribute"]
pub type PrivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SECURE` reader - 26:26\\]
Secure attribute"]
pub type SecureR = crate::BitReader;
#[doc = "Field `SECURE` writer - 26:26\\]
Secure attribute"]
pub type SecureW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHK_SECURE` reader - 31:31\\]
Check secure control bit"]
pub type ChkSecureR = crate::BitReader;
#[doc = "Field `CHK_SECURE` writer - 31:31\\]
Check secure control bit"]
pub type ChkSecureW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Privelege ID attribute"]
    #[inline(always)]
    pub fn privid(&self) -> PrividR {
        PrividR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Privelege attribute"]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
Secure attribute"]
    #[inline(always)]
    pub fn secure(&self) -> SecureR {
        SecureR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Check secure control bit"]
    #[inline(always)]
    pub fn chk_secure(&self) -> ChkSecureR {
        ChkSecureR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Privelege ID attribute"]
    #[inline(always)]
    #[must_use]
    pub fn privid(&mut self) -> PrividW<PktdmaCredCredSpec> {
        PrividW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Privelege attribute"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PrivW<PktdmaCredCredSpec> {
        PrivW::new(self, 24)
    }
    #[doc = "Bit 26 - 26:26\\]
Secure attribute"]
    #[inline(always)]
    #[must_use]
    pub fn secure(&mut self) -> SecureW<PktdmaCredCredSpec> {
        SecureW::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
Check secure control bit"]
    #[inline(always)]
    #[must_use]
    pub fn chk_secure(&mut self) -> ChkSecureW<PktdmaCredCredSpec> {
        ChkSecureW::new(self, 31)
    }
}
#[doc = "The Credentials Register provides credentials to be used when performing memory accesses using this flow.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_cred_cred::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_cred_cred::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaCredCredSpec;
impl crate::RegisterSpec for PktdmaCredCredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_cred_cred::R`](R) reader structure"]
impl crate::Readable for PktdmaCredCredSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_cred_cred::W`](W) writer structure"]
impl crate::Writable for PktdmaCredCredSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_CRED_CRED to value 0"]
impl crate::Resettable for PktdmaCredCredSpec {
    const RESET_VALUE: u32 = 0;
}
