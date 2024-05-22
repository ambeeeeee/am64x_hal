#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTxbcSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTxbcSpec>;
#[doc = "Field `TBSA` reader - 15:2\\]
Tx Buffers Start Address"]
pub type TbsaR = crate::FieldReader<u16>;
#[doc = "Field `TBSA` writer - 15:2\\]
Tx Buffers Start Address"]
pub type TbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `NDTB` reader - 21:16\\]
Number of Dedicated Transmit Buffers"]
pub type NdtbR = crate::FieldReader;
#[doc = "Field `NDTB` writer - 21:16\\]
Number of Dedicated Transmit Buffers"]
pub type NdtbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQS` reader - 29:24\\]
Transmit FIFO/Queue Size"]
pub type TfqsR = crate::FieldReader;
#[doc = "Field `TFQS` writer - 29:24\\]
Transmit FIFO/Queue Size"]
pub type TfqsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQM` reader - 30:30\\]
Tx FIFO/Queue Mode"]
pub type TfqmR = crate::BitReader;
#[doc = "Field `TFQM` writer - 30:30\\]
Tx FIFO/Queue Mode"]
pub type TfqmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - 15:2\\]
Tx Buffers Start Address"]
    #[inline(always)]
    pub fn tbsa(&self) -> TbsaR {
        TbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    pub fn ndtb(&self) -> NdtbR {
        NdtbR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Transmit FIFO/Queue Size"]
    #[inline(always)]
    pub fn tfqs(&self) -> TfqsR {
        TfqsR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&self) -> TfqmR {
        TfqmR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - 15:2\\]
Tx Buffers Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn tbsa(&mut self) -> TbsaW<McanWrap_McanCfgVbp_McanRegsTxbcSpec> {
        TbsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    #[must_use]
    pub fn ndtb(&mut self) -> NdtbW<McanWrap_McanCfgVbp_McanRegsTxbcSpec> {
        NdtbW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Transmit FIFO/Queue Size"]
    #[inline(always)]
    #[must_use]
    pub fn tfqs(&mut self) -> TfqsW<McanWrap_McanCfgVbp_McanRegsTxbcSpec> {
        TfqsW::new(self, 24)
    }
    #[doc = "Bit 30 - 30:30\\]
Tx FIFO/Queue Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TfqmW<McanWrap_McanCfgVbp_McanRegsTxbcSpec> {
        TfqmW::new(self, 30)
    }
}
#[doc = "Configure Tx FIFO/Queue mode, Tx FIFO/Queue size, number of dedicated Tx buffers, Tx buffer start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTxbcSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_txbc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TXBC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTxbcSpec {
    const RESET_VALUE: u32 = 0;
}
