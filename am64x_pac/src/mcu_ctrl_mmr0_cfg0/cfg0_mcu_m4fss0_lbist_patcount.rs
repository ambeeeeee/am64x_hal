#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_PATCOUNT` reader"]
pub type R = crate::R<Cfg0McuM4fss0LbistPatcountSpec>;
#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_PATCOUNT` writer"]
pub type W = crate::W<Cfg0McuM4fss0LbistPatcountSpec>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SCAN_PC_DEF` reader - 3:0\\]
Number of chain test patterns to run"]
pub type McuM4fss0LbistPatcountScanPcDefR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SCAN_PC_DEF` writer - 3:0\\]
Number of chain test patterns to run"]
pub type McuM4fss0LbistPatcountScanPcDefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_RESET_PC_DEF` reader - 7:4\\]
Number of reset patterns to run"]
pub type McuM4fss0LbistPatcountResetPcDefR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_RESET_PC_DEF` writer - 7:4\\]
Number of reset patterns to run"]
pub type McuM4fss0LbistPatcountResetPcDefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SET_PC_DEF` reader - 11:8\\]
Number of set patterns to run"]
pub type McuM4fss0LbistPatcountSetPcDefR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_SET_PC_DEF` writer - 11:8\\]
Number of set patterns to run"]
pub type McuM4fss0LbistPatcountSetPcDefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_STATIC_PC_DEF` reader - 29:16\\]
Number of stuck-at patterns to run"]
pub type McuM4fss0LbistPatcountStaticPcDefR = crate::FieldReader<u16>;
#[doc = "Field `MCU_M4FSS0_LBIST_PATCOUNT_STATIC_PC_DEF` writer - 29:16\\]
Number of stuck-at patterns to run"]
pub type McuM4fss0LbistPatcountStaticPcDefW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of chain test patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_scan_pc_def(&self) -> McuM4fss0LbistPatcountScanPcDefR {
        McuM4fss0LbistPatcountScanPcDefR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of reset patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_reset_pc_def(&self) -> McuM4fss0LbistPatcountResetPcDefR {
        McuM4fss0LbistPatcountResetPcDefR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of set patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_set_pc_def(&self) -> McuM4fss0LbistPatcountSetPcDefR {
        McuM4fss0LbistPatcountSetPcDefR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Number of stuck-at patterns to run"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_patcount_static_pc_def(&self) -> McuM4fss0LbistPatcountStaticPcDefR {
        McuM4fss0LbistPatcountStaticPcDefR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of chain test patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_scan_pc_def(
        &mut self,
    ) -> McuM4fss0LbistPatcountScanPcDefW<Cfg0McuM4fss0LbistPatcountSpec> {
        McuM4fss0LbistPatcountScanPcDefW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of reset patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_reset_pc_def(
        &mut self,
    ) -> McuM4fss0LbistPatcountResetPcDefW<Cfg0McuM4fss0LbistPatcountSpec> {
        McuM4fss0LbistPatcountResetPcDefW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of set patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_set_pc_def(
        &mut self,
    ) -> McuM4fss0LbistPatcountSetPcDefW<Cfg0McuM4fss0LbistPatcountSpec> {
        McuM4fss0LbistPatcountSetPcDefW::new(self, 8)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Number of stuck-at patterns to run"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_patcount_static_pc_def(
        &mut self,
    ) -> McuM4fss0LbistPatcountStaticPcDefW<Cfg0McuM4fss0LbistPatcountSpec> {
        McuM4fss0LbistPatcountStaticPcDefW::new(self, 16)
    }
}
#[doc = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_patcount::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_patcount::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fss0LbistPatcountSpec;
impl crate::RegisterSpec for Cfg0McuM4fss0LbistPatcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss0_lbist_patcount::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fss0LbistPatcountSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss0_lbist_patcount::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fss0LbistPatcountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS0_LBIST_PATCOUNT to value 0"]
impl crate::Resettable for Cfg0McuM4fss0LbistPatcountSpec {
    const RESET_VALUE: u32 = 0;
}
