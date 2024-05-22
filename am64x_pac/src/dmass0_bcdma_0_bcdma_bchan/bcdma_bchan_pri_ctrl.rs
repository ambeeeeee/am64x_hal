#[doc = "Register `BCDMA_BCHAN_PRI_CTRL` reader"]
pub type R = crate::R<BcdmaBchanPriCtrlSpec>;
#[doc = "Register `BCDMA_BCHAN_PRI_CTRL` writer"]
pub type W = crate::W<BcdmaBchanPriCtrlSpec>;
#[doc = "Field `ORDERID` reader - 3:0\\]
Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
pub type OrderidR = crate::FieldReader;
#[doc = "Field `ORDERID` writer - 3:0\\]
Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
pub type OrderidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY` reader - 30:28\\]
Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - 30:28\\]
Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
    #[inline(always)]
    pub fn orderid(&self) -> OrderidR {
        OrderidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Order ID: This field contains the 4-bit value which will be output on the mem*_corderid output during all transactions for this channel."]
    #[inline(always)]
    #[must_use]
    pub fn orderid(&mut self) -> OrderidW<BcdmaBchanPriCtrlSpec> {
        OrderidW::new(self, 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Priority: This field contains the 3-bit value which will be output on the mem*_cpriority and mem_cepriority outputs during all transactions for this channel."]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<BcdmaBchanPriCtrlSpec> {
        PriorityW::new(self, 28)
    }
}
#[doc = "The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchan_pri_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchan_pri_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaBchanPriCtrlSpec;
impl crate::RegisterSpec for BcdmaBchanPriCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_bchan_pri_ctrl::R`](R) reader structure"]
impl crate::Readable for BcdmaBchanPriCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_bchan_pri_ctrl::W`](W) writer structure"]
impl crate::Writable for BcdmaBchanPriCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_BCHAN_PRI_CTRL to value 0"]
impl crate::Resettable for BcdmaBchanPriCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
