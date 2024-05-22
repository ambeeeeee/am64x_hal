#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXFQS` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTxfqsSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXFQS` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTxfqsSpec>;
#[doc = "Field `TFFL` reader - 5:0\\]
Tx FIFO Free Level"]
pub type TfflR = crate::FieldReader;
#[doc = "Field `TFFL` writer - 5:0\\]
Tx FIFO Free Level"]
pub type TfflW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFGI` reader - 12:8\\]
Tx Queue Get Index"]
pub type TfgiR = crate::FieldReader;
#[doc = "Field `TFGI` writer - 12:8\\]
Tx Queue Get Index"]
pub type TfgiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TFQPI` reader - 20:16\\]
Tx FIFO/Queue Put Index"]
pub type TfqpiR = crate::FieldReader;
#[doc = "Field `TFQPI` writer - 20:16\\]
Tx FIFO/Queue Put Index"]
pub type TfqpiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TFQF` reader - 21:21\\]
Tx FIFO/Queue Full"]
pub type TfqfR = crate::BitReader;
#[doc = "Field `TFQF` writer - 21:21\\]
Tx FIFO/Queue Full"]
pub type TfqfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TfflR {
        TfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Tx Queue Get Index"]
    #[inline(always)]
    pub fn tfgi(&self) -> TfgiR {
        TfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TfqpiR {
        TfqpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TfqfR {
        TfqfR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Tx FIFO Free Level"]
    #[inline(always)]
    #[must_use]
    pub fn tffl(&mut self) -> TfflW<McanWrap_McanCfgVbp_McanRegsTxfqsSpec> {
        TfflW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Tx Queue Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn tfgi(&mut self) -> TfgiW<McanWrap_McanCfgVbp_McanRegsTxfqsSpec> {
        TfgiW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Tx FIFO/Queue Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn tfqpi(&mut self) -> TfqpiW<McanWrap_McanCfgVbp_McanRegsTxfqsSpec> {
        TfqpiW::new(self, 16)
    }
    #[doc = "Bit 21 - 21:21\\]
Tx FIFO/Queue Full"]
    #[inline(always)]
    #[must_use]
    pub fn tfqf(&mut self) -> TfqfW<McanWrap_McanCfgVbp_McanRegsTxfqsSpec> {
        TfqfW::new(self, 21)
    }
}
#[doc = "Tx FIFO/Queue full indication and put index, Tx FIFO get index and fill level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTxfqsSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTxfqsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txfqs::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTxfqsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXFQS to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTxfqsSpec {
    const RESET_VALUE: u32 = 0;
}
