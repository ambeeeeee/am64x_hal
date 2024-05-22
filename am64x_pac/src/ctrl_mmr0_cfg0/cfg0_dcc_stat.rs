#[doc = "Register `CFG0_DCC_STAT` reader"]
pub type R = crate::R<Cfg0DccStatSpec>;
#[doc = "Register `CFG0_DCC_STAT` writer"]
pub type W = crate::W<Cfg0DccStatSpec>;
#[doc = "Field `DCC_STAT_DCC0_INTR_DONE` reader - 0:0\\]
DCC0 Done Interrupt Status"]
pub type DccStatDcc0IntrDoneR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC0_INTR_DONE` writer - 0:0\\]
DCC0 Done Interrupt Status"]
pub type DccStatDcc0IntrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC1_INTR_DONE` reader - 1:1\\]
DCC1 Done Interrupt Status"]
pub type DccStatDcc1IntrDoneR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC1_INTR_DONE` writer - 1:1\\]
DCC1 Done Interrupt Status"]
pub type DccStatDcc1IntrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC2_INTR_DONE` reader - 2:2\\]
DCC2 Done Interrupt Status"]
pub type DccStatDcc2IntrDoneR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC2_INTR_DONE` writer - 2:2\\]
DCC2 Done Interrupt Status"]
pub type DccStatDcc2IntrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC3_INTR_DONE` reader - 3:3\\]
DCC3 Done Interrupt Status"]
pub type DccStatDcc3IntrDoneR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC3_INTR_DONE` writer - 3:3\\]
DCC3 Done Interrupt Status"]
pub type DccStatDcc3IntrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC4_INTR_DONE` reader - 4:4\\]
DCC4 Done Interrupt Status"]
pub type DccStatDcc4IntrDoneR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC4_INTR_DONE` writer - 4:4\\]
DCC4 Done Interrupt Status"]
pub type DccStatDcc4IntrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_DCC5_INTR_DONE` reader - 5:5\\]
DCC5 Done Interrupt Status"]
pub type DccStatDcc5IntrDoneR = crate::BitReader;
#[doc = "Field `DCC_STAT_DCC5_INTR_DONE` writer - 5:5\\]
DCC5 Done Interrupt Status"]
pub type DccStatDcc5IntrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCC_STAT_MCU_DCC0_INTR_DONE` reader - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
pub type DccStatMcuDcc0IntrDoneR = crate::BitReader;
#[doc = "Field `DCC_STAT_MCU_DCC0_INTR_DONE` writer - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
pub type DccStatMcuDcc0IntrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DCC0 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc0_intr_done(&self) -> DccStatDcc0IntrDoneR {
        DccStatDcc0IntrDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DCC1 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc1_intr_done(&self) -> DccStatDcc1IntrDoneR {
        DccStatDcc1IntrDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DCC2 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc2_intr_done(&self) -> DccStatDcc2IntrDoneR {
        DccStatDcc2IntrDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DCC3 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc3_intr_done(&self) -> DccStatDcc3IntrDoneR {
        DccStatDcc3IntrDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
DCC4 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc4_intr_done(&self) -> DccStatDcc4IntrDoneR {
        DccStatDcc4IntrDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
DCC5 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_dcc5_intr_done(&self) -> DccStatDcc5IntrDoneR {
        DccStatDcc5IntrDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
    #[inline(always)]
    pub fn dcc_stat_mcu_dcc0_intr_done(&self) -> DccStatMcuDcc0IntrDoneR {
        DccStatMcuDcc0IntrDoneR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DCC0 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc0_intr_done(&mut self) -> DccStatDcc0IntrDoneW<Cfg0DccStatSpec> {
        DccStatDcc0IntrDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DCC1 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc1_intr_done(&mut self) -> DccStatDcc1IntrDoneW<Cfg0DccStatSpec> {
        DccStatDcc1IntrDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
DCC2 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc2_intr_done(&mut self) -> DccStatDcc2IntrDoneW<Cfg0DccStatSpec> {
        DccStatDcc2IntrDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
DCC3 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc3_intr_done(&mut self) -> DccStatDcc3IntrDoneW<Cfg0DccStatSpec> {
        DccStatDcc3IntrDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
DCC4 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc4_intr_done(&mut self) -> DccStatDcc4IntrDoneW<Cfg0DccStatSpec> {
        DccStatDcc4IntrDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
DCC5 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_dcc5_intr_done(&mut self) -> DccStatDcc5IntrDoneW<Cfg0DccStatSpec> {
        DccStatDcc5IntrDoneW::new(self, 5)
    }
    #[doc = "Bit 16 - 16:16\\]
MCU_DCC0 Done Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dcc_stat_mcu_dcc0_intr_done(&mut self) -> DccStatMcuDcc0IntrDoneW<Cfg0DccStatSpec> {
        DccStatMcuDcc0IntrDoneW::new(self, 16)
    }
}
#[doc = "CFG0_DCC_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dcc_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dcc_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DccStatSpec;
impl crate::RegisterSpec for Cfg0DccStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_dcc_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0DccStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_dcc_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0DccStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DCC_STAT to value 0"]
impl crate::Resettable for Cfg0DccStatSpec {
    const RESET_VALUE: u32 = 0;
}
