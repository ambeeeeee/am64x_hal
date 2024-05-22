#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY` reader"]
pub type R = crate::R<Cfg0McuM4fss0LbistPatcountProxySpec>;
#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY` writer"]
pub type W = crate::W<Cfg0McuM4fss0LbistPatcountProxySpec>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SCAN_PC_DEF_PROXY` reader - 3:0\\]
Number of chain test patterns to run"]
pub type McuM4fss0LbistPatcountScanPcDefProxyR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SCAN_PC_DEF_PROXY` writer - 3:0\\]
Number of chain test patterns to run"]
pub type McuM4fss0LbistPatcountScanPcDefProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_RESET_PC_DEF_PROXY` reader - 7:4\\]
Number of reset patterns to run"]
pub type McuM4fss0LbistPatcountResetPcDefProxyR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_RESET_PC_DEF_PROXY` writer - 7:4\\]
Number of reset patterns to run"]
pub type McuM4fss0LbistPatcountResetPcDefProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SET_PC_DEF_PROXY` reader - 11:8\\]
Number of set patterns to run"]
pub type McuM4fss0LbistPatcountSetPcDefProxyR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SET_PC_DEF_PROXY` writer - 11:8\\]
Number of set patterns to run"]
pub type McuM4fss0LbistPatcountSetPcDefProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_STATIC_PC_DEF_PROXY` reader - 29:16\\]
Number of stuck-at patterns to run"]
pub type McuM4fss0LbistPatcountStaticPcDefProxyR = crate::FieldReader<u16>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_STATIC_PC_DEF_PROXY` writer - 29:16\\]
Number of stuck-at patterns to run"]
pub type McuM4fss0LbistPatcountStaticPcDefProxyW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of chain test patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_scan_pc_def_proxy(
        &self,
    ) -> McuM4fss0LbistPatcountScanPcDefProxyR {
        McuM4fss0LbistPatcountScanPcDefProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of reset patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_reset_pc_def_proxy(
        &self,
    ) -> McuM4fss0LbistPatcountResetPcDefProxyR {
        McuM4fss0LbistPatcountResetPcDefProxyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of set patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_set_pc_def_proxy(
        &self,
    ) -> McuM4fss0LbistPatcountSetPcDefProxyR {
        McuM4fss0LbistPatcountSetPcDefProxyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Number of stuck-at patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_static_pc_def_proxy(
        &self,
    ) -> McuM4fss0LbistPatcountStaticPcDefProxyR {
        McuM4fss0LbistPatcountStaticPcDefProxyR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of chain test patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_scan_pc_def_proxy(
        &mut self,
    ) -> McuM4fss0LbistPatcountScanPcDefProxyW<Cfg0McuM4fss0LbistPatcountProxySpec> {
        McuM4fss0LbistPatcountScanPcDefProxyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of reset patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_reset_pc_def_proxy(
        &mut self,
    ) -> McuM4fss0LbistPatcountResetPcDefProxyW<Cfg0McuM4fss0LbistPatcountProxySpec> {
        McuM4fss0LbistPatcountResetPcDefProxyW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of set patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_set_pc_def_proxy(
        &mut self,
    ) -> McuM4fss0LbistPatcountSetPcDefProxyW<Cfg0McuM4fss0LbistPatcountProxySpec> {
        McuM4fss0LbistPatcountSetPcDefProxyW::new(self, 8)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Number of stuck-at patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_static_pc_def_proxy(
        &mut self,
    ) -> McuM4fss0LbistPatcountStaticPcDefProxyW<Cfg0McuM4fss0LbistPatcountProxySpec> {
        McuM4fss0LbistPatcountStaticPcDefProxyW::new(self, 16)
    }
}
#[doc = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_patcount_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_patcount_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fss0LbistPatcountProxySpec;
impl crate::RegisterSpec for Cfg0McuM4fss0LbistPatcountProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss0_lbist_patcount_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fss0LbistPatcountProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss0_lbist_patcount_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fss0LbistPatcountProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY to value 0"]
impl crate::Resettable for Cfg0McuM4fss0LbistPatcountProxySpec {
    const RESET_VALUE: u32 = 0;
}
