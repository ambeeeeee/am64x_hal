#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RWD` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsRwdSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RWD` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsRwdSpec>;
#[doc = "Field `WDC` reader - 7:0\\]
Watchdog Counter Value"]
pub type WdcR = crate::FieldReader;
#[doc = "Field `WDC` writer - 7:0\\]
Watchdog Counter Value"]
pub type WdcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDV` reader - 15:8\\]
Watchdog Value"]
pub type WdvR = crate::FieldReader;
#[doc = "Field `WDV` writer - 15:8\\]
Watchdog Value"]
pub type WdvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Watchdog Counter Value"]
    #[inline(always)]
    pub fn wdc(&self) -> WdcR {
        WdcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Watchdog Value"]
    #[inline(always)]
    pub fn wdv(&self) -> WdvR {
        WdvR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Watchdog Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WdcW<McanWrap_McanCfgVbp_McanRegsRwdSpec> {
        WdcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Watchdog Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WdvW<McanWrap_McanCfgVbp_McanRegsRwdSpec> {
        WdvW::new(self, 8)
    }
}
#[doc = "Monitors the READY output of the Message RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsRwdSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsRwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsRwdSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_rwd::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsRwdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_RWD to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsRwdSpec {
    const RESET_VALUE: u32 = 0;
}
