#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFS` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTxefsSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFS` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTxefsSpec>;
#[doc = "Field `EFFL` reader - 5:0\\]
Event FIFO Fill Level"]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFFL` writer - 5:0\\]
Event FIFO Fill Level"]
pub type EfflW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFGI` reader - 12:8\\]
Event FIFO Get Index"]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFGI` writer - 12:8\\]
Event FIFO Get Index"]
pub type EfgiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFPI` reader - 20:16\\]
Event FIFO Put Index"]
pub type EfpiR = crate::FieldReader;
#[doc = "Field `EFPI` writer - 20:16\\]
Event FIFO Put Index"]
pub type EfpiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFF` reader - 24:24\\]
Event FIFO Full"]
pub type EffR = crate::BitReader;
#[doc = "Field `EFF` writer - 24:24\\]
Event FIFO Full"]
pub type EffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - 25:25\\]
Tx Event FIFO Element Lost"]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFL` writer - 25:25\\]
Tx Event FIFO Element Lost"]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Event FIFO Get Index"]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Event FIFO Put Index"]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Event FIFO Full"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Event FIFO Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn effl(&mut self) -> EfflW<McanWrap_McanCfgVbp_McanRegsTxefsSpec> {
        EfflW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Event FIFO Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn efgi(&mut self) -> EfgiW<McanWrap_McanCfgVbp_McanRegsTxefsSpec> {
        EfgiW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Event FIFO Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn efpi(&mut self) -> EfpiW<McanWrap_McanCfgVbp_McanRegsTxefsSpec> {
        EfpiW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Event FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn eff(&mut self) -> EffW<McanWrap_McanCfgVbp_McanRegsTxefsSpec> {
        EffW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Tx Event FIFO Element Lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TeflW<McanWrap_McanCfgVbp_McanRegsTxefsSpec> {
        TeflW::new(self, 25)
    }
}
#[doc = "Tx event FIFO element lost/full indication, put index, get index, and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTxefsSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTxefsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txefs::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTxefsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXEFS to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTxefsSpec {
    const RESET_VALUE: u32 = 0;
}
