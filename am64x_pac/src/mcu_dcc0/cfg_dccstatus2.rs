#[doc = "Register `CFG_DCCSTATUS2` reader"]
pub type R = crate::R<CfgDccstatus2Spec>;
#[doc = "Register `CFG_DCCSTATUS2` writer"]
pub type W = crate::W<CfgDccstatus2Spec>;
#[doc = "Field `COUNT0_FIFO_EMPTY` reader - 0:0\\]
Count0 FIFO Empty. Indicates whether Count0 FIFO is empty. User, privilege, and debug mode (read): 0: Count0 FIFO is not empty 1: Count0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
pub type Count0FifoEmptyR = crate::BitReader;
#[doc = "Field `COUNT0_FIFO_EMPTY` writer - 0:0\\]
Count0 FIFO Empty. Indicates whether Count0 FIFO is empty. User, privilege, and debug mode (read): 0: Count0 FIFO is not empty 1: Count0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
pub type Count0FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID0_FIFO_EMPTY` reader - 1:1\\]
Valid0 FIFO Empty. Indicates whether Valid0 FIFO is empty. User, privilege, and debug mode (read): 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
pub type Valid0FifoEmptyR = crate::BitReader;
#[doc = "Field `VALID0_FIFO_EMPTY` writer - 1:1\\]
Valid0 FIFO Empty. Indicates whether Valid0 FIFO is empty. User, privilege, and debug mode (read): 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
pub type Valid0FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNT1_FIFO_EMPTY` reader - 2:2\\]
Count1 FIFO Empty. Indicates whether Count1 FIFO is empty. User, privilege, and debug mode (read): 0: Count1 FIFO is not empty 1: Count1 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
pub type Count1FifoEmptyR = crate::BitReader;
#[doc = "Field `COUNT1_FIFO_EMPTY` writer - 2:2\\]
Count1 FIFO Empty. Indicates whether Count1 FIFO is empty. User, privilege, and debug mode (read): 0: Count1 FIFO is not empty 1: Count1 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
pub type Count1FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNT0_FIFO_FULL` reader - 3:3\\]
Count0 FIFO Full. Indicates whether Count0 FIFO is full. User, privilege, and debug mode (read): 0: Count0 FIFO is not full 1: Count0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
pub type Count0FifoFullR = crate::BitReader;
#[doc = "Field `COUNT0_FIFO_FULL` writer - 3:3\\]
Count0 FIFO Full. Indicates whether Count0 FIFO is full. User, privilege, and debug mode (read): 0: Count0 FIFO is not full 1: Count0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
pub type Count0FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID0_FIFO_FULL` reader - 4:4\\]
Valid0 FIFO Full. Indicates whether Valid0 FIFO is full. User, privilege, and debug mode (read): 0: Valid0 FIFO is not full 1: Valid0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
pub type Valid0FifoFullR = crate::BitReader;
#[doc = "Field `VALID0_FIFO_FULL` writer - 4:4\\]
Valid0 FIFO Full. Indicates whether Valid0 FIFO is full. User, privilege, and debug mode (read): 0: Valid0 FIFO is not full 1: Valid0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
pub type Valid0FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNT1_FIFO_FULL` reader - 5:5\\]
Count1 FIFO Full. Indicates whether Count1 FIFO is full. User, privilege, and debug mode (read): 0: Count1 FIFO is not full 1: Count1 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
pub type Count1FifoFullR = crate::BitReader;
#[doc = "Field `COUNT1_FIFO_FULL` writer - 5:5\\]
Count1 FIFO Full. Indicates whether Count1 FIFO is full. User, privilege, and debug mode (read): 0: Count1 FIFO is not full 1: Count1 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
pub type Count1FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Count0 FIFO Empty. Indicates whether Count0 FIFO is empty. User, privilege, and debug mode (read): 0: Count0 FIFO is not empty 1: Count0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn count0_fifo_empty(&self) -> Count0FifoEmptyR {
        Count0FifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Valid0 FIFO Empty. Indicates whether Valid0 FIFO is empty. User, privilege, and debug mode (read): 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn valid0_fifo_empty(&self) -> Valid0FifoEmptyR {
        Valid0FifoEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Count1 FIFO Empty. Indicates whether Count1 FIFO is empty. User, privilege, and debug mode (read): 0: Count1 FIFO is not empty 1: Count1 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn count1_fifo_empty(&self) -> Count1FifoEmptyR {
        Count1FifoEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Count0 FIFO Full. Indicates whether Count0 FIFO is full. User, privilege, and debug mode (read): 0: Count0 FIFO is not full 1: Count0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn count0_fifo_full(&self) -> Count0FifoFullR {
        Count0FifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Valid0 FIFO Full. Indicates whether Valid0 FIFO is full. User, privilege, and debug mode (read): 0: Valid0 FIFO is not full 1: Valid0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn valid0_fifo_full(&self) -> Valid0FifoFullR {
        Valid0FifoFullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Count1 FIFO Full. Indicates whether Count1 FIFO is full. User, privilege, and debug mode (read): 0: Count1 FIFO is not full 1: Count1 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn count1_fifo_full(&self) -> Count1FifoFullR {
        Count1FifoFullR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Count0 FIFO Empty. Indicates whether Count0 FIFO is empty. User, privilege, and debug mode (read): 0: Count0 FIFO is not empty 1: Count0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn count0_fifo_empty(&mut self) -> Count0FifoEmptyW<CfgDccstatus2Spec> {
        Count0FifoEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Valid0 FIFO Empty. Indicates whether Valid0 FIFO is empty. User, privilege, and debug mode (read): 0: Valid0 FIFO is not empty 1: Valid0 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn valid0_fifo_empty(&mut self) -> Valid0FifoEmptyW<CfgDccstatus2Spec> {
        Valid0FifoEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Count1 FIFO Empty. Indicates whether Count1 FIFO is empty. User, privilege, and debug mode (read): 0: Count1 FIFO is not empty 1: Count1 FIFO is empty. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn count1_fifo_empty(&mut self) -> Count1FifoEmptyW<CfgDccstatus2Spec> {
        Count1FifoEmptyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Count0 FIFO Full. Indicates whether Count0 FIFO is full. User, privilege, and debug mode (read): 0: Count0 FIFO is not full 1: Count0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn count0_fifo_full(&mut self) -> Count0FifoFullW<CfgDccstatus2Spec> {
        Count0FifoFullW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Valid0 FIFO Full. Indicates whether Valid0 FIFO is full. User, privilege, and debug mode (read): 0: Valid0 FIFO is not full 1: Valid0 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn valid0_fifo_full(&mut self) -> Valid0FifoFullW<CfgDccstatus2Spec> {
        Valid0FifoFullW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Count1 FIFO Full. Indicates whether Count1 FIFO is full. User, privilege, and debug mode (read): 0: Count1 FIFO is not full 1: Count1 FIFO is full. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn count1_fifo_full(&mut self) -> Count1FifoFullW<CfgDccstatus2Spec> {
        Count1FifoFullW::new(self, 5)
    }
}
#[doc = "Specifies the status of the DCC FIFOs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccstatus2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccstatus2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccstatus2Spec;
impl crate::RegisterSpec for CfgDccstatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccstatus2::R`](R) reader structure"]
impl crate::Readable for CfgDccstatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccstatus2::W`](W) writer structure"]
impl crate::Writable for CfgDccstatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCSTATUS2 to value 0x07"]
impl crate::Resettable for CfgDccstatus2Spec {
    const RESET_VALUE: u32 = 0x07;
}
