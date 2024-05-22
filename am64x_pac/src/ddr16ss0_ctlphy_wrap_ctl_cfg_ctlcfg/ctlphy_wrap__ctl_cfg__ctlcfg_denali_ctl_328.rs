#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_328` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_328` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec>;
#[doc = "Field `MEMDATA_RATIO_1` reader - 2:0\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=1"]
pub type MemdataRatio1R = crate::FieldReader;
#[doc = "Field `MEMDATA_RATIO_1` writer - 2:0\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=1"]
pub type MemdataRatio1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DEVICE0_BYTE0_CS0` reader - 9:8\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 0. Used for MRRs to identify where data will be returned. DEV=0"]
pub type Device0Byte0Cs0R = crate::FieldReader;
#[doc = "Field `DEVICE0_BYTE0_CS0` writer - 9:8\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 0. Used for MRRs to identify where data will be returned. DEV=0"]
pub type Device0Byte0Cs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVICE1_BYTE0_CS0` reader - 17:16\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 0. Used for MRRs to identify where data will be returned. DEV=1"]
pub type Device1Byte0Cs0R = crate::FieldReader;
#[doc = "Field `DEVICE1_BYTE0_CS0` writer - 17:16\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 0. Used for MRRs to identify where data will be returned. DEV=1"]
pub type Device1Byte0Cs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVICE0_BYTE0_CS1` reader - 25:24\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 1. Used for MRRs to identify where data will be returned. DEV=0"]
pub type Device0Byte0Cs1R = crate::FieldReader;
#[doc = "Field `DEVICE0_BYTE0_CS1` writer - 25:24\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 1. Used for MRRs to identify where data will be returned. DEV=0"]
pub type Device0Byte0Cs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=1"]
    #[inline(always)]
    pub fn memdata_ratio_1(&self) -> MemdataRatio1R {
        MemdataRatio1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 0. Used for MRRs to identify where data will be returned. DEV=0"]
    #[inline(always)]
    pub fn device0_byte0_cs0(&self) -> Device0Byte0Cs0R {
        Device0Byte0Cs0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 0. Used for MRRs to identify where data will be returned. DEV=1"]
    #[inline(always)]
    pub fn device1_byte0_cs0(&self) -> Device1Byte0Cs0R {
        Device1Byte0Cs0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 1. Used for MRRs to identify where data will be returned. DEV=0"]
    #[inline(always)]
    pub fn device0_byte0_cs1(&self) -> Device0Byte0Cs1R {
        Device0Byte0Cs1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=1"]
    #[inline(always)]
    #[must_use]
    pub fn memdata_ratio_1(&mut self) -> MemdataRatio1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec> {
        MemdataRatio1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 0. Used for MRRs to identify where data will be returned. DEV=0"]
    #[inline(always)]
    #[must_use]
    pub fn device0_byte0_cs0(
        &mut self,
    ) -> Device0Byte0Cs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec> {
        Device0Byte0Cs0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the byte location of byte0 in the memory datapath for device 1 on chip 0. Used for MRRs to identify where data will be returned. DEV=1"]
    #[inline(always)]
    #[must_use]
    pub fn device1_byte0_cs0(
        &mut self,
    ) -> Device1Byte0Cs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec> {
        Device1Byte0Cs0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines the byte location of byte0 in the memory datapath for device 0 on chip 1. Used for MRRs to identify where data will be returned. DEV=0"]
    #[inline(always)]
    #[must_use]
    pub fn device0_byte0_cs1(
        &mut self,
    ) -> Device0Byte0Cs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec> {
        Device0Byte0Cs1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_328\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_328::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_328::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_328::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_328::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_328 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl328Spec {
    const RESET_VALUE: u32 = 0;
}
