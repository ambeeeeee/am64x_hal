#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_version` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqVersionSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_version` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqVersionSpec>;
#[doc = "Field `EMMC_VERSION_SUFFIX` reader - 3:0\\]
eMMC Version Suffix \\[2nd digit right of decimal point\\], in BCD format"]
pub type EmmcVersionSuffixR = crate::FieldReader;
#[doc = "Field `EMMC_VERSION_SUFFIX` writer - 3:0\\]
eMMC Version Suffix \\[2nd digit right of decimal point\\], in BCD format"]
pub type EmmcVersionSuffixW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EMMC_MINOR_VER_NUM` reader - 7:4\\]
eMMC Minor Version Number \\[digit right of decimal point\\], in BCD format"]
pub type EmmcMinorVerNumR = crate::FieldReader;
#[doc = "Field `EMMC_MINOR_VER_NUM` writer - 7:4\\]
eMMC Minor Version Number \\[digit right of decimal point\\], in BCD format"]
pub type EmmcMinorVerNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EMMC_MAJOR_VER_NUM` reader - 11:8\\]
eMMC Major Version Number \\[digit left of decimal point\\],in BCD format"]
pub type EmmcMajorVerNumR = crate::FieldReader;
#[doc = "Field `EMMC_MAJOR_VER_NUM` writer - 11:8\\]
eMMC Major Version Number \\[digit left of decimal point\\],in BCD format"]
pub type EmmcMajorVerNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
eMMC Version Suffix \\[2nd digit right of decimal point\\], in BCD format"]
    #[inline(always)]
    pub fn emmc_version_suffix(&self) -> EmmcVersionSuffixR {
        EmmcVersionSuffixR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
eMMC Minor Version Number \\[digit right of decimal point\\], in BCD format"]
    #[inline(always)]
    pub fn emmc_minor_ver_num(&self) -> EmmcMinorVerNumR {
        EmmcMinorVerNumR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
eMMC Major Version Number \\[digit left of decimal point\\],in BCD format"]
    #[inline(always)]
    pub fn emmc_major_ver_num(&self) -> EmmcMajorVerNumR {
        EmmcMajorVerNumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
eMMC Version Suffix \\[2nd digit right of decimal point\\], in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_version_suffix(
        &mut self,
    ) -> EmmcVersionSuffixW<SdhcWrap_CtlCfg_CtlcfgCqVersionSpec> {
        EmmcVersionSuffixW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
eMMC Minor Version Number \\[digit right of decimal point\\], in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_minor_ver_num(&mut self) -> EmmcMinorVerNumW<SdhcWrap_CtlCfg_CtlcfgCqVersionSpec> {
        EmmcMinorVerNumW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
eMMC Major Version Number \\[digit left of decimal point\\],in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_major_ver_num(&mut self) -> EmmcMajorVerNumW<SdhcWrap_CtlCfg_CtlcfgCqVersionSpec> {
        EmmcMajorVerNumW::new(self, 8)
    }
}
#[doc = "This register provides information about the version of the eMMC CQ standard which is 285 implemented by the CQE, in BCD format. The current version is rev 5.1 The following table describes the CQBASE+00h: CQVReservedER Command Queueing Version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqVersionSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_version::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqVersionSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_version::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqVersionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_version to value 0x0510"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqVersionSpec {
    const RESET_VALUE: u32 = 0x0510;
}
