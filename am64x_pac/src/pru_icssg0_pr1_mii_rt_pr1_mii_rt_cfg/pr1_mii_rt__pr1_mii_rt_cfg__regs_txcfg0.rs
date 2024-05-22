#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec>;
#[doc = "Field `TX_ENABLE0` reader - "]
pub type TxEnable0R = crate::BitReader;
#[doc = "Field `TX_ENABLE0` writer - "]
pub type TxEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_AUTO_PREAMBLE0` reader - "]
pub type TxAutoPreamble0R = crate::BitReader;
#[doc = "Field `TX_AUTO_PREAMBLE0` writer - "]
pub type TxAutoPreamble0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EN_MODE0` reader - "]
pub type TxEnMode0R = crate::BitReader;
#[doc = "Field `TX_EN_MODE0` writer - "]
pub type TxEnMode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BYTE_SWAP0` reader - "]
pub type TxByteSwap0R = crate::BitReader;
#[doc = "Field `TX_BYTE_SWAP0` writer - "]
pub type TxByteSwap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MUX_SEL0` reader - "]
pub type TxMuxSel0R = crate::BitReader;
#[doc = "Field `TX_MUX_SEL0` writer - "]
pub type TxMuxSel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_TX_AUTO_SEQUENCE0` reader - "]
pub type PreTxAutoSequence0R = crate::BitReader;
#[doc = "Field `PRE_TX_AUTO_SEQUENCE0` writer - "]
pub type PreTxAutoSequence0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_TX_AUTO_ESC_ERR0` reader - "]
pub type PreTxAutoEscErr0R = crate::BitReader;
#[doc = "Field `PRE_TX_AUTO_ESC_ERR0` writer - "]
pub type PreTxAutoEscErr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_32_MODE_EN0` reader - "]
pub type Tx32ModeEn0R = crate::BitReader;
#[doc = "Field `TX_32_MODE_EN0` writer - "]
pub type Tx32ModeEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_IPG_WIRE_CLK_EN0` reader - "]
pub type TxIpgWireClkEn0R = crate::BitReader;
#[doc = "Field `TX_IPG_WIRE_CLK_EN0` writer - "]
pub type TxIpgWireClkEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START_DELAY0` reader - "]
pub type TxStartDelay0R = crate::FieldReader<u16>;
#[doc = "Field `TX_START_DELAY0` writer - "]
pub type TxStartDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_CLK_DELAY0` reader - "]
pub type TxClkDelay0R = crate::FieldReader;
#[doc = "Field `TX_CLK_DELAY0` writer - "]
pub type TxClkDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_enable0(&self) -> TxEnable0R {
        TxEnable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_auto_preamble0(&self) -> TxAutoPreamble0R {
        TxAutoPreamble0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_en_mode0(&self) -> TxEnMode0R {
        TxEnMode0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_byte_swap0(&self) -> TxByteSwap0R {
        TxByteSwap0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_mux_sel0(&self) -> TxMuxSel0R {
        TxMuxSel0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pre_tx_auto_sequence0(&self) -> PreTxAutoSequence0R {
        PreTxAutoSequence0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pre_tx_auto_esc_err0(&self) -> PreTxAutoEscErr0R {
        PreTxAutoEscErr0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_32_mode_en0(&self) -> Tx32ModeEn0R {
        Tx32ModeEn0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_ipg_wire_clk_en0(&self) -> TxIpgWireClkEn0R {
        TxIpgWireClkEn0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tx_start_delay0(&self) -> TxStartDelay0R {
        TxStartDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn tx_clk_delay0(&self) -> TxClkDelay0R {
        TxClkDelay0R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_enable0(&mut self) -> TxEnable0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxEnable0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_auto_preamble0(&mut self) -> TxAutoPreamble0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxAutoPreamble0W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_en_mode0(&mut self) -> TxEnMode0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxEnMode0W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_swap0(&mut self) -> TxByteSwap0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxByteSwap0W::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mux_sel0(&mut self) -> TxMuxSel0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxMuxSel0W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pre_tx_auto_sequence0(
        &mut self,
    ) -> PreTxAutoSequence0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        PreTxAutoSequence0W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pre_tx_auto_esc_err0(
        &mut self,
    ) -> PreTxAutoEscErr0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        PreTxAutoEscErr0W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tx_32_mode_en0(&mut self) -> Tx32ModeEn0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        Tx32ModeEn0W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ipg_wire_clk_en0(&mut self) -> TxIpgWireClkEn0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxIpgWireClkEn0W::new(self, 12)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_delay0(&mut self) -> TxStartDelay0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxStartDelay0W::new(self, 16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_delay0(&mut self) -> TxClkDelay0W<Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec> {
        TxClkDelay0W::new(self, 28)
    }
}
#[doc = "MIITXCFG0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg0 to value 0x0064_0100"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec {
    const RESET_VALUE: u32 = 0x0064_0100;
}
