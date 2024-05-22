#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_282` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_282` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec>;
#[doc = "Field `PI_PREAMBLE_SUPPORT_F1` reader - 1:0\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
pub type PiPreambleSupportF1R = crate::FieldReader;
#[doc = "Field `PI_PREAMBLE_SUPPORT_F1` writer - 1:0\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
pub type PiPreambleSupportF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_PREAMBLE_SUPPORT_F2` reader - 9:8\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
pub type PiPreambleSupportF2R = crate::FieldReader;
#[doc = "Field `PI_PREAMBLE_SUPPORT_F2` writer - 9:8\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
pub type PiPreambleSupportF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_MEMDATA_RATIO_0` reader - 18:16\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type PiMemdataRatio0R = crate::FieldReader;
#[doc = "Field `PI_MEMDATA_RATIO_0` writer - 18:16\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type PiMemdataRatio0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_MEMDATA_RATIO_1` reader - 26:24\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type PiMemdataRatio1R = crate::FieldReader;
#[doc = "Field `PI_MEMDATA_RATIO_1` writer - 26:24\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
pub type PiMemdataRatio1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
    #[inline(always)]
    pub fn pi_preamble_support_f1(&self) -> PiPreambleSupportF1R {
        PiPreambleSupportF1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
    #[inline(always)]
    pub fn pi_preamble_support_f2(&self) -> PiPreambleSupportF2R {
        PiPreambleSupportF2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    pub fn pi_memdata_ratio_0(&self) -> PiMemdataRatio0R {
        PiMemdataRatio0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    pub fn pi_memdata_ratio_1(&self) -> PiMemdataRatio1R {
        PiMemdataRatio1R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
    #[inline(always)]
    #[must_use]
    pub fn pi_preamble_support_f1(
        &mut self,
    ) -> PiPreambleSupportF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec> {
        PiPreambleSupportF1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
    #[inline(always)]
    #[must_use]
    pub fn pi_preamble_support_f2(
        &mut self,
    ) -> PiPreambleSupportF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec> {
        PiPreambleSupportF2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    #[must_use]
    pub fn pi_memdata_ratio_0(
        &mut self,
    ) -> PiMemdataRatio0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec> {
        PiMemdataRatio0W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Defines the ratio of the DRAM device size on chip select 1 to the memory data width. Program with the log2 ratio of the memory data width to the device data width."]
    #[inline(always)]
    #[must_use]
    pub fn pi_memdata_ratio_1(
        &mut self,
    ) -> PiMemdataRatio1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec> {
        PiMemdataRatio1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_282\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_282::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_282::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_282::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_282::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_282 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi282Spec {
    const RESET_VALUE: u32 = 0;
}
