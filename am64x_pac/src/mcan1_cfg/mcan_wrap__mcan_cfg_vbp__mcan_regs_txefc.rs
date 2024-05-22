#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTxefcSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTxefcSpec>;
#[doc = "Field `EFSA` reader - 15:2\\]
Event FIFO Start Address"]
pub type EfsaR = crate::FieldReader<u16>;
#[doc = "Field `EFSA` writer - 15:2\\]
Event FIFO Start Address"]
pub type EfsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `EFS` reader - 21:16\\]
Event FIFO Size"]
pub type EfsR = crate::FieldReader;
#[doc = "Field `EFS` writer - 21:16\\]
Event FIFO Size"]
pub type EfsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFWM` reader - 29:24\\]
Event FIFO Watermark"]
pub type EfwmR = crate::FieldReader;
#[doc = "Field `EFWM` writer - 29:24\\]
Event FIFO Watermark"]
pub type EfwmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 2:15 - 15:2\\]
Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&self) -> EfsaR {
        EfsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&self) -> EfsR {
        EfsR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&self) -> EfwmR {
        EfwmR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - 15:2\\]
Event FIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn efsa(&mut self) -> EfsaW<McanWrap_McanCfgVbp_McanRegsTxefcSpec> {
        EfsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Event FIFO Size"]
    #[inline(always)]
    #[must_use]
    pub fn efs(&mut self) -> EfsW<McanWrap_McanCfgVbp_McanRegsTxefcSpec> {
        EfsW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Event FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn efwm(&mut self) -> EfwmW<McanWrap_McanCfgVbp_McanRegsTxefcSpec> {
        EfwmW::new(self, 24)
    }
}
#[doc = "Tx event FIFO watermark, size and start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTxefcSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTxefcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTxefcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTxefcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTxefcSpec {
    const RESET_VALUE: u32 = 0;
}
