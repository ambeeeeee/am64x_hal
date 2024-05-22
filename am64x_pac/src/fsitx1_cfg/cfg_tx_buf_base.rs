#[doc = "Register `CFG_TX_BUF_BASE` reader"]
pub type R = crate::R<CfgTxBufBaseSpec>;
#[doc = "Register `CFG_TX_BUF_BASE` writer"]
pub type W = crate::W<CfgTxBufBaseSpec>;
#[doc = "Field `BASE_ADDRESS` reader - 15:0\\]
Transmit Data Buffer Base AddressThis is the base address of the 16-word data buffer used by the transmitter."]
pub type BaseAddressR = crate::FieldReader<u16>;
#[doc = "Field `BASE_ADDRESS` writer - 15:0\\]
Transmit Data Buffer Base AddressThis is the base address of the 16-word data buffer used by the transmitter."]
pub type BaseAddressW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit Data Buffer Base AddressThis is the base address of the 16-word data buffer used by the transmitter."]
    #[inline(always)]
    pub fn base_address(&self) -> BaseAddressR {
        BaseAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit Data Buffer Base AddressThis is the base address of the 16-word data buffer used by the transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn base_address(&mut self) -> BaseAddressW<CfgTxBufBaseSpec> {
        BaseAddressW::new(self, 0)
    }
}
#[doc = "Base address for transmit buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_buf_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_buf_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxBufBaseSpec;
impl crate::RegisterSpec for CfgTxBufBaseSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_buf_base::R`](R) reader structure"]
impl crate::Readable for CfgTxBufBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_buf_base::W`](W) writer structure"]
impl crate::Writable for CfgTxBufBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_BUF_BASE to value 0"]
impl crate::Resettable for CfgTxBufBaseSpec {
    const RESET_VALUE: u16 = 0;
}
