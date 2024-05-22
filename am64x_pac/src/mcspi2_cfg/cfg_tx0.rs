#[doc = "Register `CFG_TX0` reader"]
pub type R = crate::R<CfgTx0Spec>;
#[doc = "Register `CFG_TX0` writer"]
pub type W = crate::W<CfgTx0Spec>;
#[doc = "Field `TDATA` reader - 31:0\\]
Channel 0 Data to transmit"]
pub type TdataR = crate::FieldReader<u32>;
#[doc = "Field `TDATA` writer - 31:0\\]
Channel 0 Data to transmit"]
pub type TdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 0 Data to transmit"]
    #[inline(always)]
    pub fn tdata(&self) -> TdataR {
        TdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 0 Data to transmit"]
    #[inline(always)]
    #[must_use]
    pub fn tdata(&mut self) -> TdataW<CfgTx0Spec> {
        TdataW::new(self, 0)
    }
}
#[doc = "This register contains a single SPI word to transmit on the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTx0Spec;
impl crate::RegisterSpec for CfgTx0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tx0::R`](R) reader structure"]
impl crate::Readable for CfgTx0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx0::W`](W) writer structure"]
impl crate::Writable for CfgTx0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TX0 to value 0"]
impl crate::Resettable for CfgTx0Spec {
    const RESET_VALUE: u32 = 0;
}
