#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_adma_err_status` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_adma_err_status` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec>;
#[doc = "Field `ADMA_ERR_STATE` reader - 1:0\\]
This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 D00 : ADMA Error State when error occurred Contents of SYS_SDR register."]
pub type AdmaErrStateR = crate::FieldReader;
#[doc = "Field `ADMA_ERR_STATE` writer - 1:0\\]
This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 D00 : ADMA Error State when error occurred Contents of SYS_SDR register."]
pub type AdmaErrStateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADMA_LENGTH_ERR` reader - 2:2\\]
This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
pub type AdmaLengthErrR = crate::BitReader;
#[doc = "Field `ADMA_LENGTH_ERR` writer - 2:2\\]
This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
pub type AdmaLengthErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 D00 : ADMA Error State when error occurred Contents of SYS_SDR register."]
    #[inline(always)]
    pub fn adma_err_state(&self) -> AdmaErrStateR {
        AdmaErrStateR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - 2:2\\]
This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
    #[inline(always)]
    pub fn adma_length_err(&self) -> AdmaLengthErrR {
        AdmaLengthErrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates 10 because ADMA never stops in this state. D01 D00 : ADMA Error State when error occurred Contents of SYS_SDR register."]
    #[inline(always)]
    #[must_use]
    pub fn adma_err_state(&mut self) -> AdmaErrStateW<SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec> {
        AdmaErrStateW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This error occurs in the following 2 cases. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
    #[inline(always)]
    #[must_use]
    pub fn adma_length_err(&mut self) -> AdmaLengthErrW<SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec> {
        AdmaLengthErrW::new(self, 2)
    }
}
#[doc = "When the ADMA Error interrupt occur, this register holds the ADMA State in ADMA Error States field and ADMA System Address holds address around the error descriptor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_adma_err_status to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec {
    const RESET_VALUE: u8 = 0;
}
