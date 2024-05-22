#[doc = "Register `CPSW_NUSS_VBUSP_SGMII_IDVER_REG` reader"]
pub type R = crate::R<CpswNussVbuspSgmiiIdverRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SGMII_IDVER_REG` writer"]
pub type W = crate::W<CpswNussVbuspSgmiiIdverRegSpec>;
#[doc = "Field `MINOR_VER` reader - 7:0\\]
Minor version value"]
pub type MinorVerR = crate::FieldReader;
#[doc = "Field `MINOR_VER` writer - 7:0\\]
Minor version value"]
pub type MinorVerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAJOR_VER` reader - 10:8\\]
Major version value"]
pub type MajorVerR = crate::FieldReader;
#[doc = "Field `MAJOR_VER` writer - 10:8\\]
Major version value"]
pub type MajorVerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VER` reader - 15:11\\]
RTL version value"]
pub type RtlVerR = crate::FieldReader;
#[doc = "Field `RTL_VER` writer - 15:11\\]
RTL version value"]
pub type RtlVerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TX_IDENT` reader - 31:16\\]
MODULE value"]
pub type TxIdentR = crate::FieldReader<u16>;
#[doc = "Field `TX_IDENT` writer - 31:16\\]
MODULE value"]
pub type TxIdentW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minor version value"]
    #[inline(always)]
    pub fn minor_ver(&self) -> MinorVerR {
        MinorVerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version value"]
    #[inline(always)]
    pub fn major_ver(&self) -> MajorVerR {
        MajorVerR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version value"]
    #[inline(always)]
    pub fn rtl_ver(&self) -> RtlVerR {
        RtlVerR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
MODULE value"]
    #[inline(always)]
    pub fn tx_ident(&self) -> TxIdentR {
        TxIdentR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minor version value"]
    #[inline(always)]
    #[must_use]
    pub fn minor_ver(&mut self) -> MinorVerW<CpswNussVbuspSgmiiIdverRegSpec> {
        MinorVerW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version value"]
    #[inline(always)]
    #[must_use]
    pub fn major_ver(&mut self) -> MajorVerW<CpswNussVbuspSgmiiIdverRegSpec> {
        MajorVerW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version value"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_ver(&mut self) -> RtlVerW<CpswNussVbuspSgmiiIdverRegSpec> {
        RtlVerW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
MODULE value"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ident(&mut self) -> TxIdentW<CpswNussVbuspSgmiiIdverRegSpec> {
        TxIdentW::new(self, 16)
    }
}
#[doc = "SGMII IDVER register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_sgmii_idver_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_sgmii_idver_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSgmiiIdverRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSgmiiIdverRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_sgmii_idver_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSgmiiIdverRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_sgmii_idver_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSgmiiIdverRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SGMII_IDVER_REG to value 0x1102"]
impl crate::Resettable for CpswNussVbuspSgmiiIdverRegSpec {
    const RESET_VALUE: u32 = 0x1102;
}
