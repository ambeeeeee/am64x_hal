#[doc = "Register `CFG0_DCC_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0DccStatProxySpec>;
#[doc = "Register `CFG0_DCC_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0DccStatProxySpec>;
#[doc = "Field `DCC_STAT_DCC0_INTR_DONE_PROXY` reader - 0:0\\]
DCC0 Done Interrupt Status"]
pub type DccStatDcc0IntrDoneProxyR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC0_INTR_DONE_PROXY` writer - 0:0\\]
DCC0 Done Interrupt Status"]
pub type DccStatDcc0IntrDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC1_INTR_DONE_PROXY` reader - 1:1\\]
DCC1 Done Interrupt Status"]
pub type DccStatDcc1IntrDoneProxyR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC1_INTR_DONE_PROXY` writer - 1:1\\]
DCC1 Done Interrupt Status"]
pub type DccStatDcc1IntrDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC2_INTR_DONE_PROXY` reader - 2:2\\]
DCC2 Done Interrupt Status"]
pub type DccStatDcc2IntrDoneProxyR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC2_INTR_DONE_PROXY` writer - 2:2\\]
DCC2 Done Interrupt Status"]
pub type DccStatDcc2IntrDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC3_INTR_DONE_PROXY` reader - 3:3\\]
DCC3 Done Interrupt Status"]
pub type DccStatDcc3IntrDoneProxyR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC3_INTR_DONE_PROXY` writer - 3:3\\]
DCC3 Done Interrupt Status"]
pub type DccStatDcc3IntrDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC4_INTR_DONE_PROXY` reader - 4:4\\]
DCC4 Done Interrupt Status"]
pub type DccStatDcc4IntrDoneProxyR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC4_INTR_DONE_PROXY` writer - 4:4\\]
DCC4 Done Interrupt Status"]
pub type DccStatDcc4IntrDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC5_INTR_DONE_PROXY` reader - 5:5\\]
DCC5 Done Interrupt Status"]
pub type DccStatDcc5IntrDoneProxyR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC5_INTR_DONE_PROXY` writer - 5:5\\]
DCC5 Done Interrupt Status"]
pub type DccStatDcc5IntrDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_MCU_DCC0_INTR_DONE_PROXY` reader - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
pub type DccStatMcuDcc0IntrDoneProxyR = crate::BitReader;
#[doc = "Field `DCC_STAT_MCU_DCC0_INTR_DONE_PROXY` writer - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
pub type DccStatMcuDcc0IntrDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DCC0 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc0_intr_done_proxy(&self) -> DccStatDcc0IntrDoneProxyR {
        DccStatDcc0IntrDoneProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DCC1 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc1_intr_done_proxy(&self) -> DccStatDcc1IntrDoneProxyR {
        DccStatDcc1IntrDoneProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DCC2 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc2_intr_done_proxy(&self) -> DccStatDcc2IntrDoneProxyR {
        DccStatDcc2IntrDoneProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DCC3 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc3_intr_done_proxy(&self) -> DccStatDcc3IntrDoneProxyR {
        DccStatDcc3IntrDoneProxyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
DCC4 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc4_intr_done_proxy(&self) -> DccStatDcc4IntrDoneProxyR {
        DccStatDcc4IntrDoneProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
DCC5 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc5_intr_done_proxy(&self) -> DccStatDcc5IntrDoneProxyR {
        DccStatDcc5IntrDoneProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_mcu_dcc0_intr_done_proxy(&self) -> DccStatMcuDcc0IntrDoneProxyR {
        DccStatMcuDcc0IntrDoneProxyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DCC0 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc0_intr_done_proxy(
        &mut self,
    ) -> DccStatDcc0IntrDoneProxyW<Cfg0DccStatProxySpec> {
        DccStatDcc0IntrDoneProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DCC1 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc1_intr_done_proxy(
        &mut self,
    ) -> DccStatDcc1IntrDoneProxyW<Cfg0DccStatProxySpec> {
        DccStatDcc1IntrDoneProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
DCC2 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc2_intr_done_proxy(
        &mut self,
    ) -> DccStatDcc2IntrDoneProxyW<Cfg0DccStatProxySpec> {
        DccStatDcc2IntrDoneProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
DCC3 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc3_intr_done_proxy(
        &mut self,
    ) -> DccStatDcc3IntrDoneProxyW<Cfg0DccStatProxySpec> {
        DccStatDcc3IntrDoneProxyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
DCC4 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc4_intr_done_proxy(
        &mut self,
    ) -> DccStatDcc4IntrDoneProxyW<Cfg0DccStatProxySpec> {
        DccStatDcc4IntrDoneProxyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
DCC5 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc5_intr_done_proxy(
        &mut self,
    ) -> DccStatDcc5IntrDoneProxyW<Cfg0DccStatProxySpec> {
        DccStatDcc5IntrDoneProxyW::new(self, 5)
    }
    #[doc = "Bit 16 - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_mcu_dcc0_intr_done_proxy(
        &mut self,
    ) -> DccStatMcuDcc0IntrDoneProxyW<Cfg0DccStatProxySpec> {
        DccStatMcuDcc0IntrDoneProxyW::new(self, 16)
    }
}
#[doc = "CFG0_DCC_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dcc_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dcc_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DccStatProxySpec;
impl crate::RegisterSpec for Cfg0DccStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_dcc_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0DccStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_dcc_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0DccStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DCC_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0DccStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
