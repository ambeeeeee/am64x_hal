#[doc = "Register `CFG_CH1STAT` reader"]
pub type R = crate::R<CfgCh1statSpec>;
#[doc = "Register `CFG_CH1STAT` writer"]
pub type W = crate::W<CfgCh1statSpec>;
#[doc = "Field `RXS` reader - 0:0\\]
Channel i Receiver Register Status"]
pub type RxsR = crate::BitReader;
#[doc = "Field `RXS` writer - 0:0\\]
Channel i Receiver Register Status"]
pub type RxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXS` reader - 1:1\\]
Channel i Transmitter Register Status"]
pub type TxsR = crate::BitReader;
#[doc = "Field `TXS` writer - 1:1\\]
Channel i Transmitter Register Status"]
pub type TxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOT` reader - 2:2\\]
Channel i End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes, Turbo mode\\]
See dedicated chapters for details"]
pub type EotR = crate::BitReader;
#[doc = "Field `EOT` writer - 2:2\\]
Channel i End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes, Turbo mode\\]
See dedicated chapters for details"]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFE` reader - 3:3\\]
Channel i FIFO Transmit Buffer Empty Status"]
pub type TxffeR = crate::BitReader;
#[doc = "Field `TXFFE` writer - 3:3\\]
Channel i FIFO Transmit Buffer Empty Status"]
pub type TxffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFF` reader - 4:4\\]
Channel i FIFO Transmit Buffer Full Status"]
pub type TxfffR = crate::BitReader;
#[doc = "Field `TXFFF` writer - 4:4\\]
Channel i FIFO Transmit Buffer Full Status"]
pub type TxfffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFE` reader - 5:5\\]
Channel i FIFO Receive Buffer Empty Status"]
pub type RxffeR = crate::BitReader;
#[doc = "Field `RXFFE` writer - 5:5\\]
Channel i FIFO Receive Buffer Empty Status"]
pub type RxffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFF` reader - 6:6\\]
Channel i FIFO Receive Buffer Full Status"]
pub type RxfffR = crate::BitReader;
#[doc = "Field `RXFFF` writer - 6:6\\]
Channel i FIFO Receive Buffer Full Status"]
pub type RxfffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel i Receiver Register Status"]
    #[inline(always)]
    pub fn rxs(&self) -> RxsR {
        RxsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel i Transmitter Register Status"]
    #[inline(always)]
    pub fn txs(&self) -> TxsR {
        TxsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel i End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes, Turbo mode\\]
See dedicated chapters for details"]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel i FIFO Transmit Buffer Empty Status"]
    #[inline(always)]
    pub fn txffe(&self) -> TxffeR {
        TxffeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel i FIFO Transmit Buffer Full Status"]
    #[inline(always)]
    pub fn txfff(&self) -> TxfffR {
        TxfffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel i FIFO Receive Buffer Empty Status"]
    #[inline(always)]
    pub fn rxffe(&self) -> RxffeR {
        RxffeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel i FIFO Receive Buffer Full Status"]
    #[inline(always)]
    pub fn rxfff(&self) -> RxfffR {
        RxfffR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel i Receiver Register Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RxsW<CfgCh1statSpec> {
        RxsW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel i Transmitter Register Status"]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TxsW<CfgCh1statSpec> {
        TxsW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel i End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes, Turbo mode\\]
See dedicated chapters for details"]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EotW<CfgCh1statSpec> {
        EotW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel i FIFO Transmit Buffer Empty Status"]
    #[inline(always)]
    #[must_use]
    pub fn txffe(&mut self) -> TxffeW<CfgCh1statSpec> {
        TxffeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel i FIFO Transmit Buffer Full Status"]
    #[inline(always)]
    #[must_use]
    pub fn txfff(&mut self) -> TxfffW<CfgCh1statSpec> {
        TxfffW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel i FIFO Receive Buffer Empty Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxffe(&mut self) -> RxffeW<CfgCh1statSpec> {
        RxffeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel i FIFO Receive Buffer Full Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxfff(&mut self) -> RxfffW<CfgCh1statSpec> {
        RxfffW::new(self, 6)
    }
}
#[doc = "This register provides status information about transmitter and receiver registers of channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch1stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch1stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCh1statSpec;
impl crate::RegisterSpec for CfgCh1statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_ch1stat::R`](R) reader structure"]
impl crate::Readable for CfgCh1statSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_ch1stat::W`](W) writer structure"]
impl crate::Writable for CfgCh1statSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CH1STAT to value 0"]
impl crate::Resettable for CfgCh1statSpec {
    const RESET_VALUE: u32 = 0;
}
