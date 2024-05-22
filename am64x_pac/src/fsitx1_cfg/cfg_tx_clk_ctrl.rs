#[doc = "Register `CFG_TX_CLK_CTRL` reader"]
pub type R = crate::R<CfgTxClkCtrlSpec>;
#[doc = "Register `CFG_TX_CLK_CTRL` writer"]
pub type W = crate::W<CfgTxClkCtrlSpec>;
#[doc = "Field `CLK_RST` reader - 0:0\\]
Clock Divider Reset bitThis bit will reset the clock counter in the clock divider. 0h \\[R/W\\]
= The clock divider is set based on the value in PRESCALE_VAL. The input clock will be divided by PRESCALE_VAL if CLK_EN is set.1h \\[R/W\\]
= The clock divider will be reset to 0 and will stay reset until software writes a 0 to this bit."]
pub type ClkRstR = crate::BitReader;
#[doc = "Field `CLK_RST` writer - 0:0\\]
Clock Divider Reset bitThis bit will reset the clock counter in the clock divider. 0h \\[R/W\\]
= The clock divider is set based on the value in PRESCALE_VAL. The input clock will be divided by PRESCALE_VAL if CLK_EN is set.1h \\[R/W\\]
= The clock divider will be reset to 0 and will stay reset until software writes a 0 to this bit."]
pub type ClkRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - 1:1\\]
Clock Divider Enable bitThis bit will enable and disable the input clock divider and start the clock to the transmitter core. 0h \\[R/W\\]
= The input clock divider is not enabled and the clock is not connected to the transmitter core.1h \\[R/W\\]
= The input clock to the transmitter core is being divided by the PRESCALE_VAL and enabled."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1:1\\]
Clock Divider Enable bitThis bit will enable and disable the input clock divider and start the clock to the transmitter core. 0h \\[R/W\\]
= The input clock divider is not enabled and the clock is not connected to the transmitter core.1h \\[R/W\\]
= The input clock to the transmitter core is being divided by the PRESCALE_VAL and enabled."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALE_VAL` reader - 9:2\\]
Clock Divider Prescale ValueThe input clock is divided by this 8-bit value and fed into the transmitter core. This divided clock is the rate at which TXCLK will operate. 0h \\[R/W\\]
= Reserved1h \\[R/W\\]
= Input clock /12h \\[R/W\\]
= Input clock /23h \\[R/W\\]
= Input clock /34h \\[R/W\\]
= Input clock /4...FFh \\[R/W\\]
= Input clock /255 TXCLKIN = Input clock / PRESCALE_VALIn FSI mode: TXCLK = TXCLKIN / 2In SPI mode: TXCLK = TXCLKIN"]
pub type PrescaleValR = crate::FieldReader;
#[doc = "Field `PRESCALE_VAL` writer - 9:2\\]
Clock Divider Prescale ValueThe input clock is divided by this 8-bit value and fed into the transmitter core. This divided clock is the rate at which TXCLK will operate. 0h \\[R/W\\]
= Reserved1h \\[R/W\\]
= Input clock /12h \\[R/W\\]
= Input clock /23h \\[R/W\\]
= Input clock /34h \\[R/W\\]
= Input clock /4...FFh \\[R/W\\]
= Input clock /255 TXCLKIN = Input clock / PRESCALE_VALIn FSI mode: TXCLK = TXCLKIN / 2In SPI mode: TXCLK = TXCLKIN"]
pub type PrescaleValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clock Divider Reset bitThis bit will reset the clock counter in the clock divider. 0h \\[R/W\\]
= The clock divider is set based on the value in PRESCALE_VAL. The input clock will be divided by PRESCALE_VAL if CLK_EN is set.1h \\[R/W\\]
= The clock divider will be reset to 0 and will stay reset until software writes a 0 to this bit."]
    #[inline(always)]
    pub fn clk_rst(&self) -> ClkRstR {
        ClkRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clock Divider Enable bitThis bit will enable and disable the input clock divider and start the clock to the transmitter core. 0h \\[R/W\\]
= The input clock divider is not enabled and the clock is not connected to the transmitter core.1h \\[R/W\\]
= The input clock to the transmitter core is being divided by the PRESCALE_VAL and enabled."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - 9:2\\]
Clock Divider Prescale ValueThe input clock is divided by this 8-bit value and fed into the transmitter core. This divided clock is the rate at which TXCLK will operate. 0h \\[R/W\\]
= Reserved1h \\[R/W\\]
= Input clock /12h \\[R/W\\]
= Input clock /23h \\[R/W\\]
= Input clock /34h \\[R/W\\]
= Input clock /4...FFh \\[R/W\\]
= Input clock /255 TXCLKIN = Input clock / PRESCALE_VALIn FSI mode: TXCLK = TXCLKIN / 2In SPI mode: TXCLK = TXCLKIN"]
    #[inline(always)]
    pub fn prescale_val(&self) -> PrescaleValR {
        PrescaleValR::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clock Divider Reset bitThis bit will reset the clock counter in the clock divider. 0h \\[R/W\\]
= The clock divider is set based on the value in PRESCALE_VAL. The input clock will be divided by PRESCALE_VAL if CLK_EN is set.1h \\[R/W\\]
= The clock divider will be reset to 0 and will stay reset until software writes a 0 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn clk_rst(&mut self) -> ClkRstW<CfgTxClkCtrlSpec> {
        ClkRstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clock Divider Enable bitThis bit will enable and disable the input clock divider and start the clock to the transmitter core. 0h \\[R/W\\]
= The input clock divider is not enabled and the clock is not connected to the transmitter core.1h \\[R/W\\]
= The input clock to the transmitter core is being divided by the PRESCALE_VAL and enabled."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<CfgTxClkCtrlSpec> {
        ClkEnW::new(self, 1)
    }
    #[doc = "Bits 2:9 - 9:2\\]
Clock Divider Prescale ValueThe input clock is divided by this 8-bit value and fed into the transmitter core. This divided clock is the rate at which TXCLK will operate. 0h \\[R/W\\]
= Reserved1h \\[R/W\\]
= Input clock /12h \\[R/W\\]
= Input clock /23h \\[R/W\\]
= Input clock /34h \\[R/W\\]
= Input clock /4...FFh \\[R/W\\]
= Input clock /255 TXCLKIN = Input clock / PRESCALE_VALIn FSI mode: TXCLK = TXCLKIN / 2In SPI mode: TXCLK = TXCLKIN"]
    #[inline(always)]
    #[must_use]
    pub fn prescale_val(&mut self) -> PrescaleValW<CfgTxClkCtrlSpec> {
        PrescaleValW::new(self, 2)
    }
}
#[doc = "Transmit clock control register. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxClkCtrlSpec;
impl crate::RegisterSpec for CfgTxClkCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgTxClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgTxClkCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_CLK_CTRL to value 0"]
impl crate::Resettable for CfgTxClkCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
