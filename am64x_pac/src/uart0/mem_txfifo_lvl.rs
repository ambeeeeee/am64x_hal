#[doc = "Register `MEM_TXFIFO_LVL` reader"]
pub type R = crate::R<MemTxfifoLvlSpec>;
#[doc = "Register `MEM_TXFIFO_LVL` writer"]
pub type W = crate::W<MemTxfifoLvlSpec>;
#[doc = "Field `TXFIFO_LVL` reader - "]
pub type TxfifoLvlR = crate::FieldReader;
#[doc = "Field `TXFIFO_LVL` writer - "]
pub type TxfifoLvlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED24` reader - "]
pub type Reserved24R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED24` writer - "]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txfifo_lvl(&self) -> TxfifoLvlR {
        TxfifoLvlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_lvl(&mut self) -> TxfifoLvlW<MemTxfifoLvlSpec> {
        TxfifoLvlW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<MemTxfifoLvlSpec> {
        Reserved24W::new(self, 8)
    }
}
#[doc = "Level of the TX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_txfifo_lvl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_txfifo_lvl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTxfifoLvlSpec;
impl crate::RegisterSpec for MemTxfifoLvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_txfifo_lvl::R`](R) reader structure"]
impl crate::Readable for MemTxfifoLvlSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_txfifo_lvl::W`](W) writer structure"]
impl crate::Writable for MemTxfifoLvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TXFIFO_LVL to value 0"]
impl crate::Resettable for MemTxfifoLvlSpec {
    const RESET_VALUE: u32 = 0;
}
