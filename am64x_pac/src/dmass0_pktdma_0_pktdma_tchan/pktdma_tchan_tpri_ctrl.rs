#[doc = "Register `PKTDMA_TCHAN_TPRI_CTRL` reader"]
pub type R = crate::R<PktdmaTchanTpriCtrlSpec>;
#[doc = "Register `PKTDMA_TCHAN_TPRI_CTRL` writer"]
pub type W = crate::W<PktdmaTchanTpriCtrlSpec>;
#[doc = "Field `ORDERID` reader - 3:0\\]
Tx Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
pub type OrderidR = crate::FieldReader;
#[doc = "Field `ORDERID` writer - 3:0\\]
Tx Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
pub type OrderidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY` reader - 30:28\\]
Tx Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - 30:28\\]
Tx Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Tx Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
    #[inline(always)]
    pub fn orderid(&self) -> OrderidR {
        OrderidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Tx Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Tx Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
    #[inline(always)]
    #[must_use]
    pub fn orderid(&mut self) -> OrderidW<PktdmaTchanTpriCtrlSpec> {
        OrderidW::new(self, 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Tx Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<PktdmaTchanTpriCtrlSpec> {
        PriorityW::new(self, 28)
    }
}
#[doc = "The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchan_tpri_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchan_tpri_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaTchanTpriCtrlSpec;
impl crate::RegisterSpec for PktdmaTchanTpriCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_tchan_tpri_ctrl::R`](R) reader structure"]
impl crate::Readable for PktdmaTchanTpriCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_tchan_tpri_ctrl::W`](W) writer structure"]
impl crate::Writable for PktdmaTchanTpriCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_TCHAN_TPRI_CTRL to value 0"]
impl crate::Resettable for PktdmaTchanTpriCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
