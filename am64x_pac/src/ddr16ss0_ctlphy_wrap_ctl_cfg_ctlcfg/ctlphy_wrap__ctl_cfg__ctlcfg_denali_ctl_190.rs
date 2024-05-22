#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_190` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_190` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec>;
#[doc = "Field `PERIPHERAL_MRR_DATA` reader - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits \\[7:0\\]
indicate the read data and bits \\[15:8\\]
indicate the chip. READ-ONLY"]
pub type PeripheralMrrDataR = crate::FieldReader<u32>;
#[doc = "Field `PERIPHERAL_MRR_DATA` writer - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits \\[7:0\\]
indicate the read data and bits \\[15:8\\]
indicate the chip. READ-ONLY"]
pub type PeripheralMrrDataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `AUTO_TEMPCHK_VAL_0` reader - 31:24\\]
MR4 data for all devices accessed by automatic MRR commands. Bits \\[3:0\\]
correlate to the device on the lower byte, bits \\[7:4\\]
correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits. READ-ONLY."]
pub type AutoTempchkVal0R = crate::FieldReader;
#[doc = "Field `AUTO_TEMPCHK_VAL_0` writer - 31:24\\]
MR4 data for all devices accessed by automatic MRR commands. Bits \\[3:0\\]
correlate to the device on the lower byte, bits \\[7:4\\]
correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits. READ-ONLY."]
pub type AutoTempchkVal0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits \\[7:0\\]
indicate the read data and bits \\[15:8\\]
indicate the chip. READ-ONLY"]
    #[inline(always)]
    pub fn peripheral_mrr_data(&self) -> PeripheralMrrDataR {
        PeripheralMrrDataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
MR4 data for all devices accessed by automatic MRR commands. Bits \\[3:0\\]
correlate to the device on the lower byte, bits \\[7:4\\]
correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits. READ-ONLY."]
    #[inline(always)]
    pub fn auto_tempchk_val_0(&self) -> AutoTempchkVal0R {
        AutoTempchkVal0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter. Bits \\[7:0\\]
indicate the read data and bits \\[15:8\\]
indicate the chip. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn peripheral_mrr_data(
        &mut self,
    ) -> PeripheralMrrDataW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec> {
        PeripheralMrrDataW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
MR4 data for all devices accessed by automatic MRR commands. Bits \\[3:0\\]
correlate to the device on the lower byte, bits \\[7:4\\]
correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn auto_tempchk_val_0(
        &mut self,
    ) -> AutoTempchkVal0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec> {
        AutoTempchkVal0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_190\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_190::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_190::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_190::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_190::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_190 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl190Spec {
    const RESET_VALUE: u32 = 0;
}
