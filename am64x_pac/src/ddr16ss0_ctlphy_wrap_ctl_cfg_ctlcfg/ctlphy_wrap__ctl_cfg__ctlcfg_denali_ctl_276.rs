#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_276` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_276` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec>;
#[doc = "Field `MR23_DATA` reader - 16:0\\]
Data to program into memory mode register 23."]
pub type Mr23DataR = crate::FieldReader<u32>;
#[doc = "Field `MR23_DATA` writer - 16:0\\]
Data to program into memory mode register 23."]
pub type Mr23DataW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `MR_FSP_DATA_VALID_F0` reader - 24:24\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=0"]
pub type MrFspDataValidF0R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F0` writer - 24:24\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=0"]
pub type MrFspDataValidF0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 23."]
    #[inline(always)]
    pub fn mr23_data(&self) -> Mr23DataR {
        Mr23DataR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=0"]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f0(&self) -> MrFspDataValidF0R {
        MrFspDataValidF0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 23."]
    #[inline(always)]
    #[must_use]
    pub fn mr23_data(&mut self) -> Mr23DataW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec> {
        Mr23DataW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f0(
        &mut self,
    ) -> MrFspDataValidF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec> {
        MrFspDataValidF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_276\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_276::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_276::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_276::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_276::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_276 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl276Spec {
    const RESET_VALUE: u32 = 0;
}
