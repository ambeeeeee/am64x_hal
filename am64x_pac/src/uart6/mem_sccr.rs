#[doc = "Register `MEM_SCCR` reader"]
pub type R = crate::R<MemSccrSpec>;
#[doc = "Register `MEM_SCCR` writer"]
pub type W = crate::W<MemSccrSpec>;
#[doc = "Field `MAX_ITERATION` reader - 2:0\\]
Number of times to repeat transmitted character, if the receiver did not acknowledge. If not acknowledged after the max value is reached, the USART transmitter will set parity error, stop and not continue until it is cleared."]
pub type MaxIterationR = crate::FieldReader;
#[doc = "Field `MAX_ITERATION` writer - 2:0\\]
Number of times to repeat transmitted character, if the receiver did not acknowledge. If not acknowledged after the max value is reached, the USART transmitter will set parity error, stop and not continue until it is cleared."]
pub type MaxIterationW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INACK` reader - 6:6\\]
Inhibit NACK when receiving, even if an error is received. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
pub type InackR = crate::BitReader;
#[doc = "Field `INACK` writer - 6:6\\]
Inhibit NACK when receiving, even if an error is received. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
pub type InackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSNACK` reader - 7:7\\]
Applies Max_Iteration to receiver aswell - when maximum number of NACKs have been returned, the receiver will accept the data regardless of error. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
pub type DsnackR = crate::BitReader;
#[doc = "Field `DSNACK` writer - 7:7\\]
Applies Max_Iteration to receiver aswell - when maximum number of NACKs have been returned, the receiver will accept the data regardless of error. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
pub type DsnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of times to repeat transmitted character, if the receiver did not acknowledge. If not acknowledged after the max value is reached, the USART transmitter will set parity error, stop and not continue until it is cleared."]
    #[inline(always)]
    pub fn max_iteration(&self) -> MaxIterationR {
        MaxIterationR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Inhibit NACK when receiving, even if an error is received. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
    #[inline(always)]
    pub fn inack(&self) -> InackR {
        InackR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Applies Max_Iteration to receiver aswell - when maximum number of NACKs have been returned, the receiver will accept the data regardless of error. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
    #[inline(always)]
    pub fn dsnack(&self) -> DsnackR {
        DsnackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of times to repeat transmitted character, if the receiver did not acknowledge. If not acknowledged after the max value is reached, the USART transmitter will set parity error, stop and not continue until it is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn max_iteration(&mut self) -> MaxIterationW<MemSccrSpec> {
        MaxIterationW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Inhibit NACK when receiving, even if an error is received. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
    #[inline(always)]
    #[must_use]
    pub fn inack(&mut self) -> InackW<MemSccrSpec> {
        InackW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Applies Max_Iteration to receiver aswell - when maximum number of NACKs have been returned, the receiver will accept the data regardless of error. The data will be loaded into the receiver FIFO and PE will be set when reading it."]
    #[inline(always)]
    #[must_use]
    pub fn dsnack(&mut self) -> DsnackW<MemSccrSpec> {
        DsnackW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MemSccrSpec> {
        Reserved1W::new(self, 8)
    }
}
#[doc = "Smartcard (ISO7816) mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSccrSpec;
impl crate::RegisterSpec for MemSccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sccr::R`](R) reader structure"]
impl crate::Readable for MemSccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_sccr::W`](W) writer structure"]
impl crate::Writable for MemSccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SCCR to value 0x07"]
impl crate::Resettable for MemSccrSpec {
    const RESET_VALUE: u32 = 0x07;
}
